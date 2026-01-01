use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::theme::{palette, styles, AnimationState};
use crate::views::{Launcher, LauncherState};

pub struct RenderCache<K: Hash + Eq, V> {
    cache: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K: Hash + Eq, V: Clone> RenderCache<K, V> {
    pub fn new(ttl_ms: u64) -> Self {
        Self { cache: HashMap::new(), ttl: Duration::from_millis(ttl_ms) }
    }

    pub fn get_or_compute<F: FnOnce() -> V>(&mut self, key: K, compute: F) -> V {
        let now = Instant::now();
        if let Some((val, ts)) = self.cache.get(&key) {
            if now.duration_since(*ts) < self.ttl { return val.clone(); }
        }
        let val = compute();
        self.cache.insert(key, (val.clone(), now));
        val
    }

    pub fn invalidate(&mut self, key: &K) { self.cache.remove(key); }
    pub fn clear(&mut self) { self.cache.clear(); }
}

pub struct LazyValue<T> {
    value: Option<T>,
    dirty: bool,
}

impl<T> Default for LazyValue<T> {
    fn default() -> Self { Self { value: None, dirty: true } }
}

impl<T> LazyValue<T> {
    pub fn get_or_compute<F: FnOnce() -> T>(&mut self, compute: F) -> &T {
        if self.dirty || self.value.is_none() {
            self.value = Some(compute());
            self.dirty = false;
        }
        self.value.as_ref().expect("value computed")
    }

    pub fn mark_dirty(&mut self) { self.dirty = true; }
    pub fn is_dirty(&self) -> bool { self.dirty }
}

pub struct LayoutCache {
    cache: HashMap<(u16, u16), Vec<Rect>>,
}

impl LayoutCache {
    pub fn new() -> Self { Self { cache: HashMap::new() } }

    pub fn get_or_compute(&mut self, area: Rect, compute: impl FnOnce(Rect) -> Vec<Rect>) -> Vec<Rect> {
        let key = (area.width, area.height);
        self.cache.entry(key).or_insert_with(|| compute(area)).clone()
    }

    pub fn invalidate(&mut self) { self.cache.clear(); }
}

pub fn render(frame: &mut Frame, animation: &AnimationState, launcher_state: &mut LauncherState) {
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
        " [Esc] Quit  [Up/Down] Navigate  [Tab] Focus  [Enter] Execute  [Ctrl+U] Clear",
    ))
    .style(styles::text_muted());
    frame.render_widget(help, area);
}
