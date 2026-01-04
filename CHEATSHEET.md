# OpenCode Cheatsheet

> Complete reference for your OpenCode setup with all MCPs, agents, skills, and commands.

---

## Table of Contents
1. [MCP Servers](#1-mcp-servers)
2. [Shell Commands](#2-shell-commands)
3. [Slash Commands](#3-slash-commands)
4. [Skills](#4-skills)
5. [Agents/Subagents](#5-agentssubagents)
6. [Memcord (Memory)](#6-memcord-memory)
7. [Neovim Integration](#7-neovim-integration)
8. [Themes](#8-themes)
9. [@ Mentions / Context](#9--mentions--context)
10. [Quick Reference Card](#10-quick-reference-card)

---

## 1. MCP Servers

### Standalone MCPs (in opencode.json)
| MCP | Purpose |
|-----|---------|
| `supervisor` | Process manager - list/kill/restart MCP servers |
| `neovim` | Neovim integration - cursor, buffers, LSP, execute lua |

### NCP Profiles

**Run with:** `ncp` (default/all), `ncp-coding`, `ncp-research`, `ncp-creative`, `ncp-productivity`

#### Coding Profile (31 MCPs) - `ncp-coding`
| MCP | Purpose |
|-----|---------|
| `console-automation` | Terminal/console automation |
| `context7` | Library documentation lookup |
| `devtools-debugger` | Chrome DevTools debugging |
| `docker` | Docker container management |
| `docs-rs` | Rust documentation |
| `droidmind` | Android device control |
| `filesystem` | File read/write/search |
| `git-mcp` | Git operations |
| `github` | GitHub API (issues, PRs, repos) |
| `gitmcp` | Alternative git operations |
| `gradle-mcp-server` | Gradle build system |
| `in-memoria` | Local memory/context |
| `jarvis-mcp` | Multi-purpose assistant |
| `kubernetes` | K8s cluster management |
| `local-rag` | Local RAG search |
| `mcp-lsp` | LSP integration |
| `mcp-prompt-engine` | Prompt management |
| `memcord` | Persistent chat memory |
| `mermaid-mcp` | Diagram generation |
| `mobile-mcp` | Mobile device control |
| `mysql` | MySQL database |
| `neovim` | Neovim integration |
| `nodejs-debugger` | Node.js debugging |
| `openapi-mcp` | OpenAPI spec tools |
| `playwright` | Browser automation |
| `postgres` | PostgreSQL database |
| `sentry` | Error tracking |
| `sequentialthinking` | Step-by-step reasoning |
| `sqlite` | SQLite database |
| `task_master` | Task management |
| `troubleshooter` | Debug assistance |

#### Research Profile (8 MCPs) - `ncp-research`
| MCP | Purpose |
|-----|---------|
| `context7` | Library docs lookup |
| `fetch` | HTTP requests |
| `jarvis-mcp` | Multi-purpose assistant |
| `memcord` | Persistent memory |
| `memory` | Session memory |
| `sequentialthinking` | Step-by-step reasoning |
| `system-prompts-mcp` | System prompt management |
| `web-search` | Web search (Exa) |

#### Creative Profile (10 MCPs) - `ncp-creative`
| MCP | Purpose |
|-----|---------|
| `fetch` | HTTP requests |
| `filesystem` | File operations |
| `jarvis-mcp` | Multi-purpose assistant |
| `memcord` | Persistent memory |
| `memory` | Session memory |
| `mermaid-mcp` | Diagram generation |
| `playwright` | Browser automation |
| `sequentialthinking` | Step-by-step reasoning |
| `system-prompts-mcp` | System prompt management |
| `web-search` | Web search |

#### Productivity Profile (13 MCPs) - `ncp-productivity`
| MCP | Purpose |
|-----|---------|
| `console-automation` | Terminal automation |
| `filesystem` | File operations |
| `google-sheets` | Google Sheets API |
| `google-tasks` | Google Tasks API |
| `jarvis-mcp` | Multi-purpose assistant |
| `mcp-prompt-engine` | Prompt management |
| `memcord` | Persistent memory |
| `memory` | Session memory |
| `notify` | System notifications |
| `sequentialthinking` | Step-by-step reasoning |
| `sqlite` | SQLite database |
| `task_master` | Task management |
| `time` | Time/date utilities |

#### Default Profile (39 MCPs) - `ncp`
All MCPs from all 4 profiles combined.

---

## 2. Shell Commands

### OpenCode CLI

| Command | Purpose |
|---------|---------|
| `opencode` | Start TUI in current directory |
| `opencode [path]` | Start TUI in specified directory |
| `opencode -c` / `--continue` | Continue last session |
| `opencode -s <id>` | Continue specific session |
| `opencode -m provider/model` | Use specific model |
| `opencode run [message]` | Run with a message (non-interactive) |
| `opencode serve` | Start headless server |
| `opencode models [provider]` | List available models |
| `opencode stats` | Show token usage/cost stats |
| `opencode pr <number>` | Fetch PR and start OpenCode |
| `opencode session` | Manage sessions |
| `opencode auth` | Manage credentials |
| `opencode upgrade` | Upgrade to latest version |

### Beads (bd) - Issue Tracker

**Core Commands:**
| Command | Purpose |
|---------|---------|
| `bd ready` | Show issues ready to work (no blockers) |
| `bd list --status=open` | List all open issues |
| `bd list --status=in_progress` | List your active work |
| `bd show <id>` | Show issue details |
| `bd create --title="..." --type=task\|bug\|feature --priority=2` | Create issue |
| `bd update <id> --status=in_progress` | Claim work |
| `bd close <id>` | Mark complete |
| `bd close <id1> <id2> ...` | Close multiple at once |

**Dependencies:**
| Command | Purpose |
|---------|---------|
| `bd dep add <issue> <depends-on>` | Add dependency |
| `bd blocked` | Show blocked issues |
| `bd graph` | Display dependency graph |

**Other:**
| Command | Purpose |
|---------|---------|
| `bd stats` | Project statistics |
| `bd sync` | Sync with git |
| `bd doctor` | Check for issues |
| `bd search <query>` | Search issues by text |

### Wrapper Scripts

| Script | Purpose |
|--------|---------|
| `ncp-wrapper.sh` | Default NCP (all MCPs) |
| `ncp-coding.sh` | Coding profile |
| `ncp-research.sh` | Research profile |
| `ncp-creative.sh` | Creative profile |
| `ncp-productivity.sh` | Productivity profile |
| `nvim-wrapper.sh` | Neovim MCP wrapper |
| `memcord-wrapper.sh` | Memcord memory wrapper |
| `mcp-lsp-wrapper.sh` | LSP MCP wrapper |
| `theme-sync.sh` | Sync themes |
| `openagents-install.sh` | Install openagents |

---

## 3. Slash Commands

### Command Categories

| Category | Commands | Purpose |
|----------|----------|---------|
| `agentic/` | commit, execute, plan, review | Agentic workflow |
| `codex/` | models, status | OpenAI Codex integration |
| `github/` | actions, issues, my-prs, pr-create, pr-review | GitHub operations |
| `openspec/` | init, proposal, apply, archive, validate, status, discover | Spec-driven development |
| `superpowers/` | brainstorm, write-plan, execute-plan | Planning workflows |

### Key Commands by Use Case

**Planning & Execution:**
| Command | Purpose |
|---------|---------|
| `/agentic/plan` | Create implementation plan |
| `/agentic/execute` | Execute a plan file |
| `/agentic/commit` | Commit changes atomically |
| `/superpowers/brainstorm` | Brainstorm ideas |
| `/superpowers/write-plan` | Write detailed plan |
| `/superpowers/execute-plan` | Execute plan |

**GitHub:**
| Command | Purpose |
|---------|---------|
| `/github/issues` | Manage issues |
| `/github/pr-create` | Create pull request |
| `/github/pr-review` | Review pull request |
| `/github/my-prs` | List your PRs |
| `/github/actions` | GitHub Actions |

**OpenSpec (Spec-driven dev):**
| Command | Purpose |
|---------|---------|
| `/openspec/init` | Initialize spec structure |
| `/openspec/proposal` | Create change proposal |
| `/openspec/apply` | Apply approved spec |
| `/openspec/validate` | Validate spec |
| `/openspec/discover` | Discovery workflow |

**MCP Tools:**
| Command | Purpose |
|---------|---------|
| `/context7/*` | Library documentation |
| `/docker/*` | Docker operations |
| `/git/*` | Git operations |
| `/github/*` | GitHub API |
| `/kubernetes/*` | K8s management |
| `/memcord/*` | Memory management |
| `/neovim/*` | Neovim integration |
| `/playwright/*` | Browser automation |
| `/postgres/*` | PostgreSQL |
| `/sqlite/*` | SQLite |

---

## 4. Skills

### Superpowers Skills (Core Workflow)

| Skill | When to Use |
|-------|-------------|
| `brainstorming` | **Before ANY creative work** - features, components, modifications |
| `writing-plans` | Have spec/requirements, before touching code |
| `executing-plans` | Have a written plan to execute with review checkpoints |
| `subagent-driven-development` | Executing plans with independent parallel tasks |
| `dispatching-parallel-agents` | 2+ independent tasks without shared state |
| `test-driven-development` | Before writing implementation code |
| `systematic-debugging` | Any bug, test failure, unexpected behavior |
| `requesting-code-review` | Completing tasks, before merging |
| `receiving-code-review` | Got review feedback, before implementing |
| `verification-before-completion` | Before claiming work is done |
| `finishing-a-development-branch` | Implementation complete, deciding how to integrate |
| `using-git-worktrees` | Starting isolated feature work |
| `writing-skills` | Creating/editing skills |
| `using-superpowers` | Auto-loaded at conversation start |

### Registry Skills (Collections)

| Collection | Contents |
|------------|----------|
| `awesome-claude-skills` | Community Claude skills |
| `awesome-agent-skills` | General agent skills |
| `awesome-llm-skills` | LLM-focused skills |
| `mcp` | MCP-related skills |
| `mcp-for-security` | Security-focused MCP skills |
| `LLM-PromptEngineering-Agents` | Prompt engineering |
| `Awesome-Context-Engineering` | Context management |
| `awesome-mcp-personas` | MCP persona templates |
| `awesome-reviewers` | Code review skills |

### Workflow: How Skills Chain Together

```
1. brainstorming → Explore requirements
2. writing-plans → Create detailed plan
3. using-git-worktrees → Isolate work (optional)
4. test-driven-development → Write tests first
5. executing-plans / subagent-driven-development → Implement
6. systematic-debugging → Fix issues
7. verification-before-completion → Verify everything works
8. requesting-code-review → Get review
9. receiving-code-review → Address feedback
10. finishing-a-development-branch → Merge/PR
```

---

## 5. Agents/Subagents

**Total: 125 specialized subagents**

### How to Invoke
```
Task tool with subagent_type="agent-name"
```

### By Category

#### Frontend/UI
| Agent | Specialty |
|-------|-----------|
| `frontend-developer` | React, general frontend |
| `react-specialist` | React 18+, hooks, server components |
| `angular-architect` | Angular 15+, NgRx, RxJS |
| `vue-expert` | Vue 3, Composition API, Nuxt |
| `nextjs-developer` | Next.js 14+, App Router |
| `ui-designer` | Visual design, UI patterns |
| `ux-researcher` | User research, usability |
| `accessibility-tester` | WCAG compliance |

#### Backend
| Agent | Specialty |
|-------|-----------|
| `backend-developer` | General backend, APIs |
| `java-architect` | Spring, enterprise Java |
| `spring-boot-engineer` | Spring Boot 3+ |
| `python-pro` | Python 3.11+, async |
| `django-developer` | Django 4+ |
| `golang-pro` | Go, concurrency |
| `rust-engineer` | Rust, memory safety |
| `php-pro` | PHP 8.3+, Laravel/Symfony |
| `laravel-specialist` | Laravel 10+ |
| `rails-expert` | Rails 8.1 |

#### Full-Stack
| Agent | Specialty |
|-------|-----------|
| `fullstack-developer` | End-to-end features |
| `typescript-pro` | TypeScript, type safety |
| `javascript-pro` | ES2023+, browser/Node |

#### Mobile
| Agent | Specialty |
|-------|-----------|
| `mobile-developer` | Cross-platform |
| `mobile-app-developer` | iOS/Android native |
| `flutter-expert` | Flutter 3+ |
| `swift-expert` | Swift 5.9+, SwiftUI |
| `kotlin-specialist` | Kotlin, Android |

#### DevOps/Infrastructure
| Agent | Specialty |
|-------|-----------|
| `devops-engineer` | CI/CD, containers |
| `kubernetes-specialist` | K8s orchestration |
| `terraform-engineer` | IaC, multi-cloud |
| `cloud-architect` | AWS/Azure/GCP |
| `platform-engineer` | Internal platforms |
| `sre-engineer` | Site reliability |
| `deployment-engineer` | Release automation |

#### Database
| Agent | Specialty |
|-------|-----------|
| `database-administrator` | HA, DR, optimization |
| `database-optimizer` | Query tuning |
| `postgres-pro` | PostgreSQL expert |
| `sql-pro` | Complex SQL, optimization |
| `data-engineer` | Pipelines, ETL |

#### AI/ML
| Agent | Specialty |
|-------|-----------|
| `ai-engineer` | AI system design |
| `ml-engineer` | ML lifecycle |
| `mlops-engineer` | ML infrastructure |
| `llm-architect` | LLM deployment |
| `nlp-engineer` | NLP systems |
| `prompt-engineer` | Prompt design |
| `data-scientist` | Analysis, modeling |

#### Security
| Agent | Specialty |
|-------|-----------|
| `security-engineer` | DevSecOps, cloud security |
| `security-auditor` | Assessments, compliance |
| `penetration-tester` | Ethical hacking |
| `compliance-auditor` | GDPR, HIPAA, PCI |

#### Architecture/Review
| Agent | Specialty |
|-------|-----------|
| `architect-reviewer` | System design validation |
| `microservices-architect` | Distributed systems |
| `graphql-architect` | GraphQL schemas |
| `api-designer` | REST/GraphQL APIs |
| `code-reviewer` | Code quality, security |

#### Documentation
| Agent | Specialty |
|-------|-----------|
| `technical-writer` | Docs, guides |
| `api-documenter` | API documentation |
| `documentation-engineer` | Doc systems |

#### Testing/QA
| Agent | Specialty |
|-------|-----------|
| `qa-expert` | Test strategy |
| `test-automator` | Test frameworks |
| `chaos-engineer` | Resilience testing |

#### Debugging
| Agent | Specialty |
|-------|-----------|
| `debugger` | Complex issue diagnosis |
| `error-detective` | Error pattern analysis |
| `error-coordinator` | Distributed error handling |

#### Windows/PowerShell
| Agent | Specialty |
|-------|-----------|
| `windows-infra-admin` | Windows infrastructure |
| `powershell-5.1-expert` | Legacy PowerShell |
| `powershell-7-expert` | Modern PowerShell |
| `powershell-module-architect` | Module design |
| `m365-admin` | Microsoft 365 |
| `azure-infra-engineer` | Azure infrastructure |

#### Specialized Tech
| Agent | Specialty |
|-------|-----------|
| `blockchain-developer` | Smart contracts, DeFi |
| `game-developer` | Game engines |
| `embedded-systems` | Microcontrollers, RTOS |
| `iot-engineer` | IoT platforms |
| `websocket-engineer` | Real-time comms |
| `electron-pro` | Desktop apps |
| `mcp-developer` | MCP servers |

#### Business/Product
| Agent | Specialty |
|-------|-----------|
| `product-manager` | Product strategy |
| `project-manager` | Project delivery |
| `business-analyst` | Requirements |
| `scrum-master` | Agile facilitation |
| `research-analyst` | Information gathering |

---

## 6. Memcord (Memory)

### Overview
Memcord provides persistent memory across sessions. Available in ALL NCP profiles.

### Core Workflow

```bash
# Starting a new project/topic
memcord_name "project_name"        # Create/select a slot

# Resuming existing work
memcord_use "project_name"         # Activate existing slot
memcord_read                       # Load previous context

# During work - save important content
memcord_save "key decisions and context..."
memcord_save_progress              # Auto-summarize and timestamp

# Find past information
memcord_search "API design"        # Full-text search
memcord_query "What did we decide about auth?"  # Natural language

# Privacy mode (sensitive conversations)
memcord_zero                       # Disable saving until slot change
```

### All 19 Tools

#### Basic Tools
| Tool | Example | Purpose |
|------|---------|---------|
| `memcord_name` | `memcord_name "my_project"` | Create/select memory slot |
| `memcord_use` | `memcord_use "my_project"` | Switch to existing slot |
| `memcord_save` | `memcord_save "Summary..."` | Save text to current slot |
| `memcord_read` | `memcord_read` | Retrieve memory content |
| `memcord_save_progress` | `memcord_save_progress` | Auto-summarize + timestamp |
| `memcord_list` | `memcord_list` | List all slots |
| `memcord_search` | `memcord_search "schema"` | Full-text search |
| `memcord_query` | `memcord_query "What failed?"` | Natural language query |
| `memcord_zero` | `memcord_zero` | Privacy mode |
| `memcord_select_entry` | `relative_time="2 hours ago"` | Navigate timeline |
| `memcord_merge` | `source_slots=["a","b"]` | Combine slots |

#### Advanced Tools
| Tool | Example | Purpose |
|------|---------|---------|
| `memcord_tag` | `action="add" tags="urgent"` | Add/remove tags |
| `memcord_list_tags` | `memcord_list_tags` | Show all tags |
| `memcord_group` | `group_path="projects/api"` | Organize into folders |
| `memcord_import` | `source="/path/to/doc.pdf"` | Import PDFs, URLs, files |
| `memcord_compress` | `action="analyze"` | Optimize storage |
| `memcord_export` | `format="md"` | Export as MD/TXT/JSON |
| `memcord_share` | `slot_name="proj"` | Generate shareable files |
| `memcord_archive` | `slot_name="old"` | Long-term storage |

### Best Practices
1. **Use descriptive slot names**: `project_api_v2`, `meeting_2024_01_15`
2. **Tag consistently**: `urgent`, `review`, `decision`, `bug`
3. **Group by project**: `projects/api/meetings`
4. **Regular summaries**: `memcord_save_progress` after significant work
5. **Query before starting**: Recall past decisions
6. **Merge related content**: Consolidate scattered notes
7. **Archive completed work**: Free up active space

---

## 7. Neovim Integration

### Overview
33 tools for interacting with Neovim via MCP. Standalone server (not NCP).

### Workflow
```typescript
// 1. Get connection_id (auto-connects from ~/.config/opencode)
neovim_nvim_get_targets({})  // Returns "f7ba42a"

// 2. Use connection_id in all subsequent calls
neovim_nvim_cursor_position({ params: { connection_id: "f7ba42a" }})
```

### Core Tools

| Tool | Purpose |
|------|---------|
| `nvim_get_targets` | List sockets, get connection_id |
| `nvim_cursor_position` | Get current cursor position |
| `nvim_navigate` | Go to line/character |
| `nvim_list_buffers` | List open buffers |
| `nvim_read` | Read file content |
| `nvim_exec_lua` | Execute Lua code (most flexible) |
| `nvim_buffer_diagnostics` | Get errors/warnings |

### LSP Tools

| Tool | Purpose |
|------|---------|
| `nvim_lsp_clients` | List LSP clients |
| `nvim_lsp_hover` | Type info, docs at position |
| `nvim_lsp_definition` | Jump to definition |
| `nvim_lsp_references` | Find all references |
| `nvim_lsp_document_symbols` | File outline |
| `nvim_lsp_workspace_symbols` | Search symbols globally |
| `nvim_lsp_code_actions` | Quick fixes, refactorings |
| `nvim_lsp_rename` | Rename symbol |
| `nvim_lsp_formatting` | Format document |

### Document Identifier Formats
```json
{"buffer_id": 123}                              // By buffer ID
{"project_relative_path": "src/main.ts"}        // Relative path
{"absolute_path": "/home/user/file.ts"}         // Absolute path
```

### Key Tips
- **No cmdline tool** - use `nvim_exec_lua` with `vim.cmd("...")`
- All positions are **0-indexed** (LSP standard)
- Connection auto-established from `~/.config/opencode`

---

## 8. Themes

### Overview
**58 themes** available. Current: `laser-transparent`

### How to Switch Theme
Edit `~/.config/opencode/opencode.json`:
```json
{
  "tui": {
    "theme": "catppuccin-mocha"
  }
}
```

### Available Themes (by style)

#### Dark (Popular)
| Theme | Style |
|-------|-------|
| `catppuccin-mocha` | Warm pastel dark |
| `dracula` | Purple-accent dark |
| `tokyonight-night` | Deep blue dark |
| `nord` | Arctic blue-gray |
| `onedark-deep` | Atom One Dark |
| `gruvbox-dark` | Retro warm dark |
| `github-dark` | GitHub dark mode |
| `vscode-dark` | VS Code default |

#### Light
| Theme | Style |
|-------|-------|
| `catppuccin-latte` | Warm pastel light |
| `ayu-light` | Soft warm light |
| `github-light` | GitHub light mode |
| `one-light` | Atom One Light |
| `gruvbox-light` | Retro warm light |
| `nord-light` | Arctic light |
| `rose-pine-dawn` | Soft pink light |
| `solarized-light` | Solarized light |
| `tokyonight-day` | Blue-tinted light |

#### Cyberpunk/Neon
| Theme | Style |
|-------|-------|
| `laser` | Neon laser |
| `laser-transparent` | Transparent neon |
| `synthwave84` | 80s synthwave |
| `cyberdream` | Cyber aesthetic |
| `neon` | Bright neon |
| `fluoromachine` | Fluorescent |
| `matrix` | Matrix green |
| `tron` | Tron blue |
| `blade-runner` | Blade Runner |

#### Space/Cosmic
| Theme | Style |
|-------|-------|
| `andromeda` | Galaxy purple |
| `deep-space` | Deep space |
| `nebula` | Nebula colors |
| `moonlight` | Moonlit blue |
| `moonbow` | Moon rainbow |
| `aurora` | Northern lights |
| `nightfly` | Night sky |
| `nightfox` | Dark fox |

#### AI/Brand
| Theme | Style |
|-------|-------|
| `claude` | Anthropic Claude |
| `openai` | OpenAI green |
| `gemini` | Google Gemini |
| `copilot` | GitHub Copilot |
| `cursor` | Cursor IDE |
| `qwen` | Alibaba Qwen |

#### Minimal/OLED
| Theme | Style |
|-------|-------|
| `pitch-black` | Pure black |
| `github-dark-amoled` | AMOLED black |
| `oxocarbon` | IBM Carbon |
| `midnight` | Midnight blue |

---

## 9. @ Mentions / Context

### File/Directory References in OpenCode

Use `@` to reference files and directories in your messages:

| Syntax | Example | What it does |
|--------|---------|--------------|
| `@filename` | `@package.json` | Include file content |
| `@path/to/file` | `@src/index.ts` | Include file at path |
| `@folder/` | `@src/components/` | Reference directory |
| `@/` | `@/openspec/AGENTS.md` | From project root |

### Context Directories

Your `~/.config/opencode/context/` contains organized context files:

| Directory | Purpose |
|-----------|---------|
| `content/` | Content templates |
| `core/` | Core system context |
| `data/` | Data schemas, structures |
| `development/` | Development guidelines |
| `learning/` | Learning resources |
| `openagents-repo/` | OpenAgents documentation |
| `product/` | Product specs |
| `project/` | Project-specific context |
| `system-builder-templates/` | System templates |

### Plugin Directory Lookup Order

When OpenCode looks for commands/skills/agents:

1. **Project local first**: `.opencode/`, `.claude/`
2. **User global second**: `~/.config/opencode/`, `~/.claude/`

### AGENTS.md Inheritance

`AGENTS.md` files are loaded **upward** from current file to project root:

```
project/
├── AGENTS.md              # Loaded 3rd (project-wide)
├── src/
│   ├── AGENTS.md          # Loaded 2nd (src-specific)
│   └── components/
│       ├── AGENTS.md      # Loaded 1st (component-specific)
│       └── Button.tsx     # Reading this loads all 3
```

### Key Context Files

| File | Purpose | Auto-loaded? |
|------|---------|--------------|
| `AGENTS.md` | Directory-level AI instructions | Yes (per-directory) |
| `README.md` | Project overview | Yes (root only) |
| `openspec/AGENTS.md` | Spec-driven dev instructions | On spec work |
| `.claude/rules/*.md` | Conditional rules | Yes (matching conditions) |

---

## 10. Quick Reference Card

### Starting OpenCode

| Command | What it does |
|---------|--------------|
| `opencode` | Start TUI |
| `opencode -c` | Continue last session |
| `opencode -m github-copilot/claude-opus-4.5` | Use specific model |
| `opencode run "do X"` | Non-interactive run |

### NCP Profiles

| Profile | Command | Use for |
|---------|---------|---------|
| All (39 MCPs) | `ncp` | Everything |
| Coding (31) | `ncp-coding` | Development |
| Research (8) | `ncp-research` | Web search, docs |
| Creative (10) | `ncp-creative` | Design, diagrams |
| Productivity (13) | `ncp-productivity` | Tasks, calendar |

### Beads (Issue Tracking)

| Command | What it does |
|---------|--------------|
| `bd ready` | What can I work on? |
| `bd list --status=open` | All open issues |
| `bd create --title="X" --type=task --priority=2` | Create issue |
| `bd update <id> --status=in_progress` | Start work |
| `bd close <id>` | Done |
| `bd sync` | Sync with git |

### Memory (Memcord)

| Command | What it does |
|---------|--------------|
| `memcord_name "project"` | Create/switch slot |
| `memcord_save "context..."` | Save info |
| `memcord_read` | Load memory |
| `memcord_query "what did we decide?"` | Ask memory |
| `memcord_save_progress` | Auto-summarize |

### Key Slash Commands

| Command | What it does |
|---------|--------------|
| `/agentic/plan` | Create implementation plan |
| `/agentic/execute` | Execute plan |
| `/github/pr-create` | Create PR |
| `/openspec/proposal` | Create spec proposal |

### Agents (via Task tool)

| Agent | Use for |
|-------|---------|
| `explore` | Codebase exploration |
| `librarian` | External docs research |
| `frontend-developer` | UI work |
| `backend-developer` | API work |
| `code-reviewer` | Review changes |
| `debugger` | Complex bugs |

### Skills (auto-trigger or invoke)

| Skill | When |
|-------|------|
| `brainstorming` | Before any creative work |
| `writing-plans` | Before coding |
| `test-driven-development` | Before implementation |
| `systematic-debugging` | Any bug |
| `verification-before-completion` | Before saying "done" |

### Theme Switch

Edit `opencode.json`:
```json
{ "tui": { "theme": "catppuccin-mocha" } }
```

---

*Generated: 2025-12-31*
