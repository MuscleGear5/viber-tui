mod agents;
mod app;
mod data;
mod events;
mod execute;
mod integrations;
mod render;
mod theme;
mod views;
mod widgets;

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use app::App;
use data::Action;
use execute::execute_action;
use render::render;
use theme::TICK_RATE_MS;
use views::InputHandler;

enum AppResult {
    Quit,
    Execute(Action),
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match result {
        Ok(AppResult::Quit) => {}
        Ok(AppResult::Execute(action)) => {
            execute_action(&action)?;
        }
        Err(err) => {
            eprintln!("Error: {err:?}");
        }
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<AppResult> {
    let mut app = App::load()?;

    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if app.should_quit() {
            return match app.take_pending_action() {
                Some(action) => Ok(AppResult::Execute(action)),
                None => Ok(AppResult::Quit),
            };
        }

        if event::poll(Duration::from_millis(TICK_RATE_MS))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Esc) => return Ok(AppResult::Quit),
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(AppResult::Quit),
                        (_, KeyCode::Char('?')) => app.toggle_help(),

                        (_, KeyCode::Up) | (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                            app.launcher.select_previous();
                        }
                        (_, KeyCode::Down) | (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                            app.launcher.select_next();
                        }
                        (_, KeyCode::PageUp) => app.launcher.page_up(),
                        (_, KeyCode::PageDown) => app.launcher.page_down(),
                        (_, KeyCode::Home) | (KeyModifiers::CONTROL, KeyCode::Char('a')) => {
                            app.launcher.move_cursor_start();
                        }
                        (_, KeyCode::End) | (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                            app.launcher.move_cursor_end();
                        }

                        (_, KeyCode::Left) => app.launcher.move_cursor_left(),
                        (_, KeyCode::Right) => app.launcher.move_cursor_right(),
                        (_, KeyCode::Backspace) => app.launcher.delete_char(),
                        (_, KeyCode::Delete) | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                            app.launcher.delete_char_forward();
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('u')) => {
                            app.launcher.clear_input();
                        }

                        (_, KeyCode::Tab) => app.launcher.cycle_focus(),
                        (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                            app.launcher.cycle_focus_reverse()
                        }

                        (_, KeyCode::Enter) => {
                            if let Some(action) = app.launcher.selected_action() {
                                return Ok(AppResult::Execute(action.clone()));
                            }
                        }

                        (_, KeyCode::Char(c)) => app.launcher.insert_char(c),
                        _ => {}
                    }
                }
            }
        }

        app.tick();
    }
}
