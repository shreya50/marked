use marked_rs::{parse, parse_document, Block, Document};

#[test]
fn escapes_attribute_values_and_preserves_inline_markup_in_link_labels() {
    assert_eq!(
        parse("[*docs*](https://example.com/?a=1&b=2\"x)"),
        "<p><a href=\"https://example.com/?a=1&amp;b=2&quot;x\"><em>docs</em></a></p>\n"
    );
}

#[test]
fn supports_all_unordered_list_markers_and_multi_digit_ordered_lists() {
    assert_eq!(
        parse("* first\n+ second\n- third\n\n10. tenth\n11. eleventh"),
        "<ul>\n<li>first</li>\n<li>second</li>\n<li>third</li>\n</ul>\n<ol>\n<li>tenth</li>\n<li>eleventh</li>\n</ol>\n"
    );
}

#[test]
fn starts_a_new_block_when_a_paragraph_is_followed_by_markdown_syntax() {
    assert_eq!(
        parse("Intro text\n# A heading\n> A quote\n---\nAfterward"),
        "<p>Intro text</p>\n<h1>A heading</h1>\n<blockquote>\n<p>A quote</p>\n</blockquote>\n<hr />\n<p>Afterward</p>\n"
    );
}

#[test]
fn renders_tilde_free_code_fence_content_as_literal_text() {
    assert_eq!(
        parse("```html\n<strong>*literal*</strong>\n```"),
        "<pre><code class=\"language-html\">&lt;strong&gt;*literal*&lt;/strong&gt;\n</code></pre>\n"
    );
}

#[test]
fn keeps_unclosed_code_fences_as_code_until_end_of_input() {
    assert_eq!(
        parse("```\nnot *emphasis*\nand <tag>"),
        "<pre><code>not *emphasis*\nand &lt;tag&gt;\n</code></pre>\n"
    );
}

#[test]
fn exposes_a_structured_document_for_custom_renderers() {
    assert_eq!(
        parse_document("## Title\n\nBody"),
        Document {
            blocks: vec![
                Block::Heading {
                    level: 2,
                    content: "Title".to_owned(),
                },
                Block::Paragraph("Body".to_owned()),
            ],
        }
    );
}
