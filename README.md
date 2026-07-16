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

Run the built-in throughput benchmark in release mode. The optional argument is the number of parsing iterations (default: `20`).

```bash
cargo run --release --example benchmark -- 30
```

The benchmark repeats a representative 294-byte Markdown sample 5,000 times (a 1.40 MiB input) and measures parsing plus HTML rendering. It uses `std::hint::black_box` so the parse result is not optimized away.

On an Apple Silicon development machine with Rust 1.97.0, Node.js 26.3.1, and Marked 18.0.6, three 30-iteration runs produced these median results:

| Parser | Mean per 1.40 MiB document | Throughput |
| --- | ---: | ---: |
| `marked-rs` (release build) | 31.13 ms | 45.03 MiB/s |
| Marked 18.0.6 | 80.68 ms | 17.38 MiB/s |

For this synthetic corpus, `marked-rs` was approximately **2.6× faster**. This is a useful MVP signal, not a general claim of full-parser superiority: Marked supports substantially more Markdown behavior and edge cases than this project currently implements. Re-run the benchmark on your own representative documents before making a production performance decision.

## Current limitations

The MVP intentionally does not yet support every Markdown dialect or CommonMark edge case. In particular, it does not currently cover:

- Indented code blocks
- Tables, task lists, footnotes, strikethrough, and other GFM extensions
- Reference-style links, autolinks, titles, and URL edge cases
- HTML passthrough
- Full delimiter rules for complex or overlapping emphasis
- Source positions, streaming input, or incremental parsing

Please treat the output as an HTML fragment. Although all source text and generated attributes are escaped by the renderer, consumers should still apply their normal HTML safety practices when inserting output into an application.

## Roadmap

- [x] Broaden the core CommonMark subset and add fixture-based conformance tests
- [x] Add nested block parsing and richer list behavior
- [ ] Add GitHub-Flavored Markdown extensions behind an explicit option
- [ ] Benchmark against JavaScript Markdown parsers on representative files
- [ ] Offer Node.js (N-API) and WebAssembly bindings
- [ ] Support streaming and incremental parsing

## Contributing

Contributions are welcome. A strong pull request includes a focused change, a regression test for behavior changes, `cargo fmt --check`, and `cargo test` output.

## License

MIT. See the `license` field in [Cargo.toml](Cargo.toml).
