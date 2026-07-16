#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading { level: u8, content: String },
    Paragraph(String),
    CodeFence { language: Option<String>, content: String },
    List { ordered: bool, start: u64, items: Vec<ListItem> },
    BlockQuote(Vec<Block>),
    Table { header: Vec<String>, alignments: Vec<TableAlignment>, rows: Vec<Vec<String>> },
    HorizontalRule,
}

/// A list item may contain a paragraph, nested list, quote, or other blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    /// Task-list state, present only when GFM parsing is enabled.
    pub checked: Option<bool>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableAlignment { None, Left, Center, Right }
