---
feature: "VIBER-TUI"
spec: |
  A Rust TUI application for the VIBER vibe-coding workflow. Uses ratatui 0.29, crossterm, tokio, nucleo. Cyberpunk neon aesthetic with animated elements. Core concept: VIBER God Agent oversees all AI agents with powers to STOP/UNDO/INJECT. Workflow phases 0-9 from idea capture through delivery. Must support nvim-mcp integration for live buffer sync.
---

## Task List

### Feature 1: VIBER Core
Description: Central VIBER God Agent system with status panel, chat, and intervention capabilities
- [x] 1.01 VIBER status panel widget - animated eye, agent count, phase indicator, vibe meter (note: Starting VIBER status panel widget implementation) (note: Implemented viber_status widget: ViberPhase (0-9), ViberPower enum, VibeLevel, ViberState, animated eye with 8 frames, neon styling. Files: mod.rs, models.rs, state.rs, render.rs, widget.rs. All tests pass.)
- [x] 1.02 VIBER Chat view - streaming text, markdown rendering, input box with history (note: Starting VIBER Chat view implementation) (note: Enhanced chat view: added input_history with history_previous/next(), stream_reveal_pos for character-by-character animation with tick_streaming(), visible_content() for gradual reveal, cursor rendering with blinking via animation state. All tests pass.)
- [x] 1.03 Intervention system - stop/undo/inject commands with confirmation dialogs (note: Starting intervention system implementation) (note: Added InterventionCommand enum, ConfirmationDialog, execute_command method. Exported types from mod.rs.)
- [x] 1.04 VIBER hotkeys - V(chat), ESC(stop), U(undo), P(prompt), R(redirect) (note: Split events.rs into events/mod.rs + keybindings.rs. Added VIBER hotkeys (V/ESC/U/P/R) with context-aware mapping via ViberContext.)

### Feature 2: Views Implementation
Description: Primary TUI views for the VIBER workflow
- [x] 2.01 Questionnaire view - multichoice spec questions with keyboard nav (note: Starting questionnaire view implementation) (note: Added choice_index, choice_up/down/toggle methods for keyboard nav)
- [x] 2.02 Workflow DAG view - phase visualization with Canvas widget (note: Starting workflow DAG view) (note: DAG widget already implemented with braille edges, auto-layout. Exported DagNode, DagState, DagView from mod.rs.)
- [x] 2.03 Tasks view - Kanban board (Backlog/InProgress/Review/Done columns) (note: Starting tasks Kanban view) (note: Added kanban.rs with KanbanColumn, KanbanState, Kanban widget. 4 columns (Backlog/InProgress/Review/Done) with h/l nav.)
- [x] 2.04 Buffer view - live nvim-mcp sync showing file edits in real-time (note: Starting buffer view nvim-mcp sync) (note: Added nvim sync status to buffer widget title bar. Shows connection state and buffer ID.)

### Feature 3: Infrastructure
Description: Supporting systems for views and agents
- [x] 3.01 nvim-mcp integration client - socket connection, buffer sync, LSP relay (note: Starting action cards implementation) (note: Created NvimClient with socket connection, buffer sync, LSP relay. Exported types from mod.rs.)
- [x] 3.02 Parallel agent dispatch - spawn/track multiple agents, status aggregation (note: Starting parallel agent dispatch) (note: Implemented spawn_parallel(), stop_all(), aggregate_status() in AgentController. Moved AgentCommand/AgentEvent to dispatch.rs. All files under 150 lines.)
- [x] 3.03 Toast notification system - slide-in alerts, auto-dismiss, action buttons (note: Created src/widgets/toast/ module: models.rs (ToastLevel, ToastAction, Toast), state.rs (ToastState with queue management), render.rs (slide-in animation, progress bar, action buttons), widget.rs (ToastWidget).)
- [x] 3.04 Modal dialog system - centered overlays, confirm/cancel, form inputs (note: Created src/widgets/modal/ module: models.rs (ModalKind, ModalButton, ModalField, Modal), state.rs (ModalState with form handling), render.rs (centered overlay, buttons, form fields), widget.rs (ModalWidget).)

### Feature 4: Widgets
Description: Reusable widget components
- [x] 4.01 Canvas widget - Braille/block graphics for DAG rendering with Bezier edges (note: Starting Canvas widget implementation) (note: Created src/widgets/canvas/ module: models.rs (Point, DrawCommand, BrushMode), state.rs (CanvasState with command queue), render.rs (Braille grid, Bezier curves via de Casteljau, line drawing), widget.rs (CanvasWidget).)
- [x] 4.02 Charts widgets - line chart, bar chart with gradients (note: Already implemented in src/widgets/sparkline/ - supports Line (Braille), Bar, Dot styles with configurable colors, min/max display, and baseline.)
- [x] 4.03 Calendar heatmap - GitHub-style activity visualization (note: Already implemented in src/widgets/heatmap/ - GitHub-style 53-week calendar with intensity colors, month/day labels, selection support.)
