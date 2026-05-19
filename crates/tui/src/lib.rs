//! Full-screen TUI built with ratatui + crossterm.
//!
//! Replaces the v0.1/v0.2 rustyline REPL with a proper persistent layout:
//!
//! ```text
//! ╭─ ∞ mimo-tui · mimo-v2.5-pro · SGP · agent ─────── usage: 1.2k ─╮
//! │                                                                │
//! │  user > ...                                                    │
//! │  thinking: ...                                                 │
//! │  > write_file(...)                                             │
//! │  ↳ wrote 42 bytes                                              │
//! │  assistant > ...                                               │
//! │                                                                │
//! ├────────────────────────────────────────────────────────────────┤
//! │ > █                                                            │
//! ╰────────────────────────────────────────────────────────────────╯
//! ```

mod app;
mod event;
mod render;

pub use app::App;

use std::io;
use std::sync::Arc;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use mimo_tui_core::agent::Agent;
use mimo_tui_core::session::Session;
use mimo_tui_core::{auth::Auth, config::Config, load_default_skills, McpHub};
use mimo_tui_tools::{ApprovalMode, ToolContext, ToolRegistry};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc;
use tracing::error;

/// Boot the full TUI. Builds the agent and runs the main event loop.
pub async fn run() -> Result<()> {
    // Auth resolution.
    let auth = match Auth::resolve()? {
        Some(a) => a,
        None => {
            anyhow::bail!("no API key configured; run `mimo init` first");
        }
    };

    let mut config = Config::load()?;
    config.apply_env();

    let workspace = std::env::current_dir()?;
    let client = mimo_tui_anthropic_client::Client::new(auth.api_key, config.region.to_client_region());

    let mut registry = ToolRegistry::with_defaults();
    let _mcp_hub = McpHub::init(&mut registry).await.ok();

    // Install TaskTool so the agent can dispatch parallel sub-agents.
    let snapshot = Arc::new(mimo_tui_tools::StaticRegistry::from_registry(&registry));
    let task_tool = Arc::new(mimo_tui_tools::TaskTool::new(
        client.clone(),
        snapshot,
        config.model.clone(),
    ));
    registry.insert(task_tool);

    let tools = Arc::new(registry);
    let skills = Arc::new(load_default_skills(&workspace).unwrap_or_default());

    let ctx = ToolContext::new(workspace.clone()).with_mode(map_mode(config.mode));
    let session = Session::new(workspace, config.model.clone());
    let agent = Agent::new(client, config.clone(), tools.clone(), ctx, session).with_skills(skills);

    let mut app = App::new(agent, config);

    // Terminal setup.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = main_loop(&mut terminal, &mut app).await;

    // Teardown.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        error!(?e, "TUI exited with error");
        eprintln!("error: {}", e);
    }
    Ok(())
}

fn map_mode(cfg: mimo_tui_core::config::AgentModeConfig) -> ApprovalMode {
    mimo_tui_core::agent::approval_mode(cfg)
}

async fn main_loop<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    event::spawn_input_listener(event_tx.clone());
    app.set_event_tx(event_tx.clone());

    // Initial draw.
    terminal.draw(|f| render::draw(f, app))?;

    while !app.should_quit {
        tokio::select! {
            Some(ev) = event_rx.recv() => {
                app.handle_event(ev).await;
            }
            else => break,
        }
        terminal.draw(|f| render::draw(f, app))?;
    }
    Ok(())
}
