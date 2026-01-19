use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;
use sysinfo::{Pid, System};

use crate::port_monitor::PortMonitor;

pub struct App {
    ports: Vec<PortInfo>,
    selected_index: usize,
    running: bool,
    monitor: PortMonitor,
}

#[derive(Clone)]
pub struct PortInfo {
    pub port: u16,
    pub process_name: String,
    pub pid: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            ports: Vec::new(),
            selected_index: 0,
            running: true,
            monitor: PortMonitor::new(),
        }
    }

    pub fn refresh_ports(&mut self) {
        self.ports.clear();
        if let Ok(all_ports) = self.get_all_listening_ports() {
            self.ports = all_ports;
        }
    }

    fn get_all_listening_ports(&self) -> anyhow::Result<Vec<PortInfo>> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        let mut ports_info = Vec::new();
        let proc_net_file = "/proc/net/tcp";
        
        if !std::path::Path::new(proc_net_file).exists() {
            return Ok(ports_info);
        }

        let file = fs::File::open(proc_net_file)?;
        let reader = BufReader::new(file);
        let mut port_pids = Vec::new();

        for line in reader.lines().skip(1) {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 3 {
                if let Ok(port) = parse_port(&parts[1]) {
                    if let Ok(state) = parts[3].parse::<u32>() {
                        if state == 10 {
                            // LISTEN state
                            if let Ok(inode) = parts[9].parse::<u64>() {
                                if let Some(pid) = find_pid_by_inode(inode) {
                                    port_pids.push((port, pid));
                                }
                            }
                        }
                    }
                }
            }
        }

        let system = System::new_all();
        for (port, pid) in port_pids {
            if let Some(process) = system.process(Pid::from_u32(pid)) {
                ports_info.push(PortInfo {
                    port,
                    process_name: process.name().to_string(),
                    pid,
                });
            }
        }

        ports_info.sort_by_key(|p| p.port);
        Ok(ports_info)
    }

    pub fn next(&mut self) {
        if !self.ports.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.ports.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.ports.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.ports.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn kill_selected(&mut self) -> anyhow::Result<()> {
        if self.selected_index < self.ports.len() {
            let port = self.ports[self.selected_index].port;
            if let Err(e) = self.monitor.kill_port(port, true) {
                eprintln!("Error killing port: {}", e);
            }
            self.refresh_ports();
        }
        Ok(())
    }
}

pub fn run_tui() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.refresh_ports();

    loop {
        terminal.draw(|f| {
            draw_ui(f, &app);
        })?;

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.running = false;
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('d') | KeyCode::Delete => {
                        let _ = app.kill_selected();
                    }
                    KeyCode::Char('r') => app.refresh_ports(),
                    _ => {}
                }
            }
        }

        app.refresh_ports();
    }

    Ok(())
}

fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    ui(f, app);
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new("
 ____   ___  ____  _____ __  __    _    _   _ 
|  _ \\ / _ \\|  _ \\|_   _|  \\/  |  / \\  | \\ | |
| |_) | | | | |_) | | | | |\\/| | / _ \\ |  \\| |
|  __/| |_| |  _ <  | | | |  | |/ ___ \\| |\\  |
|_|    \\___/|_| \\_\\ |_| |_|  |_/_/   \\_\\_| \\_|")
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // Ports list
    let ports_list: Vec<ListItem> = app
        .ports
        .iter()
        .enumerate()
        .map(|(idx, port)| {
            let content = format!(":{:<6} → {} (PID: {})", port.port, port.process_name, port.pid);
            let style = if idx == app.selected_index {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let ports_block = Block::default()
        .borders(Borders::ALL)
        .title(" Active Ports ")
        .style(Style::default().fg(Color::Green));

    if ports_list.is_empty() {
        let empty = Paragraph::new("No ports in use")
            .style(Style::default().fg(Color::Yellow))
            .block(ports_block);
        f.render_widget(empty, chunks[1]);
    } else {
        let ports_widget = List::new(ports_list).block(ports_block);
        f.render_widget(ports_widget, chunks[1]);
    }

    // Help text
    let help_text = vec![
        Line::from(vec![
            Span::styled("↑/k", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Up  |  "),
            Span::styled("↓/j", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Down  |  "),
            Span::styled("d", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Kill  |  "),
            Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Refresh  |  "),
            Span::styled("q/Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit"),
        ]),
    ];

    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(" Controls ")
        .style(Style::default().fg(Color::Magenta));

    let help_widget = Paragraph::new(help_text).block(help_block);
    f.render_widget(help_widget, chunks[2]);
}

fn parse_port(s: &str) -> anyhow::Result<u16> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        Ok(u16::from_str_radix(parts[1], 16)?)
    } else {
        Err(anyhow::anyhow!("Invalid port format"))
    }
}

fn find_pid_by_inode(inode: u64) -> Option<u32> {
    use std::fs;

    let fd_dir = "/proc";
    if let Ok(entries) = fs::read_dir(fd_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if let Ok(pid) = dir_name.to_string_lossy().parse::<u32>() {
                        let fd_path = path.join("fd");
                        if let Ok(fds) = fs::read_dir(&fd_path) {
                            for fd_entry in fds.flatten() {
                                let fd_path = fd_entry.path();
                                if let Ok(target) = fs::read_link(&fd_path) {
                                    if let Some(target_str) = target.to_str() {
                                        if target_str.contains(&format!("socket:[{}]", inode)) {
                                            return Some(pid);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
