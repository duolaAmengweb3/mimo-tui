//! Terminal output helpers (color codes + banner + slash-command dispatch).
//!
//! Phase 1 uses plain stdout + ANSI escape codes. Phase 2 will swap this for
//! a full ratatui UI.

use crossterm::style::{Color, Stylize};
use mimo_tui_core::agent::{Agent, AgentEvent};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_banner(model: &str, region: &str) {
    let logo = "∞".with(Color::Magenta).bold();
    let name = "mimo-tui".with(Color::White).bold();
    let v = format!("v{}", VERSION).with(Color::DarkGrey);
    let m = model.with(Color::Cyan);
    let r = region.with(Color::DarkGrey);
    println!();
    println!("  {logo}  {name}  {v}   ·   {m}   ·   {r}");
    println!(
        "  {}",
        "ready · type /help for commands · Ctrl+D to exit".with(Color::DarkGrey)
    );
    println!();
}

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
        AgentEvent::ToolCall { name, args, result, is_error } => {
            let icon = if is_error { "✗" } else { "✓" };
            let color = if is_error { Color::Red } else { Color::Green };
            let args_preview = serde_json::to_string(&args)
                .unwrap_or_default();
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
            eprintln!("  {} {}", "error".with(Color::Red).bold(), e.with(Color::Red));
        }
    }
}

/// Returns `true` to keep the REPL running, `false` to exit.
pub fn handle_slash(cmd: &str, agent: &mut Agent) -> bool {
    let mut parts = cmd.split_whitespace();
    let name = parts.next().unwrap_or("");
    match name {
        "help" | "h" | "?" => {
            println!();
            println!("  /help           show this help");
            println!("  /model <name>   switch model");
            println!("  /mode plan|agent|auto   change approval mode");
            println!("  /region cn|sgp|ams      switch cluster");
            println!("  /usage          show today's token usage");
            println!("  /clear          clear screen");
            println!("  /exit           quit");
            println!();
            true
        }
        "exit" | "quit" | "q" => false,
        "clear" => {
            print!("\x1B[2J\x1B[H");
            true
        }
        "model" => {
            if let Some(m) = parts.next() {
                agent.config.model = m.to_string();
                println!("  ✓ model → {}", m);
            } else {
                println!("  current model: {}", agent.config.model);
            }
            true
        }
        "mode" => {
            use mimo_tui_core::config::AgentModeConfig;
            match parts.next() {
                Some("plan") => agent.config.mode = AgentModeConfig::Plan,
                Some("agent") => agent.config.mode = AgentModeConfig::Agent,
                Some("auto") => agent.config.mode = AgentModeConfig::Auto,
                Some(other) => {
                    println!("  unknown mode '{}', use plan|agent|auto", other);
                    return true;
                }
                None => {
                    println!("  current mode: {:?}", agent.config.mode);
                    return true;
                }
            }
            agent.ctx.mode = mimo_tui_core::agent::approval_mode(agent.config.mode);
            println!("  ✓ mode → {:?}", agent.config.mode);
            true
        }
        "region" => {
            use mimo_tui_core::region::RegionConfig;
            match parts.next() {
                Some("cn") => agent.config.region = RegionConfig::Cn,
                Some("sgp") => agent.config.region = RegionConfig::Sgp,
                Some("ams") => agent.config.region = RegionConfig::Ams,
                _ => {
                    println!("  usage: /region cn|sgp|ams");
                    return true;
                }
            }
            // Build a fresh client at the new region.
            if let Some(auth) = mimo_tui_core::auth::Auth::resolve().ok().flatten() {
                agent.client = mimo_tui_anthropic_client::Client::new(
                    auth.api_key,
                    agent.config.region.to_client_region(),
                );
                println!("  ✓ region → {}", agent.config.region.label());
            } else {
                println!("  could not load auth — aborted");
            }
            true
        }
        "usage" => {
            if let Ok(db) = mimo_tui_core::usage::UsageDb::open() {
                if let Ok(t) = db.totals_today() {
                    println!(
                        "  today {} · calls:{} in:{} out:{} cached:{}",
                        t.date, t.call_count, t.input_tokens, t.output_tokens, t.cache_read_tokens
                    );
                }
            }
            true
        }
        "" => true,
        other => {
            println!("  unknown command '/{}' — try /help", other);
            true
        }
    }
}
