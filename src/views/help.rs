use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};
use crate::theme::colors::palette;

pub struct KeyBinding {
    pub keys: &'static str,
    pub action: &'static str,
}

pub struct KeySection {
    pub title: &'static str,
    pub bindings: &'static [KeyBinding],
}

pub const GLOBAL_KEYS: KeySection = KeySection {
    title: "Global",
    bindings: &[
        KeyBinding { keys: "?", action: "Toggle help" },
        KeyBinding { keys: "q / Esc", action: "Quit / Close" },
        KeyBinding { keys: "Tab", action: "Next view" },
        KeyBinding { keys: "S-Tab", action: "Prev view" },
        KeyBinding { keys: ":", action: "Command mode" },
        KeyBinding { keys: "C-z", action: "Undo" },
    ],
};

pub const NAV_KEYS: KeySection = KeySection {
    title: "Navigation",
    bindings: &[
        KeyBinding { keys: "j / Down", action: "Move down" },
        KeyBinding { keys: "k / Up", action: "Move up" },
        KeyBinding { keys: "h / Left", action: "Move left" },
        KeyBinding { keys: "l / Right", action: "Move right" },
        KeyBinding { keys: "g g", action: "Go to top" },
        KeyBinding { keys: "G", action: "Go to bottom" },
        KeyBinding { keys: "C-d", action: "Page down" },
        KeyBinding { keys: "C-u", action: "Page up" },
    ],
};

pub const EDIT_KEYS: KeySection = KeySection {
    title: "Editing",
    bindings: &[
        KeyBinding { keys: "i", action: "Insert mode" },
        KeyBinding { keys: "Enter", action: "Confirm / Submit" },
        KeyBinding { keys: "Space", action: "Toggle / Select" },
        KeyBinding { keys: "d d", action: "Delete item" },
        KeyBinding { keys: "y y", action: "Copy item" },
        KeyBinding { keys: "p", action: "Paste" },
    ],
};

pub const WORKFLOW_KEYS: KeySection = KeySection {
    title: "Workflow",
    bindings: &[
        KeyBinding { keys: "1-5", action: "Jump to phase" },
        KeyBinding { keys: "n", action: "Next phase" },
        KeyBinding { keys: "N", action: "Prev phase" },
        KeyBinding { keys: "a", action: "Approve spec" },
        KeyBinding { keys: "r", action: "Reject / Request changes" },
    ],
};

pub struct HelpOverlayState {
    pub visible: bool,
    pub scroll: u16,
}

impl Default for HelpOverlayState {
    fn default() -> Self {
        Self { visible: false, scroll: 0 }
    }
}

impl HelpOverlayState {
    pub fn toggle(&mut self) { self.visible = !self.visible; }
    pub fn show(&mut self) { self.visible = true; self.scroll = 0; }
    pub fn hide(&mut self) { self.visible = false; }
    pub fn scroll_down(&mut self) { self.scroll = self.scroll.saturating_add(1); }
    pub fn scroll_up(&mut self) { self.scroll = self.scroll.saturating_sub(1); }
}

pub struct HelpOverlay;

impl HelpOverlay {
    pub fn new() -> Self { Self }
}

impl StatefulWidget for HelpOverlay {
    type State = HelpOverlayState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if !state.visible { return; }
        let w = area.width.min(60);
        let h = area.height.min(30);
        let x = area.x + (area.width.saturating_sub(w)) / 2;
        let y = area.y + (area.height.saturating_sub(h)) / 2;
        let popup = Rect::new(x, y, w, h);
        Clear.render(popup, buf);
        let sections = [&GLOBAL_KEYS, &NAV_KEYS, &EDIT_KEYS, &WORKFLOW_KEYS];
        let mut lines = Vec::with_capacity(40);
        for (i, sec) in sections.iter().enumerate() {
            if i > 0 { lines.push(String::new()); }
            lines.push(format!("\u{F0499} {}", sec.title));
            for kb in sec.bindings.iter() {
                lines.push(format!("  {:12} {}", kb.keys, kb.action));
            }
        }
        let text = lines.join("\n");
        let para = Paragraph::new(text)
            .block(Block::default()
                .title(" \u{F0625} Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(palette::CYAN))
                .style(Style::default().bg(palette::BG_ELEVATED)))
            .style(Style::default().fg(palette::TEXT_SECONDARY))
            .wrap(Wrap { trim: false })
            .scroll((state.scroll, 0));
        para.render(popup, buf);
    }
}
