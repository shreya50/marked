use crate::ast::{Block, Document, ListItem};
use crate::inline::{escape, escape_attr, render as inline};

pub fn render(document: &Document) -> String { render_blocks(&document.blocks) }

fn render_blocks(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            Block::Heading { level, content } => out.push_str(&format!("<h{level}>{}</h{level}>\n", inline(content))),
            Block::Paragraph(content) => out.push_str(&format!("<p>{}</p>\n", inline(content))),
            Block::CodeFence { language, content } => {
                let class = language.as_ref().map(|l| format!(" class=\"language-{}\"", escape_attr(l))).unwrap_or_default();
                out.push_str(&format!("<pre><code{class}>{}</code></pre>\n", escape(content)));
            }
            Block::List { ordered, start, items } => render_list(&mut out, *ordered, *start, items),
            Block::BlockQuote(blocks) => {
                out.push_str("<blockquote>\n");
                out.push_str(&render_blocks(blocks));
                out.push_str("</blockquote>\n");
            }
            Block::HorizontalRule => out.push_str("<hr />\n"),
        }
    }
    out
}

fn render_list(out: &mut String, ordered: bool, start: u64, items: &[ListItem]) {
    let tag = if ordered { "ol" } else { "ul" };
    if ordered && start != 1 { out.push_str(&format!("<ol start=\"{start}\">\n")); }
    else { out.push_str(&format!("<{tag}>\n")); }
    for item in items {
        out.push_str("<li>");
        render_list_item(out, &item.blocks);
        out.push_str("</li>\n");
    }
    out.push_str(&format!("</{tag}>\n"));
}

fn render_list_item(out: &mut String, blocks: &[Block]) {
    for block in blocks {
        match block {
            // Compact list items render their paragraph content without an
            // extra `<p>` wrapper, matching the common Markdown list form.
            Block::Paragraph(content) => out.push_str(&inline(content)),
            block => {
                out.push('\n');
                out.push_str(&render_blocks(std::slice::from_ref(block)));
            }
        }
    }
}
