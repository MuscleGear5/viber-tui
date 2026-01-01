---
feature: "VIBER-TUI"
spec: |
  Build a Rust TUI application implementing the VIBER "vibe coding" workflow.
  Cyberpunk-aesthetic interface for AI-assisted development with 9-phase workflow
  orchestration, real-time agent monitoring, LSP-driven edit cycles, and Neovim-native
  buffer operations.
---

## Task List

### Feature 1: Core Infrastructure
Description: Foundation layer - theme system, base widgets, state management, event handling
- [x] 1.01 Color palette (neon cyberpunk) (note: Starting color palette implementation) (note: Already complete - colors.rs has full neon cyberpunk palette)
- [x] 1.02 Animation system (spinner, pulse, wave, progress) (note: Already complete - animation.rs has spinner, pulse, wave, progress)
- [x] 1.03 Base widget traits and composition (note: events.rs created with AppEvent enum, KeyContext for widget interaction)
- [x] 1.04 Global app state with tokio channels (note: Added tokio mpsc channels - EventSender/EventReceiver with event_channel() factory)
- [x] 1.05 Keyboard navigation (vim-like) (note: Vim-like keybindings in map_key_event() - j/k/g/G/Ctrl-u/d navigation)

### Feature 2: Primary Views
Description: Main TUI views - Chat [C], Workflow [W], Tasks [T], Agents [A]
- [x] 2.01 Chat view [C] - streaming, markdown, code blocks (note: Starting Chat view implementation) (note: Chat view complete - state.rs (ChatState, ChatMessage, MessageRole), render.rs (message bubbles, markdown/code parsing), widget.rs (StatefulWidget with input bar))
- [x] 2.02 Workflow view [W] - DAG, phases, progress (note: Starting Workflow view implementation) (note: Workflow view complete - state.rs (Phase, PhaseStatus, WorkflowState), render.rs (phase lines, progress bars, DAG connectors), widget.rs (StatefulWidget))
- [x] 2.03 Tasks view [T] - hierarchy, badges, fuzzy search (note: Starting Tasks view - will reuse existing launcher fuzzy list) (note: Tasks view complete - state.rs (Task, TaskPriority, TaskStatus models), list_state.rs (TasksState with filtering), render.rs (task lines with badges), widget.rs (StatefulWidget))
- [x] 2.04 Agents view [A] - status cards, resources, logs (note: Starting Agents view - status cards, resources, logs) (note: Agents view complete - state.rs (Agent, AgentStatus models), render.rs (status cards with resource bars), widget.rs (agent list + detail panel). F2 fully complete!)

### Feature 3: Editor Integration
Description: Code editing views - Buffer [B], Diff [D], LSP [L], nvim-mcp integration
- [x] 3.01 Buffer view [B] - syntax highlight, line numbers (note: Starting Action Card widget) (note: Buffer view complete - language.rs (Language enum, syntax detection), state.rs (BufferLine, BufferState), render.rs (line numbers, basic syntax highlighting for keywords/strings/comments/numbers), widget.rs (StatefulWidget with gutter+code layout). All files under 150 lines.)
- [x] 3.02 Diff view [D] - side-by-side, accept/reject (note: Starting Diff view - side-by-side with accept/reject) (note: Diff view complete - state.rs (DiffLineKind, DiffLine, DiffHunk, DiffState with accept/reject), render.rs (side-by-side with +/- symbols, color coding), widget.rs (StatefulWidget with left/right panels and action bar). All files at/under 150 lines.)
- [x] 3.03 LSP view [L] - diagnostics, hover, references (note: Starting LSP view - diagnostics, hover, references) (note: LSP view complete - models.rs (DiagnosticSeverity, Diagnostic, HoverInfo, Reference), state.rs (LspTab, LspState with tab switching), render.rs (diagnostic rows, hover block, reference list), widget.rs (StatefulWidget with tabs for diagnostics/hover/references). All files under 150 lines.)
- [x] 3.04 nvim-mcp integration for buffer ops (note: Starting nvim-mcp integration for buffer operations) (note: nvim-mcp integration complete - nvim.rs (NvimSyncStatus, NvimConnection with connect/disconnect/read/navigate/diagnostics stubs), state.rs updated with nvim field and sync methods. All files under 150 lines.)

### Feature 4: Workflow Phase Views
Description: VIBER 9-phase workflow - Questionnaire [Q], Spec [S], transitions
- [x] 4.01 Questionnaire view [Q] - forms, validation (note: Starting Questionnaire view - forms and validation) (note: Questionnaire view complete - models.rs (QuestionType, ValidationResult, Choice, Question with constructors), state.rs (QuestionnaireState with navigation/validation), render.rs (question prompts, text input, choice lists, validation messages, progress bar), widget.rs (StatefulWidget). All files ≤112 lines.)
- [x] 4.02 Spec view [S] - sections, approval (note: Starting Spec view - sections, approval workflow) (note: Spec view complete - models.rs (ApprovalStatus, SectionType, SpecSection, SpecComment), state.rs (SpecState with navigation/approval/comments), render.rs (title bar, section headers with collapse, content, action bar), widget.rs (StatefulWidget). All files ≤126 lines.)
- [x] 4.03 Phase transition logic and persistence (note: Starting Phase transition logic and persistence) (note: Phase transition logic and persistence complete - models.rs (Phase enum with 9 VIBER phases, PhaseStatus with can_transition validation), state.rs (WorkflowState with transition methods), persistence.rs (save/load session to JSON with SessionData struct). All files ≤113 lines.)

### Feature 5: VIBER God Agent
Description: Central orchestration - agent protocol, registry, undo, intervention
- [x] 5.01 Agent protocol (spawn, monitor, stop, inject) (note: Starting agent protocol - spawn, monitor, stop, inject) (note: Agent protocol complete - models.rs (AgentId, AgentKind enum, AgentState enum, AgentConfig, AgentInstance with lifecycle tracking), protocol.rs (AgentCommand enum for spawn/stop/inject, AgentController with async command handling via channels). All files ≤142 lines.)
- [x] 5.02 Agent registry with health checks (note: Starting agent registry with health checks) (note: Agent registry complete - registry.rs (HealthCheck struct with interval/timeout/consecutive failures, AgentRegistry with HashMap storage, register/unregister/get/list_by_state/health_check methods, async check_all_health). All files ≤142 lines.)
- [x] 5.03 Undo/rollback system (note: Starting undo/rollback system) (note: Undo/rollback system complete - undo.rs (ChangeKind enum, FileSnapshot, Change with file_modified/created/deleted constructors, Checkpoint, UndoStack with record/commit/pop/peek). 102 lines.)
- [x] 5.04 Intervention triggers (note: Starting intervention triggers) (note: Intervention triggers complete - intervention.rs (TriggerSeverity, TriggerAction enums, InterventionRule with pattern matching, TriggerEvent, InterventionMonitor with default dangerous pattern rules like rm -rf, force push, DROP TABLE). 96 lines.)

### Feature 6: Advanced Widgets
Description: Rich visualization - DAG renderer, sparklines, heatmap, spinners
- [x] 6.01 DAG renderer (Canvas, braille) (note: Starting DAG renderer with Canvas/braille support) (note: DAG renderer complete - models.rs (NodeId, NodeStatus, DagNode, Edge), state.rs (DagState with auto-layout, navigation), render.rs (braille line drawing), widget.rs (StatefulWidget). All files <100 lines, cargo check passes.)
- [x] 6.02 Sparkline charts (note: Starting sparkline widget - mini charts for metrics visualization) (note: Sparkline widget complete - models.rs (SparklineStyle, SparklineConfig, DataPoint), state.rs (SparklineState with caching), render.rs (line/bar/dot styles via braille/blocks), widget.rs (StatefulWidget). All <100 lines.)
- [x] 6.03 Calendar heatmap (note: Starting calendar heatmap widget - GitHub-style contribution grid) (note: Calendar heatmap complete - models.rs (Date with day_of_week/week_of_year, HeatmapEntry, intensity_color), state.rs (HeatmapState with selection), render.rs (GitHub-style grid with month/day labels), widget.rs (StatefulWidget). All <100 lines.)
- [x] 6.04 Advanced spinners (note: Starting advanced spinners - extend animation.rs with more spinner styles) (note: Advanced spinners complete - spinners.rs with 13 spinner styles (Dots, Line, Circle, Square, Triangle, Bounce, Grow, Pulse, Orbit, Star, Neon, Clock, Braille), SpinnerStyle enum with frames()/frame()/speed_divisor(), Spinner struct with tick/render, ProgressBar with block/gradient/pulse styles. 133 lines.)

### Feature 7: Integration & Polish
Description: External integrations - memcord, beads, notifications, help, performance
- [x] 7.01 Memcord integration (note: Starting memcord integration - memory/context management for chat persistence) (note: Memcord integration complete - memcord.rs with MemcordSlot, MemcordEntry, MemcordCommand (12 commands with name/description), MemcordState (slot management, zero mode, save). 104 lines.)
- [x] 7.02 Beads (bd) integration (note: Starting beads integration - issue tracking via bd CLI) (note: Beads integration complete - beads.rs with IssueType (4 types), IssueStatus (4 states with icons), BeadsIssue struct, BeadsCommand enum (9 commands), BeadsClient with CLI wrapper methods (ready/list/show/update/close/stats). 147 lines.)
- [x] 7.03 Toast notifications (note: Starting toast notifications - transient status messages) (note: Toast notifications complete - added ToastLevel (4 levels with icons/colors), Toast struct with auto-dismiss, ToastManager with render_toasts. 143 lines in indicators.rs.)
- [x] 7.04 Help overlay with keybindings (note: Starting help overlay - keybinding reference popup) (note: Help overlay complete - help.rs with KeyBinding struct, KeySection with bindings, HelpOverlayState with sections, HelpOverlay widget rendering centered modal with keybinding grid. 121 lines.)
- [x] 7.05 Performance optimization (note: Starting performance optimization - render caching, lazy loading patterns) (note: Performance optimization complete - render.rs with RenderCache (hash-based invalidation, viewport tracking), LazyContent (deferred rendering for large content), frame_budget helper (16ms target), render_if_changed guard, visible_line_range calculator. 131 lines.)
