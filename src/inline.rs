pub fn render(source: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < source.len() {
        let rest = &source[i..];
        if let Some((html, used)) = image(rest).or_else(|| link(rest)) { out.push_str(&html); i += used; continue; }
        if let Some((html, used)) = wrapped(rest, "**", "strong").or_else(|| wrapped(rest, "__", "strong")).or_else(|| wrapped(rest, "*", "em")).or_else(|| wrapped(rest, "_", "em")).or_else(|| wrapped(rest, "`", "code")) { out.push_str(&html); i += used; continue; }
        let ch = rest.chars().next().unwrap();
        escape_char(ch, &mut out); i += ch.len_utf8();
    }
    out
}

fn image(s: &str) -> Option<(String, usize)> { parse_link(s, "![", |label, url| format!("<img src=\"{}\" alt=\"{}\" />", escape(url), render(label))) }
fn link(s: &str) -> Option<(String, usize)> { parse_link(s, "[", |label, url| format!("<a href=\"{}\">{}</a>", escape(url), render(label))) }
fn parse_link<F: FnOnce(&str, &str) -> String>(s: &str, prefix: &str, make: F) -> Option<(String, usize)> {
    let after = s.strip_prefix(prefix)?; let end_label = after.find("](")?;
    let after_url = &after[end_label + 2..]; let end_url = after_url.find(')')?;
    Some((make(&after[..end_label], &after_url[..end_url]), prefix.len() + end_label + 2 + end_url + 1))
}
fn wrapped(s: &str, marker: &str, tag: &str) -> Option<(String, usize)> {
    let body = s.strip_prefix(marker)?; let end = body.find(marker)?; if end == 0 { return None; }
    Some((format!("<{tag}>{}</{tag}>", if tag == "code" { escape(&body[..end]) } else { render(&body[..end]) }), marker.len() * 2 + end))
}
pub fn escape(s: &str) -> String { let mut out = String::new(); for ch in s.chars() { escape_char(ch, &mut out); } out }
fn escape_char(ch: char, out: &mut String) { match ch { '&' => out.push_str("&amp;"), '<' => out.push_str("&lt;"), '>' => out.push_str("&gt;"), '"' => out.push_str("&quot;"), _ => out.push(ch) } }
