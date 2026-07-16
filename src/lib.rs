//! `marked-rs` is a compact Markdown-to-HTML parser with a deliberately small,
//! hackathon-friendly surface area. It supports the most common Markdown blocks
//! and inline constructs without external dependencies.

mod ast;
mod inline;
mod parser;
mod renderer;
mod tokenizer;

pub use ast::{Block, Document, ListItem};

/// Converts Markdown source to HTML.
pub fn parse(markdown: &str) -> String {
    let document = parse_document(markdown);
    renderer::render(&document)
}

/// Parses Markdown into the block-level document representation.
///
/// This is useful when a caller wants to inspect the parser output before it is
/// rendered, or add a custom renderer later.
pub fn parse_document(markdown: &str) -> Document {
    let lines = tokenizer::tokenize(markdown);
    parser::parse(lines)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn renders_headings_and_paragraphs() {
        assert_eq!(parse("# Hello\n\nA small paragraph."), "<h1>Hello</h1>\n<p>A small paragraph.</p>\n");
    }

    #[test]
    fn renders_inline_markdown() {
        assert_eq!(
            parse("*em* and **strong** with [a link](https://example.com), ![an image](cat.png), and `code`."),
            "<p><em>em</em> and <strong>strong</strong> with <a href=\"https://example.com\">a link</a>, <img src=\"cat.png\" alt=\"an image\" />, and <code>code</code>.</p>\n"
        );
    }

    #[test]
    fn renders_fenced_code_without_parsing_contents() {
        assert_eq!(parse("```rust\nlet x = 1 < 2;\n```"), "<pre><code class=\"language-rust\">let x = 1 &lt; 2;\n</code></pre>\n");
    }

    #[test]
    fn renders_lists_quotes_and_rules() {
        assert_eq!(
            parse("- one\n- two\n\n1. first\n2. second\n\n> wise words\n> continued\n\n---"),
            "<ul>\n<li>one</li>\n<li>two</li>\n</ul>\n<ol>\n<li>first</li>\n<li>second</li>\n</ol>\n<blockquote>\n<p>wise words\ncontinued</p>\n</blockquote>\n<hr />\n"
        );
    }

    #[test]
    fn escapes_raw_html() {
        assert_eq!(parse("<script>alert('no')</script>"), "<p>&lt;script&gt;alert('no')&lt;/script&gt;</p>\n");
    }
}
