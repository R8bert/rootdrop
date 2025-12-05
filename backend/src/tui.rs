use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, BorderType, List, ListItem, Paragraph, Sparkline, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    io,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

impl LogLevel {
    fn color(&self) -> Color {
        match self {
            LogLevel::Info => Color::Cyan,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Debug => Color::Magenta,
            LogLevel::Trace => Color::Gray,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            LogLevel::Info => "ℹ",
            LogLevel::Warn => "⚠",
            LogLevel::Error => "✗",
            LogLevel::Debug => "🔍",
            LogLevel::Trace => "→",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub message: String,
    pub module: Option<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, module: Option<String>) -> Self {
        Self {
            timestamp: SystemTime::now(),
            level,
            message,
            module,
        }
    }

    fn format_time(&self) -> String {
        let duration = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        let secs = duration.as_secs();
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub struct TuiLogger {
    logs: Arc<Mutex<Vec<LogEntry>>>,
    tx: Sender<LogEntry>,
}

impl TuiLogger {
    pub fn new(tx: Sender<LogEntry>) -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
            tx,
        }
    }

    pub fn log(&self, level: LogLevel, message: String, module: Option<String>) {
        let entry = LogEntry::new(level, message, module);
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(entry.clone());
            // Keep only last 1000 logs
            let len = logs.len();
            if len > 1000 {
                logs.drain(0..len - 1000);
            }
        }
        let _ = self.tx.send(entry);
    }
}

pub struct App {
    logs: Vec<LogEntry>,
    selected_tab: usize,
    scroll_offset: usize,
    stats: Stats,
}

#[derive(Default)]
struct Stats {
    total_requests: u64,
    active_connections: u64,
    total_uploads: u64,
    total_downloads: u64,
    uptime: Duration,
}

impl App {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            selected_tab: 0,
            scroll_offset: 0,
            stats: Stats::default(),
        }
    }

    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
        let len = self.logs.len();
        if len > 1000 {
            self.logs.drain(0..len - 1000);
        }
        // Auto-scroll to bottom
        self.scroll_offset = self.logs.len().saturating_sub(1);
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 3;
    }

    pub fn previous_tab(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = 2;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset < self.logs.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }
}

pub fn run_tui(rx: Receiver<LogEntry>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let start_time = SystemTime::now();

    loop {
        // Update stats
        app.stats.uptime = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or(Duration::from_secs(0));

        // Receive new logs (non-blocking)
        while let Ok(log_entry) = rx.try_recv() {
            // Update stats based on log content
            if log_entry.message.contains("request") {
                app.stats.total_requests += 1;
            }
            if log_entry.message.contains("upload") {
                app.stats.total_uploads += 1;
            }
            if log_entry.message.contains("download") {
                app.stats.total_downloads += 1;
            }
            app.add_log(log_entry);
        }

        terminal.draw(|f| ui(f, &mut app))?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.previous_tab(),
                    KeyCode::Up => app.scroll_up(),
                    KeyCode::Down => app.scroll_down(),
                    KeyCode::PageUp => {
                        for _ in 0..10 {
                            app.scroll_up();
                        }
                    }
                    KeyCode::PageDown => {
                        for _ in 0..10 {
                            app.scroll_down();
                        }
                    }
                    KeyCode::Home => app.scroll_offset = 0,
                    KeyCode::End => {
                        app.scroll_offset = app.logs.len().saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    // Header
    render_header(f, chunks[0]);

    // Main content based on selected tab
    match app.selected_tab {
        0 => render_logs(f, chunks[1], app),
        1 => render_stats(f, chunks[1], app),
        2 => render_help(f, chunks[1]),
        _ => {}
    }

    // Footer
    render_footer(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = vec![
        Span::styled("╔═══════════════════════════════════════════════════════════════╗\n", 
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("║  ", Style::default().fg(Color::Cyan)),
        Span::styled("🚀 ", Style::default().fg(Color::Yellow)),
        Span::styled("PINGO SHARE", 
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" - ", Style::default().fg(Color::DarkGray)),
        Span::styled("Real-time Server Monitor", 
            Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)),
        Span::styled("  ║\n", Style::default().fg(Color::Cyan)),
        Span::styled("╚═══════════════════════════════════════════════════════════════╝", 
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ];

    let block = Block::default()
        .style(Style::default().bg(Color::Black));
    
    let paragraph = Paragraph::new(Line::from(title))
        .block(block)
        .alignment(Alignment::Left);
    
    f.render_widget(paragraph, area);
}

fn render_logs(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Tabs
    let tabs = Tabs::new(vec!["📋 Logs", "📊 Stats", "❓ Help"])
        .select(app.selected_tab)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Navigation ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(tabs, chunks[0]);

    // Log entries
    let visible_height = chunks[1].height.saturating_sub(2) as usize;
    let start_idx = app
        .scroll_offset
        .saturating_sub(visible_height.saturating_sub(1));
    let end_idx = (start_idx + visible_height).min(app.logs.len());

    let log_items: Vec<ListItem> = app.logs[start_idx..end_idx]
        .iter()
        .map(|entry| {
            let time_style = Style::default().fg(Color::DarkGray);
            let level_style = Style::default()
                .fg(entry.level.color())
                .add_modifier(Modifier::BOLD);
            let message_style = Style::default().fg(Color::White);

            let module_text = entry
                .module
                .as_ref()
                .map(|m| format!("[{}] ", m))
                .unwrap_or_default();

            let content = vec![
                Span::styled(format!("{} ", entry.format_time()), time_style),
                Span::styled(format!("{} ", entry.level.symbol()), level_style),
                Span::styled(module_text, Style::default().fg(Color::Magenta)),
                Span::styled(&entry.message, message_style),
            ];

            ListItem::new(Line::from(content))
        })
        .collect();

    let logs_list = List::new(log_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan))
            .title(format!(
                " Server Logs ({}/{}) ",
                app.scroll_offset + 1,
                app.logs.len()
            ))
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    );

    f.render_widget(logs_list, chunks[1]);
}

fn render_stats(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    // Tabs
    let tabs = Tabs::new(vec!["📋 Logs", "📊 Stats", "❓ Help"])
        .select(app.selected_tab)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Navigation ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(tabs, chunks[0]);

    // Stats grid
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    render_stat_box(
        f,
        stats_chunks[0],
        "⏱ Uptime",
        &format!("{:.0}s", app.stats.uptime.as_secs()),
        Color::Green,
    );
    render_stat_box(
        f,
        stats_chunks[1],
        "📨 Requests",
        &app.stats.total_requests.to_string(),
        Color::Cyan,
    );
    render_stat_box(
        f,
        stats_chunks[2],
        "⬆ Uploads",
        &app.stats.total_uploads.to_string(),
        Color::Yellow,
    );
    render_stat_box(
        f,
        stats_chunks[3],
        "⬇ Downloads",
        &app.stats.total_downloads.to_string(),
        Color::Magenta,
    );

    // Activity chart
    let activity_data: Vec<u64> = app.logs.iter().rev().take(50).map(|_| 1).collect();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Activity (last 50 events) ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .data(&activity_data)
        .style(Style::default().fg(Color::Green));

    f.render_widget(sparkline, chunks[2]);
}

fn render_stat_box(f: &mut Frame, area: Rect, title: &str, value: &str, color: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color))
        .title(title)
        .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD));

    let text = Paragraph::new(value)
        .block(block)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    f.render_widget(text, area);
}

fn render_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Tabs
    let tabs = Tabs::new(vec!["📋 Logs", "📊 Stats", "❓ Help"])
        .select(2)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Navigation ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(tabs, chunks[0]);

    let help_text = vec![
        Line::from(vec![
            Span::styled("Keyboard Shortcuts", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Tab / Shift+Tab  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  Switch between tabs"),
        ]),
        Line::from(vec![
            Span::styled("  ↑ / ↓            ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  Scroll logs up/down"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp / PgDown    ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  Scroll faster"),
        ]),
        Line::from(vec![
            Span::styled("  Home / End       ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  Jump to start/end"),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc          ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  Quit application"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Log Levels", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ℹ  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("INFO    - General information"),
        ]),
        Line::from(vec![
            Span::styled("  ⚠  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("WARN    - Warning messages"),
        ]),
        Line::from(vec![
            Span::styled("  ✗  ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("ERROR   - Error messages"),
        ]),
        Line::from(vec![
            Span::styled("  🔍 ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw("DEBUG   - Debug information"),
        ]),
        Line::from(vec![
            Span::styled("  →  ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
            Span::raw("TRACE   - Detailed traces"),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Help & Information ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, chunks[1]);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let log_count = format!("Logs: {}", app.logs.len());
    let footer_text = vec![
        Span::styled("  Press ", Style::default().fg(Color::DarkGray)),
        Span::styled("[Tab]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" to switch tabs | ", Style::default().fg(Color::DarkGray)),
        Span::styled("[↑↓]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" to scroll | ", Style::default().fg(Color::DarkGray)),
        Span::styled("[q]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::styled(" to quit  ", Style::default().fg(Color::DarkGray)),
        Span::styled(log_count, 
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(Line::from(footer_text))
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

pub fn create_logger() -> (TuiLogger, Receiver<LogEntry>) {
    let (tx, rx) = mpsc::channel();
    let logger = TuiLogger::new(tx);
    (logger, rx)
}
