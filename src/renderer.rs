use crate::ast::{Block, Document};
use crate::inline::{escape, render as inline};

pub fn render(document: &Document) -> String {
    let mut out = String::new();
    for block in &document.blocks {
        match block {
            Block::Heading { level, content } => out.push_str(&format!("<h{level}>{}</h{level}>\n", inline(content))),
            Block::Paragraph(content) => out.push_str(&format!("<p>{}</p>\n", inline(content))),
            Block::CodeFence { language, content } => {
                let class = language.as_ref().map(|l| format!(" class=\"language-{}\"", escape(l))).unwrap_or_default();
                out.push_str(&format!("<pre><code{class}>{}</code></pre>\n", escape(content)));
            }
            Block::List { ordered, items } => {
                let tag = if *ordered { "ol" } else { "ul" }; out.push_str(&format!("<{tag}>\n"));
                for item in items { out.push_str(&format!("<li>{}</li>\n", inline(item))); }
                out.push_str(&format!("</{tag}>\n"));
            }
            Block::BlockQuote(lines) => out.push_str(&format!("<blockquote>\n<p>{}</p>\n</blockquote>\n", inline(&lines.join(" ")))),
            Block::HorizontalRule => out.push_str("<hr />\n"),
        }
    }
    out
}
