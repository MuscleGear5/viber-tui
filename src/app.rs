use crate::agents::{AgentController, AgentRegistry, InterventionMonitor, UndoStack};
use crate::data::{Action, ActionRegistry};
use crate::integrations::{BeadsClient, MemcordState};
use crate::theme::{AnimationState, ToastManager};
use crate::views::LauncherState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Launcher,
    Chat,
    Workflow,
    Tasks,
    Agents,
    Buffer,
    Diff,
    Lsp,
    Help,
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
    pub agents: AgentRegistry,
    pub controller: AgentController,
    pub undo: UndoStack,
    pub intervention: InterventionMonitor,
    pub toasts: ToastManager,
    pub memcord: MemcordState,
    pub beads: BeadsClient,
    pub show_help: bool,
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
            agents: AgentRegistry::new(),
            controller: AgentController::new(),
            undo: UndoStack::new(50),
            intervention: InterventionMonitor::new(),
            toasts: ToastManager::new(),
            memcord: MemcordState::new(),
            beads: BeadsClient::new(),
            show_help: false,
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
        self.toasts.tick();
        self.intervention.tick();
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn take_pending_action(&mut self) -> Option<Action> {
        self.pending_action.take()
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
