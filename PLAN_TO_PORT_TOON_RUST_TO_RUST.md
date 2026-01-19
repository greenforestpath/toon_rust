# Plan: Port toon_rust to Rust

## Executive Summary

[What we're porting and why]

## Why Port to Rust?

1. Performance benefits
2. Binary distribution (no runtime dependencies)
3. Ecosystem integration
4. Bug fix opportunity

## What We're Porting

- [ ] Core feature 1
- [ ] Core feature 2
- [ ] Core feature 3

## What We're NOT Porting

| Feature | Reason |
|---------|--------|
| Feature X | [Why excluded] |
| Feature Y | [Why excluded] |

## Reference Projects

| Project | Path | What to Copy |
|---------|------|--------------|
| dcg | /data/projects/destructive_command_guard | Clap, errors |
| cass | /data/projects/coding_agent_session_search | SQLite patterns |

## Implementation Phases

### Phase 1: Foundation
- Cargo.toml, errors, config

### Phase 2: Core
- Data models, storage

### Phase 3: CLI
- Commands, output formatting

### Phase 4: Polish
- Tests, docs, optimization

## Success Criteria

- [ ] All core features working
- [ ] Tests passing
- [ ] Binary size < X MB
- [ ] Startup time < X ms
