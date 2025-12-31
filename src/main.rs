mod data;
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
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};

use data::ActionRegistry;
use theme::{palette, styles, AnimationState, TICK_RATE_MS};
use views::{Launcher, LauncherState};

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

    if let Err(err) = result {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let registry = ActionRegistry::load_from_file("data/actions.yaml")?;
    let mut animation = AnimationState::new();
    let mut launcher_state = LauncherState::new(&registry);

    loop {
        terminal.draw(|frame| render(frame, &animation, &mut launcher_state))?;

        if event::poll(Duration::from_millis(TICK_RATE_MS))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Esc) => return Ok(()),
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(()),

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

                        (_, KeyCode::Tab) => launcher_state.toggle_preview(),

                        (_, KeyCode::Enter) => {
                            if let Some(action) = launcher_state.selected_action() {
                                eprintln!("Selected: {} ({})", action.name, action.invocation);
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

fn render(frame: &mut Frame, animation: &AnimationState, launcher_state: &mut LauncherState) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(ratatui::style::Style::default().bg(palette::BG_VOID)),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, chunks[0], animation);
    render_launcher(frame, chunks[1], animation, launcher_state);
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, area: Rect, animation: &AnimationState) {
    let viber_eye = animation.viber_eye();
    let vibe_wave = animation.vibe_wave_short();

    let header = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(styles::border())
        .style(ratatui::style::Style::default().bg(palette::BG_PANEL));

    let inner = header.inner(area);
    frame.render_widget(header, area);

    let title =
        Paragraph::new(format!("  VIBER TUI  {} {}", viber_eye, vibe_wave)).style(styles::title());
    frame.render_widget(title, inner);
}

fn render_launcher(
    frame: &mut Frame,
    area: Rect,
    animation: &AnimationState,
    launcher_state: &mut LauncherState,
) {
    let launcher = Launcher::new(animation);
    frame.render_stateful_widget(launcher, area, launcher_state);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(Span::raw(
        " [Esc] Quit  [↑↓] Navigate  [Tab] Toggle Preview  [Enter] Select",
    ))
    .style(styles::text_muted());
    frame.render_widget(help, area);
}
