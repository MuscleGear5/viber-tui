# VIBER-TUI Tool Utilization Map

> Complete mapping of OpenCode ecosystem to VIBER-TUI development

---

## Overview: What We Have Available

### NCP Profiles (4 total)
| Profile | MCPs | Primary Use |
|---------|------|-------------|
| `ncp-coding` | 31 | Code intelligence, LSP, AST, GitHub search |
| `ncp-research` | 8 | Web search, documentation, deep research |
| `ncp-creative` | 10 | Image generation, design, visual content |
| `ncp-productivity` | 13 | Calendar, email, task management |

### Subagents (125+ available)
Organized by category - see detailed mapping below.

### Skills (100+ available)
Project skills, personal skills, superpowers.

### MCP Servers (Direct)
- `nvim-mcp` (33 tools) - Neovim integration
- `memcord` (19 tools) - Persistent memory
- `context7` - Library documentation
- `grep_app` - GitHub code search
- `exa_web_search` - Web search
- `ast_grep` - AST-aware code search/replace
- `lsp_*` - Language server protocol tools

### Native Tools
- `roadmap` tools - Project planning
- `todo` tools - Task tracking
- `beads` (bd CLI) - Issue tracking
- File operations (read, write, edit, glob, grep)
- Bash execution

---

## Phase-by-Phase Tool Utilization

### Feature 1: Core Infrastructure

#### Action 1.01 - Color Palette
```yaml
tools:
  research:
    - skill: /ui-styling          # Tailwind + shadcn patterns
    - skill: /frontend-design     # Design quality principles
    - ncp-research:
        - exa_web_search: "ratatui color palette cyberpunk terminal"
        - context7: "/ratatui/ratatui" query="Color RGB styling"
    - grep_app: "Color::Rgb" lang=["Rust"] repo="ratatui"
    
  implementation:
    - subagent: rust-engineer     # Primary implementer
    - tools:
        - read: src/theme/colors.rs
        - edit: Apply color constants
        - lsp_diagnostics: Verify no errors
        
  verification:
    - bash: "cargo check"
    - lsp_diagnostics: src/theme/colors.rs
```

#### Action 1.02 - Animation System
```yaml
tools:
  research:
    - skill: /frontend-design
    - ncp-research:
        - context7: "/ratatui/ratatui" query="animation tick frame"
        - grep_app: "impl.*Animated" lang=["Rust"]
    - ncp-coding:
        - ast_grep_search: pattern="fn tick($$$)" lang="rust"
        
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/theme/animation.rs
        - lsp_hover: Check trait implementations
        - lsp_diagnostics: Verify
        
  verification:
    - bash: "cargo test animation"
```

#### Action 1.03 - Base Widget Traits
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="Widget trait impl"
    - grep_app: "impl Widget for" lang=["Rust"] repo="ratatui"
    - ast_grep_search: pattern="impl Widget for $NAME { $$$ }" lang="rust"
    
  implementation:
    - subagent: rust-engineer
    - backup: frontend-developer
    - tools:
        - read: Existing widget files
        - write: src/widgets/mod.rs (base traits)
        - lsp_document_symbols: Check structure
        
  verification:
    - bash: "cargo check"
    - lsp_diagnostics: src/widgets/*.rs
```

#### Action 1.04 - App State (Tokio Channels)
```yaml
tools:
  research:
    - skill: /backend-development
    - context7: "/tokio-rs/tokio" query="mpsc channel broadcast"
    - grep_app: "tokio::sync::mpsc" lang=["Rust"]
    
  implementation:
    - subagent: rust-engineer
    - subagent: backend-developer (async patterns)
    - tools:
        - write: src/app.rs
        - write: src/events.rs
        
  verification:
    - bash: "cargo test"
    - lsp_diagnostics: src/app.rs
```

#### Action 1.05 - Keyboard Navigation
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="crossterm KeyEvent"
    - grep_app: "KeyCode::" lang=["Rust"] repo="ratatui"
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - edit: src/main.rs (event loop)
        - lsp_find_references: KeyEvent usage
        
  verification:
    - bash: "cargo run" (manual test)
```

---

### Feature 2: Primary Views

#### Action 2.01 - Chat View
```yaml
tools:
  research:
    - skill: /frontend-design
    - skill: /ai-sdk-ui           # Chat UI patterns
    - context7: "/ratatui/ratatui" query="Paragraph wrap scroll"
    - grep_app: "streaming.*response" lang=["Rust"]
    
  implementation:
    - subagent: frontend-developer
    - subagent: ui-engineer
    - tools:
        - write: src/views/chat.rs
        - nvim-mcp: Real-time editing in Neovim
        
  verification:
    - bash: "cargo check"
    - skill: /code-review
```

#### Action 2.02 - Workflow View (DAG)
```yaml
tools:
  research:
    - skill: /mermaidjs-v11       # DAG visualization concepts
    - context7: "/ratatui/ratatui" query="Canvas draw line"
    - grep_app: "Canvas::new" lang=["Rust"]
    
  implementation:
    - subagent: frontend-developer
    - subagent: ui-designer
    - tools:
        - write: src/views/workflow.rs
        - write: src/widgets/dag.rs
        
  verification:
    - visual inspection via cargo run
```

#### Action 2.03 - Tasks View
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="List StatefulWidget"
    - skill: /frontend-design
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - read: src/widgets/fuzzy_list.rs (existing)
        - write: src/views/tasks.rs
        - lsp_references: Reuse existing components
        
  verification:
    - bash: "cargo check"
```

#### Action 2.04 - Agents View
```yaml
tools:
  research:
    - grep_app: "agent.*status" lang=["Rust"]
    - context7: "/ratatui/ratatui" query="Table rows columns"
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/views/agents.rs
        - write: src/widgets/agent_card.rs
        
  verification:
    - lsp_diagnostics: src/views/agents.rs
```

---

### Feature 3: Editor Integration Views

#### Action 3.01 - Buffer View
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="syntax highlight code"
    - grep_app: "syntect" lang=["Rust"]
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/views/buffer.rs
        - write: src/widgets/code_block.rs
        
  verification:
    - bash: "cargo check"
```

#### Action 3.02 - Diff View
```yaml
tools:
  research:
    - grep_app: "similar.*diff" lang=["Rust"]
    - context7: query="unified diff side-by-side"
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/views/diff.rs
        - write: src/widgets/diff.rs
        
  verification:
    - bash: "cargo test diff"
```

#### Action 3.03 - LSP View
```yaml
tools:
  research:
    - All lsp_* tools documentation
    - nvim-mcp tool reference
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/views/lsp.rs
        - lsp_diagnostics: Self-referential testing
        
  verification:
    - Manual test with real LSP data
```

#### Action 3.04 - nvim-mcp Integration
```yaml
tools:
  critical_tools:
    - neovim_nvim_get_targets     # Connection
    - neovim_nvim_read            # Buffer content
    - neovim_nvim_cursor_position # Cursor tracking
    - neovim_nvim_navigate        # Jump to location
    - neovim_nvim_buffer_diagnostics  # Errors/warnings
    - neovim_nvim_lsp_hover       # Type info
    - neovim_nvim_lsp_definition  # Go to def
    - neovim_nvim_lsp_references  # Find usages
    - neovim_nvim_lsp_code_actions    # Quick fixes
    - neovim_nvim_lsp_rename      # Refactor
    - neovim_nvim_lsp_formatting  # Format code
    - neovim_nvim_exec_lua        # Arbitrary Neovim commands
    
  implementation:
    - subagent: rust-engineer
    - subagent: mcp-developer     # MCP integration expertise
    - tools:
        - write: src/nvim/mod.rs
        - write: src/nvim/client.rs
        
  verification:
    - Integration test with running Neovim
```

---

### Feature 4: Workflow Phase Views

#### Action 4.01 - Questionnaire View
```yaml
tools:
  research:
    - skill: /kiro-skill          # EARS format requirements
    - skill: /openspec-discovery  # Questionnaire patterns
    - context7: query="dynamic form validation"
    
  implementation:
    - subagent: frontend-developer
    - subagent: ux-researcher     # Form UX
    - tools:
        - write: src/views/questionnaire.rs
        
  verification:
    - Manual test with sample questions
```

#### Action 4.02 - Spec View
```yaml
tools:
  research:
    - skill: /spec-kit-skill
    - skill: /doc-coauthoring
    
  implementation:
    - subagent: frontend-developer
    - subagent: technical-writer
    - tools:
        - write: src/views/spec.rs
        
  verification:
    - Display sample spec
```

#### Action 4.03 - Phase Transitions
```yaml
tools:
  research:
    - skill: /planning
    - grep_app: "state.*machine" lang=["Rust"]
    
  implementation:
    - subagent: rust-engineer
    - subagent: workflow-orchestrator
    - tools:
        - write: src/workflow/mod.rs
        - write: src/workflow/phases.rs
        
  verification:
    - Unit tests for state transitions
```

---

### Feature 5: VIBER God Agent

#### Action 5.01 - Agent Protocol
```yaml
tools:
  research:
    - skill: /mcp-builder         # Protocol design
    - skill: /backend-development
    - grep_app: "agent.*protocol" lang=["Rust"]
    
  implementation:
    - subagent: rust-engineer
    - subagent: architect-reviewer
    - subagent: multi-agent-coordinator
    - tools:
        - write: src/agents/protocol.rs
        
  verification:
    - Protocol documentation
    - Unit tests
```

#### Action 5.02 - Agent Registry
```yaml
tools:
  research:
    - grep_app: "registry.*agent" lang=["Rust"]
    - skill: /backend-development
    
  implementation:
    - subagent: rust-engineer
    - subagent: agent-organizer
    - tools:
        - write: src/agents/registry.rs
        
  verification:
    - Health check tests
```

#### Action 5.03 - Undo System
```yaml
tools:
  research:
    - grep_app: "undo.*redo.*stack" lang=["Rust"]
    - context7: query="command pattern undo"
    
  implementation:
    - subagent: rust-engineer
    - subagent: refactoring-specialist
    - tools:
        - write: src/agents/undo.rs
        
  verification:
    - Rollback integration tests
```

#### Action 5.04 - Intervention Triggers
```yaml
tools:
  research:
    - skill: /backend-development
    - grep_app: "circuit.*breaker" lang=["Rust"]
    
  implementation:
    - subagent: rust-engineer
    - subagent: sre-engineer       # Reliability patterns
    - tools:
        - write: src/agents/triggers.rs
        
  verification:
    - Timeout/error threshold tests
```

---

### Feature 6: Advanced Widgets

#### Action 6.01 - DAG Renderer
```yaml
tools:
  research:
    - skill: /mermaidjs-v11
    - context7: "/ratatui/ratatui" query="Canvas Painter"
    - grep_app: "braille.*pattern" lang=["Rust"]
    
  implementation:
    - subagent: frontend-developer
    - subagent: ui-designer
    - tools:
        - write: src/widgets/dag.rs
        
  verification:
    - Visual test
```

#### Action 6.02 - Sparkline Charts
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="Sparkline"
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/widgets/sparkline.rs
        
  verification:
    - cargo check
```

#### Action 6.03 - Calendar Heatmap
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="Calendar monthly"
    - grep_app: "heatmap.*calendar" lang=["Rust"]
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/widgets/heatmap.rs
        
  verification:
    - Visual test
```

#### Action 6.04 - Advanced Spinners
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="spinner animation"
    - grep_app: "braille.*spinner" lang=["Rust"]
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/widgets/spinner.rs
        
  verification:
    - Animation visual test
```

---

### Feature 7: Integration & Polish

#### Action 7.01 - Memcord Integration
```yaml
tools:
  memcord_tools:
    - memcord_name: "viber-tui-sessions"
    - memcord_save: Persist session state
    - memcord_read: Restore previous context
    - memcord_query: Search past decisions
    - memcord_save_progress: Auto-summary
    - memcord_tag: Categorize entries
    - memcord_group: Organize by feature
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/integrations/memcord.rs
```

#### Action 7.02 - Beads Integration
```yaml
tools:
  beads_commands:
    - bd ready: Find available work
    - bd show: Display issue details
    - bd create: Create new issues
    - bd update: Change status
    - bd close: Complete issues
    - bd sync: Git synchronization
    
  implementation:
    - subagent: rust-engineer
    - tools:
        - write: src/integrations/beads.rs
```

#### Action 7.03 - Notification System
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="popup overlay"
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/widgets/toast.rs
```

#### Action 7.04 - Help Overlay
```yaml
tools:
  research:
    - context7: "/ratatui/ratatui" query="modal overlay"
    
  implementation:
    - subagent: frontend-developer
    - tools:
        - write: src/widgets/help.rs
```

#### Action 7.05 - Performance Optimization
```yaml
tools:
  research:
    - skill: /code-refactoring
    - grep_app: "lazy.*render" lang=["Rust"]
    
  implementation:
    - subagent: performance-engineer
    - subagent: rust-engineer
    - tools:
        - Profile with cargo flamegraph
        - ast_grep_replace: Batch optimizations
```

---

## Cross-Cutting Tool Usage

### Every Implementation Task Uses:
```yaml
always:
  - read/write/edit: File operations
  - lsp_diagnostics: Error checking after every edit
  - bash "cargo check": Compilation verification
  - todowrite/todoread: Progress tracking
  - roadmap tools: Status updates

frequently:
  - lsp_hover: Type information
  - lsp_find_references: Impact analysis
  - ast_grep_search: Pattern finding
  - grep: Content search
  - glob: File discovery
```

### Research Phase Template:
```yaml
research_stack:
  1. skill: Load relevant skill first
  2. context7: Check library documentation
  3. grep_app: Find real-world examples
  4. ast_grep_search: Find patterns in codebase
  5. ncp-research/exa_web_search: External resources if needed
```

### Verification Phase Template:
```yaml
verification_stack:
  1. lsp_diagnostics: No errors
  2. bash "cargo check": Compiles
  3. bash "cargo test": Tests pass
  4. skill /code-review: Quality check
  5. updateroadmap: Mark complete
```

---

## Subagent Dispatch Matrix

| Task Type | Primary | Secondary | Tertiary |
|-----------|---------|-----------|----------|
| Rust systems code | `rust-engineer` | `backend-developer` | `performance-engineer` |
| UI/Views | `frontend-developer` | `ui-engineer` | `ui-designer` |
| Architecture | `architect-reviewer` | `microservices-architect` | `code-reviewer` |
| Protocol design | `mcp-developer` | `api-designer` | `backend-developer` |
| Agent orchestration | `multi-agent-coordinator` | `workflow-orchestrator` | `agent-organizer` |
| Testing | `qa-expert` | `test-automator` | `debugger` |
| Documentation | `technical-writer` | `documentation-engineer` | `api-documenter` |
| Performance | `performance-engineer` | `dx-optimizer` | `refactoring-specialist` |
| Security | `security-engineer` | `security-auditor` | `penetration-tester` |

---

## Background Task Patterns

### Parallel Research (DEFAULT)
```typescript
// Launch multiple research tasks simultaneously
background_task(agent="explore", prompt="Find ratatui animation patterns...")
background_task(agent="explore", prompt="Find crossterm event handling...")
background_task(agent="librarian", prompt="Get tokio channel best practices...")

// Continue with other work...
// Collect when needed:
const results = await Promise.all([
  background_output(task_id="..."),
  background_output(task_id="..."),
  background_output(task_id="...")
]);
```

### Parallel Implementation (for independent components)
```typescript
// Multiple subagents working on separate files
Task(subagent="frontend-developer", prompt="Build Chat view in src/views/chat.rs...")
Task(subagent="frontend-developer", prompt="Build Tasks view in src/views/tasks.rs...")
// These can run in parallel since they don't share files
```

### Sequential Implementation (for dependencies)
```typescript
// Must complete in order
await Task(subagent="rust-engineer", prompt="Create base Widget trait...")
await Task(subagent="frontend-developer", prompt="Implement ChatWidget using base trait...")
```

---

## Session Workflow

### Starting Work
```bash
# 1. Check memory
memcord_use "viber-tui-dev"
memcord_read

# 2. Check roadmap
readroadmap

# 3. Check issues
bd ready

# 4. Load relevant skill
skill "rust-development" # or whatever applies

# 5. Mark action in progress
updateroadmap actionNumber="X.XX" status="in_progress" note="Starting..."

# 6. Create todos for subtasks
todowrite [...]
```

### During Work
```bash
# Continuous verification
lsp_diagnostics filePath="src/..."
bash "cargo check"

# Save progress periodically
memcord_save_progress
```

### Completing Work
```bash
# 1. Final verification
bash "cargo test"
lsp_diagnostics filePath="src/..."

# 2. Update tracking
updateroadmap actionNumber="X.XX" status="completed" note="Done: ..."
bd close <id>

# 3. Save context
memcord_save_progress

# 4. Commit
git add . && git commit -m "feat(viber): ..."
bd sync
git push
```

---

## Quick Reference Card

### Most Used Tools
| Tool | Frequency | Purpose |
|------|-----------|---------|
| `read/write/edit` | Every task | File operations |
| `lsp_diagnostics` | After every edit | Error checking |
| `bash "cargo check"` | Every task | Compilation |
| `context7_query-docs` | Research phase | Library docs |
| `grep_app_searchGitHub` | Research phase | Real examples |
| `ast_grep_search` | Pattern finding | Code patterns |
| `todowrite` | Every task | Progress tracking |
| `updateroadmap` | Task boundaries | Roadmap status |
| `Task` (subagent) | Complex work | Delegation |
| `background_task` | Research | Parallel execution |

### Key Skills
| Skill | When |
|-------|------|
| `/frontend-design` | UI work |
| `/backend-development` | Systems code |
| `/code-review` | Before completion |
| `/test-driven-development` | New features |
| `/mcp-builder` | Protocol work |
| `/planning` | Complex features |

### Essential Subagents
| Agent | When |
|-------|------|
| `rust-engineer` | All Rust code |
| `frontend-developer` | UI/views |
| `architect-reviewer` | Design decisions |
| `code-reviewer` | Quality gates |
| `qa-expert` | Testing |

---

*Tool Utilization Map Version: 1.0*
*Generated: 2025-12-31*
