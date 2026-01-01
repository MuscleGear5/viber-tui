<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# VIBER-TUI Hard Rules (ENFORCED)

## BLOCKING VIOLATIONS - Code will be rejected if:

| Rule | Violation | Fix |
|------|-----------|-----|
| MAX 150 LINES | Any file > 150 lines | Split into modules |
| NO EMOJIS | Any emoji character | Use nerd font unicode `\u{XXXXX}` |
| ICONS AS STR | `icon: char` | Must be `icon: &'static str` |
| LSP CLEAN | Unverified edits | Run `lsp_diagnostics` after every edit |
| TESTS PASS | `cargo test` fails | Fix before commit |
| NO UNWRAP | `.unwrap()` in non-test | Use `?` or `expect("reason")` |

## Session Init (MANDATORY)

```bash
readroadmap                    # Load project state
bd ready                       # Check available work  
cat roadmap/roadmap.md         # Quick status overview
```

## Tool Chain (USE THESE)

| Phase | Tools |
|-------|-------|
| Research | `context7`, `grep_app`, `explore` agent, `librarian` agent |
| Implement | `rust-engineer` subagent, `lsp_*` tools, `ast_grep` |
| Verify | `lsp_diagnostics`, `cargo check`, `cargo test` |
| Review | `code-reviewer` subagent |
| Track | `roadmap` tools, `bd` CLI, `todowrite` |

## Nerd Font Icons (REQUIRED)

```rust
pub const ICON_CHECK: &str = "\u{F00C0}";    // nf-md-check
pub const ICON_ERROR: &str = "\u{F0159}";    // nf-md-close
pub const ICON_WARN: &str = "\u{F0028}";     // nf-md-alert
pub const ICON_INFO: &str = "\u{F064E}";     // nf-md-information
pub const ICON_GEAR: &str = "\u{F013}";      // nf-fa-gear
pub const ICON_CODE: &str = "\u{F121}";      // nf-fa-code
```

Reference: https://www.nerdfonts.com/cheat-sheet

# Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

