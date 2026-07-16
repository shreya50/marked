use crate::ast::{Block, Document};
use crate::tokenizer::Line;

pub fn parse(lines: Vec<Line<'_>>) -> Document {
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].text;
        if line.trim().is_empty() { i += 1; continue; }

        if let Some((level, content)) = heading(line) {
            blocks.push(Block::Heading { level, content: content.to_owned() });
            i += 1;
        } else if let Some(language) = fence_opening(line) {
            let (content, next) = code_fence(&lines, i + 1);
            blocks.push(Block::CodeFence { language, content });
            i = next;
        } else if is_rule(line) {
            blocks.push(Block::HorizontalRule);
            i += 1;
        } else if let Some((ordered, item)) = list_item(line) {
            let (items, next) = list(&lines, i, ordered, item);
            blocks.push(Block::List { ordered, items });
            i = next;
        } else if quote_content(line).is_some() {
            let (content, next) = quote(&lines, i);
            blocks.push(Block::BlockQuote(content));
            i = next;
        } else {
            let (content, next) = paragraph(&lines, i);
            blocks.push(Block::Paragraph(content));
            i = next;
        }
    }
    Document { blocks }
}

fn heading(line: &str) -> Option<(u8, &str)> {
    let trimmed = line.trim_start();
    let level = trimmed.bytes().take_while(|b| *b == b'#').count();
    if !(1..=6).contains(&level) || !trimmed.as_bytes().get(level).is_some_and(u8::is_ascii_whitespace) { return None; }
    Some((level as u8, trimmed[level..].trim().trim_end_matches('#').trim()))
}

fn fence_opening(line: &str) -> Option<Option<String>> {
    let rest = line.trim_start().strip_prefix("```")?;
    let language = rest.trim();
    Some((!language.is_empty()).then(|| language.to_owned()))
}

fn code_fence(lines: &[Line<'_>], mut i: usize) -> (String, usize) {
    let mut output = String::new();
    while i < lines.len() && !lines[i].text.trim_start().starts_with("```") {
        output.push_str(lines[i].text);
        output.push('\n');
        i += 1;
    }
    (output, if i < lines.len() { i + 1 } else { i })
}

fn is_rule(line: &str) -> bool {
    let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    compact.len() >= 3 && matches!(compact.chars().next(), Some('-' | '*' | '_')) && compact.chars().all(|c| c == compact.chars().next().unwrap())
}

fn list_item(line: &str) -> Option<(bool, &str)> {
    let line = line.trim_start();
    if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")).or_else(|| line.strip_prefix("+ ")) { return Some((false, item)); }
    let digits = line.bytes().take_while(u8::is_ascii_digit).count();
    (digits > 0 && line[digits..].starts_with(". ")).then(|| (true, &line[digits + 2..]))
}

fn list(lines: &[Line<'_>], mut i: usize, ordered: bool, first: &str) -> (Vec<String>, usize) {
    let mut items = vec![first.to_owned()]; i += 1;
    while i < lines.len() {
        match list_item(lines[i].text) {
            Some((kind, item)) if kind == ordered => { items.push(item.to_owned()); i += 1; }
            _ => break,
        }
    }
    (items, i)
}

fn quote_content(line: &str) -> Option<&str> { line.trim_start().strip_prefix('>').map(str::trim_start) }

fn quote(lines: &[Line<'_>], mut i: usize) -> (Vec<String>, usize) {
    let mut content = Vec::new();
    while i < lines.len() { match quote_content(lines[i].text) { Some(text) => { content.push(text.to_owned()); i += 1; }, None => break } }
    (content, i)
}

fn paragraph(lines: &[Line<'_>], mut i: usize) -> (String, usize) {
    let mut content = Vec::new();
    while i < lines.len() {
        let line = lines[i].text;
        if line.trim().is_empty() || fence_opening(line).is_some() || heading(line).is_some() || is_rule(line) || list_item(line).is_some() || quote_content(line).is_some() { break; }
        content.push(line.trim()); i += 1;
    }
    (content.join(" "), i)
}
