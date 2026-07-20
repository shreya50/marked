use crate::ast::{Block, Document, ListItem, TableAlignment};
use crate::inline::{escape, escape_attr, render as inline};
use crate::Options;

pub fn render(document: &Document, options: Options) -> String {
    render_blocks(&document.blocks, options)
}

fn render_blocks(blocks: &[Block], options: Options) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            Block::Heading { level, content } => out.push_str(&format!(
                "<h{level}>{}</h{level}>\n",
                inline(content, options)
            )),
            Block::Paragraph(content) => {
                out.push_str(&format!("<p>{}</p>\n", inline(content, options)))
            }
            Block::CodeFence { language, content } => {
                let class = language
                    .as_ref()
                    .map(|l| format!(" class=\"language-{}\"", escape_attr(l)))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "<pre><code{class}>{}</code></pre>\n",
                    escape(content)
                ));
            }
            Block::List {
                ordered,
                start,
                items,
            } => render_list(&mut out, *ordered, *start, items, options),
            Block::BlockQuote(blocks) => {
                out.push_str("<blockquote>\n");
                out.push_str(&render_blocks(blocks, options));
                out.push_str("</blockquote>\n");
            }
            Block::Table {
                header,
                alignments,
                rows,
            } => render_table(&mut out, header, alignments, rows, options),
            Block::HorizontalRule => out.push_str("<hr />\n"),
        }
    }
    out
}

fn render_list(out: &mut String, ordered: bool, start: u64, items: &[ListItem], options: Options) {
    let tag = if ordered { "ol" } else { "ul" };
    if ordered && start != 1 {
        out.push_str(&format!("<ol start=\"{start}\">\n"));
    } else {
        out.push_str(&format!("<{tag}>\n"));
    }
    for item in items {
        if options.gfm && item.checked.is_some() {
            out.push_str("<li class=\"task-list-item\">");
        } else {
            out.push_str("<li>");
        }
        if let Some(checked) = item.checked {
            out.push_str("<input type=\"checkbox\" disabled");
            if checked {
                out.push_str(" checked");
            }
            out.push_str("> ");
        }
        render_list_item(out, &item.blocks, options);
        out.push_str("</li>\n");
    }
    out.push_str(&format!("</{tag}>\n"));
}

fn render_list_item(out: &mut String, blocks: &[Block], options: Options) {
    for block in blocks {
        match block {
            // Compact list items render their paragraph content without an
            // extra `<p>` wrapper, matching the common Markdown list form.
            Block::Paragraph(content) => out.push_str(&inline(content, options)),
            block => {
                out.push('\n');
                out.push_str(&render_blocks(std::slice::from_ref(block), options));
            }
        }
    }
}

fn render_table(
    out: &mut String,
    header: &[String],
    alignments: &[TableAlignment],
    rows: &[Vec<String>],
    options: Options,
) {
    out.push_str("<table>\n<thead>\n<tr>");
    for (cell, alignment) in header.iter().zip(alignments) {
        out.push_str(&format!(
            "<th{}>{}</th>",
            alignment_attr(*alignment),
            inline(cell, options)
        ));
    }
    out.push_str("</tr>\n</thead>\n<tbody>\n");
    for row in rows {
        out.push_str("<tr>");
        for (cell, alignment) in row.iter().zip(alignments) {
            out.push_str(&format!(
                "<td{}>{}</td>",
                alignment_attr(*alignment),
                inline(cell, options)
            ));
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</tbody>\n</table>\n");
}

fn alignment_attr(alignment: TableAlignment) -> &'static str {
    match alignment {
        TableAlignment::None => "",
        TableAlignment::Left => " align=\"left\"",
        TableAlignment::Center => " align=\"center\"",
        TableAlignment::Right => " align=\"right\"",
    }
}
