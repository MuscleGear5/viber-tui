mod agents;
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

use data::{Action, ActionRegistry};
use execute::execute_action;
use render::render;
use theme::{AnimationState, TICK_RATE_MS};
use views::{InputHandler, LauncherState};

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
    let registry = ActionRegistry::load_from_file("data/actions.yaml")?;
    let mut animation = AnimationState::new();
    let mut launcher_state = LauncherState::new(&registry);

    loop {
        terminal.draw(|frame| render(frame, &animation, &mut launcher_state))?;

        if event::poll(Duration::from_millis(TICK_RATE_MS))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Esc) => return Ok(AppResult::Quit),
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(AppResult::Quit),

                        (_, KeyCode::Up) | (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                            launcher_state.select_previous();
                        }
                        (_, KeyCode::Down) | (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                            launcher_state.select_next();
                        }
                        (_, KeyCode::PageUp) => launcher_state.page_up(),
                        (_, KeyCode::PageDown) => launcher_state.page_down(),
                        (_, KeyCode::Home) | (KeyModifiers::CONTROL, KeyCode::Char('a')) => {
                            launcher_state.move_cursor_start();
                        }
                        (_, KeyCode::End) | (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                            launcher_state.move_cursor_end();
                        }

                        (_, KeyCode::Left) => launcher_state.move_cursor_left(),
                        (_, KeyCode::Right) => launcher_state.move_cursor_right(),
                        (_, KeyCode::Backspace) => launcher_state.delete_char(),
                        (_, KeyCode::Delete) | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                            launcher_state.delete_char_forward();
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('u')) => {
                            launcher_state.clear_input();
                        }

                        (_, KeyCode::Tab) => launcher_state.cycle_focus(),
                        (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                            launcher_state.cycle_focus_reverse()
                        }

                        (_, KeyCode::Enter) => {
                            if let Some(action) = launcher_state.selected_action() {
                                return Ok(AppResult::Execute(action.clone()));
                            }
                        }

                        (_, KeyCode::Char(c)) => launcher_state.insert_char(c),
                        _ => {}
                    }
                }
            }
        }

        animation.tick();
        launcher_state.tick();
    }
}
