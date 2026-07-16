use crate::ast::{Block, Document, ListItem, TableAlignment};
use crate::tokenizer::{tokenize, Line};
use crate::Options;

pub fn parse(lines: Vec<Line<'_>>, options: Options) -> Document {
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].text;
        if line.trim().is_empty() { i += 1; continue; }

        if let Some((level, content)) = heading(line) {
            blocks.push(Block::Heading { level, content: content.to_owned() });
            i += 1;
        } else if options.gfm && table(&lines, i).is_some() {
            let (table, next) = table(&lines, i).expect("table was checked above");
            blocks.push(table);
            i = next;
        } else if let Some((level, content)) = setext_heading(&lines, i) {
            blocks.push(Block::Heading { level, content: content.to_owned() });
            i += 2;
        } else if let Some(fence) = fence_opening(line) {
            let (content, next) = code_fence(&lines, i + 1, fence.marker, fence.length);
            blocks.push(Block::CodeFence { language: fence.language, content });
            i = next;
        } else if is_rule(line) {
            blocks.push(Block::HorizontalRule);
            i += 1;
        } else if let Some(marker) = list_marker(line) {
            let (list, next) = list(&lines, i, marker, options);
            blocks.push(list);
            i = next;
        } else if quote_content(line).is_some() {
            let (content, next) = quote(&lines, i, options);
            blocks.push(Block::BlockQuote(content));
            i = next;
        } else {
            let (content, next) = paragraph(&lines, i, options);
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

fn setext_heading<'a>(lines: &[Line<'a>], index: usize) -> Option<(u8, &'a str)> {
    let content = lines.get(index)?.text.trim();
    let underline = lines.get(index + 1)?.text.trim();
    if content.is_empty() { return None; }
    if heading(content).is_some() || fence_opening(content).is_some() || is_rule(content)
        || list_marker(content).is_some() || quote_content(content).is_some() { return None; }
    let marker = underline.chars().next()?;
    if !matches!(marker, '=' | '-') || !underline.chars().all(|ch| ch == marker) { return None; }
    Some((if marker == '=' { 1 } else { 2 }, content))
}

struct Fence { marker: char, length: usize, language: Option<String> }

fn fence_opening(line: &str) -> Option<Fence> {
    let trimmed = line.trim_start();
    let marker = trimmed.chars().next()?;
    if !matches!(marker, '`' | '~') { return None; }
    let length = trimmed.chars().take_while(|ch| *ch == marker).count();
    if length < 3 { return None; }
    let language = trimmed[length..].trim();
    Some(Fence { marker, length, language: (!language.is_empty()).then(|| language.to_owned()) })
}

fn code_fence(lines: &[Line<'_>], mut i: usize, marker: char, length: usize) -> (String, usize) {
    let mut output = String::new();
    while i < lines.len() && !is_fence_closing(lines[i].text, marker, length) {
        output.push_str(lines[i].text);
        output.push('\n');
        i += 1;
    }
    (output, if i < lines.len() { i + 1 } else { i })
}

fn is_fence_closing(line: &str, marker: char, length: usize) -> bool {
    let trimmed = line.trim_start();
    trimmed.chars().take_while(|ch| *ch == marker).count() >= length
        && trimmed.chars().skip_while(|ch| *ch == marker).all(char::is_whitespace)
}

fn is_rule(line: &str) -> bool {
    let compact: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    compact.len() >= 3 && matches!(compact.chars().next(), Some('-' | '*' | '_')) && compact.chars().all(|c| c == compact.chars().next().unwrap())
}

struct ListMarker<'a> { ordered: bool, start: u64, indent: usize, content: &'a str }

fn list_marker(line: &str) -> Option<ListMarker<'_>> {
    let indent = line.len() - line.trim_start().len();
    let content = &line[indent..];
    for prefix in ["- ", "* ", "+ "] {
        if let Some(item) = content.strip_prefix(prefix) {
            return Some(ListMarker { ordered: false, start: 1, indent, content: item });
        }
    }
    let digits = content.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 || !content[digits..].starts_with(". ") { return None; }
    Some(ListMarker {
        ordered: true,
        start: content[..digits].parse().expect("ASCII digits parse as an integer"),
        indent,
        content: &content[digits + 2..],
    })
}

fn list(lines: &[Line<'_>], mut i: usize, first: ListMarker<'_>, options: Options) -> (Block, usize) {
    let ordered = first.ordered;
    let start = first.start;
    let indent = first.indent;
    let mut items = Vec::new();

    while i < lines.len() {
        let Some(marker) = list_marker(lines[i].text) else { break; };
        if marker.ordered != ordered || marker.indent != indent { break; }

        let (checked, content) = task_marker(marker.content, options);
        let mut item_lines = vec![content.to_owned()];
        i += 1;
        while i < lines.len() {
            let current = lines[i].text;
            if current.trim().is_empty() {
                item_lines.push(String::new());
                i += 1;
                continue;
            }
            let current_indent = current.len() - current.trim_start().len();
            if current_indent > indent {
                let dedent = (indent + 2).min(current_indent);
                item_lines.push(current[dedent..].to_owned());
                i += 1;
                continue;
            }
            break;
        }
        let blocks = parse(tokenize(&item_lines.join("\n")), options).blocks;
        items.push(ListItem { checked, blocks });
    }
    (Block::List { ordered, start, items }, i)
}

fn quote_content(line: &str) -> Option<&str> {
    line.trim_start().strip_prefix('>').map(|content| content.strip_prefix(' ').unwrap_or(content))
}

fn quote(lines: &[Line<'_>], mut i: usize, options: Options) -> (Vec<Block>, usize) {
    let mut content = Vec::new();
    while i < lines.len() {
        match quote_content(lines[i].text) {
            Some(text) => { content.push(text); i += 1; }
            None => break,
        }
    }
    (parse(tokenize(&content.join("\n")), options).blocks, i)
}

fn paragraph(lines: &[Line<'_>], mut i: usize, options: Options) -> (String, usize) {
    let mut content = Vec::new();
    while i < lines.len() {
        let line = lines[i].text;
        if line.trim().is_empty() || fence_opening(line).is_some() || heading(line).is_some() || (options.gfm && table(lines, i).is_some()) || setext_heading(lines, i).is_some() || is_rule(line) || list_marker(line).is_some() || quote_content(line).is_some() { break; }
        content.push(line.trim_start()); i += 1;
    }
    (content.join("\n"), i)
}

fn task_marker(content: &str, options: Options) -> (Option<bool>, &str) {
    if !options.gfm { return (None, content); }
    for (prefix, checked) in [("[ ] ", false), ("[x] ", true), ("[X] ", true)] {
        if let Some(rest) = content.strip_prefix(prefix) { return (Some(checked), rest); }
    }
    (None, content)
}

fn table(lines: &[Line<'_>], index: usize) -> Option<(Block, usize)> {
    let header = split_table_row(lines.get(index)?.text)?;
    let alignments = table_divider(lines.get(index + 1)?.text)?;
    if header.len() != alignments.len() { return None; }

    let mut rows = Vec::new();
    let mut next = index + 2;
    while let Some(line) = lines.get(next) {
        let Some(row) = split_table_row(line.text) else { break; };
        if row.len() != header.len() { break; }
        rows.push(row.into_iter().map(str::to_owned).collect());
        next += 1;
    }
    Some((Block::Table { header: header.into_iter().map(str::to_owned).collect(), alignments, rows }, next))
}

fn split_table_row(line: &str) -> Option<Vec<&str>> {
    let trimmed = line.trim();
    if !trimmed.contains('|') { return None; }
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    Some(trimmed.split('|').map(str::trim).collect())
}

fn table_divider(line: &str) -> Option<Vec<TableAlignment>> {
    let cells = split_table_row(line)?;
    let mut alignments = Vec::new();
    for cell in cells {
        let cell = cell.trim();
        let left = cell.starts_with(':');
        let right = cell.ends_with(':');
        let dashes = cell.trim_matches(':');
        if dashes.len() < 3 || !dashes.chars().all(|ch| ch == '-') { return None; }
        alignments.push(match (left, right) {
            (true, true) => TableAlignment::Center,
            (true, false) => TableAlignment::Left,
            (false, true) => TableAlignment::Right,
            (false, false) => TableAlignment::None,
        });
    }
    Some(alignments)
}
