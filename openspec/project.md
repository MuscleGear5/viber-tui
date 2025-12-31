# Project Context

## Purpose
Viber TUI is a cyberpunk-themed terminal user interface for "vibe-coding" - a launcher that provides quick access to MCP tools, AI agents, skills, and commands. It serves as a central hub for AI-assisted development workflows.

## Tech Stack
- **Language**: Rust (2021 edition)
- **TUI Framework**: ratatui 0.29 with crossterm 0.28 backend
- **Async Runtime**: tokio (full features)
- **Data Format**: YAML (serde_yaml) for action definitions
- **Fuzzy Matching**: nucleo 0.5
- **Error Handling**: anyhow

## Project Conventions

### Code Style
- Follow standard Rust conventions (rustfmt defaults)
- Use `clippy` for linting
- Prefer `anyhow::Result` for error propagation in application code
- Use descriptive variable names, avoid single-letter names except for iterators
- Module structure: separate concerns into `data/`, `theme/`, `views/`, `widgets/`

### Architecture Patterns
- **Module Organization**:
  - `data/` - Data structures and loading (Action, ActionRegistry)
  - `theme/` - Visual theming (colors, styles, animations)
  - `views/` - Full-screen views (Launcher)
  - `widgets/` - Reusable UI components (ActionCard, FuzzyList)
- **State Management**: Views own their state structs (e.g., `LauncherState`)
- **Rendering**: Use ratatui's `StatefulWidget` trait for stateful components
- **Event Loop**: Single-threaded event loop with tick-based animation

### Widget Patterns
- Widgets are stateless renderers; state lives in companion `*State` structs
- Use `impl StatefulWidget` for widgets that need external state
- Animation state is centralized in `AnimationState`

### Data Loading
- Actions defined in `data/actions.yaml`
- YAML schema: id, name, category, subcategory, description, keywords, invocation, params
- Categories: mcp, agent, skill, command

### Testing Strategy
- Unit tests for data parsing and fuzzy matching logic
- Integration tests for TUI rendering (if added)
- Manual testing for visual appearance and interactions

### Git Workflow
- Use conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`
- Keep commits atomic and focused
- Use beads (`bd`) for issue tracking

## Domain Context
- **MCP**: Model Context Protocol - tools that AI assistants can invoke
- **Agents**: Specialized AI workers for specific domains (coding, research, etc.)
- **Skills**: Knowledge and workflow guides loaded by AI assistants
- **Commands**: Slash commands that trigger specific behaviors

## Important Constraints
- Must remain responsive (tick rate: 50ms for animations)
- Terminal compatibility: support standard 256-color terminals
- Keyboard-only navigation (no mouse required, though mouse enabled)

## External Dependencies
- Actions are defined externally in YAML, not hardcoded
- Executes shell commands via `sh -c` for Command category
- Integrates with OpenCode/Claude Code ecosystem
