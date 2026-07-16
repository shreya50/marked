use wasm_bindgen::prelude::wasm_bindgen;

use crate::{parse_with_options, Options};

/// Parses Markdown in browsers and other WebAssembly hosts.
#[wasm_bindgen]
pub fn parse_markdown(markdown: &str, gfm: bool) -> String {
    parse_with_options(markdown, Options { gfm })
}
