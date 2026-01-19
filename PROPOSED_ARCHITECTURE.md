# Proposed Architecture for toon_rust (Rust)

Based on patterns from reference projects.

## Module Structure

```
src/
├── main.rs           # Entry point
├── lib.rs            # Library root
├── error.rs          # Custom errors
├── config.rs         # Configuration
├── model/
│   ├── mod.rs
│   └── [entity].rs
├── storage/
│   ├── mod.rs
│   └── sqlite.rs
├── cli/
│   ├── mod.rs
│   └── commands/
└── format/
    ├── mod.rs
    ├── text.rs
    └── json.rs
```

## Error Handling

[From reference projects]

## CLI Pattern

[Clap derive structure]

## Storage Pattern

[SQLite with rusqlite]

## Testing Strategy

[Unit, integration, golden tests]
