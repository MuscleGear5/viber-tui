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
use std::{io, process::Command, time::Duration};

use data::{Action, ActionRegistry};
use theme::{palette, styles, AnimationState, TICK_RATE_MS};
use views::{Launcher, LauncherState};

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

                        (_, KeyCode::Tab) => launcher_state.toggle_preview(),

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

fn execute_action(action: &Action) -> Result<()> {
    println!(
        "\x1b[36m>\x1b[0m Executing: \x1b[1;35m{}\x1b[0m",
        action.name
    );
    println!("\x1b[90m  {}\x1b[0m", action.description);
    println!();

    match action.category {
        data::ActionCategory::Mcp => {
            println!("\x1b[33mMCP Tool:\x1b[0m {}", action.invocation);
            println!("\x1b[90mCopy this to your AI assistant or run via MCP client\x1b[0m");
        }
        data::ActionCategory::Agent => {
            println!("\x1b[32mAgent:\x1b[0m {}", action.invocation);
            println!("\x1b[90mUse this agent type in your Task tool calls\x1b[0m");
        }
        data::ActionCategory::Skill => {
            println!("\x1b[34mSkill:\x1b[0m {}", action.invocation);
            println!("\x1b[90mInvoke this skill in your AI assistant\x1b[0m");
        }
        data::ActionCategory::Command => {
            println!("\x1b[35mRunning command:\x1b[0m {}", action.invocation);
            println!();

            let status = Command::new("sh")
                .arg("-c")
                .arg(&action.invocation)
                .status()?;

            if !status.success() {
                eprintln!("\x1b[31mCommand exited with status: {}\x1b[0m", status);
            }
        }
    }

    Ok(())
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
        " [Esc] Quit  [Up/Down] Navigate  [Tab] Preview  [Enter] Execute",
    ))
    .style(styles::text_muted());
    frame.render_widget(help, area);
}
