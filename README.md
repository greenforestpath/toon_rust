# tr - TOON Rust

<div align="center">

```
  _______  ____   ____  _   _
 |__   __|/ __ \ / __ \| \ | |
    | |  | |  | | |  | |  \| |
    | |  | |  | | |  | | . ` |
    | |  | |__| | |__| | |\  |
    |_|   \____/ \____/|_| \_|
```

[![CI](https://github.com/Dicklesworthstone/toon_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Dicklesworthstone/toon_rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![TOON Spec](https://img.shields.io/badge/spec-v3.0-fef3c0)](https://github.com/toon-format/spec)

</div>

**TOON Rust** is a spec-first Rust port of the TOON reference implementation. It turns JSON <-> TOON with deterministic output, strict validation, and token-efficiency options (delimiter + key folding).

**Status:** Core encode/decode is implemented and spec-fixture tested. CLI wiring is in progress.

Quick links: [TL;DR](#tldr) | [Quick Example](#quick-example) | [Installation](#installation) | [Command Reference](#command-reference) | [Architecture](#architecture)

---

## TL;DR

### The Problem
The official TOON implementation is TypeScript/JavaScript. If you want a small, fast, native binary that runs without Node and supports streaming and strict validation, you need a Rust port that matches the spec exactly.

### The Solution
`tr` is a spec-first Rust implementation with streaming decode, deterministic output, and TOON-specific optimizations (delimiters, key folding, path expansion).

### Why Use `tr`?

| Feature | Why it matters |
| --- | --- |
| Spec-first parity | Matches the reference behavior, not a re-interpretation |
| Streaming decode | Process large TOON inputs without full buffering |
| Deterministic output | Stable diffs, reproducible pipelines |
| Token efficiency | Delimiters + key folding to minimize tokens |
| Native binary | No runtime dependency on Node |

---

## Quick Example

Encode and decode using the library (works today):

```bash
# 1) Add the dependency
cargo add toon_rust

# 2) Create a tiny program
cat > /tmp/toon_demo.rs <<'RS'
use toon_rust::{encode, decode};

fn main() {
    let json = r#"{\"user\":{\"id\":1,\"name\":\"Ada\"},\"tags\":[\"a\",\"b\"]}"#;
    let toon = encode(serde_json::from_str::<serde_json::Value>(json).unwrap(), None);
    println!("{toon}");

    let back = decode(&toon, None);
    println!("{:?}", back);
}
RS

# 3) Run it
rustc /tmp/toon_demo.rs -o /tmp/toon_demo && /tmp/toon_demo
```

CLI usage is the target behavior and will mirror the reference tool (see [Command Reference](#command-reference)).

---

## Design Philosophy

1. **Spec-first**: The spec doc is the source of truth, not translations.
2. **Streaming by default**: Encode and decode are designed for large inputs.
3. **Deterministic output**: Identical inputs produce identical TOON.
4. **Minimal dependencies**: Keep binaries small and fast.

---

## Comparison

| Tool | Runtime | Streaming | Spec fidelity | Notes |
| --- | --- | --- | --- | --- |
| `tr` (this repo) | Native | Yes | Target: full parity | Rust port of reference |
| `toon` (reference, TS) | Node | Yes | Yes | Canonical behavior |
| `jq` + custom format | Native | Partial | No | Not TOON-compatible |

---

## Installation

### Prebuilt binaries (once releases exist)
When releases are published, you will be able to download a prebuilt binary directly:

```bash
curl -fsSL https://github.com/Dicklesworthstone/toon_rust/releases/latest/download/tr-linux-amd64.tar.xz -o tr.tar.xz
tar -xf tr.tar.xz
./tr --help
```

### Cargo (from source)
```bash
cargo install --git https://github.com/Dicklesworthstone/toon_rust --locked
```

### Build locally
```bash
git clone https://github.com/Dicklesworthstone/toon_rust
cd toon_rust
cargo build --release
./target/release/tr --help
```

---

## Quick Start

1. Build the binary:
   ```bash
   cargo build --release
   ```
2. Encode:
   ```bash
   cat input.json | ./target/release/tr --encode
   ```
3. Decode:
   ```bash
   cat data.toon | ./target/release/tr --decode
   ```

Note: CLI wiring is in progress; library APIs are production-ready for encode/decode.

---

## Command Reference

Target CLI (matches the reference tool):

```bash
tr [options] [input]
```

Auto-detection:
- `.json` -> encode
- `.toon` -> decode
- stdin defaults to encode unless `--decode` is provided

Common flags:
- `-o, --output <file>`
- `-e, --encode`
- `-d, --decode`
- `--delimiter <,|\\t|\\|>`
- `--indent <n>`
- `--no-strict`
- `--keyFolding <off|safe>`
- `--flattenDepth <n>`
- `--expandPaths <off|safe>`
- `--stats` (encode only)

---

## Configuration

There is no config file. All configuration is via CLI flags or library options:

```rust
use toon_rust::options::{EncodeOptions, KeyFoldingMode};

let options = EncodeOptions {
    indent: Some(2),
    delimiter: Some(','),
    key_folding: Some(KeyFoldingMode::Safe),
    flatten_depth: Some(usize::MAX),
    replacer: None,
};
```

---

## Architecture

```
           +--------------------+
           |    CLI (tr)        |
           |  args + IO + stats |
           +---------+----------+
                     |
                     v
 +-------------------+-------------------+
 |          Core Library                 |
 |  encode: normalize -> folding -> emit |
 |  decode: scan -> parse -> events      |
 +-------------------+-------------------+
                     |
                     v
          +----------+-----------+
          |  Shared Utilities    |
          |  escaping + validation|
          +----------------------+
```

---

## Troubleshooting

Common errors and fixes:

1. **Failed to parse JSON**  
   Ensure your input is valid JSON. Use `jq .` to validate before encoding.

2. **Tabs are not allowed in indentation**  
   Strict mode forbids tabs. Replace leading tabs with spaces or use `--no-strict`.

3. **Blank lines inside list/tabular arrays**  
   Strict mode disallows blank lines inside array blocks.

4. **Expected N list array items, but found more**  
   Declared array length in header must match items in strict mode.

5. **Path expansion conflict**  
   When expanding dotted keys, conflicts throw in strict mode. Use `--no-strict` or fix the input.

---

## Limitations

- CLI wiring is still in progress (library encode/decode is complete).
- `encode_stream_events` is not implemented yet.
- Release binaries are not published yet (see Installation section).

---

## FAQ

**Q: Is this a new format?**  
A: No. This is a spec-first port of TOON.

**Q: Does it match the reference implementation?**  
A: Yes for core encode/decode behavior; spec fixtures are tested. CLI parity is in progress.

**Q: Does it stream?**  
A: Decode uses event streaming internally; CLI streaming output is implemented.

**Q: Why nightly Rust?**  
A: The project targets Rust 2024 with strict linting and nightly toolchain components.

**Q: Can I use this as a library?**  
A: Yes. The `encode` and `decode` APIs are stable and spec-driven.

---

## About Contributions

*About Contributions:* Please don't take this the wrong way, but I do not accept outside contributions for any of my projects. I simply don't have the mental bandwidth to review anything, and it's my name on the thing, so I'm responsible for any problems it causes; thus, the risk-reward is highly asymmetric from my perspective. I'd also have to worry about other "stakeholders," which seems unwise for tools I mostly make for myself for free. Feel free to submit issues, and even PRs if you want to illustrate a proposed fix, but know I won't merge them directly. Instead, I'll have Claude or Codex review submissions via `gh` and independently decide whether and how to address them. Bug reports in particular are welcome. Sorry if this offends, but I want to avoid wasted time and hurt feelings. I understand this isn't in sync with the prevailing open-source ethos that seeks community contributions, but it's the only way I can move at this velocity and keep my sanity.

---

## License

MIT. See [LICENSE](LICENSE).
