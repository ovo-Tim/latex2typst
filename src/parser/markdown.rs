//! Markdown parser using pulldown-cmark

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::ast::document::{Block, Document, Inline, ListKind, Style};
use crate::error::Result;

/// Parse Markdown text into a Document AST
pub fn parse(input: &str) -> Result<Document> {
    // Enable math parsing in pulldown-cmark
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);
    let mut converter = MarkdownConverter::new();
    converter.process_events(parser)?;
    Ok(converter.into_document())
}

/// Converter from pulldown-cmark events to our AST
struct MarkdownConverter {
    document: Document,
    current_block: Option<BlockBuilder>,
    inline_stack: Vec<InlineBuilder>,
}

/// Helper for building blocks
enum BlockBuilder {
    Heading {
        level: u8,
        content: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    CodeBlock {
        lang: Option<String>,
        code: String,
    },
    List {
        kind: ListKind,
        items: Vec<Vec<Block>>,
        current_item: Option<Vec<Block>>,
    },
    Quote(Vec<Block>),
}

/// Helper for building inline elements
enum InlineBuilder {
    Formatted { style: Style, content: Vec<Inline> },
    Link { url: String, text: Vec<Inline> },
}

impl MarkdownConverter {
    fn new() -> Self {
        Self {
            document: Document::new(),
            current_block: None,
            inline_stack: Vec::new(),
        }
    }

    fn process_events<'a>(&mut self, parser: Parser<'a>) -> Result<()> {
        for event in parser {
            self.process_event(event)?;
        }

        // Finalize any remaining block
        self.finalize_current_block();

        Ok(())
    }

    fn process_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Start(tag) => self.handle_start_tag(tag)?,
            Event::End(tag_end) => self.handle_end_tag(tag_end)?,
            Event::Text(text) => self.handle_text(text.as_ref()),
            Event::Code(code) => self.handle_inline_code(code.as_ref()),
            Event::SoftBreak => self.handle_text(" "),
            Event::HardBreak => self.add_inline(Inline::LineBreak),
            Event::Rule => self.add_block(Block::HorizontalRule),
            Event::Html(_) | Event::InlineHtml(_) => {
                // Skip HTML for now
            }
            Event::FootnoteReference(_) | Event::TaskListMarker(_) => {
                // Skip advanced features for now
            }
            Event::InlineMath(math) => {
                // Parse LaTeX math expression
                match super::latex::math::parse(math.as_ref()) {
                    Ok(expr) => self.add_inline(Inline::MathInline(expr)),
                    Err(_) => {
                        // Fallback: preserve as text if parsing fails
                        self.add_inline(Inline::Text(format!("${math}$")));
                    }
                }
            }
            Event::DisplayMath(math) => {
                // For display math, finalize current block and add math block
                self.finalize_current_block();
                match super::latex::math::parse(math.as_ref()) {
                    Ok(expr) => {
                        self.add_block(Block::MathBlock {
                            expr,
                            numbered: false,
                        });
                    }
                    Err(_) => {
                        // Fallback: preserve as text
                        self.add_block(Block::Paragraph(vec![Inline::Text(format!(
                            "$$ {math} $$"
                        ))]));
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_start_tag(&mut self, tag: Tag) -> Result<()> {
        match tag {
            Tag::Paragraph => {
                self.current_block = Some(BlockBuilder::Paragraph(Vec::new()));
            }
            Tag::Heading { level, .. } => {
                let level_num = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                self.current_block = Some(BlockBuilder::Heading {
                    level: level_num,
                    content: Vec::new(),
                });
            }
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        if lang.is_empty() {
                            None
                        } else {
                            Some(lang.to_string())
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                self.current_block = Some(BlockBuilder::CodeBlock {
                    lang,
                    code: String::new(),
                });
            }
            Tag::List(first_item_number) => {
                let kind = if first_item_number.is_some() {
                    ListKind::Ordered
                } else {
                    ListKind::Unordered
                };
                self.current_block = Some(BlockBuilder::List {
                    kind,
                    items: Vec::new(),
                    current_item: None,
                });
            }
            Tag::Item => {
                if let Some(BlockBuilder::List { current_item, .. }) = &mut self.current_block {
                    *current_item = Some(Vec::new());
                }
            }
            Tag::BlockQuote(_) => {
                self.current_block = Some(BlockBuilder::Quote(Vec::new()));
            }
            Tag::Strong => {
                self.inline_stack.push(InlineBuilder::Formatted {
                    style: Style::Bold,
                    content: Vec::new(),
                });
            }
            Tag::Emphasis => {
                self.inline_stack.push(InlineBuilder::Formatted {
                    style: Style::Italic,
                    content: Vec::new(),
                });
            }
            Tag::Strikethrough => {
                self.inline_stack.push(InlineBuilder::Formatted {
                    style: Style::Strikethrough,
                    content: Vec::new(),
                });
            }
            Tag::Link { dest_url, .. } => {
                self.inline_stack.push(InlineBuilder::Link {
                    url: dest_url.to_string(),
                    text: Vec::new(),
                });
            }
            // Skip other tags for now
            _ => {}
        }
        Ok(())
    }

    fn handle_end_tag(&mut self, tag_end: TagEnd) -> Result<()> {
        match tag_end {
            TagEnd::Paragraph => {
                self.finalize_current_block();
            }
            TagEnd::Heading(_) => {
                self.finalize_current_block();
            }
            TagEnd::CodeBlock => {
                self.finalize_current_block();
            }
            TagEnd::List(_) => {
                // Finalize the last item
                if let Some(BlockBuilder::List {
                    items,
                    current_item,
                    ..
                }) = &mut self.current_block
                {
                    if let Some(item_blocks) = current_item.take() {
                        items.push(item_blocks);
                    }
                }
                self.finalize_current_block();
            }
            TagEnd::Item => {
                if let Some(BlockBuilder::List {
                    items,
                    current_item,
                    ..
                }) = &mut self.current_block
                {
                    if let Some(item_blocks) = current_item.take() {
                        items.push(item_blocks);
                    }
                }
            }
            TagEnd::BlockQuote => {
                self.finalize_current_block();
            }
            TagEnd::Strong | TagEnd::Emphasis | TagEnd::Strikethrough => {
                if let Some(builder) = self.inline_stack.pop() {
                    match builder {
                        InlineBuilder::Formatted { style, content } => {
                            self.add_inline(Inline::Formatted { style, content });
                        }
                        _ => {}
                    }
                }
            }
            TagEnd::Link => {
                if let Some(builder) = self.inline_stack.pop() {
                    match builder {
                        InlineBuilder::Link { url, text } => {
                            self.add_inline(Inline::Link { text, url });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_text(&mut self, text: &str) {
        self.add_inline(Inline::Text(text.to_string()));
    }

    fn handle_inline_code(&mut self, code: &str) {
        self.add_inline(Inline::Code(code.to_string()));
    }

    fn add_inline(&mut self, inline: Inline) {
        // Add to the innermost inline builder, or to the current block
        if let Some(builder) = self.inline_stack.last_mut() {
            match builder {
                InlineBuilder::Formatted { content, .. } => {
                    content.push(inline);
                }
                InlineBuilder::Link { text, .. } => {
                    text.push(inline);
                }
            }
        } else if let Some(block) = &mut self.current_block {
            match block {
                BlockBuilder::Heading { content, .. } => {
                    content.push(inline);
                }
                BlockBuilder::Paragraph(content) => {
                    content.push(inline);
                }
                BlockBuilder::List { current_item, .. } => {
                    if let Some(item_blocks) = current_item {
                        // Add a paragraph to the item if needed
                        if item_blocks.is_empty() {
                            item_blocks.push(Block::Paragraph(vec![inline]));
                        } else if let Some(Block::Paragraph(content)) = item_blocks.last_mut() {
                            content.push(inline);
                        } else {
                            item_blocks.push(Block::Paragraph(vec![inline]));
                        }
                    }
                }
                BlockBuilder::CodeBlock { code, .. } => {
                    // Code blocks get text directly
                    if let Inline::Text(text) = inline {
                        code.push_str(&text);
                    }
                }
                BlockBuilder::Quote(blocks) => {
                    // Add to the last paragraph in the quote, or create a new one
                    if let Some(Block::Paragraph(content)) = blocks.last_mut() {
                        content.push(inline);
                    } else {
                        blocks.push(Block::Paragraph(vec![inline]));
                    }
                }
            }
        }
    }

    fn add_block(&mut self, block: Block) {
        self.document.content.push(block);
    }

    fn finalize_current_block(&mut self) {
        if let Some(builder) = self.current_block.take() {
            let block = match builder {
                BlockBuilder::Heading { level, content } => Block::Heading { level, content },
                BlockBuilder::Paragraph(content) => {
                    if !content.is_empty() {
                        Block::Paragraph(content)
                    } else {
                        return; // Skip empty paragraphs
                    }
                }
                BlockBuilder::CodeBlock { lang, code } => Block::CodeBlock { lang, code },
                BlockBuilder::List { kind, items, .. } => Block::List { kind, items },
                BlockBuilder::Quote(blocks) => Block::Quote(blocks),
            };
            self.add_block(block);
        }
    }

    fn into_document(self) -> Document {
        self.document
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let input = "# Hello World";
        let doc = parse(input).unwrap();

        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            Block::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content.len(), 1);
                match &content[0] {
                    Inline::Text(text) => assert_eq!(text, "Hello World"),
                    _ => panic!("Expected text"),
                }
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_parse_paragraph() {
        let input = "This is a paragraph.";
        let doc = parse(input).unwrap();

        assert_eq!(doc.content.len(), 1);
        match &doc.content[0] {
            Block::Paragraph(content) => {
                assert_eq!(content.len(), 1);
                match &content[0] {
                    Inline::Text(text) => assert_eq!(text, "This is a paragraph."),
                    _ => panic!("Expected text"),
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_parse_bold_italic() {
        let input = "**bold** and _italic_";
        let doc = parse(input).unwrap();

        match &doc.content[0] {
            Block::Paragraph(content) => {
                assert_eq!(content.len(), 3); // bold, text, italic
                match &content[0] {
                    Inline::Formatted { style, .. } => assert_eq!(*style, Style::Bold),
                    _ => panic!("Expected bold"),
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_parse_list() {
        let input = "- Item 1\n- Item 2";
        let doc = parse(input).unwrap();

        match &doc.content[0] {
            Block::List { kind, items } => {
                assert_eq!(*kind, ListKind::Unordered);
                assert_eq!(items.len(), 2);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_code_block() {
        let input = "```rust\nfn main() {}\n```";
        let doc = parse(input).unwrap();

        match &doc.content[0] {
            Block::CodeBlock { lang, code } => {
                assert_eq!(lang.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}\n");
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn test_parse_inline_code() {
        let input = "Some `code` here";
        let doc = parse(input).unwrap();

        match &doc.content[0] {
            Block::Paragraph(content) => {
                assert_eq!(content.len(), 3); // text, code, text
                match &content[1] {
                    Inline::Code(code) => assert_eq!(code, "code"),
                    _ => panic!("Expected code"),
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_parse_link() {
        let input = "[text](https://example.com)";
        let doc = parse(input).unwrap();

        match &doc.content[0] {
            Block::Paragraph(content) => match &content[0] {
                Inline::Link { text, url } => {
                    assert_eq!(url, "https://example.com");
                    assert_eq!(text.len(), 1);
                }
                _ => panic!("Expected link"),
            },
            _ => panic!("Expected paragraph"),
        }
    }
}
