# VIBER-TUI Session Init

## Quick Start (RUN FIRST)

```bash
readroadmap                    # Load project state
bd ready                       # Check available work  
cat roadmap/roadmap.md         # Quick status overview
```

## Hard Rules (ENFORCED)

| Rule | Violation | Fix |
|------|-----------|-----|
| MAX 150 LINES | Any file > 150 lines | Split into modules |
| NO EMOJIS | Any emoji character | Use nerd font `\u{XXXXX}` |
| ICONS AS STR | `icon: char` | Must be `icon: &'static str` |
| LSP CLEAN | Unverified edits | Run `lsp_diagnostics` after edit |
| TESTS PASS | `cargo test` fails | Fix before commit |

## Beads Issues (Persistent Tracking)

| ID | Feature | Priority |
|----|---------|----------|
| viber-tui-x9t | F1: Core Infrastructure | P1 |
| viber-tui-3ef | F2: Primary Views | P1 |
| viber-tui-a60 | F3: Editor Integration | P2 |
| viber-tui-yos | F4: Workflow Phases | P2 |
| viber-tui-f6v | F5: VIBER God Agent | P2 |
| viber-tui-g0v | F6: Advanced Widgets | P3 |
| viber-tui-20z | F7: Integration/Polish | P3 |

## Workflow

1. `bd update <id> --status=in_progress` - claim work
2. `updateroadmap actionNumber="X.XX" status="in_progress"` - track
3. Implement (max 150 lines, no emojis, LSP clean)
4. `cargo check && cargo test`
5. `lsp_diagnostics` on changed files
6. `updateroadmap actionNumber="X.XX" status="completed"`
7. `bd close <id>` when feature done

## Tool Chain

| Phase | Tools |
|-------|-------|
| Research | context7, grep_app, explore agent |
| Implement | rust-engineer subagent, lsp_* tools |
| Verify | lsp_diagnostics, cargo check/test |
| Track | roadmap tools, bd CLI, todowrite |

## Nerd Font Icons (USE THESE)

```rust
pub const ICON_CHECK: &str = "\u{F00C0}";
pub const ICON_ERROR: &str = "\u{F0159}";
pub const ICON_WARN: &str = "\u{F0028}";
pub const ICON_INFO: &str = "\u{F064E}";
pub const ICON_GEAR: &str = "\u{F013}";
pub const ICON_CODE: &str = "\u{F121}";
```

Reference: https://www.nerdfonts.com/cheat-sheet
