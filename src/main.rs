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
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use app::{App, AppAction, View};
use data::Action;
use events::{map_key_event, AppEvent};
use views::InputHandler;
use execute::execute_action;
use integrations::{NvimMcpRunner, NvimMcpCommand};
use render::render;
use theme::TICK_RATE_MS;

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

    if let Ok(runner) = NvimMcpRunner::spawn() {
        runner.send(NvimMcpCommand::GetTargets).ok();
        app.set_nvim_runner(runner);
    }

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
                    if let Some(app_event) = map_key_event(key) {
                        let action = handle_event(&mut app, app_event);

                        match action {
                            AppAction::Continue => {}
                            AppAction::Quit => return Ok(AppResult::Quit),
                            AppAction::Execute(action) => return Ok(AppResult::Execute(action)),
                            AppAction::SwitchView(view) => app.switch_view(view),
                        }
                    }
                }
            }
        }

        app.tick();
    }
}

fn handle_event(app: &mut App, event: AppEvent) -> AppAction {
    if app.modal.has_modal() {
        match event {
            AppEvent::Cancel => {
                app.modal.dismiss();
                return AppAction::Continue;
            }
            AppEvent::Select => {
                if let Some(_key) = app.modal.selected_key() {
                    app.modal.dismiss();
                }
                return AppAction::Continue;
            }
            AppEvent::NextFocus => {
                app.modal.select_next();
                return AppAction::Continue;
            }
            AppEvent::PrevFocus => {
                app.modal.select_prev();
                return AppAction::Continue;
            }
            AppEvent::Char(c) => {
                if app.modal.handle_key(c).is_none() {
                    app.modal.input_char(c);
                }
                return AppAction::Continue;
            }
            AppEvent::Backspace => {
                app.modal.input_backspace();
                return AppAction::Continue;
            }
            _ => return AppAction::Continue,
        }
    }

    match &event {
        AppEvent::ForceQuit => return AppAction::Quit,
        AppEvent::SwitchView(c) => {
            if let Some(view) = char_to_view(*c) {
                return AppAction::SwitchView(view);
            }
        }
        AppEvent::Quit => {
            if app.show_help {
                app.toggle_help();
                return AppAction::Continue;
            }
            if app.current_view != View::Launcher {
                return AppAction::SwitchView(View::Launcher);
            }
            return AppAction::Quit;
        }
        _ => {}
    }

    match app.current_view {
        View::Launcher => handle_launcher_event(app, event),
        View::Chat => handle_chat_event(app, event),
        View::Workflow => handle_workflow_event(app, event),
        View::Tasks => handle_tasks_event(app, event),
        View::Agents => handle_agents_event(app, event),
        View::Buffer => handle_buffer_event(app, event),
        View::Diff => handle_diff_event(app, event),
        View::Lsp => handle_lsp_event(app, event),
        View::Help => AppAction::Continue,
    }
}

fn char_to_view(c: char) -> Option<View> {
    match c {
        '1' | 'l' => Some(View::Launcher),
        '2' | 'c' => Some(View::Chat),
        '3' | 'w' => Some(View::Workflow),
        '4' | 't' => Some(View::Tasks),
        '5' | 'a' => Some(View::Agents),
        '6' | 'b' => Some(View::Buffer),
        '7' | 'd' => Some(View::Diff),
        '8' | 's' => Some(View::Lsp),
        _ => None,
    }
}

fn handle_launcher_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.launcher.select_previous(),
        AppEvent::Down => app.launcher.select_next(),
        AppEvent::PageUp => app.launcher.page_up(),
        AppEvent::PageDown => app.launcher.page_down(),
        AppEvent::Home => app.launcher.move_cursor_start(),
        AppEvent::End => app.launcher.move_cursor_end(),
        AppEvent::Left => app.launcher.move_cursor_left(),
        AppEvent::Right => app.launcher.move_cursor_right(),
        AppEvent::Backspace => app.launcher.delete_char(),
        AppEvent::Delete => app.launcher.delete_char_forward(),
        AppEvent::ClearLine => app.launcher.clear_input(),
        AppEvent::NextFocus => app.launcher.cycle_focus(),
        AppEvent::PrevFocus => app.launcher.cycle_focus_reverse(),
        AppEvent::Select => {
            if let Some(action) = app.launcher.selected_action() {
                return AppAction::Execute(action.clone());
            }
        }
        AppEvent::Char(c) => {
            if c == '?' {
                app.toggle_help();
            } else {
                app.launcher.insert_char(c);
            }
        }
        _ => {}
    }
    AppAction::Continue
}

fn handle_chat_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.chat.scroll_up(1),
        AppEvent::Down => app.chat.scroll_down(1),
        AppEvent::PageUp => app.chat.scroll_up(10),
        AppEvent::PageDown => app.chat.scroll_down(10),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_workflow_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.workflow.select_prev(),
        AppEvent::Down => app.workflow.select_next(),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_tasks_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.tasks.select_prev(),
        AppEvent::Down => app.tasks.select_next(),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_agents_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.agents_view.select_prev(),
        AppEvent::Down => app.agents_view.select_next(),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_buffer_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.buffer.move_cursor_up(),
        AppEvent::Down => app.buffer.move_cursor_down(30),
        AppEvent::PageUp => app.buffer.scroll_up(10),
        AppEvent::PageDown => app.buffer.scroll_down(10, 30),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_diff_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.diff.select_prev(),
        AppEvent::Down => app.diff.select_next(),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}

fn handle_lsp_event(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Up => app.lsp.select_prev(),
        AppEvent::Down => app.lsp.select_next(),
        AppEvent::Char('?') => app.toggle_help(),
        _ => {}
    }
    AppAction::Continue
}
