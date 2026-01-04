# Rust Patterns for VIBER-TUI

## Ratatui Conventions

### Widget Pattern
```rust
pub struct MyWidget<'a> {
    data: &'a SomeData,
    style: Style,
}

impl<'a> MyWidget<'a> {
    pub fn new(data: &'a SomeData) -> Self {
        Self { data, style: Style::default() }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for MyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // render logic
    }
}
```

### State Pattern
```rust
pub struct MyWidgetState {
    selected: usize,
    offset: usize,
}

impl MyWidgetState {
    pub fn new() -> Self { Self { selected: 0, offset: 0 } }
    pub fn select_next(&mut self, total: usize) { ... }
    pub fn select_prev(&mut self) { ... }
}
```

## Error Handling

```rust
use anyhow::{Result, Context};

fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("failed to read config file")?;
    serde_yaml::from_str(&content)
        .context("failed to parse config")
}
```

## Nerd Font Icons

```rust
pub mod icons {
    pub const FOLDER: &str = "\u{F07B}";
    pub const FILE: &str = "\u{F15B}";
    pub const GIT: &str = "\u{F1D3}";
    pub const CHECK: &str = "\u{F00C}";
    pub const ERROR: &str = "\u{F00D}";
    pub const WARN: &str = "\u{F071}";
    pub const INFO: &str = "\u{F05A}";
    pub const SEARCH: &str = "\u{F002}";
    pub const COG: &str = "\u{F013}";
    pub const PLAY: &str = "\u{F04B}";
    pub const PAUSE: &str = "\u{F04C}";
    pub const STOP: &str = "\u{F04D}";
}
```

## Async Pattern

```rust
use tokio::sync::mpsc;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    AgentUpdate(AgentId, AgentStatus),
}

async fn event_loop(tx: mpsc::Sender<AppEvent>) {
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                tx.send(AppEvent::Key(key)).await?;
            }
        }
        tx.send(AppEvent::Tick).await?;
    }
}
```

## Module Organization

```
src/
  main.rs           # Entry, app loop
  lib.rs            # Library exports (if needed)
  app.rs            # App state, event handling
  data/
    mod.rs          # pub use re-exports
    actions.rs      # Action definitions
    config.rs       # Config loading
  theme/
    mod.rs
    colors.rs       # Color palette
    styles.rs       # Style builders
    animation.rs    # Animation state
    icons.rs        # Nerd font icons
  views/
    mod.rs
    chat.rs
    workflow.rs
    agents.rs
    tasks.rs
  widgets/
    mod.rs
    fuzzy_list.rs
    action_card.rs
    dag.rs
    sparkline.rs
```
