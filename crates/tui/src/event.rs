//! Input / agent events fed into the App.

use crossterm::event::{Event as CtEvent, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use mimo_tui_core::agent::AgentEvent;
use tokio::sync::mpsc::UnboundedSender;

/// Unified event type the App processes.
#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Resize,
    Agent(AgentEvent),
    /// Agent loop finished (per-turn).
    AgentDone,
    /// Internal tick (for animations / state updates).
    Tick,
}

/// Spawn a task that forwards crossterm input events to `tx`.
pub fn spawn_input_listener(tx: UnboundedSender<Event>) {
    tokio::spawn(async move {
        let mut stream = EventStream::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(CtEvent::Key(k)) if matches!(k.kind, KeyEventKind::Press | KeyEventKind::Repeat) => {
                    if tx.send(Event::Key(k)).is_err() {
                        break;
                    }
                }
                Ok(CtEvent::Resize(_, _)) => {
                    if tx.send(Event::Resize).is_err() {
                        break;
                    }
                }
                Err(_) => break,
                _ => {}
            }
        }
    });
}

/// Convenience: is this key Ctrl+C or Ctrl+D (request to exit)?
pub fn is_exit(key: &KeyEvent) -> bool {
    matches!(
        (key.modifiers, key.code),
        (KeyModifiers::CONTROL, KeyCode::Char('c'))
            | (KeyModifiers::CONTROL, KeyCode::Char('d'))
    )
}
