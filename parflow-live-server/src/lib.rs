use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub session_id: String,
    pub project_name: String,
    pub participants: Vec<Participant>,
    pub shared_terminal: SharedTerminal,
    pub code_files: Vec<CodeFile>,
    pub compilation_results: CompilationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub terminal_tab: TerminalTab,
    pub resources: ParticipantResources,
    pub cursor_position: CursorPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResources {
    pub available_cpu_cores: u32,
    pub available_memory_gb: f64,
    pub available_gpu_memory_gb: f64,
    pub network_bandwidth_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: u32,
    pub column: u32,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalTab {
    pub tab_id: String,
    pub tab_name: String,
    pub content: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTerminal {
    pub active_tabs: Vec<TerminalTab>,
    pub broadcast_channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub filename: String,
    pub content: String,
    pub language: String,
    pub last_modified_by: String,
    pub compilation_status: CompilationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationStatus {
    pub status: CompilationState,
    pub output: String,
    pub errors: Vec<CompilationError>,
    pub warnings: Vec<CompilationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationError {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationWarning {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompilationState {
    NotCompiled,
    Compiling,
    Success,
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Driver,
    Navigator,
    Reviewer,
    ResourceProvider,
}

pub struct LiveServer {
    sessions: Arc<DashMap<String, LiveSession>>,
    broadcast_senders: Arc<DashMap<String, broadcast::Sender<LiveUpdate>>>,
}

impl LiveServer {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            broadcast_senders: Arc::new(DashMap::new()),
        }
    }

    pub async fn create_session(&self, project_name: &str) -> String {
        let session_id = Uuid::new_v4().to_string();

        let session = LiveSession {
            session_id: session_id.clone(),
            project_name: project_name.to_string(),
            participants: Vec::new(),
            shared_terminal: SharedTerminal {
                active_tabs: vec![
                    TerminalTab {
                        tab_id: "main".to_string(),
                        tab_name: "Main Terminal".to_string(),
                        content: "Welcome to ParFlow Live! 👋\n\nType 'help' for available commands.".to_string(),
                        is_active: true,
                    }
                ],
                broadcast_channel: format!("session_{}", session_id),
            },
            code_files: vec![],
            compilation_results: CompilationStatus {
                status: CompilationState::NotCompiled,
                output: String::new(),
                errors: Vec::new(),
                warnings: Vec::new(),
            },
        };

        let (tx, _) = broadcast::channel(100);
        self.broadcast_senders.insert(session_id.clone(), tx);
        self.sessions.insert(session_id.clone(), session);

        session_id
    }

    pub async fn join_session(&self, session_id: &str, user_name: &str) -> Option<LiveSession> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            let participant = Participant {
                id: Uuid::new_v4().to_string(),
                name: user_name.to_string(),
                terminal_tab: TerminalTab {
                    tab_id: Uuid::new_v4().to_string(),
                    tab_name: format!("{}'s Terminal", user_name),
                    content: String::new(),
                    is_active: false,
                },
                resources: ParticipantResources::default(),
                cursor_position: CursorPosition::default(),
            };

            session.participants.push(participant);

            if let Some(tx) = self.broadcast_senders.get(session_id) {
                let _ = tx.send(LiveUpdate::UserJoined {
                    user_name: user_name.to_string(),
                    participant_count: session.participants.len(),
                });
            }

            return Some(session.clone());
        }
        None
    }

    pub async fn handle_terminal_input(&self, session_id: &str, user_id: &str, input: &str) -> Result<(), anyhow::Error> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            if let Some(_participant) = session.participants.iter_mut().find(|p| p.id == user_id) {
                if let Some(active_tab) = session.shared_terminal.active_tabs.iter_mut().find(|t| t.is_active) {
                    let output = self.execute_command(input, session_id).await?;

                    active_tab.content.push_str(&format!("\n$ {}\n{}", input, output));

                    if let Some(tx) = self.broadcast_senders.get(session_id) {
                        let _ = tx.send(LiveUpdate::TerminalOutput {
                            tab_id: active_tab.tab_id.clone(),
                            content: active_tab.content.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn handle_code_edit(&self, session_id: &str, user_id: &str, filename: &str, new_content: &str) -> Result<(), anyhow::Error> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            if let Some(file) = session.code_files.iter_mut().find(|f| f.filename == filename) {
                file.content = new_content.to_string();
                file.last_modified_by = user_id.to_string();
            } else {
                session.code_files.push(CodeFile {
                    filename: filename.to_string(),
                    content: new_content.to_string(),
                    language: self.detect_language(filename),
                    last_modified_by: user_id.to_string(),
                    compilation_status: CompilationStatus::default(),
                });
            }

            self.trigger_compilation(session_id).await?;

            if let Some(tx) = self.broadcast_senders.get(session_id) {
                let _ = tx.send(LiveUpdate::CodeChanged {
                    filename: filename.to_string(),
                    content: new_content.to_string(),
                    modified_by: user_id.to_string(),
                });
            }
        }
        Ok(())
    }

    pub async fn update_cursor_position(&self, session_id: &str, user_id: &str, filename: &str, line: u32, column: u32) -> Result<(), anyhow::Error> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            // FIXED: Remove underscore from _participant
            if let Some(participant) = session.participants.iter_mut().find(|p| p.id == user_id) {
                participant.cursor_position = CursorPosition {
                    line,
                    column,
                    filename: Some(filename.to_string()),
                };

                if let Some(tx) = self.broadcast_senders.get(session_id) {
                    let _ = tx.send(LiveUpdate::CursorMoved {
                        user_id: user_id.to_string(),
                        user_name: participant.name.clone(),
                        filename: filename.to_string(),
                        position: participant.cursor_position.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    async fn execute_command(&self, command: &str, session_id: &str) -> Result<String, anyhow::Error> {
        match command.trim() {
            "help" => Ok(
                "Available commands:\n\
                 • code <file> - Edit a code file\n\
                 • compile - Trigger compilation\n\
                 • status - Show session status\n\
                 • resources - Show shared resources\n\
                 • invite <user> - Invite another user".to_string()
            ),
            "compile" => {
                self.trigger_compilation(session_id).await?;
                Ok("Compilation triggered!".to_string())
            }
            "status" => {
                if let Some(session) = self.sessions.get(session_id) {
                    Ok(format!(
                        "Session: {}\nParticipants: {}\nFiles: {}\nResources: {} cores, {}GB memory",
                        session.project_name,
                        session.participants.len(),
                        session.code_files.len(),
                        session.participants.iter().map(|p| p.resources.available_cpu_cores).sum::<u32>(),
                        session.participants.iter().map(|p| p.resources.available_memory_gb).sum::<f64>(),
                    ))
                } else {
                    Ok("Session not found".to_string())
                }
            }
            "resources" => {
                if let Some(session) = self.sessions.get(session_id) {
                    let total_cores: u32 = session.participants.iter().map(|p| p.resources.available_cpu_cores).sum();
                    let total_memory: f64 = session.participants.iter().map(|p| p.resources.available_memory_gb).sum();
                    let total_gpu: f64 = session.participants.iter().map(|p| p.resources.available_gpu_memory_gb).sum();

                    Ok(format!(
                        "Shared Resources:\n\
                         • CPU Cores: {}\n\
                         • Memory: {:.1}GB\n\
                         • GPU Memory: {:.1}GB\n\
                         • Participants: {}",
                        total_cores, total_memory, total_gpu, session.participants.len()
                    ))
                } else {
                    Ok("Session not found".to_string())
                }
            }
            _ => Ok(format!("Executed: {}", command)),
        }
    }

    async fn trigger_compilation(&self, session_id: &str) -> Result<(), anyhow::Error> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            session.compilation_results.status = CompilationState::Compiling;

            if let Some(tx) = self.broadcast_senders.get(session_id) {
                let _ = tx.send(LiveUpdate::CompilationStarted);
            }

            // Simulate compilation
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            // Mock compilation results
            session.compilation_results = CompilationStatus {
                status: CompilationState::Success,
                output: "Compilation successful! 🎉".to_string(),
                errors: Vec::new(),
                warnings: vec![
                    CompilationWarning {
                        file: "main.rs".to_string(),
                        line: 10,
                        column: 5,
                        message: "Unused variable".to_string(),
                        suggestion: Some("Consider removing or using the variable".to_string()),
                    }
                ],
            };

            if let Some(tx) = self.broadcast_senders.get(session_id) {
                let _ = tx.send(LiveUpdate::CompilationFinished {
                    status: session.compilation_results.status.clone(),
                    output: session.compilation_results.output.clone(),
                    errors: session.compilation_results.errors.clone(),
                    warnings: session.compilation_results.warnings.clone(),
                });
            }
        }
        Ok(())
    }

    fn detect_language(&self, filename: &str) -> String {
        if filename.ends_with(".rs") {
            "rust".to_string()
        } else if filename.ends_with(".py") {
            "python".to_string()
        } else if filename.ends_with(".js") || filename.ends_with(".ts") {
            "javascript".to_string()
        } else {
            "unknown".to_string()
        }
    }

    pub fn subscribe_to_updates(&self, session_id: &str) -> Option<broadcast::Receiver<LiveUpdate>> {
        self.broadcast_senders
            .get(session_id)
            .map(|tx| tx.subscribe())
    }

    pub async fn distribute_compilation(&self, session_id: &str) -> Result<(), anyhow::Error> {
        if let Some(session) = self.sessions.get(session_id) {
            let total_cores: u32 = session.participants.iter().map(|p| p.resources.available_cpu_cores).sum();
            let total_memory: f64 = session.participants.iter().map(|p| p.resources.available_memory_gb).sum();

            println!("{} {} cores, {}GB memory",
                "🔄 Distributing compilation across:".bright_green(),
                total_cores, total_memory);

            // Distribute compilation tasks
            for (i, file) in session.code_files.iter().enumerate() {
                if let Some(participant) = session.participants.get(i % session.participants.len()) {
                    println!("{} {} → {}", "📦 Compiling:".bright_blue(), file.filename, participant.name);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiveUpdate {
    UserJoined {
        user_name: String,
        participant_count: usize,
    },
    UserLeft {
        user_name: String,
        participant_count: usize,
    },
    TerminalOutput {
        tab_id: String,
        content: String,
    },
    CodeChanged {
        filename: String,
        content: String,
        modified_by: String,
    },
    CursorMoved {
        user_id: String,
        user_name: String,
        filename: String,
        position: CursorPosition,
    },
    CompilationStarted,
    CompilationFinished {
        status: CompilationState,
        output: String,
        errors: Vec<CompilationError>,
        warnings: Vec<CompilationWarning>,
    },
}

impl Default for ParticipantResources {
    fn default() -> Self {
        Self {
            available_cpu_cores: 4,
            available_memory_gb: 8.0,
            available_gpu_memory_gb: 2.0,
            network_bandwidth_mbps: 100.0,
        }
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            filename: None,
        }
    }
}

impl Default for CompilationStatus {
    fn default() -> Self {
        Self {
            status: CompilationState::NotCompiled,
            output: String::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

// Import colored for the distribute_compilation method
use colored::*;
