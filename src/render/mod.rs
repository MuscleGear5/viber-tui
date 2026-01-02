mod cache;

pub use cache::{LayoutCache, LazyValue, RenderCache};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, View};
use crate::theme::{palette, styles, AnimationState};
use crate::views::{
    Agents, BufferView, Chat, DiffView, HelpOverlay, Launcher, LspView, Tasks, Workflow,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(ratatui::style::Style::default().bg(palette::BG_VOID)),
        area,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    render_header(frame, chunks[0], &app.animation);
    render_content(frame, chunks[1], app);
    render_footer(frame, chunks[2], app.current_view);

    if app.show_help {
        let overlay = HelpOverlay::new();
        frame.render_stateful_widget(overlay, area, &mut app.help);
    }
}

fn render_content(frame: &mut Frame, area: Rect, app: &mut App) {
    match app.current_view {
        View::Launcher => {
            let w = Launcher::new(&app.animation);
            frame.render_stateful_widget(w, area, &mut app.launcher);
        }
        View::Chat => {
            let w = Chat::new(&app.animation);
            frame.render_stateful_widget(w, area, &mut app.chat);
        }
        View::Workflow => {
            let w = Workflow::new();
            frame.render_stateful_widget(w, area, &mut app.workflow);
        }
        View::Tasks => {
            let w = Tasks::new();
            frame.render_stateful_widget(w, area, &mut app.tasks);
        }
        View::Agents => {
            let w = Agents::new();
            frame.render_stateful_widget(w, area, &mut app.agents_view);
        }
        View::Buffer => {
            let w = BufferView::new();
            frame.render_stateful_widget(w, area, &mut app.buffer);
        }
        View::Diff => {
            let w = DiffView::new();
            frame.render_stateful_widget(w, area, &mut app.diff);
        }
        View::Lsp => {
            let w = LspView::new();
            frame.render_stateful_widget(w, area, &mut app.lsp);
        }
        View::Help => {
            let overlay = HelpOverlay::new();
            frame.render_stateful_widget(overlay, area, &mut app.help);
        }
    }
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

fn render_footer(frame: &mut Frame, area: Rect, current_view: View) {
    let help_text = match current_view {
        View::Launcher => " [Esc] Quit  [Up/Down] Navigate  [Tab] Focus  [Enter] Execute  [?] Help",
        View::Chat => " [Esc] Back  [Enter] Send  [Up/Down] Scroll  [?] Help",
        View::Workflow => " [Esc] Back  [Tab] Next Phase  [Enter] Select  [?] Help",
        View::Tasks => " [Esc] Back  [j/k] Navigate  [Enter] Toggle  [?] Help",
        View::Agents => " [Esc] Back  [j/k] Navigate  [Space] Pause  [?] Help",
        View::Buffer => " [Esc] Back  [Ctrl+S] Save  [Ctrl+Z] Undo  [?] Help",
        View::Diff => " [Esc] Back  [j/k] Navigate  [Enter] Accept  [?] Help",
        View::Lsp => " [Esc] Back  [Enter] Go to  [Tab] Switch  [?] Help",
        View::Help => " [Esc] Close  [j/k] Scroll  [q] Quit",
    };
    let help = Paragraph::new(Span::raw(help_text)).style(styles::text_muted());
    frame.render_widget(help, area);
}
