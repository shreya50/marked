use std::{fmt, mem, ops::Range};

use crate::{parse_with_options, Options};

/// Renders Markdown arriving in arbitrary text chunks.
///
/// Completed block groups are emitted whenever a blank line is received. The
/// final unfinished group is emitted by [`StreamParser::finish`]. This keeps
/// output ordering stable while allowing callers to forward HTML as data is
/// read from a network or file stream.
pub struct StreamParser {
    options: Options,
    line_buffer: String,
    pending: String,
    fence: Option<char>,
}

impl StreamParser {
    pub fn new() -> Self {
        Self::with_options(Options::default())
    }

    pub fn with_options(options: Options) -> Self {
        Self {
            options,
            line_buffer: String::new(),
            pending: String::new(),
            fence: None,
        }
    }

    /// Accepts a source chunk and returns HTML for block groups that are now complete.
    pub fn push(&mut self, chunk: &str) -> String {
        self.line_buffer.push_str(chunk);
        let mut output = String::new();
        while let Some(newline) = self.line_buffer.find('\n') {
            let line: String = self.line_buffer.drain(..=newline).collect();
            output.push_str(&self.push_line(&line));
        }
        output
    }

    /// Finishes the stream and returns HTML for any remaining source.
    pub fn finish(mut self) -> String {
        let mut output = String::new();
        if !self.line_buffer.is_empty() {
            let line = mem::take(&mut self.line_buffer);
            output.push_str(&self.push_line(&line));
        }
        output.push_str(&self.flush());
        output
    }

    fn push_line(&mut self, line: &str) -> String {
        self.update_fence(line);
        self.pending.push_str(line);
        if line.trim().is_empty() && self.fence.is_none() {
            self.flush()
        } else {
            String::new()
        }
    }

    fn flush(&mut self) -> String {
        if self.pending.trim().is_empty() {
            self.pending.clear();
            return String::new();
        }
        parse_with_options(&mem::take(&mut self.pending), self.options)
    }

    fn update_fence(&mut self, line: &str) {
        let trimmed = line.trim_start();
        let Some(marker) = trimmed.chars().next() else {
            return;
        };
        if !matches!(marker, '`' | '~') {
            return;
        }
        let count = trimmed.chars().take_while(|ch| *ch == marker).count();
        if count < 3 {
            return;
        }
        match self.fence {
            None => self.fence = Some(marker),
            Some(open)
                if open == marker
                    && trimmed
                        .chars()
                        .skip_while(|ch| *ch == marker)
                        .all(char::is_whitespace) =>
            {
                self.fence = None
            }
            _ => {}
        }
    }
}

impl Default for StreamParser {
    fn default() -> Self {
        Self::new()
    }
}

/// An editable Markdown document that immediately refreshes its rendered HTML.
///
/// The public edit API is incremental: callers replace only the changed byte
/// range and read the refreshed HTML. The current implementation reparses the
/// affected document synchronously, leaving room for future block-level cache
/// reuse without changing consumers.
pub struct IncrementalParser {
    options: Options,
    source: String,
    html: String,
}

impl IncrementalParser {
    pub fn new(source: impl Into<String>) -> Self {
        Self::with_options(source, Options::default())
    }

    pub fn with_options(source: impl Into<String>, options: Options) -> Self {
        let source = source.into();
        let html = parse_with_options(&source, options);
        Self {
            options,
            source,
            html,
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn html(&self) -> &str {
        &self.html
    }

    /// Replaces a UTF-8 byte range and returns the refreshed HTML.
    pub fn replace(&mut self, range: Range<usize>, replacement: &str) -> Result<&str, EditError> {
        if range.start > range.end || range.end > self.source.len() {
            return Err(EditError::OutOfBounds);
        }
        if !self.source.is_char_boundary(range.start) || !self.source.is_char_boundary(range.end) {
            return Err(EditError::InvalidUtf8Boundary);
        }
        self.source.replace_range(range, replacement);
        self.html = parse_with_options(&self.source, self.options);
        Ok(&self.html)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    OutOfBounds,
    InvalidUtf8Boundary,
}

impl fmt::Display for EditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds => formatter.write_str("edit range is outside the source"),
            Self::InvalidUtf8Boundary => {
                formatter.write_str("edit range does not fall on UTF-8 character boundaries")
            }
        }
    }
}

impl std::error::Error for EditError {}
