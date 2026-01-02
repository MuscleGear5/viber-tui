use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use super::AppEvent;

pub fn poll_event(timeout_ms: u64) -> Option<Event> {
    if event::poll(Duration::from_millis(timeout_ms)).ok()? {
        event::read().ok()
    } else {
        None
    }
}

pub fn map_key_event(key: KeyEvent) -> Option<AppEvent> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);

    match (ctrl, alt, key.code) {
        (true, _, KeyCode::Char('c')) => Some(AppEvent::ForceQuit),
        (_, _, KeyCode::Esc) => Some(AppEvent::Cancel),
        (true, _, KeyCode::Char('q')) => Some(AppEvent::Quit),

        (false, false, KeyCode::Char('j')) => None,
        (false, false, KeyCode::Char('k')) => None,

        (_, _, KeyCode::Up) => Some(AppEvent::Up),
        (_, _, KeyCode::Down) => Some(AppEvent::Down),
        (_, _, KeyCode::Left) => Some(AppEvent::Left),
        (_, _, KeyCode::Right) => Some(AppEvent::Right),

        (true, _, KeyCode::Char('p')) => Some(AppEvent::Up),
        (true, _, KeyCode::Char('n')) => Some(AppEvent::Down),
        (true, _, KeyCode::Char('b')) => Some(AppEvent::Left),
        (true, _, KeyCode::Char('f')) => Some(AppEvent::Right),

        (_, _, KeyCode::PageUp) => Some(AppEvent::PageUp),
        (_, _, KeyCode::PageDown) => Some(AppEvent::PageDown),
        (true, _, KeyCode::Char('u')) => Some(AppEvent::PageUp),
        (true, _, KeyCode::Char('d')) => Some(AppEvent::PageDown),

        (_, _, KeyCode::Home) => Some(AppEvent::Home),
        (_, _, KeyCode::End) => Some(AppEvent::End),
        (true, _, KeyCode::Char('a')) => Some(AppEvent::Home),
        (true, _, KeyCode::Char('e')) => Some(AppEvent::End),

        (_, _, KeyCode::Enter) => Some(AppEvent::Select),

        (_, _, KeyCode::Backspace) => Some(AppEvent::Backspace),
        (_, _, KeyCode::Delete) => Some(AppEvent::Delete),
        (true, _, KeyCode::Char('w')) => Some(AppEvent::ClearLine),

        (_, _, KeyCode::Tab) => Some(AppEvent::NextFocus),
        (_, _, KeyCode::BackTab) => Some(AppEvent::PrevFocus),

        (_, true, KeyCode::Char(c)) => Some(AppEvent::SwitchView(c)),

        (true, _, KeyCode::Char('o')) => Some(AppEvent::TogglePreview),

        (_, _, KeyCode::F(1)) => Some(AppEvent::ToggleHelp),
        (true, _, KeyCode::Char('?')) => Some(AppEvent::ToggleHelp),

        (false, false, KeyCode::Char('V')) => Some(AppEvent::ViberChat),
        (false, false, KeyCode::Char('U')) => Some(AppEvent::ViberUndo),
        (false, false, KeyCode::Char('P')) => Some(AppEvent::ViberPrompt),
        (false, false, KeyCode::Char('R')) => Some(AppEvent::ViberRedirect),

        (false, false, KeyCode::Char(c)) => Some(AppEvent::Char(c)),

        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyContext {
    #[default]
    Normal,
    Input,
    List,
    Preview,
}

impl KeyContext {
    pub fn vim_motions_active(self) -> bool {
        matches!(self, KeyContext::Normal | KeyContext::List)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViberContext {
    #[default]
    Idle,
    AgentsRunning,
    ConfirmPending,
}

impl ViberContext {
    pub fn map_esc(self) -> AppEvent {
        match self {
            ViberContext::AgentsRunning => AppEvent::ViberStop,
            _ => AppEvent::Cancel,
        }
    }

    pub fn map_shift_esc(self) -> Option<AppEvent> {
        match self {
            ViberContext::AgentsRunning => Some(AppEvent::ViberStopAll),
            _ => None,
        }
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
