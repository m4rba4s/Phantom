//! TUI module - Terminal User Interface (optional)
//!
//! Provides a ratatui-based dashboard for monitoring PHANTOM operations.
//! Enable with `--features tui`

#![cfg(feature = "tui")]

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

/// Dashboard state
pub struct Dashboard {
    pub active_tab: usize,
    pub proxy_stats: ProxyStats,
    pub tunnel_stats: TunnelStats,
    pub logs: Vec<String>,
}

/// Proxy statistics
#[derive(Default)]
pub struct ProxyStats {
    pub connections: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub fragments_sent: u64,
}

/// Tunnel statistics
#[derive(Default)]
pub struct TunnelStats {
    pub mode: String,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            proxy_stats: ProxyStats::default(),
            tunnel_stats: TunnelStats::default(),
            logs: Vec::new(),
        }
    }

    /// Run the TUI dashboard
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Tab => {
                        self.active_tab = (self.active_tab + 1) % 3;
                    }
                    KeyCode::Left => {
                        if self.active_tab > 0 {
                            self.active_tab -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.active_tab < 2 {
                            self.active_tab += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Header with tabs
        let titles = ["Proxy", "Tunnel", "Logs"];
        let tabs = Tabs::new(titles.iter().map(|t| Line::from(*t)).collect::<Vec<_>>())
            .block(Block::default().borders(Borders::ALL).title("PHANTOM Dashboard"))
            .select(self.active_tab)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        f.render_widget(tabs, chunks[0]);

        // Main content based on selected tab
        match self.active_tab {
            0 => self.render_proxy_tab(f, chunks[1]),
            1 => self.render_tunnel_tab(f, chunks[1]),
            2 => self.render_logs_tab(f, chunks[1]),
            _ => {}
        }

        // Footer
        let footer = Paragraph::new("Press 'q' to quit | Tab to switch views | ←/→ to navigate")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }

    fn render_proxy_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left: Statistics
        let stats_text = vec![
            Line::from(vec![
                Span::raw("Connections: "),
                Span::styled(self.proxy_stats.connections.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("Bytes Sent: "),
                Span::styled(format_bytes(self.proxy_stats.bytes_sent), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Bytes Received: "),
                Span::styled(format_bytes(self.proxy_stats.bytes_received), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Fragments Sent: "),
                Span::styled(self.proxy_stats.fragments_sent.to_string(), Style::default().fg(Color::Yellow)),
            ]),
        ];

        let stats = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Statistics"));
        f.render_widget(stats, chunks[0]);

        // Right: Activity gauge
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Activity"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(50);
        f.render_widget(gauge, chunks[1]);
    }

    fn render_tunnel_tab(&self, f: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(vec![
                Span::raw("Mode: "),
                Span::styled(&self.tunnel_stats.mode, Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::raw("Packets Sent: "),
                Span::styled(self.tunnel_stats.packets_sent.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("Packets Received: "),
                Span::styled(self.tunnel_stats.packets_received.to_string(), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Errors: "),
                Span::styled(self.tunnel_stats.errors.to_string(), Style::default().fg(Color::Red)),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Tunnel Status"));
        f.render_widget(paragraph, area);
    }

    fn render_logs_tab(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.logs
            .iter()
            .map(|log| ListItem::new(Line::from(log.as_str())))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Logs"))
            .style(Style::default().fg(Color::White));
        f.render_widget(list, area);
    }

    /// Add a log entry
    pub fn log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }
}

/// Format bytes for display
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}
