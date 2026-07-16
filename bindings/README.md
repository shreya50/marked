# Bindings

## Node.js (N-API)

Build the native library with the Node feature enabled:

```bash
cargo build --release --features node
```

The generated `libmarked_rs` dynamic library exposes `parse_markdown(markdown, gfm?)` through N-API. Package it as `marked_rs.node` next to `node/index.js` (or use a N-API packaging tool such as `@napi-rs/cli` for platform-specific prebuilds).

```js
const { parseMarkdown } = require("@marked-rs/node");
const html = parseMarkdown("# Hello", true);
```

## WebAssembly

Install the WebAssembly target and the `wasm-bindgen` CLI once:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

Build and generate browser-friendly JavaScript glue:

```bash
cargo build --release --target wasm32-unknown-unknown --features wasm
wasm-bindgen --target web --out-dir bindings/wasm/pkg target/wasm32-unknown-unknown/release/marked_rs.wasm
```

```js
import init, { parse_markdown } from "./pkg/marked_rs.js";

await init();
const html = parse_markdown("# Hello", true);
```

Both bindings share the same parser. Set the `gfm` argument to `true` to enable tables, task lists, and strikethrough.
