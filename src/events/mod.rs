mod keybindings;

use tokio::sync::mpsc;

pub use keybindings::{map_key_event, poll_event, KeyContext, ViberContext};

pub type EventSender = mpsc::UnboundedSender<AppEvent>;
pub type EventReceiver = mpsc::UnboundedReceiver<AppEvent>;

pub fn event_channel() -> (EventSender, EventReceiver) {
    mpsc::unbounded_channel()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Select,
    Cancel,
    Char(char),
    Backspace,
    Delete,
    ClearLine,
    NextFocus,
    PrevFocus,
    SwitchView(char),
    TogglePreview,
    ToggleHelp,
    Quit,
    ForceQuit,
    Tick,
    ViberChat,
    ViberStop,
    ViberStopAll,
    ViberUndo,
    ViberPrompt,
    ViberRedirect,
}
