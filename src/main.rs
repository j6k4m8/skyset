mod app;
mod cli;
mod color;
mod editor;
mod io;
mod model;
mod output;
mod paths;
mod ui;

use std::{
    io::{self as stdio, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;
use cli::{OutputMode, parse_cli, print_help};
use output::{print_json, print_oneline};
use paths::normalize_path;
use ui::draw_ui;

const TICK_INTERVAL: Duration = Duration::from_secs(1);

fn main() -> stdio::Result<()> {
    let cli = parse_cli(std::env::args().skip(1));
    if cli.show_help {
        print_help();
        return Ok(());
    }

    let normalized_path = normalize_path(cli.config_path);
    let mut app = App::new(normalized_path);
    app.apply_overrides(&cli.overrides);

    match cli.output_mode {
        OutputMode::Oneline => {
            print_oneline(&app);
            return Ok(());
        }
        OutputMode::Json => {
            if let Err(err) = print_json(&app) {
                return Err(stdio::Error::new(stdio::ErrorKind::Other, err));
            }
            return Ok(());
        }
        OutputMode::Tui => {}
    }

    enable_raw_mode()?;
    let mut stdout = stdio::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> stdio::Result<()> {
    loop {
        terminal.draw(|frame| draw_ui(frame, app))?;

        let poll_deadline = app
            .poll_interval()
            .checked_sub(app.last_poll_elapsed())
            .unwrap_or(Duration::from_secs(0));
        let timeout = std::cmp::min(poll_deadline, TICK_INTERVAL);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if handle_key(app, key)? {
                    return Ok(());
                }
            }
        }

        if app.last_poll_elapsed() >= app.poll_interval() {
            app.reload_from_disk();
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> stdio::Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Tab => app.editor_next(),
        KeyCode::BackTab => app.editor_previous(),
        KeyCode::Enter => app.apply_editor_input(),
        KeyCode::Char('s') => app.save(),
        KeyCode::Char('r') => {
            if app.is_editing() {
                app.push_char('r');
            } else {
                app.reload_from_disk();
            }
        }
        KeyCode::Char('x') => app.reset(),
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(false);
            }
            app.push_char(c);
        }
        KeyCode::Backspace => {
            app.pop_char();
        }
        _ => {}
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_hex_formats() {
        assert_eq!(editor::normalize_hex("#abcdef").unwrap(), "#ABCDEF");
        assert_eq!(editor::normalize_hex("123456").unwrap(), "#123456");
        assert!(editor::normalize_hex("#12345").is_err());
    }

    #[test]
    fn normalize_path_for_config_dir() {
        let input = Some(paths::home_dir().join(".config"));
        let result = paths::normalize_path(input);
        assert!(result.ends_with("skyset/latest.yml"));
    }
}
