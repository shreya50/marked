# Release notes: marked-rs

`marked-rs` turns **Markdown** into safe HTML. It supports [links](https://example.com),
![images](logo.svg), and `inline code` without external runtime dependencies.

> Keep the parser small, then grow it through fixtures.
>
> - Validate output
> - Measure performance

## Checklist

- [x] Core Markdown
- [x] Nested lists
- [x] GFM tables

| Parser | Language | Status |
| :-- | :--: | --: |
| marked-rs | Rust | ~~prototype~~ ready |
| Marked | JavaScript | mature |

```rust
let html = marked_rs::parse("# Hello");
assert_eq!(html, "<h1>Hello</h1>\n");
```

---

1. Build a release binary.
2. Compare representative Markdown documents.
