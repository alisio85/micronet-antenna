use std::collections::{HashMap, VecDeque};
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};
use micronet_antenna_core::{Decision, Message, NodeId, Proposal, ProposalId, Runtime};
use rand::Rng;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

struct Node {
    id: NodeId,
    rt: Runtime,
    auto_accept: bool,
}

struct App {
    nodes: Vec<Node>,
    queue: VecDeque<(usize, usize, Message)>,
    log: VecDeque<String>,
    selected: usize,
    tick: u64,
    partitioned: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(7);

    let mut last = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = Duration::from_millis(50);
        if event::poll(timeout)? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if app.selected + 1 < app.nodes.len() {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Char('p') => app.submit_random_proposal(),
                    KeyCode::Char('h') => app.broadcast_heartbeat(),
                    KeyCode::Char('v') => app.toggle_policy(app.selected),
                    KeyCode::Char('x') => app.toggle_partition(),
                    _ => {}
                }
            }
        }

        if last.elapsed() >= Duration::from_millis(120) {
            last = Instant::now();
            app.step();
        }
    }

    Ok(())
}

impl App {
    fn new(n: usize) -> Self {
        let mut nodes = Vec::new();
        for i in 0..n {
            let mut rng = rand::thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill(&mut bytes);
            let id = NodeId::new(bytes);
            let rt = Runtime::new(id);
            nodes.push(Node {
                id,
                rt,
                auto_accept: i % 2 == 0,
            });
        }

        let mut app = Self {
            nodes,
            queue: VecDeque::new(),
            log: VecDeque::new(),
            selected: 0,
            tick: 0,
            partitioned: false,
        };

        app.bootstrap_gossip();
        app
    }

    fn push_log(&mut self, s: impl Into<String>) {
        self.log.push_front(s.into());
        while self.log.len() > 200 {
            self.log.pop_back();
        }
    }

    fn bootstrap_gossip(&mut self) {
        for i in 0..self.nodes.len() {
            let id = self.nodes[i].id;
            for j in 0..self.nodes.len() {
                if i != j {
                    self.enqueue(i, j, Message::Hello { node: id });
                }
            }
        }
        self.push_log("boot: peer discovery (Hello flood)");
    }

    fn enqueue(&mut self, from: usize, to: usize, msg: Message) {
        self.queue.push_back((from, to, msg));
    }

    fn can_deliver(&self, from: usize, to: usize) -> bool {
        if !self.partitioned {
            return true;
        }
        let split = (self.nodes.len() / 2).max(1);
        let a = from < split;
        let b = to < split;
        a == b
    }

    fn step(&mut self) {
        self.tick = self.tick.saturating_add(1);

        let deliver = (self.nodes.len() * 2).max(1);
        for _ in 0..deliver {
            let Some((from, to, msg)) = self.queue.pop_front() else {
                break;
            };

            if !self.can_deliver(from, to) {
                self.push_log(format!("net: DROP n{} -> n{} (partition)", from, to));
                continue;
            }

            let events = self.nodes[to].rt.apply(msg);
            for e in events {
                self.push_log(format!("n{}: {:?}", to, e));
            }
        }

        if self.tick.is_multiple_of(10) {
            self.broadcast_heartbeat();
        }

        if self.tick.is_multiple_of(3) {
            self.auto_vote();
        }
    }

    fn broadcast_heartbeat(&mut self) {
        for i in 0..self.nodes.len() {
            let id = self.nodes[i].id;
            for j in 0..self.nodes.len() {
                if i != j {
                    self.enqueue(i, j, Message::Heartbeat { node: id });
                }
            }
        }
        self.push_log("net: heartbeat broadcast");
    }

    fn submit_random_proposal(&mut self) {
        let mut rng = rand::thread_rng();
        let proposer = rng.gen_range(0..self.nodes.len());

        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        let pid = ProposalId::new(bytes);
        let p = Proposal::with_id(pid, "enable_feature", b"spectacular_syscall".to_vec());

        for i in 0..self.nodes.len() {
            self.enqueue(proposer, i, Message::Proposal(p.clone()));
        }

        self.push_log(format!("law: proposal {} by n{}", hex32(pid.0), proposer));
    }

    fn auto_vote(&mut self) {
        let mut undecided: HashMap<ProposalId, Decision> = HashMap::new();
        for n in &self.nodes {
            for (pid, _) in n.rt.state().proposals().iter() {
                let d = n.rt.state().decision(*pid).unwrap_or(Decision::Pending);
                undecided.entry(*pid).or_insert(d);
            }
        }

        for (pid, d) in undecided {
            if d != Decision::Pending {
                continue;
            }

            for i in 0..self.nodes.len() {
                let accept = self.nodes[i].auto_accept;
                let from = self.nodes[i].id;
                for j in 0..self.nodes.len() {
                    if i != j {
                        self.enqueue(
                            i,
                            j,
                            Message::Vote {
                                from,
                                vote: micronet_antenna_core::Vote {
                                    proposal_id: pid,
                                    accept,
                                },
                            },
                        );
                    }
                }
            }
        }
    }

    fn toggle_policy(&mut self, idx: usize) {
        self.nodes[idx].auto_accept = !self.nodes[idx].auto_accept;
        self.push_log(format!(
            "policy: n{} auto_accept={} (press v)",
            idx, self.nodes[idx].auto_accept
        ));
    }

    fn toggle_partition(&mut self) {
        self.partitioned = !self.partitioned;
        if self.partitioned {
            let split = (self.nodes.len() / 2).max(1);
            self.push_log(format!(
                "scenario: NETWORK PARTITION enabled (n0..n{} | n{}..n{}) (press x)",
                split.saturating_sub(1),
                split,
                self.nodes.len().saturating_sub(1)
            ));
        } else {
            self.push_log("scenario: network healed (press x)".to_string());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(10),
        ])
        .split(size);

    let partition_status = if app.partitioned {
        "PARTITIONED"
    } else {
        "HEALTHY"
    };

    let header = Paragraph::new(Line::from(vec![
        Span::styled("Micronet Live", Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::raw("q=quit  p=propose  h=heartbeat  v=toggle auto-vote  x=partition/heal"),
        Span::raw("  "),
        Span::styled(
            format!("net={partition_status}"),
            Style::default().fg(if app.partitioned {
                Color::Red
            } else {
                Color::Green
            }),
        ),
        Span::raw("  "),
        Span::raw("←/→ select node"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("controls"));

    f.render_widget(header, chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    let items: Vec<ListItem> = app
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let peers = n.rt.state().peers().len();
            let props = n.rt.state().proposals().len();
            let policy = if n.auto_accept { "ACCEPT" } else { "REJECT" };
            let s = format!("n{}  peers={}  laws={}  policy={}", i, peers, props, policy);
            ListItem::new(s)
        })
        .collect();

    let node_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("citizens"))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_stateful_widget(node_list, body[0], &mut list_state(app.selected));

    let selected = &app.nodes[app.selected];

    let mut decision_lines: Vec<Line> = Vec::new();
    for (pid, _p) in selected.rt.state().proposals().iter() {
        let d = selected
            .rt
            .state()
            .decision(*pid)
            .unwrap_or(Decision::Pending);
        decision_lines.push(Line::from(vec![
            Span::raw(hex32(pid.0)),
            Span::raw("  "),
            Span::styled(
                format!("{:?}", d),
                match d {
                    Decision::Accepted => Style::default().fg(Color::Green),
                    Decision::Rejected => Style::default().fg(Color::Red),
                    Decision::Pending => Style::default().fg(Color::Gray),
                },
            ),
        ]));
        if decision_lines.len() >= 18 {
            break;
        }
    }

    if decision_lines.is_empty() {
        decision_lines.push(Line::from(Span::raw("(no proposals yet)")));
    }

    let laws = Paragraph::new(decision_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("laws (selected node)"),
    );

    f.render_widget(laws, body[1]);

    let log_items: Vec<ListItem> = app
        .log
        .iter()
        .take(50)
        .map(|s| ListItem::new(s.clone()))
        .collect();

    let logs = List::new(log_items).block(Block::default().borders(Borders::ALL).title("events"));
    f.render_widget(logs, chunks[2]);
}

fn list_state(selected: usize) -> ratatui::widgets::ListState {
    let mut st = ratatui::widgets::ListState::default();
    st.select(Some(selected));
    st
}

fn hex32(bytes: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
