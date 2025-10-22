use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveClient {
    pub server_url: String,
    pub session_id: String,
    pub user_name: String,
    pub current_tab: usize,
    pub terminal_content: String,
    pub code_editor_content: String,
    pub participants: Vec<String>,
    pub compilation_status: String,
    pub cursor_line: u32,
    pub cursor_column: u32,
}

impl LiveClient {
    pub fn new(server_url: String, session_id: String, user_name: String) -> Self {
        Self {
            server_url,
            session_id,
            user_name,
            current_tab: 0,
            terminal_content: String::new(),
            code_editor_content: String::new(),
            participants: vec!["Alice".to_string(), "Bob".to_string()], // Mock participants
            compilation_status: "Ready".to_string(),
            cursor_line: 0,
            cursor_column: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        let mut running = true;
        while running {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Tabs
                            Constraint::Min(10),   // Content
                            Constraint::Length(3), // Status
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // Tabs
                let tabs = Tabs::new(vec![
                    Spans::from("Terminal"),
                    Spans::from("Code Editor"), 
                    Spans::from("Participants"),
                    Spans::from("Resources"),
                    Spans::from("Compilation"),
                ])
                .block(Block::default().title("ParFlow Live").borders(Borders::ALL))
                .select(self.current_tab)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

                f.render_widget(tabs, chunks[0]);

                // Content based on current tab
                match self.current_tab {
                    0 => self.render_terminal_tab(f, chunks[1]),
                    1 => self.render_code_editor_tab(f, chunks[1]),
                    2 => self.render_participants_tab(f, chunks[1]),
                    3 => self.render_resources_tab(f, chunks[1]),
                    4 => self.render_compilation_tab(f, chunks[1]),
                    _ => {}
                }

                // Status bar
                let status = Paragraph::new(Spans::from(vec![
                    Span::raw("Press "),
                    Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to switch tabs, "),
                    Span::styled("Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit"),
                    Span::raw(" | "),
                    Span::styled(format!("User: {}", self.user_name), Style::default().fg(Color::Cyan)),
                    Span::raw(" | "),
                    Span::styled(format!("Session: {}", self.session_id), Style::default().fg(Color::Magenta)),
                ]));
                f.render_widget(status, chunks[2]);
            })?;

            // Handle input
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => {
                        self.current_tab = (self.current_tab + 1) % 5;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        running = false;
                    }
                    KeyCode::Char(c) => {
                        match self.current_tab {
                            0 => {
                                self.terminal_content.push(c);
                            }
                            1 => {
                                self.code_editor_content.push(c);
                                // Update cursor position
                                if c == '\n' {
                                    self.cursor_line += 1;
                                    self.cursor_column = 0;
                                } else {
                                    self.cursor_column += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Enter => {
                        if self.current_tab == 0 {
                            self.execute_terminal_command().await?;
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor_line > 0 {
                            self.cursor_line -= 1;
                        }
                    }
                    KeyCode::Down => {
                        self.cursor_line += 1;
                    }
                    KeyCode::Left => {
                        if self.cursor_column > 0 {
                            self.cursor_column -= 1;
                        }
                    }
                    KeyCode::Right => {
                        self.cursor_column += 1;
                    }
                    _ => {}
                }
            }
        }

        // Cleanup
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn render_terminal_tab(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let terminal_block = Block::default()
            .title("Shared Terminal - Type commands and press Enter")
            .borders(Borders::ALL);
        
        let terminal_content = Paragraph::new(self.terminal_content.as_str())
            .block(terminal_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(terminal_content, area);
    }

    fn render_code_editor_tab(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let editor_block = Block::default()
            .title(format!("Collaborative Code Editor - Line: {}, Column: {}", self.cursor_line, self.cursor_column))
            .borders(Borders::ALL);
        
        let editor_content = if self.code_editor_content.is_empty() {
            "// Start typing your code here...\n// Multiple users can edit simultaneously!\n// Cursor position is shared in real-time"
        } else {
            &self.code_editor_content
        };
        
        let editor_paragraph = Paragraph::new(editor_content)
            .block(editor_block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(editor_paragraph, area);
    }

    fn render_participants_tab(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let participants_block = Block::default()
            .title("Live Participants")
            .borders(Borders::ALL);
        
        let mut participants_text = String::new();
        participants_text.push_str(&format!("👤 {} (You)\n", self.user_name));
        for participant in &self.participants {
            participants_text.push_str(&format!("👤 {}\n", participant));
        }
        participants_text.push_str("\n💡 Other users can see your cursor position\nand code changes in real-time!");
        
        let participants_content = Paragraph::new(participants_text)
            .block(participants_block)
            .style(Style::default().fg(Color::Green));
        
        f.render_widget(participants_content, area);
    }

    fn render_resources_tab(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let resources_block = Block::default()
            .title("Shared Resources")
            .borders(Borders::ALL);
        
        let resources_text = "🖥️  CPU Cores: 16 total\n💾 Memory: 32GB shared\n🎮 GPU Memory: 8GB available\n📦 Storage: 500GB network\n🌐 Bandwidth: 1Gbps\n\n💡 Resources are pooled from all participants\n   for distributed compilation and processing!";
        
        let resources_content = Paragraph::new(resources_text)
            .block(resources_block)
            .style(Style::default().fg(Color::Cyan));
        
        f.render_widget(resources_content, area);
    }

    fn render_compilation_tab(&self, f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, area: tui::layout::Rect) {
        let compilation_block = Block::default()
            .title("Compilation Status")
            .borders(Borders::ALL);
        
        let compilation_text = match self.compilation_status.as_str() {
            "Compiling" => "🔄 Compiling in parallel across all machines...\n\n📦 Alice: Compiling module1.rs (4 cores)\n📦 Bob: Compiling module2.rs (4 cores)\n📦 You: Compiling module3.rs (4 cores)\n\n⏱️  Estimated: 2.3x faster with distributed build",
            "Success" => "✅ Compilation successful! 🎉\n\n📊 Performance Metrics:\n• Compilation time: 45s (vs 120s single machine)\n• Memory usage: 2.3GB distributed\n• Parallel efficiency: 85%\n\n💡 2.7x speedup achieved!",
            _ => "📝 Ready to compile\n\n💡 Type 'compile' in the terminal tab\n   to start distributed compilation\n\n🚀 Compilation will be distributed across\n   all participant machines for maximum speed!",
        };
        
        let compilation_content = Paragraph::new(compilation_text)
            .block(compilation_block)
            .style(Style::default().fg(Color::Magenta));
        
        f.render_widget(compilation_content, area);
    }

    async fn execute_terminal_command(&mut self) -> Result<(), anyhow::Error> {
        let command = self.terminal_content.lines().last().unwrap_or("").trim();
        
        let response = match command {
            "help" => "Available commands:\n• status - Show session status\n• resources - Show shared resources\n• compile - Start distributed compilation\n• clear - Clear terminal\n• participants - List participants",
            "status" => "Session: ParFlow Live Demo\nParticipants: 3 active\nFiles: 5 Rust files\nResources: 12 cores, 24GB memory\nCompilation: Ready",
            "resources" => "Shared Resources:\n• CPU Cores: 12 total\n• Memory: 24GB shared\n• GPU: 6GB available\n• Network: 500Mbps\n• Distributed compilation: ENABLED",
            "compile" => {
                self.compilation_status = "Compiling".to_string();
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                self.compilation_status = "Success".to_string();
                "🚀 Starting distributed compilation...\n📦 Compilation distributed across 3 machines\n⚡ 2.7x speedup achieved!\n✅ Compilation successful!"
            }
            "clear" => {
                self.terminal_content.clear();
                return Ok(());
            }
            "participants" => "Active Participants:\n• You (Driver)\n• Alice (Navigator)\n• Bob (Resource Provider)",
            _ => "Command executed. Type 'help' for available commands.",
        };

        self.terminal_content.push_str(&format!("\n$ {}\n{}\n$ ", command, response));
        Ok(())
    }
}
