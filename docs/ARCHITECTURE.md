# VIBER-TUI Architecture Guide

> Patterns and conventions for views, widgets, and state management

## Overview

VIBER-TUI follows a strict separation between **Views** (full-screen UI components) and **Widgets** (reusable building blocks). Both use the same fundamental pattern but serve different purposes.

```
src/
├── views/           # Full-screen UI components (Chat, Tasks, Workflow...)
│   └── <view>/
│       ├── mod.rs       # Exports
│       ├── state.rs     # State struct + methods
│       ├── widget.rs    # StatefulWidget impl
│       ├── render.rs    # Helper functions
│       └── preview.rs   # Optional preview component
│
└── widgets/         # Reusable building blocks (Modal, Toast, Sparkline...)
    └── <widget>/
        ├── mod.rs       # Exports
        ├── models.rs    # Data types
        ├── state.rs     # State management
        ├── widget.rs    # Widget impl
        └── render.rs    # Render helpers
```

---

## The Three-Layer Pattern

Every view and widget follows this structure:

### Layer 1: Models (`models.rs`)

Pure data types with no behavior. These define WHAT can exist.

```rust
// src/widgets/toast/models.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub kind: ToastKind,
    pub title: String,
    pub message: Option<String>,
    pub actions: Vec<ToastAction>,
    pub ttl: Duration,
    pub created: Instant,
}

#[derive(Debug, Clone)]
pub struct ToastAction {
    pub label: String,
    pub hotkey: Option<char>,
}
```

**Rules:**
- Derive `Debug`, `Clone` at minimum
- Use `&'static str` for icons (nerd font unicode)
- Keep models simple - no methods beyond constructors
- Factory methods OK for common patterns

### Layer 2: State (`state.rs`)

Manages the runtime state. Provides mutation methods and queries.

```rust
// src/widgets/toast/state.rs

pub struct ToastState {
    toasts: Vec<Toast>,
    next_id: u64,
    selected_action: usize,
}

impl ToastState {
    // Constructor
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 0,
            selected_action: 0,
        }
    }

    // Mutations
    pub fn push(&mut self, kind: ToastKind, title: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(Toast { id, kind, title: title.into(), ... });
        id
    }

    pub fn dismiss(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.toasts.retain(|t| now.duration_since(t.created) < t.ttl);
    }

    // Queries (read-only)
    pub fn active(&self) -> Option<&Toast> {
        self.toasts.first()
    }

    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    pub fn count(&self) -> usize {
        self.toasts.len()
    }
}
```

**Rules:**
- State is the ONLY place mutation happens
- Query methods return `&T` or primitives, never owned data
- Use `impl Into<String>` for string parameters
- Implement `Default` if sensible defaults exist

### Layer 3: Widget (`widget.rs`)

Renders state to the terminal. Implements ratatui traits.

```rust
// src/widgets/toast/widget.rs

pub struct ToastWidget<'a> {
    state: &'a ToastState,
}

impl<'a> ToastWidget<'a> {
    pub fn new(state: &'a ToastState) -> Self {
        Self { state }
    }

    /// Render as overlay (uses Frame directly)
    pub fn render_overlay(&self, frame: &mut Frame) {
        if let Some(toast) = self.state.active() {
            let area = toast_area(frame.area());
            frame.render_widget(Clear, area);
            // ... render toast content
        }
    }
}

/// For inline rendering (uses Buffer)
impl Widget for ToastWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(toast) = self.state.active() {
            // ... render to buffer
        }
    }
}
```

**Two Rendering Modes:**

| Mode | Trait | Use Case |
|------|-------|----------|
| `Widget` | `fn render(self, area: Rect, buf: &mut Buffer)` | Inline content within a layout |
| Overlay | `fn render_overlay(&self, frame: &mut Frame)` | Floating UI (modals, toasts) |

---

## Views vs Widgets

### Views

Full-screen components that occupy the entire terminal or a major section.

```rust
// src/views/launcher/widget.rs

pub struct Launcher<'a> {
    animation: &'a AnimationState,
}

impl<'a> StatefulWidget for Launcher<'a> {
    type State = LauncherState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Content
            Constraint::Length(1),  // Footer
        ]).split(area);

        render_header(chunks[0], buf, self.animation);
        render_content(chunks[1], buf, state);
        render_footer(chunks[2], buf);
    }
}
```

**View Characteristics:**
- Use `StatefulWidget` trait (state passed to render)
- Own their layout (split area into chunks)
- Call render helpers for sub-sections
- May contain widgets

### Widgets

Reusable components that render within a given area.

**Widget Characteristics:**
- Use `Widget` or provide `render_overlay`
- Receive area, don't define layout
- Self-contained rendering logic
- May be nested in views or other widgets

---

## Module Structure

### Standard `mod.rs` Pattern

```rust
// src/widgets/toast/mod.rs

mod models;
mod render;
mod state;
mod widget;

pub use models::{Toast, ToastAction, ToastKind};
pub use state::ToastState;
pub use widget::ToastWidget;
```

**Rules:**
- Keep `mod.rs` to exports only
- Private modules: `render` (helpers)
- Public exports: types users need

### Render Helpers (`render.rs`)

Pure functions that handle specific rendering tasks.

```rust
// src/widgets/toast/render.rs

use ratatui::prelude::*;
use crate::theme::colors;

pub fn toast_area(frame_area: Rect) -> Rect {
    let width = 50.min(frame_area.width.saturating_sub(4));
    let height = 5;
    Rect::new(
        frame_area.width.saturating_sub(width + 2),
        1,
        width,
        height,
    )
}

pub fn kind_color(kind: ToastKind) -> Color {
    match kind {
        ToastKind::Success => colors::SUCCESS,
        ToastKind::Error => colors::ERROR,
        ToastKind::Warning => colors::WARNING,
        ToastKind::Info => colors::INFO,
    }
}

pub fn render_toast_block(toast: &Toast) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(kind_color(toast.kind)))
        .title(format!(" {} ", toast.title))
}
```

**Rules:**
- Pure functions (no state mutation)
- Take minimal parameters
- Return ratatui primitives (Block, Span, Style)

---

## Wiring to App

### 1. Add State to App

```rust
// src/app.rs

use crate::widgets::modal::ModalState;
use crate::widgets::toast::ToastState;

pub struct App {
    pub view: View,
    pub animation: AnimationState,
    pub modal: ModalState,      // <-- Add here
    pub toast: ToastState,      // <-- Add here
}

impl App {
    pub fn new() -> Self {
        Self {
            view: View::Launcher,
            animation: AnimationState::new(),
            modal: ModalState::new(),
            toast: ToastState::new(),
        }
    }
}
```

### 2. Add Rendering Call

```rust
// src/render/mod.rs

use crate::widgets::modal::ModalWidget;
use crate::widgets::toast::ToastWidget;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // 1. Render main view
    match app.view {
        View::Launcher => { /* ... */ }
        View::Chat => { /* ... */ }
    }

    // 2. Render overlays (order matters - last = on top)
    render_toasts(frame, area, &app.toast);
    render_modal(frame, area, &app.modal);
}

fn render_toasts(frame: &mut Frame, _area: Rect, state: &ToastState) {
    if !state.is_empty() {
        let widget = ToastWidget::new(state);
        widget.render_overlay(frame);
    }
}

fn render_modal(frame: &mut Frame, _area: Rect, state: &ModalState) {
    if state.has_modal() {
        let widget = ModalWidget::new(state);
        widget.render_overlay(frame);
    }
}
```

### 3. Wire Input Handling

```rust
// src/main.rs (in handle_event)

fn handle_event(app: &mut App, event: AppEvent) -> AppAction {
    // Modal intercepts ALL input when active
    if app.modal.has_modal() {
        return handle_modal_input(app, event);
    }

    // Toast intercepts when it has actions
    if app.toast.active().map_or(false, |t| !t.actions.is_empty()) {
        if let Some(action) = handle_toast_input(&mut app.toast, &event) {
            return action;
        }
    }

    // Normal view handling
    match app.view {
        View::Launcher => handle_launcher_input(app, event),
        // ...
    }
}

fn handle_modal_input(app: &mut App, event: AppEvent) -> AppAction {
    match event {
        AppEvent::Cancel => {
            app.modal.dismiss();
            AppAction::Continue
        }
        AppEvent::Select => {
            if let Some(result) = app.modal.confirm() {
                // Handle button selection
            }
            AppAction::Continue
        }
        AppEvent::NextFocus => {
            app.modal.select_next();
            AppAction::Continue
        }
        AppEvent::PrevFocus => {
            app.modal.select_prev();
            AppAction::Continue
        }
        _ => AppAction::Continue,
    }
}
```

---

## File Size Limits

**HARD RULE: 150 lines max per file**

If a file exceeds 150 lines:
1. Extract render helpers to `render.rs`
2. Split complex state into sub-modules
3. Create focused helper files

```
# Good structure (all files < 150 lines)
widgets/modal/
├── mod.rs       (~15 lines)  - exports only
├── models.rs    (~60 lines)  - Modal, ModalButton, ModalKind
├── state.rs     (~80 lines)  - ModalState + methods
├── widget.rs    (~50 lines)  - ModalWidget + render_overlay
└── render.rs    (~100 lines) - layout, colors, block helpers
```

---

## Icon Convention

**NEVER use emoji characters. Use nerd font unicode.**

```rust
// src/theme/indicators.rs

pub const ICON_CHECK: &str = "\u{F00C0}";    // nf-md-check
pub const ICON_ERROR: &str = "\u{F0159}";    // nf-md-close
pub const ICON_WARN: &str = "\u{F0028}";     // nf-md-alert
pub const ICON_INFO: &str = "\u{F064E}";     // nf-md-information
pub const ICON_GEAR: &str = "\u{F013}";      // nf-fa-gear
pub const ICON_CODE: &str = "\u{F121}";      // nf-fa-code

// Usage in models
pub struct Toast {
    pub icon: &'static str,  // NOT char
    // ...
}
```

Reference: https://www.nerdfonts.com/cheat-sheet

---

## Quick Reference

### Creating a New Widget

```bash
mkdir -p src/widgets/my_widget
touch src/widgets/my_widget/{mod,models,state,widget,render}.rs
```

1. Define types in `models.rs`
2. Implement state in `state.rs`
3. Implement widget in `widget.rs`
4. Add helpers in `render.rs`
5. Export in `mod.rs`
6. Register in `src/widgets/mod.rs`
7. Wire to App if needed

### Creating a New View

```bash
mkdir -p src/views/my_view
touch src/views/my_view/{mod,state,widget,render}.rs
```

1. Define `MyViewState` in `state.rs`
2. Implement `StatefulWidget` in `widget.rs`
3. Add render helpers in `render.rs`
4. Export in `mod.rs`
5. Register in `src/views/mod.rs`
6. Add to `View` enum in `app.rs`
7. Add keybinding for view switch

### Checklist Before Commit

- [ ] All files < 150 lines
- [ ] No emoji (nerd font only)
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `lsp_diagnostics` clean

---

## Current Views

| View | Key | Purpose |
|------|-----|---------|
| Launcher | - | Start screen with action cards |
| Chat | C | AI conversation interface |
| Workflow | W | DAG visualization of tasks |
| Tasks | T | Task list management |
| Agents | A | Agent pool monitoring |
| Buffer | B | Code display |
| Diff | D | Change visualization |
| LSP | L | Diagnostics panel |
| Questionnaire | Q | Dynamic forms |
| Spec | S | Specification display |
| Help | ? | Keybinding reference |

## Current Widgets

| Widget | Purpose |
|--------|---------|
| ActionCard | Display selectable actions |
| Canvas | Low-level drawing surface |
| DAG | Directed acyclic graph renderer |
| FuzzyList | Searchable list with filtering |
| Heatmap | Activity calendar visualization |
| Modal | Dialog boxes (confirm, input, info) |
| Sparkline | Mini line charts |
| Toast | Notification popups |
| ViberStatus | Status bar component |

---

*Architecture Version: 1.0*
*Last Updated: 2026-01-03*
