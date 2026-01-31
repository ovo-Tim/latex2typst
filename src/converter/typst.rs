//! Convert AST to Typst syntax

use crate::ast::document::{Block, Document, Inline, ListKind, Style};
use crate::ast::math::MathExpr;
use crate::error::{Error, Result};

/// Convert a Document AST to Typst syntax
pub fn render(document: &Document) -> Result<String> {
    let mut renderer = TypstRenderer::new();
    renderer.render_document(document)?;
    Ok(renderer.output)
}

struct TypstRenderer {
    output: String,
    #[allow(dead_code)]
    indent_level: usize,
}

impl TypstRenderer {
    fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    fn render_document(&mut self, document: &Document) -> Result<()> {
        // Render metadata if present
        if let Some(title) = &document.metadata.title {
            self.output.push_str("#set document(title: \"");
            self.output.push_str(title);
            self.output.push_str("\")\n");
        }

        if let Some(author) = &document.metadata.author {
            self.output.push_str("#set document(author: \"");
            self.output.push_str(author);
            self.output.push_str("\")\n");
        }

        if document.metadata.title.is_some() || document.metadata.author.is_some() {
            self.output.push('\n');
        }

        // Render content blocks
        for (i, block) in document.content.iter().enumerate() {
            self.render_block(block)?;

            // Add spacing between blocks (except after the last one)
            if i < document.content.len() - 1 {
                self.output.push('\n');
            }
        }

        Ok(())
    }

    fn render_block(&mut self, block: &Block) -> Result<()> {
        match block {
            Block::Heading { level, content } => {
                self.render_heading(*level, content)?;
            }
            Block::Paragraph(content) => {
                self.render_paragraph(content)?;
            }
            Block::List { kind, items } => {
                self.render_list(*kind, items)?;
            }
            Block::CodeBlock { lang, code } => {
                self.render_code_block(lang.as_deref(), code)?;
            }
            Block::MathBlock { expr, numbered } => {
                self.render_math_block(expr, *numbered)?;
            }
            Block::Quote(blocks) => {
                self.render_quote(blocks)?;
            }
            Block::HorizontalRule => {
                self.output.push_str("#line(length: 100%)");
                self.output.push('\n');
            }
            Block::Figure { .. } | Block::Table { .. } => {
                // TODO: Implement in later phases
                return Err(Error::ConversionError(
                    "Figures and tables not yet supported".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn render_heading(&mut self, level: u8, content: &[Inline]) -> Result<()> {
        // Typst uses = for headings: = h1, == h2, === h3, etc.
        for _ in 0..level {
            self.output.push('=');
        }
        self.output.push(' ');
        self.render_inline_content(content)?;
        self.output.push('\n');
        Ok(())
    }

    fn render_paragraph(&mut self, content: &[Inline]) -> Result<()> {
        self.render_inline_content(content)?;
        self.output.push('\n');
        Ok(())
    }

    fn render_list(&mut self, kind: ListKind, items: &[Vec<Block>]) -> Result<()> {
        for item in items {
            match kind {
                ListKind::Unordered => self.output.push_str("- "),
                ListKind::Ordered => self.output.push_str("+ "),
                ListKind::Description => {
                    // For description lists, we'll use the term list syntax
                    // This is a simplification; real implementation would need to parse term/desc
                    self.output.push_str("/ ");
                }
            }

            // Render item content (usually a single paragraph)
            if let Some(first_block) = item.first() {
                match first_block {
                    Block::Paragraph(content) => {
                        self.render_inline_content(content)?;
                        self.output.push('\n');
                    }
                    _ => {
                        // For complex items, render all blocks
                        for block in item {
                            self.render_block(block)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render_code_block(&mut self, lang: Option<&str>, code: &str) -> Result<()> {
        self.output.push_str("```");
        if let Some(lang) = lang {
            self.output.push_str(lang);
        }
        self.output.push('\n');
        self.output.push_str(code);
        if !code.ends_with('\n') {
            self.output.push('\n');
        }
        self.output.push_str("```\n");
        Ok(())
    }

    fn render_math_block(&mut self, expr: &MathExpr, _numbered: bool) -> Result<()> {
        // Display math with spaces around content
        self.output.push_str("$ ");
        let math_str = super::math::render(expr)?;
        self.output.push_str(&math_str);
        self.output.push_str(" $\n");
        Ok(())
    }

    fn render_quote(&mut self, blocks: &[Block]) -> Result<()> {
        // Typst doesn't have built-in quote syntax like markdown
        // We'll use a show rule approach or just indent
        // For now, let's use a simple approach with visual indication
        for block in blocks {
            self.output.push_str("> ");
            match block {
                Block::Paragraph(content) => {
                    self.render_inline_content(content)?;
                    self.output.push('\n');
                }
                _ => {
                    self.render_block(block)?;
                }
            }
        }
        Ok(())
    }

    fn render_inline_content(&mut self, content: &[Inline]) -> Result<()> {
        for inline in content {
            self.render_inline(inline)?;
        }
        Ok(())
    }

    fn render_inline(&mut self, inline: &Inline) -> Result<()> {
        match inline {
            Inline::Text(text) => {
                // Escape special Typst characters
                let escaped = self.escape_text(text);
                self.output.push_str(&escaped);
            }
            Inline::Formatted { style, content } => {
                self.render_formatted(*style, content)?;
            }
            Inline::Code(code) => {
                self.output.push('`');
                self.output.push_str(code);
                self.output.push('`');
            }
            Inline::Link { text, url } => {
                self.output.push_str("#link(\"");
                self.output.push_str(url);
                self.output.push_str("\")[");
                self.render_inline_content(text)?;
                self.output.push(']');
            }
            Inline::MathInline(expr) => {
                // Inline math without spaces
                self.output.push('$');
                let math_str = super::math::render(expr)?;
                self.output.push_str(&math_str);
                self.output.push('$');
            }
            Inline::Ref(label) => {
                self.output.push('@');
                self.output.push_str(label);
            }
            Inline::LineBreak => {
                self.output.push_str(" \\\n");
            }
        }
        Ok(())
    }

    fn render_formatted(&mut self, style: Style, content: &[Inline]) -> Result<()> {
        match style {
            Style::Bold => {
                self.output.push('*');
                self.render_inline_content(content)?;
                self.output.push('*');
            }
            Style::Italic => {
                self.output.push('_');
                self.render_inline_content(content)?;
                self.output.push('_');
            }
            Style::Underline => {
                self.output.push_str("#underline[");
                self.render_inline_content(content)?;
                self.output.push(']');
            }
            Style::Monospace => {
                self.output.push('`');
                self.render_inline_content(content)?;
                self.output.push('`');
            }
            Style::Strikethrough => {
                self.output.push_str("#strike[");
                self.render_inline_content(content)?;
                self.output.push(']');
            }
        }
        Ok(())
    }

    fn escape_text(&self, text: &str) -> String {
        // Escape special Typst characters
        // Main ones are: # (command), * (bold), _ (italic), ` (code), @ (ref)
        // Also: < (labels), $ (math)
        let mut result = String::with_capacity(text.len());
        for ch in text.chars() {
            match ch {
                '#' | '*' | '_' | '`' | '@' | '<' | '$' => {
                    result.push('\\');
                    result.push(ch);
                }
                _ => result.push(ch),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::document::Metadata;

    #[test]
    fn test_render_heading() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::Heading {
                level: 1,
                content: vec![Inline::Text("Hello".to_string())],
            }],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "= Hello\n");
    }

    #[test]
    fn test_render_paragraph() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::Paragraph(vec![Inline::Text(
                "Hello world".to_string(),
            )])],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "Hello world\n");
    }

    #[test]
    fn test_render_bold_italic() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::Paragraph(vec![
                Inline::Formatted {
                    style: Style::Bold,
                    content: vec![Inline::Text("bold".to_string())],
                },
                Inline::Text(" and ".to_string()),
                Inline::Formatted {
                    style: Style::Italic,
                    content: vec![Inline::Text("italic".to_string())],
                },
            ])],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "*bold* and _italic_\n");
    }

    #[test]
    fn test_render_list() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::List {
                kind: ListKind::Unordered,
                items: vec![
                    vec![Block::Paragraph(vec![Inline::Text("Item 1".to_string())])],
                    vec![Block::Paragraph(vec![Inline::Text("Item 2".to_string())])],
                ],
            }],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "- Item 1\n- Item 2\n");
    }

    #[test]
    fn test_render_code_block() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::CodeBlock {
                lang: Some("rust".to_string()),
                code: "fn main() {}\n".to_string(),
            }],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "```rust\nfn main() {}\n```\n");
    }

    #[test]
    fn test_render_inline_code() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::Paragraph(vec![
                Inline::Text("Some ".to_string()),
                Inline::Code("code".to_string()),
                Inline::Text(" here".to_string()),
            ])],
        };

        let result = render(&doc).unwrap();
        assert_eq!(result, "Some `code` here\n");
    }

    #[test]
    fn test_escape_special_chars() {
        let doc = Document {
            metadata: Metadata::default(),
            content: vec![Block::Paragraph(vec![Inline::Text(
                "Special: # * _ ` @ < $".to_string(),
            )])],
        };

        let result = render(&doc).unwrap();
        assert!(result.contains("\\#"));
        assert!(result.contains("\\*"));
        assert!(result.contains("\\_"));
    }

    #[test]
    fn test_render_with_metadata() {
        let doc = Document {
            metadata: Metadata {
                title: Some("My Document".to_string()),
                author: Some("Author Name".to_string()),
                date: None,
            },
            content: vec![Block::Heading {
                level: 1,
                content: vec![Inline::Text("Introduction".to_string())],
            }],
        };

        let result = render(&doc).unwrap();
        assert!(result.contains("#set document(title: \"My Document\")"));
        assert!(result.contains("#set document(author: \"Author Name\")"));
        assert!(result.contains("= Introduction"));
    }
}
