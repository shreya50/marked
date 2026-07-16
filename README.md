# marked-rs

`marked-rs` is a small, dependency-free Markdown-to-HTML parser written in Rust. It is a hackathon-friendly exploration of moving the core parsing pipeline behind a JavaScript-style Markdown renderer into Rust: compact enough to understand in an afternoon, structured enough to extend.

It accepts Markdown text and produces an escaped HTML fragment.

```text
Markdown source → tokenizer → block parser / AST → HTML renderer
```

## Why this project?

Markdown parsing is a useful migration target: it has well-defined inputs and outputs, a large potential test corpus, and a performance-sensitive core. `marked-rs` keeps that core deliberately small while demonstrating a Rust-first architecture that can later support a native CLI, Node bindings, or WebAssembly.

This repository is an MVP, not a complete implementation of [Marked](https://marked.js.org/) or the full CommonMark specification.

## Features

- No runtime dependencies
- Safe-by-default HTML escaping
- Reusable library API and command-line interface
- Inspectable block-level AST for custom renderers
- Fenced code blocks with language classes
- Optional GitHub-Flavored Markdown extensions
- Unit and integration coverage for common and edge-case inputs

## Supported Markdown

| Construct | Example | HTML output |
| --- | --- | --- |
| Headings | `## Hello` or `Hello\n---` | `<h2>Hello</h2>` |
| Paragraphs | `A paragraph` | `<p>A paragraph</p>` |
| Emphasis | `*text*`, `_text_` | `<em>text</em>` |
| Strong text | `**text**`, `__text__` | `<strong>text</strong>` |
| Inline code | `` `let x = 1` `` | `<code>let x = 1</code>` |
| Autolinks | `<https://example.com>` | `<a href="https://example.com">…</a>` |
| Escaped punctuation | `\*literal\*` | `*literal*` |
| Hard breaks | `first··\nsecond` | `first<br />second` |
| Links | `[docs](https://example.com)` | `<a href="https://example.com">docs</a>` |
| Images | `![alt](image.png)` | `<img src="image.png" alt="alt" />` |
| Unordered lists | `- first` | `<ul>…</ul>` |
| Ordered lists | `1. first` | `<ol>…</ol>` |
| Blockquotes | `> quoted` | `<blockquote>…</blockquote>` |
| Horizontal rules | `---`, `***`, `___` | `<hr />` |
| Fenced code | <code>```rust</code> or `~~~rust` | `<pre><code class="language-rust">…` |

Raw HTML is escaped rather than passed through. Text in code blocks and inline code is also rendered literally.

## Install and run

### Prerequisites

Install the stable [Rust toolchain](https://www.rust-lang.org/tools/install), which includes Cargo.

### From a checkout

```bash
git clone https://github.com/shreya50/marked.git
cd marked
cargo build --release
```

The release executable is available at `target/release/marked-rs`.

### Command-line interface

Pass a Markdown file as the first argument:

```bash
cargo run -- README.md
```

Or pipe Markdown through standard input:

```bash
printf '# Hello, Rust!\n\n**Fast** Markdown.\n' | cargo run
```

Example:

```console
$ printf '[Rust](https://www.rust-lang.org/)\n' | cargo run --quiet
<p><a href="https://www.rust-lang.org/">Rust</a></p>
```

## Library usage

Add `marked-rs` to your project when it is published to crates.io, or use a path dependency during development:

```toml
[dependencies]
marked-rs = { path = "../marked" }
```

Render a Markdown string directly:

```rust
let html = marked_rs::parse("# Hello\n\nA **small** parser.");

assert_eq!(
    html,
    "<h1>Hello</h1>\n<p>A <strong>small</strong> parser.</p>\n"
);
```

For custom rendering or inspection, parse into the public document model first:

```rust
use marked_rs::{parse_document, Block};

let document = parse_document("## Changelog\n\n- Added parser");
assert!(matches!(document.blocks[0], Block::Heading { level: 2, .. }));
```

### GitHub-Flavored Markdown (opt-in)

The core parser does not enable extensions by default. Enable the supported GFM subset when you need tables, task-list checkboxes, and strikethrough:

```rust
use marked_rs::{parse_with_options, Options};

let html = parse_with_options(
    "- [x] Ship `marked-rs`\n\nThis is ~~done~~.",
    Options { gfm: true },
);
```

GFM parsing is covered by fixtures in `tests/fixtures/gfm`. This is an intentionally focused subset; footnotes, alerts, and autolink literals remain future work.

### Node.js and WebAssembly bindings

Feature-gated bindings expose the same parser outside Rust:

```bash
# N-API native library
cargo build --release --features node

# Browser/WebAssembly artifact
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown --features wasm
```

The bindings export `parseMarkdown(markdown, gfm?)` for Node.js and `parse_markdown(markdown, gfm)` for WebAssembly. See [bindings/README.md](bindings/README.md) for packaging and browser glue-generation steps.

## Architecture

The project separates the pipeline into small modules so that individual layers can evolve without entangling parsing and rendering concerns.

| Module | Responsibility |
| --- | --- |
| `src/tokenizer.rs` | Preserves source as lightweight lines for block parsing. |
| `src/parser.rs` | Recognizes block structures and builds a `Document`. |
| `src/ast.rs` | Defines the public `Document` and `Block` types. |
| `src/inline.rs` | Renders and escapes inline constructs. |
| `src/renderer.rs` | Converts the AST into an HTML fragment. |
| `src/main.rs` | Provides the file/stdin command-line interface. |

## Development

Run the full test suite:

```bash
cargo test
```

Check formatting before opening a pull request:

```bash
cargo fmt --check
```

Build an optimized binary:

```bash
cargo build --release
```

The test suite includes focused unit tests and integration tests for parsing boundaries, escaping, list variants, code fences, and public AST access. Core CommonMark-style behavior is also covered by Markdown/HTML fixture pairs in `tests/fixtures/commonmark-core`; add a matching `.md` and `.html` file there to create a new conformance case.

## Benchmarking

The repository includes a mixed-feature corpus—headings, quotes, lists, code, tables, task markers, and strikethrough—and matching Rust and JavaScript runners. Both runners concatenate the same corpus, repeat it to a controlled size, warm up their parser, and report mean time and throughput.

```bash
# Rust parser, with GFM enabled for parity with the corpus
cargo run --release --example benchmark -- 30 2000 benchmarks/corpus/mixed-features.md

# JavaScript parsers
cd benchmarks
npm install
node compare.mjs 30 2000 corpus/mixed-features.md
```

On an Apple Silicon development machine with Rust 1.97.0 and Node.js 26.3.1, three 30-iteration runs over the same 1.27 MiB input produced these median results:

| Parser | Mean per document | Throughput |
| --- | ---: | ---: |
| `marked-rs` (release, GFM enabled) | 39.18 ms | 32.52 MiB/s |
| markdown-it 14.1.0 | 58.60 ms | 21.74 MiB/s |
| Marked 18.0.6 | 68.11 ms | 18.71 MiB/s |

For this representative corpus, `marked-rs` was approximately **1.5× faster than markdown-it** and **1.7× faster than Marked**. This is a useful MVP signal, not a general claim of full-parser superiority: the JavaScript parsers support more Markdown behavior and edge cases. Re-run the corpus alongside your own representative documents before making a production performance decision.

## Current limitations

The MVP intentionally does not yet support every Markdown dialect or CommonMark edge case. In particular, it does not currently cover:

- Indented code blocks
- Footnotes, alerts, autolink literals, and other GFM extensions
- Reference-style links, autolinks, titles, and URL edge cases
- HTML passthrough
- Full delimiter rules for complex or overlapping emphasis
- Source positions, streaming input, or incremental parsing

Please treat the output as an HTML fragment. Although all source text and generated attributes are escaped by the renderer, consumers should still apply their normal HTML safety practices when inserting output into an application.

## Roadmap

- [x] Broaden the core CommonMark subset and add fixture-based conformance tests
- [x] Add nested block parsing and richer list behavior
- [x] Add GitHub-Flavored Markdown extensions behind an explicit option
- [x] Benchmark against JavaScript Markdown parsers on representative files
- [x] Offer Node.js (N-API) and WebAssembly bindings
- [ ] Support streaming and incremental parsing

## Contributing

Contributions are welcome. A strong pull request includes a focused change, a regression test for behavior changes, `cargo fmt --check`, and `cargo test` output.

## License

MIT. See the `license` field in [Cargo.toml](Cargo.toml).
