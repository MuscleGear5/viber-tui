use crate::agents::{AgentController, AgentEvent, AgentPool, AgentRegistry, InterventionMonitor, UndoStack};
use crate::data::{Action, ActionRegistry};
use crate::integrations::{
    BeadsClient, MemcordState, NvimClient, NvimMcpCommand, NvimMcpRunner, NvimMcpResponse,
};
use crate::theme::{AnimationState, ToastManager};
use crate::views::{
    AgentsState, BufferListState, BufferState, ChatState, DiffState, HelpOverlayState,
    LauncherState, LspState, TasksState, WorkflowState,
};
use crate::widgets::modal::ModalState;

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
    pub current_view: View,
    pub show_help: bool,
    pub launcher: LauncherState,
    pub chat: ChatState,
    pub workflow: WorkflowState,
    pub tasks: TasksState,
    pub agents_view: AgentsState,
    pub buffer: BufferState,
    pub buffer_list: BufferListState,
    pub diff: DiffState,
    pub lsp: LspState,
    pub help: HelpOverlayState,
    pub agents: AgentRegistry,
    pub controller: AgentController,
    pub undo: UndoStack,
    pub intervention: InterventionMonitor,
    pub toasts: ToastManager,
    pub memcord: MemcordState,
    pub beads: BeadsClient,
    pub nvim: NvimClient,
    pub nvim_mcp: Option<NvimMcpRunner>,
    pub agent_pool: AgentPool,
    pub modal: ModalState,
    should_quit: bool,
    pending_action: Option<Action>,
}

impl App {
    pub fn new(registry: ActionRegistry) -> Self {
        let launcher = LauncherState::new(&registry);
        Self {
            registry,
            animation: AnimationState::new(),
            current_view: View::default(),
            show_help: false,
            launcher,
            chat: ChatState::default(),
            workflow: WorkflowState::default(),
            tasks: TasksState::default(),
            agents_view: AgentsState::default(),
            buffer: BufferState::default(),
            buffer_list: BufferListState::default(),
            diff: DiffState::default(),
            lsp: LspState::default(),
            help: HelpOverlayState::default(),
            agents: AgentRegistry::new(),
            controller: AgentController::new(),
            undo: UndoStack::new(50),
            intervention: InterventionMonitor::new(),
            toasts: ToastManager::new(),
            memcord: MemcordState::new(),
            beads: BeadsClient::new(),
            nvim: NvimClient::new(),
            nvim_mcp: None,
            agent_pool: AgentPool::new(),
            modal: ModalState::new(),
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
        self.poll_nvim_updates();
        self.poll_agent_events();
    }

    fn poll_nvim_updates(&mut self) {
        if let Some(ref mut runner) = self.nvim_mcp {
            while let Some(update) = runner.try_recv() {
                match update {
                    NvimMcpResponse::Targets(targets) => {
                        if let Some(first) = targets.first() {
                            let _ = runner.send(NvimMcpCommand::Connect {
                                target: first.socket_path.clone(),
                            });
                        }
                    }
                    NvimMcpResponse::Connected { connection_id } => {
                        self.nvim.connection_id = Some(connection_id);
                        self.nvim.state = crate::integrations::NvimConnectionState::Connected;
                        let _ = runner.send(NvimMcpCommand::ListBuffers);
                        let _ = runner.send(NvimMcpCommand::LspClients);
                    }
                    NvimMcpResponse::Buffers(buffers) => {
                        self.nvim.buffers.clear();
                        for buf in &buffers {
                            self.nvim.buffers.insert(buf.id, crate::integrations::NvimBuffer {
                                id: buf.id,
                                name: buf.name.clone(),
                                line_count: buf.line_count as usize,
                                is_modified: buf.modified,
                            });
                        }
                        let nvim_buffers: Vec<_> = buffers.into_iter().map(|b| {
                            crate::integrations::NvimBuffer {
                                id: b.id,
                                name: b.name,
                                line_count: b.line_count as usize,
                                is_modified: b.modified,
                            }
                        }).collect();
                        self.buffer_list.set_buffers(nvim_buffers);
                    }
                    NvimMcpResponse::Cursor { line, column, buffer_id } => {
                        self.nvim.cursor = Some(crate::integrations::NvimCursor {
                            buffer_id,
                            line: line as usize,
                            column: column as usize,
                        });
                    }
                    NvimMcpResponse::LspClients(clients) => {
                        self.nvim.lsp_clients = clients.into_iter().map(|c| {
                            crate::integrations::LspClient {
                                name: c.name,
                                root_dir: c.root_dir,
                            }
                        }).collect();
                    }
                    NvimMcpResponse::Error { message } => {
                        self.nvim.set_error();
                        self.toasts.push(crate::theme::Toast::error(&message));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn set_nvim_runner(&mut self, runner: NvimMcpRunner) {
        self.nvim_mcp = Some(runner);
    }

    fn poll_agent_events(&mut self) {
        for event in self.agent_pool.poll_events() {
            match event {
                AgentEvent::Spawned(id) => {
                    self.toasts.push(crate::theme::Toast::info(&format!("Agent {} spawned", id.0)));
                }
                AgentEvent::Started(id) => {
                    self.toasts.push(crate::theme::Toast::info(&format!("Agent {} started", id.0)));
                }
                AgentEvent::Completed(id, _output) => {
                    self.toasts.push(crate::theme::Toast::success(&format!("Agent {} completed", id.0)));
                }
                AgentEvent::Failed(id, err) => {
                    self.toasts.push(crate::theme::Toast::error(&format!("Agent {} failed: {}", id.0, err)));
                }
                AgentEvent::Stopped(id) => {
                    self.toasts.push(crate::theme::Toast::warning(&format!("Agent {} stopped", id.0)));
                }
                _ => {}
            }
        }
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

    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
    }
}
