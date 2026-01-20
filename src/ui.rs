use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    app::App,
    color::parse_hex_rgb,
    editor::{FieldId, field_value},
    model::theme_mode_label,
};

pub fn draw_ui(frame: &mut ratatui::Frame<'_>, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(frame.area());

    draw_preview(frame, body[0], app);
    draw_editor(frame, body[1], app);
}

fn draw_preview(frame: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
    let title = Span::styled(
        "Skyset Preview",
        Style::default().add_modifier(Modifier::BOLD),
    );
    let lines = vec![
        Line::from(title),
        Line::from(Span::raw("")),
        Line::from(label_value("Message", &app.skyset().message)),
        Line::from(label_value("Submessage", &app.skyset().submessage)),
        Line::from(Span::raw("")),
        Line::from(label_value(
            "Theme mode",
            theme_mode_label(app.skyset().theme.mode),
        )),
        Line::from(label_with_color("Accent", &app.skyset().theme.accent)),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Palette")),
        Line::from(label_with_color("  primary", &app.skyset().palette.primary)),
        Line::from(label_with_color(
            "  secondary",
            &app.skyset().palette.secondary,
        )),
        Line::from(label_with_color(
            "  tertiary",
            &app.skyset().palette.tertiary,
        )),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Gradients")),
        Line::from(gradient_line(
            "  background",
            &app.skyset().gradients.background,
        )),
        Line::from(gradient_line("  hero", &app.skyset().gradients.hero)),
    ];

    let block = Block::default().borders(Borders::ALL).title("Preview");
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_editor(frame: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
    let editor = app.editor();
    let fields = editor.fields().iter().enumerate().map(|(idx, field)| {
        let is_selected = idx == editor.selected();
        let value = if is_selected {
            editor.input().to_string()
        } else {
            field_value(app.skyset(), *field)
        };
        let style = if is_selected {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        if let Some(color) = color_for_field(app, *field) {
            Line::from(vec![
                Span::styled(format!("{:>16}: ", field.label()), style),
                Span::styled("  ", style.bg(color)),
                Span::raw(" "),
                Span::styled(value, style),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{:>16}: ", field.label()), style),
                Span::styled(value, style),
            ])
        }
    });

    let mut lines: Vec<Line> = vec![Line::from(Span::styled(
        "Editor",
        Style::default().add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(Span::raw(format!(
        "Config file: {}",
        app.path().display()
    ))));
    lines.push(Line::from(Span::raw("Press Tab/Shift+Tab to move.")));
    lines.push(Line::from(Span::raw("Type to edit, Enter to apply.")));
    lines.push(Line::from(Span::raw("")));
    lines.extend(fields);

    let block = Block::default().borders(Borders::ALL).title("Editor");
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn label_value(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{label}: "),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(value.to_string()),
    ])
}

fn label_with_color(label: &str, value: &str) -> Line<'static> {
    if let Some(color) = parse_color(value) {
        Line::from(vec![
            Span::raw(format!("{label}: ")),
            Span::styled("  ", Style::default().bg(color)),
            Span::raw(format!(" {}", value)),
        ])
    } else {
        Line::from(Span::raw(format!("{label}: {value}")))
    }
}

fn gradient_line(label: &str, values: &[String]) -> Line<'static> {
    let mut spans = vec![Span::raw(format!("{label}: "))];
    for (idx, value) in values.iter().enumerate() {
        if let Some(color) = parse_color(value) {
            spans.push(Span::styled("  ", Style::default().bg(color)));
            spans.push(Span::raw(format!(" {value}")));
        } else {
            spans.push(Span::raw(value.clone()));
        }
        if idx + 1 < values.len() {
            spans.push(Span::raw(" → "));
        }
    }
    Line::from(spans)
}

fn color_for_field(app: &App, field: FieldId) -> Option<Color> {
    match field {
        FieldId::Accent => parse_color(&app.skyset().theme.accent),
        FieldId::Primary => parse_color(&app.skyset().palette.primary),
        FieldId::Secondary => parse_color(&app.skyset().palette.secondary),
        FieldId::Tertiary => parse_color(&app.skyset().palette.tertiary),
        FieldId::Background1 => app
            .skyset()
            .gradients
            .background
            .get(0)
            .and_then(|value| parse_color(value)),
        FieldId::Background2 => app
            .skyset()
            .gradients
            .background
            .get(1)
            .and_then(|value| parse_color(value)),
        FieldId::Background3 => app
            .skyset()
            .gradients
            .background
            .get(2)
            .and_then(|value| parse_color(value)),
        FieldId::Hero1 => app
            .skyset()
            .gradients
            .hero
            .get(0)
            .and_then(|value| parse_color(value)),
        FieldId::Hero2 => app
            .skyset()
            .gradients
            .hero
            .get(1)
            .and_then(|value| parse_color(value)),
        _ => None,
    }
}

fn parse_color(value: &str) -> Option<Color> {
    parse_hex_rgb(value).map(|(r, g, b)| Color::Rgb(r, g, b))
}
