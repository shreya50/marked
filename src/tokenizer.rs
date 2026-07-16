/// A source line retained with its original text. Keeping tokenization light
/// makes block parsing predictable and leaves room for a streaming tokenizer.
#[derive(Debug, Clone, Copy)]
pub struct Line<'a> {
    pub text: &'a str,
}

pub fn tokenize(source: &str) -> Vec<Line<'_>> {
    source.lines().map(|text| Line { text }).collect()
}
