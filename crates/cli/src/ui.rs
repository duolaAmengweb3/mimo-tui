//! Terminal output helpers for one-shot (`mimo -p "..."`) non-TUI mode.
//!
//! Interactive mode uses the full ratatui TUI in `mimo-tui-tui`.

use crossterm::style::{Color, Stylize};
use mimo_tui_core::agent::AgentEvent;

pub fn render_event(event: AgentEvent) {
    use std::io::Write as _;
    match event {
        AgentEvent::TextDelta(chunk) => {
            print!("{}", chunk);
            let _ = std::io::stdout().flush();
        }
        AgentEvent::ThinkingDelta(_) => {
            // Quietly accumulate during stream; the full block fires Thinking at end.
        }
        AgentEvent::AssistantText(_) => {
            // Already rendered piecewise via TextDelta.
        }
        AgentEvent::Thinking(t) => {
            // Render full thinking block dimmed + italics at end of block.
            let preview: String = t.chars().take(160).collect();
            let suffix = if t.chars().count() > 160 { "..." } else { "" };
            eprintln!(
                "\n  {} {}{}",
                "thinking".with(Color::Magenta).italic(),
                preview.with(Color::DarkGrey).italic(),
                suffix
            );
        }
        AgentEvent::ToolCall {
            name,
            args,
            result,
            is_error,
        } => {
            let icon = if is_error { "✗" } else { "✓" };
            let color = if is_error { Color::Red } else { Color::Green };
            let args_preview = serde_json::to_string(&args).unwrap_or_default();
            let args_short: String = args_preview.chars().take(80).collect();
            eprintln!(
                "  {} {} {}",
                icon.with(color),
                name.with(Color::Cyan).bold(),
                args_short.with(Color::DarkGrey),
            );
            let result_preview: String = result.lines().take(8).collect::<Vec<_>>().join("\n");
            for line in result_preview.lines() {
                eprintln!("    {}", line.with(Color::DarkGrey));
            }
            if result.lines().count() > 8 {
                eprintln!("    {}", "...".with(Color::DarkGrey));
            }
        }
        AgentEvent::Usage(u) => {
            let cache_part = if u.cache_read_input_tokens > 0 {
                format!(" · {} cached", u.cache_read_input_tokens)
            } else {
                String::new()
            };
            eprintln!(
                "  {} in:{} out:{}{}",
                "usage".with(Color::DarkGrey),
                u.input_tokens.to_string().with(Color::DarkGrey),
                u.output_tokens.to_string().with(Color::DarkGrey),
                cache_part.with(Color::DarkGrey),
            );
        }
        AgentEvent::Error(e) => {
            eprintln!(
                "  {} {}",
                "error".with(Color::Red).bold(),
                e.with(Color::Red)
            );
        }
    }
}

// Slash commands are now handled by the ratatui App in mimo-tui-tui.
