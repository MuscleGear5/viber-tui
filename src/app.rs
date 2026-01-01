use crate::data::{Action, ActionRegistry};
use crate::theme::AnimationState;
use crate::views::LauncherState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Launcher,
}

#[derive(Debug, Clone)]
pub enum AppAction {
    Continue,
    Quit,
    Execute(Action),
    SwitchView(View),
}

pub struct App {
    pub registry: ActionRegistry,
    pub animation: AnimationState,
    pub launcher: LauncherState,
    pub current_view: View,
    should_quit: bool,
    pending_action: Option<Action>,
}

impl App {
    pub fn new(registry: ActionRegistry) -> Self {
        let launcher = LauncherState::new(&registry);
        Self {
            registry,
            animation: AnimationState::new(),
            launcher,
            current_view: View::default(),
            should_quit: false,
            pending_action: None,
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        let registry = ActionRegistry::load_from_file("data/actions.yaml")?;
        Ok(Self::new(registry))
    }

    pub fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::Continue => {}
            AppAction::Quit => self.should_quit = true,
            AppAction::Execute(act) => {
                self.pending_action = Some(act);
                self.should_quit = true;
            }
            AppAction::SwitchView(view) => self.current_view = view,
        }
    }

    pub fn tick(&mut self) {
        self.animation.tick();
        self.launcher.tick();
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn take_pending_action(&mut self) -> Option<Action> {
        self.pending_action.take()
    }
}
