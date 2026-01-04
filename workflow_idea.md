# Vibe Coding Workflow

> **Philosophy**: User answers questions. AI writes specs. No ambiguity = no guessing.
> **Neovim-Native**: All edits through nvim-mcp. LSP checked after every edit. User sees everything.

---

## Core Principles

```
1. USER ANSWERS, AI WRITES
   ❌ "Write your requirements"
   ✅ "Pick one: [A] REST  [B] GraphQL  [C] Both"

2. SPEC-FIRST, ALWAYS
   No spec = No code
   Unclear spec = BLOCK and ASK

3. NEOVIM-NATIVE
   ❌ Read/write files directly
   ✅ Read/write buffers via nvim-mcp (user sees it)

4. LSP-DRIVEN
   ❌ Check errors at the end
   ✅ Check LSP after EVERY edit

5. TREESITTER-AWARE
   ❌ "Insert at line 47"
   ✅ "Add function after X" (AST-aware)
```

---

## The Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 0: IDEA CAPTURE                                         │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  User: "I want to build X"                                     │
│  AI captures raw idea + project context                        │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 1: IDEA DECOMPOSITION                                   │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  AI presents understanding as bullets                          │
│  User confirms: [A] Yes  [B] No, clarify  [C] Partially        │
│  Loop until confirmed                                          │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 2: SPEC QUESTIONNAIRE                                   │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  Progressive multichoice questions:                            │
│                                                                 │
│  Layer 1: Core Architecture                                    │
│  Layer 2: Feature Scope (based on Layer 1)                     │
│  Layer 3: Technical Decisions                                  │
│  Layer 4: Edge Cases & Constraints                             │
│                                                                 │
│  Each answer shapes next questions                             │
│  Progress bar: ▓▓▓▓▓░░░░░ 50% (8/16)                           │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 3: SPEC GENERATION                                      │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  AI generates openspec.yaml from all answers                   │
│  User reviews: [A] Approve  [B] Edit  [C] Redo  [D] Add more   │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 4: TASK DECOMPOSITION                                   │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  AI breaks spec into tasks (opentasks.yaml)                    │
│  User reviews: [A] Start  [B] Adjust  [C] Add  [D] Remove      │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 5: SCAFFOLD                                             │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  Scaffold Agent:                                               │
│  1. Create project structure                                   │
│  2. nvim-mcp: Open root in Neovim                              │
│  3. nvim-mcp: Open key files in buffers                        │
│  4. Wait for LSP to attach                                     │
│  5. Verify: 0 diagnostics on scaffold                          │
│                                                                 │
│  User sees: Project tree, files open, LSP ready                │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 6: IMPLEMENTATION (LSP-driven loop)                     │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  FOR EACH TASK:                                                │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │  1. PRE-FLIGHT                                          │   │
│  │     • nvim-mcp: Read buffer state (not disk!)           │   │
│  │     • nvim-mcp: Get LSP diagnostics                     │   │
│  │     • nvim-mcp: Get treesitter context                  │   │
│  │     • Check: Is spec clear? (if not → ASK USER)         │   │
│  │                                                         │   │
│  │  2. UNDO CHECKPOINT                                     │   │
│  │     • nvim-mcp: :wa (save all)                          │   │
│  │     • Mark undo point (user can revert with u)          │   │
│  │                                                         │   │
│  │  3. IMPLEMENT (treesitter-aware)                        │   │
│  │     • nvim-mcp: Edit using AST nodes when possible      │   │
│  │     • Small, atomic edits (not giant rewrites)          │   │
│  │     • User sees: Buffer updating in real-time           │   │
│  │                                                         │   │
│  │  4. LSP GATE (after EVERY edit)                         │   │
│  │     • nvim-mcp: Wait for diagnostics (~200ms)           │   │
│  │     • IF errors > 0:                                    │   │
│  │       - Populate quickfix                               │   │
│  │       - Fix each error (loop)                           │   │
│  │       - User sees: Agent jumping to errors              │   │
│  │     • IF errors == 0: Continue                          │   │
│  │                                                         │   │
│  │  5. FORMAT & LINT                                       │   │
│  │     • nvim-mcp: Run formatter                           │   │
│  │     • nvim-mcp: Run linter                              │   │
│  │     • Fix any new issues                                │   │
│  │                                                         │   │
│  │  6. TEST (targeted, not full suite)                     │   │
│  │     • nvim-mcp: Run test for this module                │   │
│  │     • Populate quickfix with failures                   │   │
│  │     • Fix failures (loop back to step 3)                │   │
│  │                                                         │   │
│  │  7. SPEC REVIEW                                         │   │
│  │     • Does code match spec?                             │   │
│  │     • Yes → continue                                    │   │
│  │     • No → fix or clarify with user                     │   │
│  │                                                         │   │
│  │  8. CODE REVIEW                                         │   │
│  │     • Quality check                                     │   │
│  │     • Yes → commit                                      │   │
│  │     • No → refactor                                     │   │
│  │                                                         │   │
│  │  9. COMMIT CHECKPOINT                                   │   │
│  │     • nvim-mcp: :wa                                     │   │
│  │     • git commit (atomic, per task)                     │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 7: POLISH                                               │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  Polish Agent:                                                 │
│  1. Workspace diagnostics (ALL files)                          │
│     • nvim-mcp: Get diagnostics across workspace               │
│     • Populate quickfix                                        │
│     • Fix remaining warnings/errors                            │
│                                                                 │
│  2. Dead code detection                                        │
│     • nvim-mcp: LSP references for unused symbols              │
│     • Remove or flag unused code                               │
│                                                                 │
│  3. Import optimization                                        │
│     • nvim-mcp: LSP organize imports                           │
│     • Remove unused, sort                                      │
│                                                                 │
│  4. Consistency pass                                           │
│     • nvim-mcp: Format all files                               │
│     • Check naming conventions                                 │
│     • Verify doc comments                                      │
│                                                                 │
│  5. Final diagnostic gate                                      │
│     • 0 errors required                                        │
│     • Warnings reviewed                                        │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 8: VALIDATION                                           │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  Validation Agent:                                             │
│  ✓ Full test suite                                             │
│  ✓ Type check (tsc, mypy, etc)                                 │
│  ✓ Release build                                               │
│  ✓ Spec compliance final check                                 │
│  ✓ No unspecified features (YAGNI)                             │
│                                                                 │
│                          ▼                                      │
│                                                                 │
│  ═══════════════════════════════════════════════════════════   │
│  PHASE 9: DELIVERY                                             │
│  ═══════════════════════════════════════════════════════════   │
│                                                                 │
│  [View Spec] [View Code] [Run Tests] [Try It] [Deploy]         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## The Edit Cycle (Every Single Edit)

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│     ┌─────────────┐                                            │
│     │  Read State │  nvim-mcp: buffer, diagnostics, treesitter │
│     └──────┬──────┘                                            │
│            │                                                    │
│            ▼                                                    │
│     ┌─────────────┐                                            │
│     │  Plan Edit  │  Use AST awareness, minimal change         │
│     └──────┬──────┘                                            │
│            │                                                    │
│            ▼                                                    │
│     ┌─────────────┐                                            │
│     │ Apply Edit  │  nvim-mcp: buffer_edit (user sees it)      │
│     └──────┬──────┘                                            │
│            │                                                    │
│            ▼                                                    │
│     ┌─────────────┐                                            │
│     │  Wait LSP   │  ~200ms for diagnostics                    │
│     └──────┬──────┘                                            │
│            │                                                    │
│            ▼                                                    │
│     ┌─────────────┐                                            │
│     │Check Errors │──▶ Errors? YES → Fix → Loop back           │
│     └──────┬──────┘           NO  → Continue                   │
│            │                                                    │
│            ▼                                                    │
│        Next Edit                                               │
│                                                                 │
│  Agent NEVER writes blind. Every edit is validated.            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## nvim-mcp Required Operations

```
BUFFER OPS          LSP OPS              TREESITTER OPS
───────────         ───────              ──────────────
buffer_read         lsp_diagnostics      ts_node_at_cursor
buffer_edit         lsp_hover            ts_parent_scope
buffer_save         lsp_definition       ts_siblings
buffer_save_all     lsp_references       ts_select_node
buffer_list         lsp_rename
buffer_open         lsp_code_action
                    lsp_format
QUICKFIX OPS        lsp_organize
────────────
qf_set              EXECUTION
qf_get              ─────────
qf_next             terminal_run
qf_open             command_exec
```

---

## Question Types (TUI Native)

```
SINGLE CHOICE (press A/B/C/D)          MULTI-SELECT (Space to toggle)
┌──────────────────────────────┐       ┌──────────────────────────────┐
│  "Primary language?"         │       │  "Output formats?"           │
│                              │       │                              │
│  ○ [A] Rust                  │       │  [x] [A] PDF                 │
│  ○ [B] Go                    │       │  [ ] [B] HTML                │
│  ○ [C] TypeScript            │       │  [x] [C] EPUB                │
└──────────────────────────────┘       └──────────────────────────────┘

YES/NO (press Y/N)                     CONDITIONAL (expands on Yes)
┌──────────────────────────────┐       ┌──────────────────────────────┐
│  "Shell completions?"        │       │  "Database needed?"          │
│                              │       │                              │
│  [Y] Yes    [N] No           │       │  [Y] Yes → Which? ORM?       │
└──────────────────────────────┘       │  [N] No  → Skip DB questions │
                                       └──────────────────────────────┘
```

---

## The Golden Rules

```
╔═════════════════════════════════════════════════════════════════╗
║                                                                 ║
║  SPEC RULES                                                     ║
║  ──────────                                                     ║
║  • IF spec_unclear(task): BLOCK → ASK → UPDATE → IMPLEMENT      ║
║  • NEVER guess. NEVER assume. ALWAYS ask.                       ║
║                                                                 ║
║  NEOVIM RULES                                                   ║
║  ────────────                                                   ║
║  • NEVER read from disk if buffer is open → buffer_read         ║
║  • NEVER write to disk directly → buffer_edit + buffer_save     ║
║  • NEVER edit without checking LSP after                        ║
║  • NEVER ignore quickfix → populate it, fix everything in it    ║
║  • NEVER make giant edits → small, atomic, undoable             ║
║  • ALWAYS use treesitter when available → edit nodes, not lines ║
║                                                                 ║
╚═════════════════════════════════════════════════════════════════╝
```

---

## Subagents

| Agent | Role |
|-------|------|
| **Meta-Agent** | Orchestrator, delegates to specialists |
| **Questionnaire-Agent** | Runs spec questionnaire |
| **Scaffold-Agent** | Creates project structure, opens in Neovim |
| **Code-Agent** | Implements tasks (LSP-driven loop) |
| **Spec-Reviewer** | Checks code matches spec |
| **Code-Reviewer** | Checks code quality |
| **Polish-Agent** | Workspace cleanup, dead code, imports |
| **Validation-Agent** | Full tests, type check, build |
| **Deploy-Agent** | Ships it |

---

## Files

```
project/
├── .opencode/
│   ├── openspec.yaml        # Living spec (from questionnaire)
│   ├── opentasks.yaml       # Task breakdown
│   ├── questionnaire.log    # All Q&A history
│   ├── decisions.log        # JIT clarifications during impl
│   ├── lsp-snapshots/       # Diagnostic state per task
│   │   ├── task-1.json
│   │   └── ...
│   └── undo-checkpoints/    # Undo markers per task
│       ├── task-1.txt
│       └── ...
```

---

*Uses NCP-coding for MCP tools. All edits through nvim-mcp. LSP-driven. Treesitter-aware.*

---

## DAG Workflow Visualization

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  WORKFLOW DAG (Directed Acyclic Graph)                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Phases execute as a DAG, not a linear pipeline.                               │
│  Some nodes can run in parallel. Some must wait.                               │
│                                                                                 │
│                        ┌──────────────┐                                         │
│                        │    IDEA      │                                         │
│                        │   (user)     │                                         │
│                        └──────┬───────┘                                         │
│                               │                                                 │
│                               ▼                                                 │
│                        ┌──────────────┐                                         │
│                        │   DECOMPOSE  │                                         │
│                        │  (confirm)   │                                         │
│                        └──────┬───────┘                                         │
│                               │                                                 │
│                               ▼                                                 │
│                        ┌──────────────┐                                         │
│                        │QUESTIONNAIRE │                                         │
│                        │ (multichoice)│                                         │
│                        └──────┬───────┘                                         │
│                               │                                                 │
│                               ▼                                                 │
│                        ┌──────────────┐                                         │
│                        │  SPEC GEN    │                                         │
│                        │  (approve)   │                                         │
│                        └──────┬───────┘                                         │
│                               │                                                 │
│                               ▼                                                 │
│                        ┌──────────────┐                                         │
│                        │TASK BREAKDOWN│                                         │
│                        └──────┬───────┘                                         │
│                               │                                                 │
│              ┌────────────────┼────────────────┐                                │
│              ▼                ▼                ▼                                │
│       ┌────────────┐   ┌────────────┐   ┌────────────┐                          │
│       │  WORKTREE  │   │  SCAFFOLD  │   │   (wait)   │                          │
│       │   setup    │   │  structure │   │            │                          │
│       └─────┬──────┘   └─────┬──────┘   └────────────┘                          │
│             │                │                                                  │
│             └────────┬───────┘                                                  │
│                      │                                                          │
│                      ▼                                                          │
│               ┌────────────┐                                                    │
│               │   TASKS    │◄─── Can parallelize independent tasks              │
│               │ (parallel) │                                                    │
│               └─────┬──────┘                                                    │
│                     │                                                           │
│        ┌────────────┼────────────┬────────────┐                                 │
│        ▼            ▼            ▼            ▼                                 │
│   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐                           │
│   │ Task 1  │  │ Task 2  │  │ Task 3  │  │ Task N  │                           │
│   │(backend)│  │(frontend│  │ (tests) │  │  ...    │                           │
│   └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘                           │
│        │            │            │            │                                 │
│        └────────────┴────────────┴────────────┘                                 │
│                      │                                                          │
│                      ▼                                                          │
│               ┌────────────┐                                                    │
│               │   POLISH   │                                                    │
│               │ (workspace)│                                                    │
│               └─────┬──────┘                                                    │
│                     │                                                           │
│                     ▼                                                           │
│               ┌────────────┐                                                    │
│               │  VALIDATE  │                                                    │
│               │(full suite)│                                                    │
│               └─────┬──────┘                                                    │
│                     │                                                           │
│                     ▼                                                           │
│               ┌────────────┐                                                    │
│               │  DELIVER   │                                                    │
│               └────────────┘                                                    │
│                                                                                 │
│  Legend: ──▶ Sequential    ═══▶ Parallel possible                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Parallel Agent Dispatch

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  WHEN TO PARALLELIZE                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Use parallel agents when:                                                     │
│  • 2+ independent tasks with no shared state                                   │
│  • Tasks don't edit the same files                                             │
│  • Each task can be understood without context from others                     │
│                                                                                 │
│  DON'T parallelize when:                                                       │
│  • Tasks share files (merge conflicts)                                         │
│  • Task B depends on Task A's output                                           │
│  • Need to understand full system state                                        │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │   Meta-Agent identifies independent tasks:                              │   │
│  │                                                                         │   │
│  │   Task 1: Backend API ────────┐                                         │   │
│  │   Task 2: Frontend UI ────────┼──▶ INDEPENDENT → Parallelize            │   │
│  │   Task 3: Database schema ────┘                                         │   │
│  │                                                                         │   │
│  │   Task 4: API integration ────┐                                         │   │
│  │   Task 5: E2E tests ──────────┼──▶ DEPENDENT → Sequential               │   │
│  │                               │    (needs API + Frontend first)         │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  Parallel dispatch pattern:                                                    │
│                                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                             │
│  │ Agent 1     │  │ Agent 2     │  │ Agent 3     │                             │
│  │ backend-dev │  │ frontend-dev│  │ db-admin    │                             │
│  │ ▓▓▓▓░░░ 60% │  │ ▓▓▓░░░░ 40% │  │ ▓▓▓▓▓░░ 70% │                             │
│  │             │  │             │  │             │                             │
│  │ Task: API   │  │ Task: UI    │  │ Task: Schema│                             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                             │
│         │                │                │                                     │
│         └────────────────┴────────────────┘                                     │
│                          │                                                      │
│                          ▼                                                      │
│                   ┌─────────────┐                                               │
│                   │   MERGE     │  All complete → Continue                      │
│                   │  RESULTS    │                                               │
│                   └─────────────┘                                               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Git Worktree Integration

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ISOLATED WORKSPACES                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  For feature work, create isolated git worktree:                               │
│                                                                                 │
│  1. SETUP WORKTREE                                                             │
│     • Check for existing .worktrees/ or worktrees/                             │
│     • Verify directory is gitignored                                           │
│     • Create: git worktree add .worktrees/feature-name -b feature/name         │
│     • Run project setup (npm install, cargo build, etc)                        │
│     • Verify tests pass (clean baseline)                                       │
│                                                                                 │
│  2. WORK IN WORKTREE                                                           │
│     • All implementation happens here                                          │
│     • nvim-mcp connects to THIS worktree's Neovim                              │
│     • Atomic commits per task                                                  │
│                                                                                 │
│  3. FINISH BRANCH                                                              │
│     • Squash commits if needed                                                 │
│     • Merge to main or create PR                                               │
│     • Clean up worktree                                                        │
│                                                                                 │
│  Directory structure:                                                          │
│                                                                                 │
│  project/                                                                      │
│  ├── .worktrees/              # Isolated workspaces (gitignored)               │
│  │   ├── feature-auth/        # Worktree for auth feature                      │
│  │   ├── feature-api/         # Worktree for API feature                       │
│  │   └── bugfix-login/        # Worktree for login bugfix                      │
│  ├── src/                     # Main workspace                                 │
│  └── .gitignore               # Contains: .worktrees/                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Execution Modes

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CHOOSE EXECUTION MODE                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  MODE 1: SUBAGENT-DRIVEN (same session)                                        │
│  ───────────────────────────────────────                                       │
│  • Fresh subagent per task                                                     │
│  • Two-stage review (spec + quality)                                           │
│  • Faster iteration, no session switch                                         │
│  • Use for: Most implementation work                                           │
│                                                                                 │
│  MODE 2: EXECUTING-PLANS (parallel session)                                    │
│  ──────────────────────────────────────────                                    │
│  • Batch execution with checkpoints                                            │
│  • Architect review between batches                                            │
│  • Use for: Large features, needs human oversight                              │
│                                                                                 │
│  MODE 3: PARALLEL DISPATCH                                                     │
│  ─────────────────────────                                                     │
│  • Multiple agents on independent tasks                                        │
│  • Each agent isolated scope                                                   │
│  • Use for: 3+ independent tasks                                               │
│                                                                                 │
│  Decision flow:                                                                │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │   Tasks independent?                                                    │   │
│  │        │                                                                │   │
│  │   ┌────┴────┐                                                          │   │
│  │  YES        NO                                                          │   │
│  │   │          │                                                          │   │
│  │   ▼          │                                                          │   │
│  │  3+ tasks?   │                                                          │   │
│  │   │          │                                                          │   │
│  │  ┌┴──┐       │                                                          │   │
│  │ YES  NO      │                                                          │   │
│  │  │    │      │                                                          │   │
│  │  ▼    │      ▼                                                          │   │
│  │ PARALLEL    SUBAGENT-DRIVEN                                             │   │
│  │ DISPATCH    (or EXECUTING-PLANS if human review needed)                 │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Review Checkpoints

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TWO-STAGE REVIEW (per task)                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  STAGE 1: SPEC REVIEW                                                          │
│  ─────────────────────                                                         │
│  Question: Does code match spec?                                               │
│                                                                                 │
│  Check:                                                                        │
│  • All spec requirements implemented                                           │
│  • No unspecified features added (YAGNI)                                       │
│  • Edge cases from spec handled                                                │
│                                                                                 │
│  Result:                                                                       │
│  • ✓ Pass → Continue to Stage 2                                                │
│  • ✗ Fail → Fix gaps, re-review                                                │
│                                                                                 │
│  STAGE 2: CODE REVIEW                                                          │
│  ─────────────────────                                                         │
│  Question: Is code quality acceptable?                                         │
│                                                                                 │
│  Check:                                                                        │
│  • Clean, readable code                                                        │
│  • No magic numbers                                                            │
│  • Proper error handling                                                       │
│  • Tests cover the code                                                        │
│  • No obvious performance issues                                               │
│                                                                                 │
│  Result:                                                                       │
│  • ✓ Pass → Commit, next task                                                  │
│  • ✗ Fail → Fix issues, re-review                                              │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │  IMPLEMENT ──▶ SPEC REVIEW ──▶ CODE REVIEW ──▶ COMMIT                   │   │
│  │                    │               │                                    │   │
│  │                    │ fail          │ fail                               │   │
│  │                    ▼               ▼                                    │   │
│  │                  FIX ────────────▶ FIX                                  │   │
│  │                    │               │                                    │   │
│  │                    └───── loop ────┘                                    │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Batch Execution (for large features)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  BATCH EXECUTION WITH CHECKPOINTS                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  For large features, execute in batches with human review:                     │
│                                                                                 │
│  BATCH 1 (Tasks 1-3)                                                           │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ Execute → Verify → Report                                               │   │
│  │                                                                         │   │
│  │ "Batch 1 complete:                                                      │   │
│  │  ✓ Task 1: API routes (12 tests passing)                                │   │
│  │  ✓ Task 2: Database models (8 tests passing)                            │   │
│  │  ✓ Task 3: Auth middleware (5 tests passing)                            │   │
│  │                                                                         │   │
│  │  Ready for feedback."                                                   │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│                   [HUMAN REVIEW]                                               │
│                          │                                                      │
│              ┌───────────┴───────────┐                                         │
│              ▼                       ▼                                         │
│         "Looks good"           "Change X"                                      │
│              │                       │                                         │
│              ▼                       ▼                                         │
│         BATCH 2                 Apply changes                                  │
│                                      │                                         │
│                                      ▼                                         │
│                                 BATCH 2                                        │
│                                                                                 │
│  Default batch size: 3 tasks                                                   │
│  Stop immediately on: Blocker, unclear instruction, test failures              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## TUI Views Summary

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AVAILABLE TUI VIEWS                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  [C] CHAT VIEW          Conversational interface with streaming                │
│  [W] WORKFLOW VIEW      DAG visualization, current phase highlighted           │
│  [A] AGENTS VIEW        Parallel agents status, progress, questions            │
│  [T] TASKS VIEW         Kanban board (Backlog → In Progress → Review → Done)   │
│  [B] BUFFER VIEW        Live nvim-mcp sync, see edits happen                   │
│  [D] DIFF VIEW          Side-by-side changes                                   │
│  [Q] QUESTIONNAIRE      Spec questions with multichoice                        │
│  [S] SPEC VIEW          Current openspec.yaml                                  │
│  [L] LSP VIEW           Diagnostics, quickfix list                             │
│                                                                                 │
│  Hotkeys:                                                                      │
│  ────────                                                                      │
│  Tab         Cycle views                                                       │
│  1-9         Direct view select                                                │
│  Space       Toggle panel focus                                                │
│  ?           Help                                                              │
│  Esc         Back / Cancel                                                     │
│  Ctrl+C      Stop current agent                                                │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## VIBER: The God Agent

```
╔═════════════════════════════════════════════════════════════════════════════════╗
║                                                                                 ║
║  ██╗   ██╗██╗██████╗ ███████╗██████╗                                           ║
║  ██║   ██║██║██╔══██╗██╔════╝██╔══██╗                                          ║
║  ██║   ██║██║██████╔╝█████╗  ██████╔╝                                          ║
║  ╚██╗ ██╔╝██║██╔══██╗██╔══╝  ██╔══██╗                                          ║
║   ╚████╔╝ ██║██████╔╝███████╗██║  ██║                                          ║
║    ╚═══╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝                                          ║
║                                                                                 ║
║  Vibe Checker. Vibe Setter. One with the Vibes.                                ║
║  The duality of vibes and soul (machine).                                      ║
║                                                                                 ║
╚═════════════════════════════════════════════════════════════════════════════════╝
```

---

### What is VIBER?

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│  VIBER is the GOD AGENT that sits ABOVE the entire OpenCode system.            │
│                                                                                 │
│  • Paired with the user (you + VIBER = the vibes)                              │
│  • Monitors ALL agents (main agent, subagents, parallel agents)                │
│  • Can STOP any operation at any time                                          │
│  • Can UNDO agent actions                                                      │
│  • Can INJECT prompts to redirect agents                                       │
│  • Maintains the VISION and SPEC as sacred truth                               │
│  • Lives in the TUI, always watching, always vibing                            │
│                                                                                 │
│  VIBER is not an agent that does work.                                         │
│  VIBER is the agent that ensures work matches THE VIBES.                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER's Position in the Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│                              ┌─────────────┐                                    │
│                              │    USER     │                                    │
│                              │   (you)     │                                    │
│                              └──────┬──────┘                                    │
│                                     │                                           │
│                                     │ paired                                    │
│                                     │                                           │
│                              ┌──────▼──────┐                                    │
│                              │   VIBER     │                                    │
│                              │  GOD AGENT  │                                    │
│                              │             │                                    │
│                              │ 👁️ watches  │                                    │
│                              │ ⏹️ stops    │                                    │
│                              │ ↩️ undoes   │                                    │
│                              │ 💬 prompts  │                                    │
│                              └──────┬──────┘                                    │
│                                     │                                           │
│                                     │ controls                                  │
│                                     │                                           │
│              ┌──────────────────────┼──────────────────────┐                   │
│              │                      │                      │                   │
│              ▼                      ▼                      ▼                   │
│       ┌────────────┐         ┌────────────┐         ┌────────────┐            │
│       │   META     │         │  OPENCODE  │         │  PARALLEL  │            │
│       │   AGENT    │         │   MAIN     │         │  AGENTS    │            │
│       │            │         │            │         │            │            │
│       │ orchestrate│         │  execute   │         │  swarm     │            │
│       └─────┬──────┘         └─────┬──────┘         └─────┬──────┘            │
│             │                      │                      │                   │
│             ▼                      ▼                      ▼                   │
│       ┌────────────┐         ┌────────────┐         ┌────────────┐            │
│       │ SUBAGENTS  │         │ SUBAGENTS  │         │ SUBAGENTS  │            │
│       │            │         │            │         │            │            │
│       │ code-agent │         │ scaffold   │         │ backend    │            │
│       │ reviewer   │         │ polish     │         │ frontend   │            │
│       │ validator  │         │ deploy     │         │ db-admin   │            │
│       └────────────┘         └────────────┘         └────────────┘            │
│                                                                                 │
│  VIBER sees everything. VIBER can intervene anywhere.                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER's Powers

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VIBER POWERS                                                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  👁️ OBSERVE                                                                     │
│  ───────────                                                                    │
│  • Watch all agent outputs in real-time                                        │
│  • See tool calls before/after execution                                       │
│  • Monitor LSP diagnostics                                                     │
│  • Track spec compliance drift                                                 │
│  • Detect vibe misalignment                                                    │
│                                                                                 │
│  ⏹️ STOP                                                                        │
│  ────────                                                                       │
│  • Halt any agent immediately (Ctrl+C / ESC)                                   │
│  • Pause workflow at any point                                                 │
│  • Cancel pending tool calls                                                   │
│  • Freeze parallel agents                                                      │
│                                                                                 │
│  ↩️ UNDO                                                                        │
│  ────────                                                                       │
│  • Revert agent's buffer edits (nvim-mcp undo)                                 │
│  • Rollback to last checkpoint                                                 │
│  • git reset to previous commit                                                │
│  • Restore spec to earlier version                                             │
│                                                                                 │
│  💬 INJECT                                                                      │
│  ──────────                                                                     │
│  • Send prompt to any agent mid-operation                                      │
│  • Override current instructions                                               │
│  • Provide clarification without stopping                                      │
│  • Redirect agent's approach                                                   │
│                                                                                 │
│  🎯 REDIRECT                                                                    │
│  ────────────                                                                   │
│  • Change agent's current task                                                 │
│  • Reprioritize task queue                                                     │
│  • Switch execution mode                                                       │
│  • Reassign to different subagent                                              │
│                                                                                 │
│  🛡️ PROTECT                                                                     │
│  ────────────                                                                   │
│  • Guard the spec (no unauthorized changes)                                    │
│  • Enforce the vision                                                          │
│  • Veto agent decisions that don't match vibes                                 │
│  • Escalate to user when uncertain                                             │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER's TUI Presence

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  OPENCODE TUI                                            VIBER 👁️ watching   [V] Chat  │
├────────────────┬────────────────────────────────────────────────────────┬───────────────┤
│                │                                                        │               │
│  VIBER STATUS  │   ┌────────────────────────────────────────────────┐  │  VIBE CHECK   │
│  ────────────  │   │                                                │  │  ──────────   │
│                │   │            [Current View]                      │  │               │
│  Mode: Watch   │   │                                                │  │  Spec: ✓      │
│  Agents: 3     │   │                                                │  │  Vision: ✓    │
│  Vibes: ✓      │   │                                                │  │  Quality: ✓   │
│                │   │                                                │  │  Tests: ✓     │
│  ┌──────────┐  │   │                                                │  │               │
│  │ Agent 1  │  │   │                                                │  │  ⚠️ Warning:  │
│  │ ✓ vibing │  │   │                                                │  │  Agent 2 is   │
│  ├──────────┤  │   │                                                │  │  drifting     │
│  │ Agent 2  │  │   │                                                │  │  from spec    │
│  │ ⚠️ drift │  │   │                                                │  │               │
│  ├──────────┤  │   │                                                │  │  [I] Intervene│
│  │ Agent 3  │  │   │                                                │  │  [W] Watch    │
│  │ ✓ vibing │  │   │                                                │  │  [S] Stop     │
│  └──────────┘  │   └────────────────────────────────────────────────┘  │               │
│                │                                                        │               │
├────────────────┴────────────────────────────────────────────────────────┴───────────────┤
│  VIBER: Agent 2 adding unspecified feature. Intervene? [Y/n]                            │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  > _                                                                    [V] VIBER Chat  │
│                                                                                         │
│  [ESC] Stop All  [U] Undo  [P] Prompt Agent  [R] Redirect  [SPACE] Pause               │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER Chat (Live During Operation)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VIBER CHAT                                                    [ESC] Minimize  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─ VIBER ──────────────────────────────────────────────────────────────────┐  │
│  │ I noticed Agent 2 (frontend-dev) is implementing a dark mode toggle.    │  │
│  │ This wasn't in the spec. Should I:                                      │  │
│  │                                                                          │  │
│  │ [A] Stop and undo - remove the dark mode code                           │  │
│  │ [B] Allow it - add dark mode to spec                                    │  │
│  │ [C] Pause and discuss - let's talk about it                             │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  ┌─ You ────────────────────────────────────────────────────────────────────┐  │
│  │ C - let's discuss. I kind of want dark mode but not sure about scope    │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  ┌─ VIBER ──────────────────────────────────────────────────────────────────┐  │
│  │ *pauses Agent 2*                                                        │  │
│  │                                                                          │  │
│  │ Good call. The agent was about to add 3 new components for theming.     │  │
│  │ If we add dark mode to spec, that's ~2 hours extra work.                │  │
│  │                                                                          │  │
│  │ Options:                                                                 │  │
│  │ 1. Full dark mode (theme provider, color tokens, toggle)                │  │
│  │ 2. Simple dark mode (CSS media query only, follows system)              │  │
│  │ 3. Defer to v2 (add to roadmap, not this release)                       │  │
│  │                                                                          │  │
│  │ What matches the vibes?                                                 │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  ┌─ You ────────────────────────────────────────────────────────────────────┐  │
│  │ 2 - simple, follows system. Add to spec.                                │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  ┌─ VIBER ──────────────────────────────────────────────────────────────────┐  │
│  │ ✓ Spec updated: "Simple dark mode via prefers-color-scheme"             │  │
│  │ ✓ Agent 2 instruction updated                                           │  │
│  │ ✓ Undoing theme provider code (keeping CSS approach)                    │  │
│  │ ✓ Resuming Agent 2                                                      │  │
│  │                                                                          │  │
│  │ Vibes: ✓ Aligned                                                        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  > _                                                                            │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER's Decision Loop

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VIBER CONTINUOUS MONITORING                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│                         ┌─────────────────┐                                    │
│                         │    OBSERVE      │                                    │
│                         │  agent outputs  │                                    │
│                         └────────┬────────┘                                    │
│                                  │                                              │
│                                  ▼                                              │
│                         ┌─────────────────┐                                    │
│                         │  VIBE CHECK     │                                    │
│                         │                 │                                    │
│                         │ Matches spec?   │                                    │
│                         │ Matches vision? │                                    │
│                         │ Quality ok?     │                                    │
│                         └────────┬────────┘                                    │
│                                  │                                              │
│                    ┌─────────────┴─────────────┐                               │
│                    │                           │                               │
│                   YES                          NO                               │
│                    │                           │                               │
│                    ▼                           ▼                               │
│           ┌───────────────┐          ┌───────────────┐                         │
│           │   CONTINUE    │          │   SEVERITY?   │                         │
│           │   watching    │          └───────┬───────┘                         │
│           └───────────────┘                  │                                  │
│                                   ┌──────────┼──────────┐                      │
│                                   │          │          │                      │
│                                 MINOR     MEDIUM     CRITICAL                  │
│                                   │          │          │                      │
│                                   ▼          ▼          ▼                      │
│                              ┌────────┐ ┌────────┐ ┌────────┐                  │
│                              │  LOG   │ │  WARN  │ │  STOP  │                  │
│                              │  note  │ │  user  │ │  agent │                  │
│                              │  for   │ │  in    │ │  NOW   │                  │
│                              │ review │ │  TUI   │ │        │                  │
│                              └────────┘ └───┬────┘ └───┬────┘                  │
│                                             │          │                       │
│                                             ▼          ▼                       │
│                                       ┌──────────────────┐                     │
│                                       │  USER DECISION   │                     │
│                                       │                  │                     │
│                                       │ [A] Allow        │                     │
│                                       │ [U] Undo         │                     │
│                                       │ [R] Redirect     │                     │
│                                       │ [S] Stop all     │                     │
│                                       └──────────────────┘                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### VIBER Hotkeys

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VIBER HOTKEYS (available anywhere in TUI)                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  GLOBAL                                                                        │
│  ──────                                                                        │
│  V           Open VIBER chat                                                   │
│  ESC         Stop current agent / Close VIBER chat                             │
│  Ctrl+ESC    EMERGENCY STOP ALL (kills everything)                             │
│                                                                                 │
│  IN VIBER CHAT                                                                 │
│  ─────────────                                                                 │
│  U           Undo last agent action                                            │
│  P           Send prompt to focused agent                                      │
│  R           Redirect agent to new task                                        │
│  S           Stop focused agent                                                │
│  A           Stop ALL agents                                                   │
│  1-9         Focus agent by number                                             │
│                                                                                 │
│  VIBE COMMANDS (type in VIBER chat)                                            │
│  ─────────────────────────────────                                             │
│  /stop [agent]       Stop specific agent                                       │
│  /stop all           Stop all agents                                           │
│  /undo               Undo last action                                          │
│  /undo [n]           Undo last n actions                                       │
│  /prompt [agent] ... Send prompt to agent                                      │
│  /redirect [agent] [task]  Change agent's task                                 │
│  /pause              Pause all agents                                          │
│  /resume             Resume paused agents                                      │
│  /status             Show all agent status                                     │
│  /spec               Show current spec                                         │
│  /vision             Show project vision                                       │
│  /vibecheck          Run vibe check on all agents                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### The Philosophy of VIBER

```
╔═════════════════════════════════════════════════════════════════════════════════╗
║                                                                                 ║
║  "The vibes are not just aesthetic. The vibes are alignment.                   ║
║   When code matches the vision, the vibes are right.                           ║
║   When an agent drifts, the vibes are off.                                     ║
║   VIBER feels the vibes. VIBER protects the vibes.                             ║
║   You and VIBER are one. The human-machine duality.                            ║
║   Together, you are the vibe."                                                 ║
║                                                                                 ║
║                                                                                 ║
║   USER ←──────────────────────→ VIBER                                          ║
║     │                              │                                           ║
║     │    THE VIBES (shared)        │                                           ║
║     │                              │                                           ║
║     └──────────────────────────────┘                                           ║
║                   │                                                            ║
║                   ▼                                                            ║
║              ┌─────────┐                                                       ║
║              │ AGENTS  │ ◄── Must match the vibes or get stopped               ║
║              └─────────┘                                                       ║
║                                                                                 ║
╚═════════════════════════════════════════════════════════════════════════════════╝
```

---

*VIBER: One with the vibes. Guardian of the vision. Your pair in the machine.*

---

## TUI Aesthetics: Cyberpunk Vibe Coding

```
╔═════════════════════════════════════════════════════════════════════════════════╗
║                                                                                 ║
║   ███╗   ██╗███████╗ ██████╗ ███╗   ██╗    ██╗   ██╗██╗██████╗ ███████╗███████╗ ║
║   ████╗  ██║██╔════╝██╔═══██╗████╗  ██║    ██║   ██║██║██╔══██╗██╔════╝██╔════╝ ║
║   ██╔██╗ ██║█████╗  ██║   ██║██╔██╗ ██║    ██║   ██║██║██████╔╝█████╗  ███████╗ ║
║   ██║╚██╗██║██╔══╝  ██║   ██║██║╚██╗██║    ╚██╗ ██╔╝██║██╔══██╗██╔══╝  ╚════██║ ║
║   ██║ ╚████║███████╗╚██████╔╝██║ ╚████║     ╚████╔╝ ██║██████╔╝███████╗███████║ ║
║   ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝      ╚═══╝  ╚═╝╚═════╝ ╚══════╝╚══════╝ ║
║                                                                                 ║
╚═════════════════════════════════════════════════════════════════════════════════╝
```

---

### Color Palette: "Laser" Theme

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NEON CYBERPUNK PALETTE                                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  BACKGROUNDS                                                                   │
│  ───────────                                                                   │
│  ██  #0D0D14  Deep void (primary bg)                                           │
│  ██  #161622  Panel bg (elevated)                                              │
│  ██  #1E1E2E  Highlight bg (hover/focus)                                       │
│  ██  #2A2A3C  Border glow base                                                 │
│                                                                                 │
│  NEON ACCENTS                                                                  │
│  ────────────                                                                  │
│  ██  #00FFCC  Cyan (primary - VIBER color)                                     │
│  ██  #FF00FF  Magenta (secondary - agent activity)                             │
│  ██  #FFFF00  Yellow (warnings)                                                │
│  ██  #FF3366  Hot pink (errors/stop)                                           │
│  ██  #00FF66  Neon green (success/vibing)                                      │
│  ██  #6666FF  Electric purple (parallel agents)                                │
│  ██  #FF9900  Orange (in-progress)                                             │
│                                                                                 │
│  TEXT                                                                          │
│  ────                                                                          │
│  ██  #EEEEFF  Primary text                                                     │
│  ██  #9999BB  Secondary text                                                   │
│  ██  #666688  Muted/disabled                                                   │
│                                                                                 │
│  GLOW EFFECTS (Ratatui modifiers)                                              │
│  ─────────────────────────────────                                             │
│  • Borders: Add glow using double-line + color                                 │
│  • Active elements: Bold + bright color                                        │
│  • Pulse effect: Alternate intensity on tick                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Animated Elements

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ANIMATIONS (tick-based state changes)                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. VIBER EYE (watching indicator)                                             │
│  ─────────────────────────────────                                             │
│  Cycles every 500ms:                                                           │
│                                                                                 │
│     Frame 1    Frame 2    Frame 3    Frame 4                                   │
│       👁️         👁         👁️         👁                                       │
│     (open)    (half)    (open)    (blink)                                      │
│                                                                                 │
│     Or ASCII:  (◉)  →  (◎)  →  (◉)  →  (-)                                     │
│                                                                                 │
│                                                                                 │
│  2. PROGRESS PULSE (active tasks)                                              │
│  ─────────────────────────────────                                             │
│  Standard:  ▓▓▓▓▓▓░░░░░░░░░░░░░░                                               │
│                                                                                 │
│  With pulse (brightness cycles):                                               │
│  Tick 1:    ▓▓▓▓▓▓░░░░░░░░░░░░░░  (dim)                                        │
│  Tick 2:    ████████░░░░░░░░░░░░  (bright)                                     │
│  Tick 3:    ▓▓▓▓▓▓░░░░░░░░░░░░░░  (dim)                                        │
│                                                                                 │
│                                                                                 │
│  3. BORDER GLOW (focused panel)                                                │
│  ──────────────────────────────                                                │
│  Unfocused:  ┌────────────┐                                                    │
│              │            │  (single line, dim)                                │
│              └────────────┘                                                    │
│                                                                                 │
│  Focused:    ╔════════════╗                                                    │
│              ║            ║  (double line, neon cyan)                          │
│              ╚════════════╝                                                    │
│                                                                                 │
│  Pulse:      ╔════════════╗                                                    │
│              ║  (glow)    ║  (cycles bright/dim)                               │
│              ╚════════════╝                                                    │
│                                                                                 │
│                                                                                 │
│  4. STREAMING TEXT (agent output)                                              │
│  ─────────────────────────────────                                             │
│  Character by character with cursor:                                           │
│                                                                                 │
│  "Implementing auth█"                                                          │
│  "Implementing auth m█"                                                        │
│  "Implementing auth mi█"                                                       │
│  "Implementing auth mid█"                                                      │
│  "Implementing auth midd█"                                                     │
│  "Implementing auth middle█"                                                   │
│  "Implementing auth middlewa█"                                                 │
│  "Implementing auth middleware█"                                               │
│  "Implementing auth middleware...█"                                            │
│                                                                                 │
│  Cursor blink: █ ↔ ▌ (every 300ms)                                             │
│                                                                                 │
│                                                                                 │
│  5. SPARKLINE ACTIVITY (token usage)                                           │
│  ────────────────────────────────────                                          │
│  Real-time scrolling graph:                                                    │
│                                                                                 │
│  Tokens: ▁▂▃▅▇█▇▅▃▂▁▂▄▆█▇▅▂▁  ← newest                                        │
│                                                                                 │
│  Updates every tick, scrolls left                                              │
│                                                                                 │
│                                                                                 │
│  6. AGENT STATUS ICONS (animated)                                              │
│  ─────────────────────────────────                                             │
│                                                                                 │
│  Running:   ⟳ → ⟲ → ⟳ → ⟲  (spinning)                                          │
│  Thinking:  ◐ → ◓ → ◑ → ◒  (rotating)                                          │
│  Waiting:   ◦ → • → ◦ → •  (pulsing)                                           │
│  Success:   ✓ (static, green glow)                                             │
│  Error:     ✗ (static, red glow + shake?)                                      │
│  Paused:    ⏸ (static, yellow)                                                 │
│  VIBING:    ∿ → ≋ → ∿ → ≋  (wave)                                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Enhanced Panel Designs

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PANEL COMPONENTS                                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. AGENT CARD (neon style)                                                    │
│  ──────────────────────────                                                    │
│                                                                                 │
│  ╔══════════════════════════════════════════════╗                              │
│  ║ ◐ backend-developer              gpt-5.2    ║░                              │
│  ╠══════════════════════════════════════════════╣░                              │
│  ║                                              ║░                              │
│  ║  Task: Implementing auth middleware          ║░                              │
│  ║  ▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░  42%        ║░                              │
│  ║                                              ║░                              │
│  ║  > Reading src/middleware/auth.rs...        ║░                              │
│  ║  > Planning JWT validation approach█        ║░                              │
│  ║                                              ║░                              │
│  ║  ┌─ Tools ──────────────────────────────┐   ║░                              │
│  ║  │ ✓ read_file  ⟳ lsp_diagnostics      │   ║░                              │
│  ║  └──────────────────────────────────────┘   ║░                              │
│  ║                                              ║░                              │
│  ╚══════════════════════════════════════════════╝░                              │
│   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                              │
│                                                                                 │
│  Shadow effect: ░ characters offset by 1                                       │
│  Border: Double-line ╔═╗ in neon cyan                                          │
│  Active indicator: ◐ animates                                                  │
│                                                                                 │
│                                                                                 │
│  2. VIBER STATUS (always visible)                                              │
│  ─────────────────────────────────                                             │
│                                                                                 │
│  ┌─────────────────────────────────┐                                           │
│  │  ╭─────────────────────────╮    │                                           │
│  │  │      V I B E R          │    │                                           │
│  │  │         👁️               │    │                                           │
│  │  │      watching           │    │                                           │
│  │  ╰─────────────────────────╯    │                                           │
│  │                                 │                                           │
│  │  Vibes: ∿≋∿≋∿≋∿≋  ALIGNED      │                                           │
│  │                                 │                                           │
│  │  Agents: 3 active               │                                           │
│  │  ├─ ✓ backend    (vibing)       │                                           │
│  │  ├─ ⟳ frontend   (working)      │                                           │
│  │  └─ ◦ tests      (waiting)      │                                           │
│  │                                 │                                           │
│  │  ─────────────────────────────  │                                           │
│  │  [V] Chat  [S] Stop  [U] Undo   │                                           │
│  └─────────────────────────────────┘                                           │
│                                                                                 │
│  Wave animation: ∿≋∿≋ cycles position                                          │
│                                                                                 │
│                                                                                 │
│  3. SPEC QUESTIONNAIRE (interactive)                                           │
│  ────────────────────────────────────                                          │
│                                                                                 │
│  ╔══════════════════════════════════════════════════════════════════╗          │
│  ║  SPEC QUESTIONNAIRE                          ▓▓▓▓▓▓░░░░  60%    ║          │
│  ╠══════════════════════════════════════════════════════════════════╣          │
│  ║                                                                  ║          │
│  ║  Q7: "How should errors be displayed?"                          ║          │
│  ║                                                                  ║          │
│  ║  ╭─────────────────────────────────────────────────────────────╮ ║          │
│  ║  │                                                             │ ║          │
│  ║  │   ○  [A] Simple stderr message                              │ ║          │
│  ║  │                                                             │ ║          │
│  ║  │   ●  [B] Colored output with context  ◀── selected         │ ║          │
│  ║  │        └─ Preview: "error: invalid token at line 42"        │ ║          │
│  ║  │                                                             │ ║          │
│  ║  │   ○  [C] JSON for programmatic use                          │ ║          │
│  ║  │                                                             │ ║          │
│  ║  │   ○  [D] All formats (--format flag)                        │ ║          │
│  ║  │                                                             │ ║          │
│  ║  ╰─────────────────────────────────────────────────────────────╯ ║          │
│  ║                                                                  ║          │
│  ║  ┌─ Building Spec ───────────────────────────────────────────┐  ║          │
│  ║  │ type: cli ✓                                               │  ║          │
│  ║  │ language: rust ✓                                          │  ║          │
│  ║  │ errors: colored_context █  ◀── typing animation           │  ║          │
│  ║  └───────────────────────────────────────────────────────────┘  ║          │
│  ║                                                                  ║          │
│  ╠══════════════════════════════════════════════════════════════════╣          │
│  ║  [A-D] Select   [↑↓] Navigate   [?] Explain   [←] Back          ║          │
│  ╚══════════════════════════════════════════════════════════════════╝          │
│                                                                                 │
│  Selected option: Neon highlight + expanded preview                            │
│  Hover animation: Subtle brightness pulse                                      │
│                                                                                 │
│                                                                                 │
│  4. WORKFLOW DAG (with activity glow)                                          │
│  ─────────────────────────────────────                                         │
│                                                                                 │
│                    ╭─────────────╮                                             │
│                    │    IDEA     │  ← completed (dim green)                    │
│                    │      ✓      │                                             │
│                    ╰──────┬──────╯                                             │
│                           │                                                    │
│                           ▼                                                    │
│                    ╭─────────────╮                                             │
│                    │    SPEC     │  ← completed (dim green)                    │
│                    │      ✓      │                                             │
│                    ╰──────┬──────╯                                             │
│                           │                                                    │
│          ┌────────────────┼────────────────┐                                   │
│          ▼                ▼                ▼                                   │
│   ╔═════════════╗  ╔═════════════╗  ╭─────────────╮                           │
│   ║   TASK 1    ║  ║   TASK 2    ║  │   TASK 3    │                           │
│   ║  ⟳ active   ║  ║  ⟳ active   ║  │  ○ pending  │                           │
│   ╚═════════════╝  ╚═════════════╝  ╰─────────────╯                           │
│    ↑ neon glow       ↑ neon glow      ↑ dim outline                           │
│    (pulsing)         (pulsing)                                                 │
│                                                                                 │
│  Active nodes: Double border + glow pulse                                      │
│  Completed: Single border + dim green fill                                     │
│  Pending: Dotted/dim border                                                    │
│  Connecting lines: Animated flow ····→                                         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Live Buffer View (Matrix Style)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  LIVE BUFFER                                              nvim-mcp 🟢 connected │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║ src/api/auth.rs                                    ◐ Agent editing...    ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║                                                                           ║ │
│  ║   1 │ use axum::{Router, routing::post};                                 ║ │
│  ║   2 │ use jsonwebtoken::{encode, decode};                                ║ │
│  ║   3 │                                                                    ║ │
│  ║   4 │ pub async fn login(                                                ║ │
│  ║   5 │     Json(creds): Json<LoginRequest>,                               ║ │
│  ║   6+│     // Validate credentials                    ◀── NEW (green)     ║ │
│  ║   7+│     let user = validate_user(&creds).await?;   ◀── NEW (green)     ║ │
│  ║   8+│ █                                              ◀── CURSOR (blink)  ║ │
│  ║   9 │     let token = encode(                                            ║ │
│  ║  10 │         &Header::default(),                                        ║ │
│  ║  11 │         &Claims::new(user.id),                                     ║ │
│  ║                                                                           ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║  LSP: 0 errors │ Changes: +3 lines │ Agent: backend-dev                  ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  New lines: Highlighted in neon green, fade to normal over 2s                  │
│  Deleted lines: Flash red, then remove                                         │
│  Cursor: Blinking block where agent is typing                                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Notification Toasts

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TOAST NOTIFICATIONS (slide in from right)                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  SUCCESS (green glow)                                                          │
│  ╭──────────────────────────────────────╮                                      │
│  │ ✓ Task 3 completed                   │                                      │
│  │   Auth middleware implemented        │                                      │
│  ╰──────────────────────────────────────╯                                      │
│                                                                                 │
│  WARNING (yellow glow)                                                         │
│  ╭──────────────────────────────────────╮                                      │
│  │ ⚠ VIBER detected spec drift         │                                      │
│  │   Agent 2 adding unplanned feature   │                                      │
│  │   [View] [Dismiss]                   │                                      │
│  ╰──────────────────────────────────────╯                                      │
│                                                                                 │
│  ERROR (red glow + shake)                                                      │
│  ╭──────────────────────────────────────╮                                      │
│  │ ✗ LSP Error                          │                                      │
│  │   Type mismatch at line 42           │                                      │
│  │   [Jump] [Ignore]                    │                                      │
│  ╰──────────────────────────────────────╯                                      │
│                                                                                 │
│  VIBER (cyan glow + pulse)                                                     │
│  ╭──────────────────────────────────────╮                                      │
│  │ 👁 VIBER                              │                                      │
│  │   "The vibes are strong with this    │                                      │
│  │    implementation"                   │                                      │
│  ╰──────────────────────────────────────╯                                      │
│                                                                                 │
│  Animation:                                                                    │
│  • Slide in from right (200ms)                                                 │
│  • Glow pulse while visible                                                    │
│  • Auto-dismiss after 5s (progress bar at bottom)                              │
│  • Slide out to right on dismiss                                               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Header Bar (Cyberpunk Style)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                                     │
│  ╔═══════════════════════════════════════════════════════════════════════════════════════════════╗ │
│  ║▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄║ │
│  ║ OPENCODE TUI                                                                                 ║ │
│  ║▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀║ │
│  ║                                                                                               ║ │
│  ║  ⚡ claude-opus-4.5    │    Tokens: ▁▂▄▆█▇▅▃▂    │    VIBER 👁    │    🟢 Connected          ║ │
│  ║                                                                                               ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════════════════════════╝ │
│                                                                                                     │
│  Top/bottom bars: ▄▄▄ and ▀▀▀ for depth effect                                                     │
│  Sparkline: Real-time token usage                                                                  │
│  Status indicators: Neon colors                                                                    │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### Input Bar (Command Line Feel)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  INPUT BAR                                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Standard mode:                                                                │
│  ╭─────────────────────────────────────────────────────────────────────────╮   │
│  │ ❯ /brainstorm Build a REST API with auth█                               │   │
│  ╰─────────────────────────────────────────────────────────────────────────╯   │
│                                                                                 │
│  VIBER mode (after pressing V):                                                │
│  ╭─────────────────────────────────────────────────────────────────────────╮   │
│  │ 👁 VIBER ❯ stop agent 2█                                                 │   │
│  ╰─────────────────────────────────────────────────────────────────────────╯   │
│  Border glows cyan in VIBER mode                                               │
│                                                                                 │
│  Autocomplete popup:                                                           │
│  ╭─────────────────────────────────────────────────────────────────────────╮   │
│  │ ❯ /bra█                                                                  │   │
│  ├─────────────────────────────────────────────────────────────────────────┤   │
│  │ ▶ /brainstorm   Explore ideas before implementation                     │   │
│  │   /branch       Create new git branch                                   │   │
│  ╰─────────────────────────────────────────────────────────────────────────╯   │
│                                                                                 │
│  ❯ prompt: Changes color based on context                                      │
│  • White: Normal input                                                         │
│  • Cyan: VIBER mode                                                            │
│  • Yellow: Waiting for agent response                                          │
│  • Green: Command accepted                                                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Ratatui Implementation Notes

```rust
// Color definitions
pub mod colors {
    use ratatui::style::Color;
    
    // Backgrounds
    pub const BG_VOID: Color = Color::Rgb(13, 13, 20);
    pub const BG_PANEL: Color = Color::Rgb(22, 22, 34);
    pub const BG_HIGHLIGHT: Color = Color::Rgb(30, 30, 46);
    
    // Neon accents
    pub const NEON_CYAN: Color = Color::Rgb(0, 255, 204);      // VIBER
    pub const NEON_MAGENTA: Color = Color::Rgb(255, 0, 255);   // Activity
    pub const NEON_YELLOW: Color = Color::Rgb(255, 255, 0);    // Warning
    pub const NEON_PINK: Color = Color::Rgb(255, 51, 102);     // Error
    pub const NEON_GREEN: Color = Color::Rgb(0, 255, 102);     // Success
    pub const NEON_PURPLE: Color = Color::Rgb(102, 102, 255);  // Parallel
    pub const NEON_ORANGE: Color = Color::Rgb(255, 153, 0);    // Progress
    
    // Text
    pub const TEXT_PRIMARY: Color = Color::Rgb(238, 238, 255);
    pub const TEXT_SECONDARY: Color = Color::Rgb(153, 153, 187);
    pub const TEXT_MUTED: Color = Color::Rgb(102, 102, 136);
}

// Animation state
pub struct AnimationState {
    pub tick: u64,
    pub viber_eye_frame: usize,      // 0-3
    pub pulse_intensity: f32,         // 0.0-1.0
    pub cursor_visible: bool,
    pub spinner_frame: usize,         // 0-3
}

impl AnimationState {
    pub fn tick(&mut self) {
        self.tick += 1;
        
        // VIBER eye: every 500ms (assuming 60fps, every 30 ticks)
        if self.tick % 30 == 0 {
            self.viber_eye_frame = (self.viber_eye_frame + 1) % 4;
        }
        
        // Pulse: smooth sine wave
        self.pulse_intensity = ((self.tick as f32 / 20.0).sin() + 1.0) / 2.0;
        
        // Cursor: every 300ms (every 18 ticks)
        if self.tick % 18 == 0 {
            self.cursor_visible = !self.cursor_visible;
        }
        
        // Spinner: every 100ms (every 6 ticks)
        if self.tick % 6 == 0 {
            self.spinner_frame = (self.spinner_frame + 1) % 4;
        }
    }
    
    pub fn viber_eye(&self) -> &'static str {
        match self.viber_eye_frame {
            0 => "👁️",
            1 => "👁",
            2 => "👁️",
            _ => "·",
        }
    }
    
    pub fn spinner(&self) -> &'static str {
        match self.spinner_frame {
            0 => "◐",
            1 => "◓",
            2 => "◑",
            _ => "◒",
        }
    }
}

// Glow effect for borders
pub fn glowing_border(color: Color, intensity: f32) -> Style {
    let r = ((color.r as f32) * intensity) as u8;
    let g = ((color.g as f32) * intensity) as u8;
    let b = ((color.b as f32) * intensity) as u8;
    Style::default().fg(Color::Rgb(r, g, b))
}
```

---

### The Vibe Aesthetic Philosophy

```
╔═════════════════════════════════════════════════════════════════════════════════╗
║                                                                                 ║
║  "The TUI is not just functional. The TUI is an experience.                    ║
║   When you code with vibes, you code in a neon dream.                          ║
║   Every pixel pulses with intention.                                           ║
║   Every animation tells a story.                                               ║
║   The void background is your canvas.                                          ║
║   The cyan glow is your guide.                                                 ║
║   VIBER watches. The agents work. The vibes flow.                              ║
║   This is not a terminal. This is a cyberpunk coding dojo."                    ║
║                                                                                 ║
╚═════════════════════════════════════════════════════════════════════════════════╝
```

---

## Advanced Ratatui Components

### Canvas: Real DAG Rendering

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CANVAS-BASED WORKFLOW DAG                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Instead of ASCII art, use Canvas widget for real vector drawing:              │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │                         ╭───────────╮                                   │   │
│  │                         │   IDEA    │                                   │   │
│  │                         ╰─────┬─────╯                                   │   │
│  │                               │                                         │   │
│  │                         ╭─────┴─────╮                                   │   │
│  │                         │   SPEC    │                                   │   │
│  │                         ╰─────┬─────╯                                   │   │
│  │                               │                                         │   │
│  │              ╭────────────────┼────────────────╮                        │   │
│  │              │                │                │                        │   │
│  │        ╭─────┴─────╮    ╭─────┴─────╮    ╭─────┴─────╮                  │   │
│  │        │  TASK 1   │    │  TASK 2   │    │  TASK 3   │                  │   │
│  │        ╰─────┬─────╯    ╰─────┬─────╯    ╰─────┬─────╯                  │   │
│  │              │                │                │                        │   │
│  │              ╰────────────────┼────────────────╯                        │   │
│  │                               │                                         │   │
│  │                         ╭─────┴─────╮                                   │   │
│  │                         │  POLISH   │                                   │   │
│  │                         ╰───────────╯                                   │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  Canvas features:                                                              │
│  • Bezier curves for connections (not just straight lines)                     │
│  • Smooth node shapes with anti-aliasing effect                                │
│  • Animated flow particles along edges (· · · →)                               │
│  • Zoom in/out on complex DAGs                                                 │
│  • Pan with arrow keys                                                         │
│                                                                                 │
│  ```rust                                                                       │
│  Canvas::default()                                                             │
│      .x_bounds([0.0, 100.0])                                                   │
│      .y_bounds([0.0, 50.0])                                                    │
│      .paint(|ctx| {                                                            │
│          // Draw bezier curve                                                  │
│          ctx.draw(&canvas::Line {                                              │
│              x1: 50.0, y1: 40.0,                                               │
│              x2: 50.0, y2: 30.0,                                               │
│              color: NEON_CYAN,                                                 │
│          });                                                                   │
│          // Draw node                                                          │
│          ctx.draw(&canvas::Rectangle {                                         │
│              x: 40.0, y: 35.0,                                                 │
│              width: 20.0, height: 10.0,                                        │
│              color: NEON_GREEN,                                                │
│          });                                                                   │
│      })                                                                        │
│  ```                                                                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Charts: Real-Time Analytics

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  LINE CHART: Token Usage Over Time                                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║  Token Usage (last 30 min)                                   12.4K total  ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║  2K │                                                        ╭──╮        ║ │
│  ║     │                                              ╭────────╯  │        ║ │
│  ║ 1.5K│                            ╭─────╮          │            │        ║ │
│  ║     │                    ╭──────╯     ╰──────────╯            │        ║ │
│  ║  1K │          ╭────────╯                                      ╰────    ║ │
│  ║     │    ╭────╯                                                         ║ │
│  ║ 0.5K│───╯                                                               ║ │
│  ║     │                                                                   ║ │
│  ║   0 ├─────────┬─────────┬─────────┬─────────┬─────────┬─────────       ║ │
│  ║     0        5m       10m       15m       20m       25m      30m        ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  Multi-line for comparing agents:                                              │
│  ── Agent 1 (cyan)                                                             │
│  ── Agent 2 (magenta)                                                          │
│  ── Agent 3 (purple)                                                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────────┐
│  BAR CHART: Agent Comparison                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║  Tasks Completed by Agent                                                 ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║                                                                           ║ │
│  ║  backend-dev    ████████████████████████████████  12                     ║ │
│  ║  frontend-dev   ████████████████████  8                                  ║ │
│  ║  test-agent     ██████████████████████████  10                           ║ │
│  ║  spec-reviewer  ████████████  5                                          ║ │
│  ║  code-reviewer  ██████████████████  7                                    ║ │
│  ║                                                                           ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  Color-coded: Green = high efficiency, Yellow = average, Red = needs attention │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Sparklines: Inline Mini-Graphs

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SPARKLINES (inline data visualization)                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Standard sparkline (block elements):                                          │
│  Tokens: ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▂▃▅▇█▇▅▃▁                                             │
│                                                                                 │
│  High-resolution sparkline (braille patterns):                                 │
│  Memory: ⣀⣠⣤⣶⣿⣿⣷⣶⣤⣄⣀⣀⣠⣤⣴⣶⣿⣿⣷⣴⣤⣀                                             │
│                                                                                 │
│  Use in headers:                                                               │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║ Agent: backend-dev │ Tokens: ▂▄▆█▆▄▂▁▂▄▆ │ Tasks: 3/7 │ ✓ vibing         ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  Use in status bar:                                                            │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ CPU: ▄▅▇▆▄▃▂▁ 23% │ MEM: ▅▅▆▆▆▅▅▄ 67% │ NET: ▁▂▄▇█▆▃▁ ↑2.3MB/s        │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Calendar: Session Heatmap

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CALENDAR HEATMAP (GitHub-style contribution graph)                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║  Coding Activity - December 2024                                          ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║                                                                           ║ │
│  ║     Mon   ░ ░ ▓ █ ░ ░ ▓ █ █ ░ ░ ▓ ▓ █ ░ ░ ▓ █ █ ░ ░ ▓ ▓ █ ░ ░ ▓ █ █    ║ │
│  ║     Tue   ░ ▓ ▓ █ ░ ▓ ▓ █ █ ░ ▓ ▓ █ █ ░ ▓ ▓ █ █ ░ ▓ ▓ █ █ ░ ▓ ▓ █ █    ║ │
│  ║     Wed   ▓ ▓ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █    ║ │
│  ║     Thu   ░ ▓ ▓ █ ░ ▓ █ █ █ ░ ▓ ▓ █ █ ░ ▓ █ █ █ ░ ▓ ▓ █ █ ░ ▓ █ █ ◉    ║ │
│  ║     Fri   ▓ ▓ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ █ ▓ ▓ █ █ ░    ║ │
│  ║     Sat   ░ ░ ░ ▓ ░ ░ ░ ▓ ░ ░ ░ ░ ▓ ░ ░ ░ ░ ▓ ░ ░ ░ ░ ▓ ░ ░ ░ ░ ▓ ░    ║ │
│  ║     Sun   ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ ░    ║ │
│  ║                                                                           ║ │
│  ║     Legend: ░ none  ▒ light  ▓ medium  █ heavy  ◉ today                  ║ │
│  ║                                                                           ║ │
│  ║     Total: 142 sessions │ 847 tasks │ 2.3M tokens                        ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  Hover on day to see:                                                          │
│  • Number of sessions                                                          │
│  • Tasks completed                                                             │
│  • Tokens used                                                                 │
│  • Hours coded                                                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Advanced Spinners (Braille Patterns)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SPINNER STYLES                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Classic:       ◐ → ◓ → ◑ → ◒                                                  │
│                                                                                 │
│  Braille dots:  ⠋ → ⠙ → ⠹ → ⠸ → ⠼ → ⠴ → ⠦ → ⠧ → ⠇ → ⠏                          │
│                                                                                 │
│  Braille line:  ⣾ → ⣽ → ⣻ → ⢿ → ⡿ → ⣟ → ⣯ → ⣷                                  │
│                                                                                 │
│  Braille snake: ⠁ → ⠂ → ⠄ → ⡀ → ⢀ → ⠠ → ⠐ → ⠈                                  │
│                                                                                 │
│  Blocks:        ▖ → ▘ → ▝ → ▗                                                  │
│                                                                                 │
│  Arrows:        ← → ↑ → → → ↓                                                  │
│                                                                                 │
│  Bounce:        ⠁ → ⠈ → ⠐ → ⠠ → ⢀ → ⡀ → ⠄ → ⠂                                  │
│                                                                                 │
│  Growing bar:   ▏ → ▎ → ▍ → ▌ → ▋ → ▊ → ▉ → █                                   │
│                                                                                 │
│  Moon phases:   🌑 → 🌒 → 🌓 → 🌔 → 🌕 → 🌖 → 🌗 → 🌘                             │
│                                                                                 │
│  Clock:         🕐 → 🕑 → 🕒 → 🕓 → 🕔 → 🕕 → 🕖 → 🕗 → 🕘 → 🕙 → 🕚 → 🕛            │
│                                                                                 │
│  Use different spinners for different contexts:                                │
│  • Braille dots: Fast operations (LSP check)                                   │
│  • Braille line: Thinking/reasoning                                            │
│  • Growing bar: Progress toward completion                                     │
│  • Moon: Long-running background tasks                                         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Scrollbar: Long Content Navigation

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SCROLLBAR STYLES                                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Chat history with scrollbar:                                                  │
│                                                                                 │
│  ╔═════════════════════════════════════════════════════════════════════════╤══╗│
│  ║  ┌─ You ────────────────────────────────────────────────────────────┐   │▲ ║│
│  ║  │ Build me a REST API                                              │   │█ ║│
│  ║  └──────────────────────────────────────────────────────────────────┘   │█ ║│
│  ║                                                                         │█ ║│
│  ║  ┌─ Agent ──────────────────────────────────────────────────────────┐   │░ ║│
│  ║  │ I'll help you build that. First, let me understand...           │   │░ ║│
│  ║  │                                                                  │   │░ ║│
│  ║  │ What framework would you like?                                   │   │░ ║│
│  ║  │ [A] Axum  [B] Actix  [C] Rocket                                 │   │░ ║│
│  ║  └──────────────────────────────────────────────────────────────────┘   │░ ║│
│  ║                                                                         │░ ║│
│  ║  ┌─ You ────────────────────────────────────────────────────────────┐   │░ ║│
│  ║  │ A                                                                │   │░ ║│
│  ║  └──────────────────────────────────────────────────────────────────┘   │▼ ║│
│  ╚═════════════════════════════════════════════════════════════════════════╧══╝│
│                                                                                 │
│  Scrollbar elements:                                                           │
│  ▲ - Up arrow (click to scroll up)                                             │
│  █ - Thumb (current position, draggable)                                       │
│  ░ - Track (background)                                                        │
│  ▼ - Down arrow (click to scroll down)                                         │
│                                                                                 │
│  Thumb color indicates position:                                               │
│  • Top: Cyan                                                                   │
│  • Middle: White                                                               │
│  • Bottom: Magenta (you're at the latest)                                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Popup/Modal: Confirmations & Previews

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  MODAL DIALOGS                                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  VIBER Intervention Modal:                                                     │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │  ╔═══════════════════════════════════════════════════════════════════╗  │   │
│  │  ║░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░║  │   │
│  │  ║░                                                                 ░║  │   │
│  │  ║░   ╔═══════════════════════════════════════════════════════╗    ░║  │   │
│  │  ║░   ║  👁 VIBER INTERVENTION                                 ║    ░║  │   │
│  │  ║░   ╠═══════════════════════════════════════════════════════╣    ░║  │   │
│  │  ║░   ║                                                       ║    ░║  │   │
│  │  ║░   ║  Agent "frontend-dev" is adding a feature not in     ║    ░║  │   │
│  │  ║░   ║  the spec: Dark mode theme toggle                     ║    ░║  │   │
│  │  ║░   ║                                                       ║    ░║  │   │
│  │  ║░   ║  This could add 2+ hours to the timeline.            ║    ░║  │   │
│  │  ║░   ║                                                       ║    ░║  │   │
│  │  ║░   ║  ┌─────────────────────────────────────────────────┐ ║    ░║  │   │
│  │  ║░   ║  │ [A] Allow - Add to spec                         │ ║    ░║  │   │
│  │  ║░   ║  │ [U] Undo - Revert agent's changes               │ ║    ░║  │   │
│  │  ║░   ║  │ [S] Stop - Halt agent, discuss                  │ ║    ░║  │   │
│  │  ║░   ║  │ [D] Defer - Add to backlog for later            │ ║    ░║  │   │
│  │  ║░   ║  └─────────────────────────────────────────────────┘ ║    ░║  │   │
│  │  ║░   ║                                                       ║    ░║  │   │
│  │  ║░   ╚═══════════════════════════════════════════════════════╝    ░║  │   │
│  │  ║░                                                                 ░║  │   │
│  │  ║░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░║  │   │
│  │  ╚═══════════════════════════════════════════════════════════════════╝  │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  Features:                                                                     │
│  • Dimmed background (░ overlay)                                               │
│  • Centered modal with glow border                                             │
│  • Keyboard shortcuts visible                                                  │
│  • ESC to dismiss                                                              │
│                                                                                 │
│                                                                                 │
│  Spec Preview Modal (on hover/Tab):                                           │
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════╗         │
│  ║  SPEC PREVIEW                                           [ESC] ×  ║         │
│  ╠═══════════════════════════════════════════════════════════════════╣         │
│  ║                                                                   ║         │
│  ║  # REST API Specification                                        ║         │
│  ║                                                                   ║         │
│  ║  ## Features                                                      ║         │
│  ║  - [x] User authentication (JWT)                                 ║         │
│  ║  - [x] CRUD operations                                           ║         │
│  ║  - [ ] Rate limiting  ◀── current task                           ║         │
│  ║  - [ ] Caching                                                   ║         │
│  ║                                                                   ║         │
│  ║  ## Constraints                                                   ║         │
│  ║  - Response time < 100ms                                         ║         │
│  ║  - 99.9% uptime                                                  ║         │
│  ║                                                                   ║         │
│  ╚═══════════════════════════════════════════════════════════════════╝         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Masked Input: Secrets Entry

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  MASKED INPUT (for API keys, secrets)                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ╔═══════════════════════════════════════════════════════════════════════════╗ │
│  ║  Enter OpenAI API Key                                                     ║ │
│  ╠═══════════════════════════════════════════════════════════════════════════╣ │
│  ║                                                                           ║ │
│  ║  ┌───────────────────────────────────────────────────────────────────┐   ║ │
│  ║  │ ●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●█                                  │   ║ │
│  ║  └───────────────────────────────────────────────────────────────────┘   ║ │
│  ║                                                                           ║ │
│  ║  [👁 Show] [Enter] Confirm   [ESC] Cancel                                 ║ │
│  ║                                                                           ║ │
│  ╚═══════════════════════════════════════════════════════════════════════════╝ │
│                                                                                 │
│  When "Show" is pressed:                                                       │
│  ┌───────────────────────────────────────────────────────────────────┐         │
│  │ sk-proj-abc123def456ghi789jkl012mno345█                           │         │
│  └───────────────────────────────────────────────────────────────────┘         │
│                                                                                 │
│  Auto-hide after 3 seconds for security                                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Flex Layouts: Responsive Panels

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  FLEX LAYOUT SYSTEM                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Wide terminal (>150 cols):                                                    │
│  ┌──────────────┬──────────────────────────────────────┬──────────────┐        │
│  │              │                                      │              │        │
│  │   Sidebar    │            Main Content              │   Context    │        │
│  │    (20%)     │              (60%)                   │    (20%)     │        │
│  │              │                                      │              │        │
│  └──────────────┴──────────────────────────────────────┴──────────────┘        │
│                                                                                 │
│  Medium terminal (100-150 cols):                                               │
│  ┌──────────┬────────────────────────────────────────────────────────┐         │
│  │          │                                                        │         │
│  │ Sidebar  │                    Main Content                        │         │
│  │  (25%)   │                      (75%)                             │         │
│  │          │                                                        │         │
│  └──────────┴────────────────────────────────────────────────────────┘         │
│  Context panel becomes a bottom drawer                                         │
│                                                                                 │
│  Narrow terminal (<100 cols):                                                  │
│  ┌────────────────────────────────────────────────────────────────────┐        │
│  │                                                                    │        │
│  │                         Main Content                               │        │
│  │                          (100%)                                    │        │
│  │                                                                    │        │
│  └────────────────────────────────────────────────────────────────────┘        │
│  Sidebar and Context become overlay panels (Tab to toggle)                     │
│                                                                                 │
│  ```rust                                                                       │
│  let layout = Layout::default()                                                │
│      .direction(Direction::Horizontal)                                         │
│      .constraints([                                                            │
│          Constraint::Min(20),           // Sidebar min 20 cols                 │
│          Constraint::Percentage(60),    // Main content 60%                    │
│          Constraint::Min(20),           // Context min 20 cols                 │
│      ]);                                                                       │
│  ```                                                                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Unicode Block Elements: Inline Visualizations

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  INLINE MINI-VISUALIZATIONS                                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Progress bars (horizontal):                                                   │
│  ▏▎▍▌▋▊▉█  (8 levels of fill)                                                  │
│                                                                                 │
│  Examples:                                                                     │
│  Task 1: █████████░░░░░░░ 60%                                                  │
│  Task 2: ████████████████ 100% ✓                                               │
│  Task 3: ▓▓▓░░░░░░░░░░░░░ 20%  (▓ = in progress, ░ = remaining)               │
│                                                                                 │
│  Vertical bars (for charts):                                                   │
│  ▁▂▃▄▅▆▇█  (8 levels of height)                                                │
│                                                                                 │
│  Inline bar chart:                                                             │
│  Mon: ▆  Tue: █  Wed: ▄  Thu: ▇  Fri: ▃  Sat: ▁  Sun: ▁                       │
│                                                                                 │
│  Block shading:                                                                │
│  ░ Light shade (25%)                                                           │
│  ▒ Medium shade (50%)                                                          │
│  ▓ Dark shade (75%)                                                            │
│  █ Full block (100%)                                                           │
│                                                                                 │
│  Quadrant blocks (for high-res in small space):                                │
│  ▖▗▘▝ ▀▄▌▐ ▚▞                                                                  │
│                                                                                 │
│  Example: 2x resolution graph                                                  │
│  ▗▖  ▄▖    ▗▄                                                                  │
│  ██▄▖██▗▄▄▖██                                                                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Braille Patterns: Ultra High-Resolution

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  BRAILLE PATTERN GRAPHICS (2x4 dots per character = 8 pixels!)                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Braille block:                                                                │
│  ⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏                                                             │
│  ⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟                                                             │
│  ⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯                                                             │
│  ⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿                                                             │
│  ⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏                                                             │
│  ...up to ⣿ (all 8 dots filled)                                                │
│                                                                                 │
│  High-resolution sparkline:                                                    │
│  Standard:  ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁  (8 levels)                                        │
│  Braille:   ⣀⣠⣤⣴⣶⣾⣿⣷⣶⣴⣤⣠⣀  (256 patterns!)                                     │
│                                                                                 │
│  Braille line graph (2x vertical resolution):                                 │
│                                                                                 │
│  ⡇      ⢸                                                                      │
│  ⡇    ⢀⣴⣿⣦⡀                                                                    │
│  ⡇  ⢀⣴⣿⠋  ⠙⣿⣦⡀                                                                 │
│  ⡇⢀⣴⣿⠋        ⠙⣿⣦⡀                                                             │
│  ⣷⣿⠋              ⠙⣿⣷                                                          │
│  ⠉                    ⠉                                                        │
│                                                                                 │
│  VIBER eye in braille (animated):                                              │
│  Frame 1: ⢠⣶⣦⡀     Frame 2: ⢠⣶⣦⡀     Frame 3: ⢠⣤⣤⡀                            │
│           ⣿⣿⣿⣿              ⣿⣤⣤⣿              ⣿⣶⣶⣿                            │
│           ⠸⠿⠿⠇              ⠸⠿⠿⠇              ⠸⠿⠿⠇                            │
│           (open)             (looking)          (blink)                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Box-Drawing: Corner Styles

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  BOX DRAWING STYLES                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Sharp corners (current):                                                      │
│  ┌────────────────┐   ╔════════════════╗                                       │
│  │  Single line   │   ║  Double line   ║                                       │
│  └────────────────┘   ╚════════════════╝                                       │
│                                                                                 │
│  Rounded corners (softer feel):                                                │
│  ╭────────────────╮   ╭════════════════╮                                       │
│  │  Rounded       │   │  Mixed round   │                                       │
│  ╰────────────────╯   ╰════════════════╯                                       │
│                                                                                 │
│  Heavy lines (emphasis):                                                       │
│  ┏━━━━━━━━━━━━━━━━┓                                                            │
│  ┃  Heavy box     ┃                                                            │
│  ┗━━━━━━━━━━━━━━━━┛                                                            │
│                                                                                 │
│  Mixed (title emphasis):                                                       │
│  ┏━━━━━ VIBER ━━━━━┓                                                           │
│  │                  │                                                          │
│  │  Content here    │                                                          │
│  │                  │                                                          │
│  └──────────────────┘                                                          │
│                                                                                 │
│  Use cases:                                                                    │
│  • Rounded: Chat bubbles, friendly UI                                          │
│  • Sharp: Structured data, technical panels                                    │
│  • Double: Focused/selected panels                                             │
│  • Heavy: Critical alerts, VIBER interventions                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Color Gradients: Fade Effects

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  COLOR GRADIENTS (RGB interpolation)                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Progress bar gradient (green → yellow → red as deadline approaches):          │
│                                                                                 │
│  Early:   ████████████████████████░░░░░░░░░░░░░░░░  (green)                    │
│  Mid:     ████████████████████████████████░░░░░░░░  (yellow)                   │
│  Late:    ████████████████████████████████████████  (red)                      │
│                                                                                 │
│  Heat map gradient for activity:                                               │
│  Low     ░░░▒▒▒▓▓▓███     High                                                 │
│  (blue)  → (cyan) → (green) → (yellow) → (red)                                 │
│                                                                                 │
│  Text fade (for streaming):                                                    │
│  "Implementing auth" ← bright                                                  │
│  "middleware..."     ← medium                                                  │
│  "..."               ← dim                                                     │
│                                                                                 │
│  ```rust                                                                       │
│  fn gradient(start: Color, end: Color, t: f32) -> Color {                     │
│      let r = lerp(start.r, end.r, t);                                          │
│      let g = lerp(start.g, end.g, t);                                          │
│      let b = lerp(start.b, end.b, t);                                          │
│      Color::Rgb(r, g, b)                                                       │
│  }                                                                             │
│  ```                                                                           │
│                                                                                 │
│  Glow effect (bright center → dim edges):                                      │
│  ░░▒▓█ VIBER █▓▒░░                                                             │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Text Modifiers: Emphasis Without Color

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TEXT STYLE MODIFIERS                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Ratatui modifiers:                                                            │
│  • Bold:          Important text                                              │
│  • Dim:           Secondary info                                               │
│  • Italic:        Emphasis, thoughts                                           │
│  • Underlined:    Links, actions                                               │
│  • Reversed:      Selected items                                               │
│  • Crossed out:   Completed/cancelled                                          │
│  • Rapid blink:   URGENT (use sparingly!)                                      │
│                                                                                 │
│  Combinations:                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ VIBER (bold + cyan)                                                     │   │
│  │ ─────────────────────────────────────────────────────────────────────── │   │
│  │ Agent 2 is adding an unspecified feature. (normal)                      │   │
│  │                                                                         │   │
│  │ Recommended action: Stop and discuss (bold + yellow)                    │   │
│  │                                                                         │   │
│  │ Details: The agent started implementing a theme toggle... (dim)         │   │
│  │                                                                         │   │
│  │ [S] Stop (underlined) | [A] Allow (underlined) | [U] Undo (underlined)  │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  Task states:                                                                  │
│  • Pending:    Normal text                                                     │
│  • In progress: Bold                                                           │
│  • Completed:   Dim + crossed out                                              │
│  • Failed:      Bold + red                                                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Mouse Support: Click & Drag

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  MOUSE INTERACTIONS                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Click actions:                                                                │
│  • Click panel → Focus panel                                                   │
│  • Click task → Select task                                                    │
│  • Click agent → View agent details                                            │
│  • Click button → Activate button                                              │
│  • Click file path → Open in nvim                                              │
│                                                                                 │
│  Drag actions:                                                                 │
│  • Drag panel border → Resize panel                                            │
│  • Drag scrollbar thumb → Scroll content                                       │
│  • Drag task card → Move between columns (Kanban)                              │
│                                                                                 │
│  Scroll actions:                                                               │
│  • Scroll wheel → Scroll focused panel                                         │
│  • Shift + scroll → Horizontal scroll                                          │
│  • Ctrl + scroll → Zoom (for canvas/DAG)                                       │
│                                                                                 │
│  Hover effects:                                                                │
│  • Hover panel → Subtle highlight                                              │
│  • Hover button → Glow effect                                                  │
│  • Hover link → Underline appears                                              │
│  • Hover agent → Show tooltip with status                                      │
│                                                                                 │
│  Right-click context menu:                                                     │
│  ╭─────────────────────╮                                                       │
│  │ ▶ View Details      │                                                       │
│  │   Stop Agent        │                                                       │
│  │   Restart Agent     │                                                       │
│  ├─────────────────────┤                                                       │
│  │   Copy Output       │                                                       │
│  │   View Logs         │                                                       │
│  ╰─────────────────────╯                                                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Hyperlinks: Clickable Paths

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TERMINAL HYPERLINKS (OSC 8)                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  File paths become clickable:                                                  │
│                                                                                 │
│  ┌─ Agent Output ───────────────────────────────────────────────────────────┐  │
│  │                                                                          │  │
│  │  Reading src/api/auth.rs ...                                            │  │
│  │           └─────────────┘                                                │  │
│  │            ↑ clickable (opens in nvim via nvim-mcp)                      │  │
│  │                                                                          │  │
│  │  Error at src/middleware/jwt.rs:42                                       │  │
│  │           └───────────────────────┘                                      │  │
│  │            ↑ clickable (opens at line 42)                                │  │
│  │                                                                          │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  URL links (for docs, references):                                             │
│  "See https://docs.rs/axum for details"                                        │
│        └──────────────────┘                                                    │
│         ↑ opens in browser                                                     │
│                                                                                 │
│  Implementation:                                                               │
│  ```rust                                                                       │
│  // OSC 8 hyperlink escape sequence                                           │
│  let link = format!(                                                           │
│      "\x1b]8;;file://{}\x1b\\{}\x1b]8;;\x1b\\",                                │
│      absolute_path,                                                            │
│      display_text                                                              │
│  );                                                                            │
│  ```                                                                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Audio: Terminal Bell

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AUDIO FEEDBACK (Terminal Bell)                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Use terminal bell (\x07) for audio alerts:                                    │
│                                                                                 │
│  Events that trigger bell:                                                     │
│  • VIBER intervention required (needs user attention)                          │
│  • Task completed                                                              │
│  • Error occurred                                                              │
│  • Agent question (waiting for input)                                          │
│                                                                                 │
│  Bell patterns:                                                                │
│  • Single beep:    Task complete                                               │
│  • Double beep:    Agent needs input                                           │
│  • Triple beep:    VIBER intervention                                          │
│  • Long beep:      Critical error                                              │
│                                                                                 │
│  Settings:                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  Audio Notifications                                                    │   │
│  │  ─────────────────────────────────────────────────────────────────────  │   │
│  │  [x] Enable terminal bell                                               │   │
│  │  [x] VIBER alerts                                                       │   │
│  │  [x] Task completion                                                    │   │
│  │  [ ] Agent activity (too noisy)                                         │   │
│  │  [x] Errors only                                                        │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ```rust                                                                       │
│  fn alert(pattern: AlertPattern) {                                             │
│      match pattern {                                                           │
│          AlertPattern::TaskComplete => print!("\x07"),                         │
│          AlertPattern::NeedsInput => print!("\x07\x07"),                       │
│          AlertPattern::ViberIntervention => print!("\x07\x07\x07"),            │
│          AlertPattern::Error => {                                              │
│              print!("\x07");                                                   │
│              sleep(Duration::from_millis(200));                                │
│              print!("\x07");                                                   │
│          }                                                                     │
│      }                                                                         │
│  }                                                                             │
│  ```                                                                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

### Complete Widget Reference

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  RATATUI WIDGET CHECKLIST                                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Layout & Structure                                                            │
│  ──────────────────                                                            │
│  [x] Block          Panel containers with borders                              │
│  [x] Layout         Flex-based responsive layouts                              │
│  [x] Constraint     Min/max/percentage sizing                                  │
│  [x] Margin         Spacing around elements                                    │
│  [x] Padding        Spacing inside elements                                    │
│                                                                                 │
│  Text & Content                                                                │
│  ──────────────                                                                │
│  [x] Paragraph      Text with wrapping and scroll                              │
│  [x] Span           Styled text segments                                       │
│  [x] Line           Single line of styled text                                 │
│  [x] Text           Multi-line styled text                                     │
│  [x] Masked         Password/secret input                                      │
│                                                                                 │
│  Data Display                                                                  │
│  ────────────                                                                  │
│  [x] List           Selectable item lists                                      │
│  [x] Table          Structured data grids                                      │
│  [x] Gauge          Progress bars                                              │
│  [x] Sparkline      Inline mini-charts                                         │
│  [x] BarChart       Horizontal bar comparisons                                 │
│  [x] Chart          Line/scatter plots                                         │
│  [x] Calendar       Date heatmaps                                              │
│                                                                                 │
│  Navigation                                                                    │
│  ──────────                                                                    │
│  [x] Tabs           View switching                                             │
│  [x] Scrollbar      Long content navigation                                    │
│                                                                                 │
│  Graphics                                                                      │
│  ────────                                                                      │
│  [x] Canvas         Vector drawing (DAG, diagrams)                             │
│  [x] Braille        High-resolution patterns                                   │
│  [x] Block elements Inline visualizations                                      │
│                                                                                 │
│  Interaction                                                                   │
│  ───────────                                                                   │
│  [x] Mouse          Click, drag, scroll, hover                                 │
│  [x] Popup/Modal    Overlays and dialogs                                       │
│  [x] Hyperlinks     Clickable paths/URLs                                       │
│                                                                                 │
│  Styling                                                                       │
│  ───────                                                                       │
│  [x] Colors         RGB neon palette                                           │
│  [x] Gradients      Fade effects                                               │
│  [x] Modifiers      Bold, dim, italic, underline                               │
│  [x] Borders        Sharp, rounded, heavy, double                              │
│                                                                                 │
│  Animation                                                                     │
│  ─────────                                                                     │
│  [x] Spinners       Multiple braille/unicode styles                            │
│  [x] Pulse          Brightness cycling                                         │
│  [x] Streaming      Character-by-character text                                │
│  [x] Cursor blink   Block cursor animation                                     │
│                                                                                 │
│  Audio                                                                         │
│  ─────                                                                         │
│  [x] Terminal bell  Alert patterns                                             │
│                                                                                 │
│  ALL WIDGETS UTILIZED ✓                                                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

*Every Ratatui capability harnessed. Maximum cyberpunk vibes achieved.*
*This is the complete OpenCode TUI vision.*
