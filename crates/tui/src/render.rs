//! ratatui rendering — split the screen into header / scrollback / input rows.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{item_lines, App};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Min(3),    // messages
            Constraint::Length(3), // input
            Constraint::Length(1), // footer
        ])
        .split(area);

    draw_header(f, chunks[0], app);
    draw_messages(f, chunks[1], app);
    draw_input(f, chunks[2], app);
    draw_footer(f, chunks[3], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let model = &app.config.model;
    let region = app.config.region.label();
    let mode = format!("{:?}", app.config.mode).to_lowercase();
    let usage = format!(
        "in:{} out:{} cached:{}",
        app.last_input_tokens, app.last_output_tokens, app.last_cache_tokens
    );

    let header = Line::from(vec![
        Span::styled(" ∞ ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled("mimo-tui ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")), Style::default().fg(Color::DarkGray)),
        Span::raw("   ·   "),
        Span::styled(model.clone(), Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::styled(region.to_string(), Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(format!("[{mode}]"), Style::default().fg(Color::Yellow)),
        Span::raw("   "),
        Span::styled(usage, Style::default().fg(Color::DarkGray)),
    ]);

    let brand = Line::from(vec![
        Span::styled("built by ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "duola ",
            Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD),
        ),
        Span::styled("· ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "@hunterweb303 ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ]);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(32)])
        .split(area);

    f.render_widget(Paragraph::new(header), cols[0]);
    f.render_widget(Paragraph::new(brand).alignment(Alignment::Right), cols[1]);
}

fn draw_messages(f: &mut Frame, area: Rect, app: &mut App) {
    let mut lines: Vec<Line> = Vec::new();
    for item in &app.items {
        for line in item_lines(item) {
            lines.push(line);
        }
    }

    // Append live streaming buffers.
    if !app.streaming_thinking.is_empty() {
        let preview: String = app.streaming_thinking.chars().take(200).collect();
        let suffix = if app.streaming_thinking.chars().count() > 200 { "…" } else { "" };
        lines.push(Line::from(vec![
            Span::styled(
                "  thinking ".to_string(),
                Style::default().fg(Color::LightMagenta).add_modifier(Modifier::ITALIC),
            ),
            Span::styled(
                format!("{preview}{suffix}"),
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            ),
        ]));
    }
    if !app.streaming_assistant.is_empty() {
        for line in app.streaming_assistant.lines() {
            lines.push(Line::raw(line.to_string()));
        }
    }

    // Scroll: when following tail, snap to the bottom; otherwise honor app.scroll.
    // PageDown past the bottom re-engages follow-tail so streaming output catches up.
    let body_height = area.height.saturating_sub(2) as usize;
    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(body_height) as u16;
    app.last_max_scroll = max_scroll;
    if !app.follow_tail && app.scroll >= max_scroll {
        app.follow_tail = true;
    }
    let scroll = if app.follow_tail { max_scroll } else { app.scroll };

    let block = Block::default().borders(Borders::TOP | Borders::BOTTOM).border_style(Style::default().fg(Color::DarkGray));
    let p = Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((scroll, 0)).block(block);
    f.render_widget(p, area);
}

fn draw_input(f: &mut Frame, area: Rect, app: &App) {
    let prompt = if app.is_running { "  …" } else { "> " };
    let style = if app.is_running {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
    };
    let lines = vec![Line::from(vec![
        Span::styled(prompt.to_string(), style),
        Span::raw(app.input.clone()),
    ])];
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(Paragraph::new(lines).block(block), area);

    if !app.is_running {
        // Place cursor right after the prompt + input position.
        use unicode_width::UnicodeWidthStr;
        let prefix_width = prompt.width();
        let input_prefix = &app.input[..app.cursor.min(app.input.len())];
        let cursor_col = area.x + 1 + (prefix_width + input_prefix.width()) as u16;
        let cursor_row = area.y + 1;
        f.set_cursor_position((cursor_col, cursor_row));
    }
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let style = Style::default().fg(Color::DarkGray);
    let footer = Line::from(vec![Span::styled(format!(" {}", app.status_hint), style)]);
    f.render_widget(Paragraph::new(footer), area);
}
