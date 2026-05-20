//! App state + event handlers.

use std::sync::Arc;

use mimo_tui_core::agent::{Agent, AgentEvent};
use mimo_tui_core::config::{AgentModeConfig, Config};
use mimo_tui_core::region::RegionConfig;
use ratatui::text::Line;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::event::{is_exit, Event};

/// One item in the scrollback.
#[derive(Debug, Clone)]
pub enum Item {
    User(String),
    AssistantText(String),
    Thinking(String),
    Tool {
        name: String,
        args_summary: String,
        result: String,
        is_error: bool,
    },
    Error(String),
    Note(String),
}

pub struct App {
    pub items: Vec<Item>,
    /// Streaming buffer for the current assistant turn.
    pub streaming_assistant: String,
    pub streaming_thinking: String,
    pub input: String,
    pub cursor: usize,
    pub scroll: u16,
    /// When true, scroll position auto-follows the tail of the buffer (default).
    /// Goes false the moment the user presses PageUp, returns to true on submit
    /// or PageDown past the bottom.
    pub follow_tail: bool,
    /// Latest max_scroll value seen by the renderer — used so that PageUp from
    /// the bottom can step *up* from the actual current view rather than from 0.
    pub last_max_scroll: u16,
    pub should_quit: bool,
    pub is_running: bool,
    pub agent: Arc<tokio::sync::Mutex<Agent>>,
    pub config: Config,
    pub last_input_tokens: u32,
    pub last_output_tokens: u32,
    pub last_cache_tokens: u32,
    /// Handle to the currently-running agent task (so we can ignore late events).
    pub agent_task: Option<JoinHandle<()>>,
    pub event_tx: Option<mpsc::UnboundedSender<Event>>,
    /// Footer hint that fades after a moment.
    pub status_hint: String,
}

impl App {
    pub fn new(agent: Agent, config: Config) -> Self {
        Self {
            items: vec![Item::Note(format!(
                "ready · model: {} · region: {} · mode: {:?}",
                config.model,
                config.region.label(),
                config.mode
            ))],
            streaming_assistant: String::new(),
            streaming_thinking: String::new(),
            input: String::new(),
            cursor: 0,
            scroll: 0,
            follow_tail: true,
            last_max_scroll: 0,
            should_quit: false,
            is_running: false,
            agent: Arc::new(tokio::sync::Mutex::new(agent)),
            config,
            last_input_tokens: 0,
            last_output_tokens: 0,
            last_cache_tokens: 0,
            agent_task: None,
            event_tx: None,
            status_hint:
                "Enter to send · Alt+↑/↓ or Ctrl+U/Ctrl+E scroll · Ctrl+C exit · /help"
                    .to_string(),
        }
    }

    pub fn set_event_tx(&mut self, tx: mpsc::UnboundedSender<Event>) {
        self.event_tx = Some(tx);
    }

    pub async fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::Key(k) => {
                if is_exit(&k) {
                    self.should_quit = true;
                    return;
                }
                self.handle_key(k).await;
            }
            Event::Resize => {}
            Event::Tick => {}
            Event::Agent(e) => self.handle_agent_event(e),
            Event::AgentDone => {
                self.is_running = false;
                self.flush_streaming();
                self.status_hint = "ready".to_string();
            }
        }
    }

    fn handle_agent_event(&mut self, e: AgentEvent) {
        match e {
            AgentEvent::TextDelta(s) => {
                self.streaming_assistant.push_str(&s);
            }
            AgentEvent::ThinkingDelta(s) => {
                self.streaming_thinking.push_str(&s);
            }
            AgentEvent::AssistantText(_) => {
                // Streaming already covers this.
            }
            AgentEvent::Thinking(t) => {
                if !self.streaming_thinking.is_empty() {
                    let buf = std::mem::take(&mut self.streaming_thinking);
                    self.items.push(Item::Thinking(buf));
                } else if !t.is_empty() {
                    self.items.push(Item::Thinking(t));
                }
            }
            AgentEvent::ToolCall { name, args, result, is_error } => {
                // Move streaming assistant into items before tool call shows up.
                self.flush_streaming();
                let args_summary = args_summary(&args, 80);
                self.items.push(Item::Tool {
                    name,
                    args_summary,
                    result,
                    is_error,
                });
            }
            AgentEvent::Usage(u) => {
                self.last_input_tokens = u.input_tokens;
                self.last_output_tokens = u.output_tokens;
                self.last_cache_tokens = u.cache_read_input_tokens;
            }
            AgentEvent::Error(msg) => {
                self.flush_streaming();
                self.items.push(Item::Error(msg));
            }
        }
    }

    /// Promote streaming buffers into Items.
    fn flush_streaming(&mut self) {
        if !self.streaming_thinking.is_empty() {
            let buf = std::mem::take(&mut self.streaming_thinking);
            self.items.push(Item::Thinking(buf));
        }
        if !self.streaming_assistant.is_empty() {
            let buf = std::mem::take(&mut self.streaming_assistant);
            self.items.push(Item::AssistantText(buf));
        }
    }

    /// Returns true when the key was consumed by a scroll binding.
    /// Bindings:
    /// - `PageUp` / `PageDown`           — standard (won't reach us if VS Code grabs them)
    /// - `Alt+Up` / `Alt+Down`           — most portable in VS Code / iTerm / Terminal.app
    /// - `Ctrl+U` / `Ctrl+E`             — Vim-ish; Ctrl+D is reserved for exit
    fn try_scroll(&mut self, k: crossterm::event::KeyEvent) -> bool {
        use crossterm::event::{KeyCode::*, KeyModifiers};
        let big = self.last_max_scroll.max(20) / 2 + 5;
        let small: u16 = 5;
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        let alt = k.modifiers.contains(KeyModifiers::ALT);
        let (delta, is_up) = match (ctrl, alt, k.code) {
            (false, false, PageUp) => (small, true),
            (false, false, PageDown) => (small, false),
            (false, true, Up) => (small, true),
            (false, true, Down) => (small, false),
            (true, false, Char('u' | 'U')) => (big, true),
            (true, false, Char('e' | 'E')) => (big, false),
            _ => return false,
        };
        if is_up {
            if self.follow_tail {
                self.scroll = self.last_max_scroll;
            }
            self.follow_tail = false;
            self.scroll = self.scroll.saturating_sub(delta);
        } else {
            self.scroll = self.scroll.saturating_add(delta);
        }
        true
    }

    async fn handle_key(&mut self, k: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode::*;
        if self.try_scroll(k) {
            return;
        }
        if self.is_running {
            // While the agent is running we accept only scroll keys (above) and
            // Ctrl+C (handled in the caller). Everything else is ignored.
            return;
        }
        match k.code {
            Enter => {
                if !self.input.is_empty() {
                    self.submit().await;
                }
            }
            Char(c) => {
                self.input.insert(self.cursor, c);
                self.cursor += c.len_utf8();
            }
            Backspace => {
                if self.cursor > 0 {
                    let mut idx = self.cursor - 1;
                    while idx > 0 && !self.input.is_char_boundary(idx) {
                        idx -= 1;
                    }
                    self.input.replace_range(idx..self.cursor, "");
                    self.cursor = idx;
                }
            }
            Left => {
                if self.cursor > 0 {
                    let mut idx = self.cursor - 1;
                    while idx > 0 && !self.input.is_char_boundary(idx) {
                        idx -= 1;
                    }
                    self.cursor = idx;
                }
            }
            Right => {
                if self.cursor < self.input.len() {
                    let mut idx = self.cursor + 1;
                    while idx < self.input.len() && !self.input.is_char_boundary(idx) {
                        idx += 1;
                    }
                    self.cursor = idx;
                }
            }
            Home => self.cursor = 0,
            End => self.cursor = self.input.len(),
            _ => {}
        }
    }

    async fn submit(&mut self) {
        let text = std::mem::take(&mut self.input);
        self.cursor = 0;
        self.follow_tail = true;

        if let Some(rest) = text.strip_prefix('/') {
            self.handle_slash(rest);
            return;
        }

        self.items.push(Item::User(text.clone()));
        self.is_running = true;
        self.status_hint = "thinking...".to_string();

        // Spawn the agent run task.
        let agent = self.agent.clone();
        let event_tx = self.event_tx.clone();
        let task = tokio::spawn(async move {
            let (atx, mut arx) = mpsc::unbounded_channel();
            let agent_fut = async {
                let mut agent = agent.lock().await;
                let _ = agent.run_turn(&text, atx).await;
            };
            let forward_fut = async {
                while let Some(ev) = arx.recv().await {
                    if let Some(tx) = &event_tx {
                        let _ = tx.send(Event::Agent(ev));
                    }
                }
                if let Some(tx) = &event_tx {
                    let _ = tx.send(Event::AgentDone);
                }
            };
            tokio::join!(agent_fut, forward_fut);
        });
        self.agent_task = Some(task);
    }

    fn handle_slash(&mut self, cmd: &str) {
        let mut parts = cmd.split_whitespace();
        let name = parts.next().unwrap_or("");
        match name {
            "help" | "h" | "?" => {
                self.items.push(Item::Note(
                    "/help · /model <name> · /mode plan|agent|auto · /region cn|sgp|ams · /usage · /sessions [list|resume <id>|fork <id>] · /clear · /exit".to_string(),
                ));
            }
            "exit" | "quit" | "q" => self.should_quit = true,
            "clear" => self.items.clear(),
            "model" => match parts.next() {
                Some(m) => {
                    self.config.model = m.to_string();
                    self.items.push(Item::Note(format!("model → {}", m)));
                }
                None => self.items.push(Item::Note(format!("current model: {}", self.config.model))),
            },
            "mode" => match parts.next() {
                Some("plan") => {
                    self.config.mode = AgentModeConfig::Plan;
                    self.items.push(Item::Note("mode → plan".to_string()));
                }
                Some("agent") => {
                    self.config.mode = AgentModeConfig::Agent;
                    self.items.push(Item::Note("mode → agent".to_string()));
                }
                Some("auto") => {
                    self.config.mode = AgentModeConfig::Auto;
                    self.items.push(Item::Note("mode → auto".to_string()));
                }
                _ => self.items.push(Item::Note(format!("current mode: {:?}", self.config.mode))),
            },
            "region" => match parts.next() {
                Some("cn") => self.set_region(RegionConfig::Cn),
                Some("sgp") => self.set_region(RegionConfig::Sgp),
                Some("ams") => self.set_region(RegionConfig::Ams),
                _ => self
                    .items
                    .push(Item::Note(format!("current region: {}", self.config.region.label()))),
            },
            "usage" => {
                if let Ok(db) = mimo_tui_core::usage::UsageDb::open() {
                    if let Ok(t) = db.totals_today() {
                        self.items.push(Item::Note(format!(
                            "today · calls:{} in:{} out:{} cached:{}",
                            t.call_count, t.input_tokens, t.output_tokens, t.cache_read_tokens
                        )));
                    }
                }
            }
            "sessions" => {
                self.handle_sessions(parts);
            }
            _ => self.items.push(Item::Note(format!("unknown command /{}", name))),
        }
    }

    fn handle_sessions<'a, I: Iterator<Item = &'a str>>(&mut self, mut parts: I) {
        match parts.next().unwrap_or("list") {
            "list" => match mimo_tui_core::session::Session::list_recent(10) {
                Ok(metas) if metas.is_empty() => {
                    self.items
                        .push(Item::Note("(no saved sessions yet)".to_string()));
                }
                Ok(metas) => {
                    self.items.push(Item::Note(format!(
                        "{} recent sessions (most recent first):",
                        metas.len()
                    )));
                    for m in metas {
                        let short_id: String = m.id.chars().take(8).collect();
                        let when = m.created_at.format("%Y-%m-%d %H:%M");
                        self.items.push(Item::Note(format!(
                            "  {} · {} · {} · {} turns · {}",
                            short_id,
                            when,
                            m.model,
                            m.turn_count,
                            m.workspace.display(),
                        )));
                    }
                }
                Err(e) => self.items.push(Item::Error(format!("list sessions: {e}"))),
            },
            "resume" => match parts.next() {
                Some(id_prefix) => self.resume_session(id_prefix.to_string(), false),
                None => self
                    .items
                    .push(Item::Note("usage: /sessions resume <id-prefix>".to_string())),
            },
            "fork" => match parts.next() {
                Some(id_prefix) => self.resume_session(id_prefix.to_string(), true),
                None => self
                    .items
                    .push(Item::Note("usage: /sessions fork <id-prefix>".to_string())),
            },
            other => self
                .items
                .push(Item::Note(format!("/sessions {} — try list|resume|fork", other))),
        }
    }

    /// Load a session by id-prefix and adopt its message history.
    /// If `fork` is true, give it a fresh id so subsequent saves don't overwrite.
    fn resume_session(&mut self, id_prefix: String, fork: bool) {
        let metas = match mimo_tui_core::session::Session::list_recent(50) {
            Ok(m) => m,
            Err(e) => {
                self.items.push(Item::Error(format!("list: {e}")));
                return;
            }
        };
        let id = match metas.iter().find(|m| m.id.starts_with(&id_prefix)) {
            Some(m) => m.id.clone(),
            None => {
                self.items
                    .push(Item::Note(format!("no session matches prefix {}", id_prefix)));
                return;
            }
        };
        let loaded = match mimo_tui_core::session::Session::load(&id) {
            Ok(s) => s,
            Err(e) => {
                self.items.push(Item::Error(format!("load {id}: {e}")));
                return;
            }
        };

        let agent = self.agent.clone();
        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            let mut a = agent.lock().await;
            a.session = if fork {
                mimo_tui_core::session::Session {
                    id: uuid::Uuid::new_v4().to_string(),
                    created_at: chrono::Utc::now(),
                    workspace: loaded.workspace.clone(),
                    model: loaded.model.clone(),
                    messages: loaded.messages.clone(),
                }
            } else {
                loaded
            };
            if let Some(tx) = tx {
                let _ = tx.send(crate::event::Event::Tick);
            }
        });

        let kind = if fork { "forked" } else { "resumed" };
        let short: String = id.chars().take(8).collect();
        self.items.push(Item::Note(format!("{kind} session {}", short)));
    }

    fn set_region(&mut self, region: RegionConfig) {
        self.config.region = region;
        self.items.push(Item::Note(format!("region → {}", region.label())));
        // Rebuild the agent client with the new region.
        if let Ok(Some(auth)) = mimo_tui_core::auth::Auth::resolve() {
            let new_client = mimo_tui_anthropic_client::Client::new(auth.api_key, region.to_client_region());
            let agent = self.agent.clone();
            tokio::spawn(async move {
                let mut a = agent.lock().await;
                a.client = new_client;
            });
        }
    }
}

/// Build a one-line summary of JSON args, truncated.
fn args_summary(v: &serde_json::Value, max_chars: usize) -> String {
    let s = serde_json::to_string(v).unwrap_or_default();
    let s = s.replace('\n', " ");
    let cut: String = s.chars().take(max_chars).collect();
    if cut.len() < s.len() {
        format!("{cut}…")
    } else {
        cut
    }
}

/// Render an Item into one or more `Line`s.
pub fn item_lines(item: &Item) -> Vec<Line<'static>> {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::Span;

    match item {
        Item::User(text) => {
            let mut out = Vec::new();
            for (i, line) in text.lines().enumerate() {
                let prefix = if i == 0 { "▎ user > " } else { "         " };
                out.push(Line::from(vec![
                    Span::styled(prefix.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(line.to_string()),
                ]));
            }
            out.push(Line::raw(""));
            out
        }
        Item::AssistantText(text) => {
            let mut out = Vec::new();
            for line in text.lines() {
                out.push(Line::raw(line.to_string()));
            }
            out.push(Line::raw(""));
            out
        }
        Item::Thinking(text) => {
            let preview: String = text.chars().take(200).collect();
            let suffix = if text.chars().count() > 200 { "…" } else { "" };
            vec![
                Line::from(vec![
                    Span::styled(
                        "  thinking ".to_string(),
                        Style::default().fg(Color::LightMagenta).add_modifier(Modifier::ITALIC),
                    ),
                    Span::styled(
                        format!("{preview}{suffix}"),
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                    ),
                ]),
                Line::raw(""),
            ]
        }
        Item::Tool { name, args_summary, result, is_error } => {
            let mark = if *is_error { "✗" } else { "✓" };
            let color = if *is_error { Color::Red } else { Color::Green };
            let mut out = vec![Line::from(vec![
                Span::styled(format!("  {mark} "), Style::default().fg(color)),
                Span::styled(name.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled(args_summary.to_string(), Style::default().fg(Color::DarkGray)),
            ])];
            for (i, line) in result.lines().take(8).enumerate() {
                let cut: String = line.chars().take(120).collect();
                let _ = i;
                out.push(Line::from(vec![
                    Span::raw("    ".to_string()),
                    Span::styled(cut, Style::default().fg(Color::DarkGray)),
                ]));
            }
            if result.lines().count() > 8 {
                out.push(Line::from(vec![Span::styled(
                    "    …".to_string(),
                    Style::default().fg(Color::DarkGray),
                )]));
            }
            out.push(Line::raw(""));
            out
        }
        Item::Error(msg) => vec![
            Line::from(vec![
                Span::styled("  error ".to_string(), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(msg.to_string(), Style::default().fg(Color::Red)),
            ]),
            Line::raw(""),
        ],
        Item::Note(text) => vec![
            Line::from(vec![Span::styled(
                format!("  · {text}"),
                Style::default().fg(Color::DarkGray),
            )]),
            Line::raw(""),
        ],
    }
}
