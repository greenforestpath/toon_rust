# Agent Guidelines for This Project

## Rule #1: NEVER Delete Files

Without express permission from the user, NEVER delete any files. This includes:
- `rm`, `rm -rf`, `unlink`
- `git reset --hard`
- `git clean -fd`

Ask first. Always.

## Code Editing Discipline

- No script-based mass changes (sed, awk across files)
- No file proliferation (`mainV2.rs`, `main_backup.rs`)
- One canonical version of each file

## The Three Documents

This project follows the Essence Extraction methodology:

1. **PLAN_TO_PORT_X_TO_RUST.md** - Strategy, scope, exclusions
2. **EXISTING_X_STRUCTURE.md** - THE SPEC DOC (consult this, not legacy code)
3. **PROPOSED_ARCHITECTURE.md** - Rust design based on reference projects

## During Implementation

- **Consult ONLY the spec doc, not legacy code**
- Copy patterns from reference projects
- Use `thiserror` for errors, `clap` derive for CLI

## Session Start

Always read this file and the spec documents at session start.

## Tools Available

- `bd` - Beads task tracking
- `cass` - Session search for context recovery
