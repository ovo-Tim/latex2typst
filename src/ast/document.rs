//! Document structure AST nodes

use super::math::MathExpr;

/// A complete document with metadata and content
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Document metadata (title, author, etc.)
    pub metadata: Metadata,
    /// Document content blocks
    pub content: Vec<Block>,
}

/// Document metadata
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Metadata {
    /// Document title
    pub title: Option<String>,
    /// Document author(s)
    pub author: Option<String>,
    /// Date
    pub date: Option<String>,
}

/// Block-level elements
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Heading with level (1-6) and content
    Heading { level: u8, content: Vec<Inline> },
    /// Paragraph containing inline elements
    Paragraph(Vec<Inline>),
    /// List (ordered or unordered)
    List {
        kind: ListKind,
        items: Vec<Vec<Block>>,
    },
    /// Code block with optional language
    CodeBlock { lang: Option<String>, code: String },
    /// Display math block
    MathBlock { expr: MathExpr, numbered: bool },
    /// Block quote
    Quote(Vec<Block>),
    /// Figure with content, caption, and label
    Figure {
        content: Vec<Block>,
        caption: Option<Vec<Inline>>,
        label: Option<String>,
    },
    /// Table
    Table { rows: Vec<Vec<TableCell>> },
    /// Horizontal rule
    HorizontalRule,
}

/// List kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    /// Unordered (bullet) list
    Unordered,
    /// Ordered (numbered) list
    Ordered,
    /// Description list (term/definition pairs)
    Description,
}

/// Table cell
#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    /// Cell content
    pub content: Vec<Inline>,
    /// Column span
    pub colspan: usize,
    /// Row span
    pub rowspan: usize,
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            colspan: 1,
            rowspan: 1,
        }
    }
}

/// Inline elements
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain text
    Text(String),
    /// Formatted text (bold, italic, etc.)
    Formatted { style: Style, content: Vec<Inline> },
    /// Inline code
    Code(String),
    /// Link with text and URL
    Link { text: Vec<Inline>, url: String },
    /// Inline math expression
    MathInline(MathExpr),
    /// Reference to a label
    Ref(String),
    /// Line break
    LineBreak,
}

/// Text formatting style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// Bold/strong
    Bold,
    /// Italic/emphasis
    Italic,
    /// Underline
    Underline,
    /// Monospace
    Monospace,
    /// Strikethrough
    Strikethrough,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            metadata: Metadata::default(),
            content: Vec::new(),
        }
    }

    /// Create a document with metadata
    pub fn with_metadata(metadata: Metadata) -> Self {
        Self {
            metadata,
            content: Vec::new(),
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert!(doc.content.is_empty());
        assert_eq!(doc.metadata.title, None);
    }

    #[test]
    fn test_document_with_metadata() {
        let metadata = Metadata {
            title: Some("Test Document".to_string()),
            author: Some("Test Author".to_string()),
            date: None,
        };
        let doc = Document::with_metadata(metadata.clone());
        assert_eq!(doc.metadata.title, Some("Test Document".to_string()));
        assert_eq!(doc.metadata.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_block_creation() {
        let heading = Block::Heading {
            level: 1,
            content: vec![Inline::Text("Title".to_string())],
        };

        match heading {
            Block::Heading { level, content } => {
                assert_eq!(level, 1);
                assert_eq!(content.len(), 1);
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_inline_formatting() {
        let bold = Inline::Formatted {
            style: Style::Bold,
            content: vec![Inline::Text("bold text".to_string())],
        };

        match bold {
            Inline::Formatted { style, content } => {
                assert_eq!(style, Style::Bold);
                assert_eq!(content.len(), 1);
            }
            _ => panic!("Expected formatted inline"),
        }
    }
}
