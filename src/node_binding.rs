use napi_derive::napi;

use crate::{parse_with_options, Options};

/// Parses Markdown from Node.js. Pass `true` as `gfm` to enable the supported
/// GitHub-Flavored Markdown subset.
#[napi]
pub fn parse_markdown(markdown: String, gfm: Option<bool>) -> String {
    parse_with_options(
        &markdown,
        Options {
            gfm: gfm.unwrap_or(false),
        },
    )
}
