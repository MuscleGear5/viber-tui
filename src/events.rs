//! Event handling system with vim-like keybindings
//!
//! Provides async event polling and a global keybinding system.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;

pub type EventSender = mpsc::UnboundedSender<AppEvent>;
pub type EventReceiver = mpsc::UnboundedReceiver<AppEvent>;

pub fn event_channel() -> (EventSender, EventReceiver) {
    mpsc::unbounded_channel()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    // Navigation
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    
    // Selection
    Select,
    Cancel,
    
    // Input
    Char(char),
    Backspace,
    Delete,
    ClearLine,
    
    // Focus
    NextFocus,
    PrevFocus,
    
    // Views
    SwitchView(char),
    TogglePreview,
    ToggleHelp,
    
    // System
    Quit,
    ForceQuit,
    Tick,
}

/// Poll for events with timeout, returns None if no event available
pub fn poll_event(timeout_ms: u64) -> Option<Event> {
    if event::poll(Duration::from_millis(timeout_ms)).ok()? {
        event::read().ok()
    } else {
        None
    }
}

/// Convert a crossterm KeyEvent to an AppEvent using vim-like bindings
pub fn map_key_event(key: KeyEvent) -> Option<AppEvent> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);

    match (ctrl, alt, key.code) {
        // Quit
        (true, _, KeyCode::Char('c')) => Some(AppEvent::ForceQuit),
        (_, _, KeyCode::Esc) => Some(AppEvent::Cancel),
        (true, _, KeyCode::Char('q')) => Some(AppEvent::Quit),

        // Vim navigation (when not in input mode)
        (false, false, KeyCode::Char('j')) => None, // Let views handle j/k
        (false, false, KeyCode::Char('k')) => None,
        
        // Arrow navigation (always works)
        (_, _, KeyCode::Up) => Some(AppEvent::Up),
        (_, _, KeyCode::Down) => Some(AppEvent::Down),
        (_, _, KeyCode::Left) => Some(AppEvent::Left),
        (_, _, KeyCode::Right) => Some(AppEvent::Right),
        
        // Ctrl navigation
        (true, _, KeyCode::Char('p')) => Some(AppEvent::Up),
        (true, _, KeyCode::Char('n')) => Some(AppEvent::Down),
        (true, _, KeyCode::Char('b')) => Some(AppEvent::Left),
        (true, _, KeyCode::Char('f')) => Some(AppEvent::Right),
        
        // Page navigation
        (_, _, KeyCode::PageUp) => Some(AppEvent::PageUp),
        (_, _, KeyCode::PageDown) => Some(AppEvent::PageDown),
        (true, _, KeyCode::Char('u')) => Some(AppEvent::PageUp),
        (true, _, KeyCode::Char('d')) => Some(AppEvent::PageDown),
        
        // Line navigation
        (_, _, KeyCode::Home) => Some(AppEvent::Home),
        (_, _, KeyCode::End) => Some(AppEvent::End),
        (true, _, KeyCode::Char('a')) => Some(AppEvent::Home),
        (true, _, KeyCode::Char('e')) => Some(AppEvent::End),
        
        // Selection
        (_, _, KeyCode::Enter) => Some(AppEvent::Select),
        
        // Input editing
        (_, _, KeyCode::Backspace) => Some(AppEvent::Backspace),
        (_, _, KeyCode::Delete) => Some(AppEvent::Delete),
        (true, _, KeyCode::Char('w')) => Some(AppEvent::ClearLine),
        
        // Focus
        (_, _, KeyCode::Tab) => Some(AppEvent::NextFocus),
        (_, _, KeyCode::BackTab) => Some(AppEvent::PrevFocus),
        
        // View switching (Alt+key)
        (_, true, KeyCode::Char(c)) => Some(AppEvent::SwitchView(c)),
        
        // Toggle preview
        (true, _, KeyCode::Char('o')) => Some(AppEvent::TogglePreview),
        
        // Help
        (_, _, KeyCode::F(1)) => Some(AppEvent::ToggleHelp),
        (true, _, KeyCode::Char('?')) => Some(AppEvent::ToggleHelp),
        
        // Regular character input
        (false, false, KeyCode::Char(c)) => Some(AppEvent::Char(c)),
        
        _ => None,
    }
}

/// Keybinding context - determines which keybindings are active
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyContext {
    #[default]
    Normal,
    Input,
    List,
    Preview,
}

impl KeyContext {
    /// Check if vim motion keys (j/k/h/l) should be navigation
    pub fn vim_motions_active(self) -> bool {
        matches!(self, KeyContext::Normal | KeyContext::List)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctrl_c_force_quits() {
        let key = KeyEvent::new_with_kind(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
        );
        assert_eq!(map_key_event(key), Some(AppEvent::ForceQuit));
    }

    #[test]
    fn test_arrow_navigation() {
        let key = KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::NONE, KeyEventKind::Press);
        assert_eq!(map_key_event(key), Some(AppEvent::Up));
    }
}
