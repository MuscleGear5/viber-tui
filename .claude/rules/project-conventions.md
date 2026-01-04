# VIBER-TUI Project Conventions

## CRITICAL RULES

### File Size
- MAX 150 LINES per file
- Split into modules when approaching limit
- Use `mod.rs` for re-exports

### Icons
- NERD FONTS ONLY - no emojis anywhere
- Use unicode escapes: `"\u{F06E0}"` not raw chars
- Icon fields must be `&'static str` not `char`
- Reference: https://www.nerdfonts.com/cheat-sheet

### Code Quality
- Every edit verified with `lsp_diagnostics`
- `cargo check` before commit
- `cargo test` must pass
- No `#[allow(dead_code)]` without justification
- No `unwrap()` in non-test code

### Modularity
- One concern per file
- Flat module hierarchy preferred
- Re-export public API from `mod.rs`

## TOOL USAGE

### Before Any Edit
1. Read target file
2. Plan changes
3. Edit
4. Run `lsp_diagnostics`
5. Run `cargo check`

### Before Commit
1. `cargo check`
2. `cargo test`
3. `cargo clippy` (if available)
4. Review changed files list

### For Research
- Use `explore` agent for codebase questions
- Use `librarian` agent for external docs
- Use `context7` for library documentation
- Use `grep_app` for real-world examples

### For Implementation
- Delegate UI work to `frontend-developer` or `ui-engineer`
- Delegate Rust work to `rust-engineer`
- Use `code-reviewer` after significant changes
