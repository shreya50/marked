#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading { level: u8, content: String },
    Paragraph(String),
    CodeFence { language: Option<String>, content: String },
    List { ordered: bool, items: Vec<String> },
    BlockQuote(Vec<String>),
    HorizontalRule,
}
