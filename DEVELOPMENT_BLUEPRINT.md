# VIBER-TUI Development Blueprint

> Comprehensive mapping of OpenCode resources to VIBER workflow implementation

## Executive Summary

This blueprint maps the tooling from CHEATSHEET.md to implement the vision in workflow_idea.md. The goal: a Rust TUI that embodies "vibe coding" - where you describe intent and AI agents execute with full observability.

---

## 1. Phase-by-Phase Tool Mapping

### Phase 1: Idea Capture
**Purpose**: User describes what they want in natural language

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` (primary), `ncp-research` (if research needed) |
| **MCPs** | `memcord` (save ideas), `context7` (lookup prior art) |
| **Subagents** | `product-manager` (refine requirements), `business-analyst` (clarify scope) |
| **Skills** | `/brainstorming`, `/sequential-thinking` |

**Implementation Notes**:
- Chat view receives natural language input
- memcord_save captures raw idea with timestamp
- AI extracts keywords, suggests clarifications

### Phase 2: Decomposition
**Purpose**: Break idea into component parts

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `ast_grep` (find similar patterns), `grep_app` (GitHub examples) |
| **Subagents** | `architect-reviewer`, `microservices-architect` |
| **Skills** | `/planning`, `/writing-plans` |

**Implementation Notes**:
- Workflow view shows decomposition tree
- Each component becomes a node in DAG
- Dependencies auto-detected from imports/calls

### Phase 3: Questionnaire
**Purpose**: Gather missing context through targeted questions

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `context7` (docs lookup), `exa_web_search` (external info) |
| **Subagents** | `ux-researcher` (user flow questions), `business-analyst` |
| **Skills** | `/kiro-skill`, `/openspec-discovery` |

**Implementation Notes**:
- Questionnaire view [Q] renders dynamic forms
- Questions adapt based on previous answers
- Validation prevents proceeding with incomplete info

### Phase 4: Spec Generation
**Purpose**: Transform answers into structured specification

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `memcord` (store spec), file tools |
| **Subagents** | `technical-writer`, `api-designer`, `documentation-engineer` |
| **Skills** | `/spec-kit-skill`, `/kiro-skill`, `/doc-coauthoring` |

**Implementation Notes**:
- Spec view [S] displays generated spec
- User can approve/reject/modify sections
- Spec persisted to `.viber/specs/` directory

### Phase 5: Task Breakdown
**Purpose**: Convert spec into atomic, executable tasks

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `beads` (issue tracking), `roadmap` tools |
| **Subagents** | `project-manager`, `task-distributor`, `scrum-master` |
| **Skills** | `/writing-plans`, `/task-manager` |

**Implementation Notes**:
- Tasks view [T] shows task hierarchy
- Each task has: description, acceptance criteria, estimated effort
- Dependencies create DAG edges in Workflow view

### Phase 6: Scaffold Generation
**Purpose**: Create file/folder structure before implementation

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `write`, `bash`, nvim-mcp |
| **Subagents** | `fullstack-developer`, `backend-developer`, `frontend-developer` |
| **Skills** | `/frontend-design`, `/backend-development` |

**Implementation Notes**:
- Buffer view [B] shows scaffolded files
- Diff view [D] shows what will be created
- Scaffold commits with `[scaffold]` prefix

### Phase 7: Implementation (LSP-Driven)
**Purpose**: Execute tasks with continuous LSP validation

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | nvim-mcp (ALL 33 tools), `lsp_*` tools, `ast_grep` |
| **Subagents** | Domain-specific: `rust-engineer`, `typescript-pro`, `python-pro`, etc. |
| **Skills** | `/test-driven-development`, `/code-review`, `/subagent-driven-development` |

**Critical Edit Cycle**:
```
1. nvim_read (get current buffer)
2. Plan edit (determine changes)
3. nvim_lsp_formatting or direct edit
4. Wait for LSP (nvim_wait_for_lsp_ready)
5. nvim_buffer_diagnostics (check errors)
6. If errors: fix immediately, goto step 4
7. If clean: proceed to next edit
```

**Implementation Notes**:
- LSP view [L] shows real-time diagnostics
- Every edit triggers LSP check before proceeding
- Errors block progress until resolved

### Phase 8: Polish
**Purpose**: Refine code quality, add documentation, optimize

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `lsp_*`, `ast_grep_replace` |
| **Subagents** | `code-reviewer`, `refactoring-specialist`, `performance-engineer` |
| **Skills** | `/code-refactoring`, `/code-review`, `/extract-learnings` |

**Implementation Notes**:
- Automated lint/format pass
- Documentation generation for public APIs
- Performance profiling integration

### Phase 9: Validation
**Purpose**: Verify implementation meets spec

| Resource Type | Tools/Agents |
|--------------|--------------|
| **NCP Profile** | `ncp-coding` |
| **MCPs** | `bash` (run tests), `playwright` (E2E) |
| **Subagents** | `qa-expert`, `test-automator`, `penetration-tester` |
| **Skills** | `/test-driven-development`, `/playwright`, `/qa-regression` |

**Implementation Notes**:
- Test results displayed in dedicated panel
- Coverage metrics tracked
- Spec requirements checked off as validated

---

## 2. Core Infrastructure Setup

### Primary NCP Profile: `ncp-coding`

Contains 31 MCPs optimized for development:
- **Code Intelligence**: `ast_grep`, `lsp_*` tools, `grep_app`
- **Documentation**: `context7` (library docs)
- **Version Control**: `git` operations via bash
- **Memory**: `memcord` for session persistence

### Neovim Integration (nvim-mcp)

**Required for VIBER** - All buffer operations go through Neovim:

| Tool Category | Tools | Purpose |
|--------------|-------|---------|
| **Connection** | `get_targets`, `connect`, `disconnect` | Establish nvim connection |
| **Navigation** | `cursor_position`, `navigate`, `list_buffers` | Move around codebase |
| **Reading** | `read`, `buffer_diagnostics` | Get file content and errors |
| **LSP** | `lsp_hover`, `lsp_definition`, `lsp_references`, `lsp_document_symbols`, `lsp_workspace_symbols`, `lsp_code_actions`, `lsp_rename`, `lsp_formatting` | Full LSP integration |
| **Execution** | `exec_lua` | Run arbitrary Neovim commands |

**Connection Pattern**:
```typescript
// 1. Get connection_id (auto-connects from ~/.config/opencode)
const targets = await neovim_nvim_get_targets({});
const conn = targets.connection_id; // e.g., "f7ba42a"

// 2. Use in all subsequent calls
await neovim_nvim_read({ params: { connection_id: conn, path: "src/main.rs" }});
await neovim_nvim_buffer_diagnostics({ params: { connection_id: conn, buffer_id: 1 }});
```

### Memory Strategy (memcord)

**Slot Structure for VIBER-TUI**:
```
viber-tui-dev          # Main development slot
viber-tui-specs        # Generated specifications  
viber-tui-decisions    # Architecture decisions
viber-tui-learnings    # Lessons learned
```

**Usage Pattern**:
```bash
memcord_name "viber-tui-dev"
memcord_tag action="add" tags="rust tui ratatui"
memcord_group action="set" group_path="projects/viber-tui"
```

### Issue Tracking (beads)

**Configured via**: `.beads/` directory (already exists)

**Key Commands**:
```bash
bd ready              # Find available work
bd show <id>          # View issue details  
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

---

## 3. Implementation Priority

### Wave 1: Foundation (Features 1, 6)
**Timeline**: First sprint

1. **Color Palette** (1.01) - All UI depends on this
2. **Animation System** (1.02) - Required for spinners, progress
3. **Base Widgets** (1.03) - Building blocks for views
4. **App State** (1.04) - Async event handling
5. **Keyboard Nav** (1.05) - User interaction

**Parallel**: Advanced spinners (6.04) - simple, standalone

### Wave 2: Core Views (Feature 2)
**Timeline**: Second sprint

1. **Chat View** (2.01) - Primary interaction point
2. **Tasks View** (2.03) - Work tracking
3. **Workflow View** (2.02) - DAG visualization
4. **Agents View** (2.04) - Monitoring

**Dependencies**: Wave 1 complete

### Wave 3: Editor Integration (Feature 3)
**Timeline**: Third sprint

1. **Buffer View** (3.01) - Code display
2. **nvim-mcp Integration** (3.04) - Connect to Neovim
3. **Diff View** (3.02) - Change visualization
4. **LSP View** (3.03) - Diagnostics panel

**Dependencies**: Wave 2 complete, Neovim running

### Wave 4: Workflow Phases (Feature 4)
**Timeline**: Fourth sprint

1. **Questionnaire View** (4.01) - Dynamic forms
2. **Spec View** (4.02) - Spec display
3. **Phase Transitions** (4.03) - State machine

**Dependencies**: Wave 3 complete

### Wave 5: God Agent (Feature 5)
**Timeline**: Fifth sprint

1. **Agent Protocol** (5.01) - Communication spec
2. **Agent Registry** (5.02) - Health monitoring
3. **Intervention Triggers** (5.04) - Auto-stop rules
4. **Undo System** (5.03) - Rollback capability

**Dependencies**: Wave 4 complete

### Wave 6: Polish (Features 6 remainder, 7)
**Timeline**: Final sprint

1. **DAG Renderer** (6.01) - Canvas visualization
2. **Sparklines** (6.02) - Metrics charts
3. **Calendar Heatmap** (6.03) - Activity viz
4. **External Integrations** (7.01-7.05) - memcord, beads, etc.

---

## 4. Agent Orchestration Pattern

### VIBER God Agent Architecture

```
                    ┌─────────────────┐
                    │   VIBER GOD     │
                    │    AGENT        │
                    └────────┬────────┘
                             │
           ┌─────────────────┼─────────────────┐
           │                 │                 │
           ▼                 ▼                 ▼
    ┌──────────┐      ┌──────────┐      ┌──────────┐
    │  Agent   │      │  Agent   │      │  Agent   │
    │  Pool    │      │  Pool    │      │  Pool    │
    │(Frontend)│      │(Backend) │      │  (QA)    │
    └──────────┘      └──────────┘      └──────────┘
```

### Agent Control Verbs

| Verb | Description | Trigger |
|------|-------------|---------|
| **SPAWN** | Start new agent for task | Task ready, resources available |
| **MONITOR** | Watch agent progress | Continuous |
| **PAUSE** | Temporarily halt agent | User request, resource contention |
| **RESUME** | Continue paused agent | Blocker resolved |
| **STOP** | Terminate agent | Error threshold, timeout, user request |
| **INJECT** | Add context mid-execution | Missing information discovered |
| **REDIRECT** | Change agent's task | Priority shift, better approach found |
| **UNDO** | Rollback agent's changes | Validation failure, user rejection |

### Dispatch Rules

**Parallel Dispatch** (independent tasks):
```rust
// Multiple agents can work simultaneously when:
// - No shared file dependencies
// - No blocking relationships
// - Sufficient system resources

spawn_parallel([
    Agent::new("frontend-developer", task_ui),
    Agent::new("backend-developer", task_api),
    Agent::new("test-automator", task_tests),
]);
```

**Sequential Dispatch** (dependent tasks):
```rust
// Must wait for predecessor when:
// - Output of A is input to B
// - B modifies files A created
// - Explicit dependency in task graph

spawn_sequential([
    Agent::new("api-designer", design_task),
    Agent::new("backend-developer", implement_task), // waits
    Agent::new("qa-expert", test_task),              // waits
]);
```

### Review Checkpoints

| Checkpoint | Trigger | Action |
|------------|---------|--------|
| **Pre-Edit** | Before any file modification | Show diff preview, await approval |
| **Post-Task** | Task completion | Run tests, check LSP, verify spec |
| **Phase Gate** | Phase transition | Full validation, user sign-off |
| **Error Recovery** | 3+ consecutive failures | Pause all, consult user |

### Subagent Type Mapping

| Domain | Primary Agent | Backup Agents |
|--------|--------------|---------------|
| **Rust/Systems** | `rust-engineer` | `cpp-pro`, `golang-pro` |
| **Frontend/React** | `react-specialist` | `frontend-developer`, `vue-expert` |
| **Backend/API** | `backend-developer` | `api-designer`, `graphql-architect` |
| **Database** | `database-administrator` | `postgres-pro`, `sql-pro` |
| **DevOps** | `devops-engineer` | `kubernetes-specialist`, `terraform-engineer` |
| **Testing** | `qa-expert` | `test-automator`, `playwright` skill |
| **Security** | `security-engineer` | `penetration-tester`, `security-auditor` |
| **Docs** | `technical-writer` | `documentation-engineer`, `api-documenter` |
| **Architecture** | `architect-reviewer` | `microservices-architect`, `cloud-architect` |
| **Performance** | `performance-engineer` | `database-optimizer`, `dx-optimizer` |

---

## 5. Technical Stack Summary

### Ratatui Widgets to Implement

| Widget | Use Case | Priority |
|--------|----------|----------|
| **Paragraph** | Chat messages, logs | P0 |
| **List** | Tasks, files, agents | P0 |
| **Block** | Container borders | P0 |
| **Table** | Structured data | P1 |
| **Tabs** | View switching | P1 |
| **Gauge** | Progress bars | P1 |
| **Sparkline** | Metrics mini-charts | P2 |
| **BarChart** | Statistics | P2 |
| **Canvas** | DAG visualization | P2 |
| **Calendar** | Activity heatmap | P3 |

### Animation System Architecture

```rust
pub struct AnimationState {
    frame: usize,
    last_tick: Instant,
    speed: Duration,
}

pub trait Animated {
    fn tick(&mut self, delta: Duration);
    fn render(&self, area: Rect, buf: &mut Buffer);
}

// Animation types
pub enum AnimationType {
    Spinner(SpinnerStyle),  // dots, braille, blocks
    Pulse(Color, Color),    // fade between colors
    Wave(Vec<char>),        // character wave
    Progress(f64),          // 0.0 to 1.0
}
```

### Color Palette Implementation

```rust
pub mod colors {
    use ratatui::style::Color;
    
    // Primary neon palette
    pub const CYAN: Color = Color::Rgb(0, 255, 204);      // #00FFCC
    pub const MAGENTA: Color = Color::Rgb(255, 0, 255);   // #FF00FF
    pub const YELLOW: Color = Color::Rgb(255, 255, 0);    // #FFFF00
    pub const ORANGE: Color = Color::Rgb(255, 102, 0);    // #FF6600
    pub const GREEN: Color = Color::Rgb(0, 255, 0);       // #00FF00
    pub const RED: Color = Color::Rgb(255, 0, 68);        // #FF0044
    
    // Semantic colors
    pub const SUCCESS: Color = GREEN;
    pub const ERROR: Color = RED;
    pub const WARNING: Color = ORANGE;
    pub const INFO: Color = CYAN;
    pub const ACCENT: Color = MAGENTA;
    
    // Background shades
    pub const BG_DARK: Color = Color::Rgb(13, 17, 23);    // #0D1117
    pub const BG_MID: Color = Color::Rgb(22, 27, 34);     // #161B22
    pub const BG_LIGHT: Color = Color::Rgb(33, 38, 45);   // #21262D
}
```

### nvim-mcp Integration Points

| TUI Component | nvim-mcp Tools Used |
|--------------|---------------------|
| **Buffer View** | `read`, `cursor_position`, `navigate` |
| **LSP View** | `buffer_diagnostics`, `lsp_hover`, `lsp_references` |
| **Diff View** | `read` (before/after), custom diff algorithm |
| **Code Actions** | `lsp_code_actions`, `lsp_resolve_code_action` |
| **Symbol Search** | `lsp_workspace_symbols`, `lsp_document_symbols` |
| **Rename** | `lsp_rename` |
| **Format** | `lsp_formatting` |

---

## 6. View Keybindings

| Key | Global Action |
|-----|---------------|
| `C` | Switch to Chat view |
| `W` | Switch to Workflow view |
| `A` | Switch to Agents view |
| `T` | Switch to Tasks view |
| `B` | Switch to Buffer view |
| `D` | Switch to Diff view |
| `Q` | Switch to Questionnaire view |
| `S` | Switch to Spec view |
| `L` | Switch to LSP view |
| `?` | Toggle help overlay |
| `Esc` | Back / Cancel |
| `q` | Quit (with confirmation) |

| Key | View-Specific |
|-----|---------------|
| `j/k` | Navigate down/up |
| `h/l` | Navigate left/right (or collapse/expand) |
| `Enter` | Select / Confirm |
| `/` | Search / Filter |
| `g` | Go to top |
| `G` | Go to bottom |
| `Tab` | Next pane |
| `Shift+Tab` | Previous pane |

---

## 7. File Structure

```
viber-tui/
├── src/
│   ├── main.rs                 # Entry point, event loop
│   ├── app.rs                  # Application state
│   ├── events.rs               # Event handling
│   ├── data/
│   │   ├── mod.rs
│   │   └── actions.rs          # Action definitions
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── colors.rs           # Color palette
│   │   ├── styles.rs           # Reusable styles
│   │   └── animation.rs        # Animation system
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── action_card.rs      # Action display
│   │   ├── fuzzy_list.rs       # Searchable list
│   │   ├── spinner.rs          # Animated spinner
│   │   ├── dag.rs              # DAG visualization
│   │   ├── diff.rs             # Diff display
│   │   └── code_block.rs       # Syntax highlighted code
│   ├── views/
│   │   ├── mod.rs
│   │   ├── launcher.rs         # Current launcher view
│   │   ├── chat.rs             # Chat view [C]
│   │   ├── workflow.rs         # Workflow DAG [W]
│   │   ├── agents.rs           # Agent monitor [A]
│   │   ├── tasks.rs            # Task list [T]
│   │   ├── buffer.rs           # Code buffer [B]
│   │   ├── diff.rs             # Diff view [D]
│   │   ├── questionnaire.rs    # Dynamic forms [Q]
│   │   ├── spec.rs             # Spec display [S]
│   │   └── lsp.rs              # LSP diagnostics [L]
│   └── agents/
│       ├── mod.rs
│       ├── god.rs              # VIBER god agent
│       ├── registry.rs         # Agent registry
│       ├── protocol.rs         # Communication protocol
│       └── undo.rs             # Rollback system
├── data/
│   └── actions.yaml            # Action definitions
├── .beads/                     # Issue tracking
├── .viber/
│   ├── specs/                  # Generated specs
│   ├── memory/                 # Session memory
│   └── config.yaml             # VIBER config
├── Cargo.toml
└── DEVELOPMENT_BLUEPRINT.md    # This file
```

---

## 8. Quick Reference

### Starting a Session
```bash
# 1. Check for work
bd ready

# 2. Activate memory
memcord_name "viber-tui-dev"
memcord_read

# 3. Check roadmap
readroadmap

# 4. Start Neovim (separate terminal)
nvim src/main.rs

# 5. Begin work on next action
```

### Ending a Session
```bash
# 1. Save progress
memcord_save_progress

# 2. Update roadmap
updateroadmap actionNumber="X.XX" note="Completed: ..." status="completed"

# 3. Close issues
bd close <id>

# 4. Commit and sync
git add . && git commit -m "feat: ..."
bd sync
git push
```

### Emergency Commands
```bash
# Kill stuck agent
background_cancel all=true

# Rollback last change
git checkout -- <file>

# Check LSP health
neovim_nvim_lsp_clients

# Force resync beads
bd sync --from-main
```

---

*Blueprint Version: 1.0*
*Generated: 2025-12-31*
*Project: viber-tui*
