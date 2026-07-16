use crate::Options;

pub fn render(source: &str, options: Options) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < source.len() {
        let rest = &source[i..];
        if rest.starts_with("\\\n") { out.push_str("<br />\n"); i += 2; continue; }
        if rest.starts_with("  \n") { out.push_str("<br />\n"); i += 3; continue; }
        if let Some((html, used)) = autolink(rest) { out.push_str(&html); i += used; continue; }
        if let Some((ch, used)) = escaped_character(rest) { escape_char(ch, &mut out); i += used; continue; }
        if let Some((html, used)) = image(rest, options).or_else(|| link(rest, options)) { out.push_str(&html); i += used; continue; }
        if options.gfm {
            if let Some((html, used)) = wrapped(rest, "~~", "del", options) { out.push_str(&html); i += used; continue; }
        }
        if let Some((html, used)) = wrapped(rest, "**", "strong", options).or_else(|| wrapped(rest, "__", "strong", options)).or_else(|| wrapped(rest, "*", "em", options)).or_else(|| wrapped(rest, "_", "em", options)).or_else(|| wrapped(rest, "`", "code", options)) { out.push_str(&html); i += used; continue; }
        let ch = rest.chars().next().unwrap();
        escape_char(ch, &mut out); i += ch.len_utf8();
    }
    out
}

fn autolink(s: &str) -> Option<(String, usize)> {
    let body = s.strip_prefix('<')?;
    let end = body.find('>')?;
    let destination = &body[..end];
    let href = if destination.starts_with("https://") || destination.starts_with("http://") {
        destination
    } else if is_email(destination) {
        return Some((format!("<a href=\"mailto:{}\">{}</a>", escape_attr(destination), escape(destination)), end + 2));
    } else { return None; };
    Some((format!("<a href=\"{}\">{}</a>", escape_attr(href), escape(href)), end + 2))
}

fn is_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else { return false; };
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn escaped_character(s: &str) -> Option<(char, usize)> {
    let body = s.strip_prefix('\\')?;
    let ch = body.chars().next()?;
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(ch).then_some((ch, 1 + ch.len_utf8()))
}

fn image(s: &str, options: Options) -> Option<(String, usize)> { parse_link(s, "![", |label, url| format!("<img src=\"{}\" alt=\"{}\" />", escape_attr(url), render(label, options))) }
fn link(s: &str, options: Options) -> Option<(String, usize)> { parse_link(s, "[", |label, url| format!("<a href=\"{}\">{}</a>", escape_attr(url), render(label, options))) }
fn parse_link<F: FnOnce(&str, &str) -> String>(s: &str, prefix: &str, make: F) -> Option<(String, usize)> {
    let after = s.strip_prefix(prefix)?; let end_label = after.find("](")?;
    let after_url = &after[end_label + 2..]; let end_url = after_url.find(')')?;
    Some((make(&after[..end_label], &after_url[..end_url]), prefix.len() + end_label + 2 + end_url + 1))
}
fn wrapped(s: &str, marker: &str, tag: &str, options: Options) -> Option<(String, usize)> {
    let body = s.strip_prefix(marker)?; let end = body.find(marker)?; if end == 0 { return None; }
    Some((format!("<{tag}>{}</{tag}>", if tag == "code" { escape(&body[..end]) } else { render(&body[..end], options) }), marker.len() * 2 + end))
}
pub fn escape(s: &str) -> String { let mut out = String::new(); for ch in s.chars() { escape_char(ch, &mut out); } out }
pub fn escape_attr(s: &str) -> String { escape(s).replace('"', "&quot;") }
fn escape_char(ch: char, out: &mut String) { match ch { '&' => out.push_str("&amp;"), '<' => out.push_str("&lt;"), '>' => out.push_str("&gt;"), _ => out.push(ch) } }
