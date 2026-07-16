# marked-rs

A compact Rust Markdown parser built for a weekend migration project. It turns Markdown into safe HTML without external dependencies.

## Supported MVP syntax

- ATX headings, paragraphs, horizontal rules, blockquotes
- Ordered and unordered lists
- Fenced code blocks with optional language classes
- Emphasis, strong emphasis, inline code, links, and images
- HTML escaping by default

## Run

```sh
cargo run -- README.md
printf '# Hello\n\n**fast** Markdown' | cargo run
cargo test
cargo build --release
```

## Architecture

`source → tokenizer → block parser / AST → HTML renderer`

The public Rust API is deliberately simple:

```rust
let html = marked_rs::parse("# Hello");
assert_eq!(html, "<h1>Hello</h1>\n");
```
