//! LaTeX document parser

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alpha1, char, multispace1, space0},
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::document::{Block, Document, Inline, ListKind, Metadata, Style};
use crate::error::{Error, Result};

/// Parse a complete LaTeX document into a Document AST
pub fn parse(input: &str) -> Result<Document> {
    match parse_document(input) {
        Ok((remaining, doc)) => {
            if remaining.trim().is_empty() {
                Ok(doc)
            } else {
                // Some trailing content is okay (like final newlines, comments)
                Ok(doc)
            }
        }
        Err(e) => Err(Error::latex_parse(
            0,
            format!("Failed to parse document: {}", e),
        )),
    }
}

/// Parse a complete LaTeX document
fn parse_document(input: &str) -> IResult<&str, Document> {
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Parse optional preamble (documentclass, packages, etc.)
    let (input, metadata) = opt(parse_preamble)(input)?;

    // Parse document body (between \begin{document} and \end{document})
    let (input, content) = parse_document_body(input)?;

    let doc = Document {
        metadata: metadata.unwrap_or_default(),
        content,
    };

    Ok((input, doc))
}

/// Parse the preamble (everything before \begin{document})
fn parse_preamble(input: &str) -> IResult<&str, Metadata> {
    let (input, _) = skip_whitespace_and_comments(input)?;

    let mut metadata = Metadata::default();
    let mut current_input = input;

    // Parse \documentclass
    if let Ok((input, _)) = parse_documentclass(current_input) {
        current_input = input;
    }

    // Parse any number of preamble commands
    while let Ok((input, cmd)) = parse_preamble_command(current_input) {
        match cmd {
            PreambleCommand::Title(title) => metadata.title = Some(title),
            PreambleCommand::Author(author) => metadata.author = Some(author),
            PreambleCommand::Date(date) => metadata.date = Some(date),
            PreambleCommand::UsePackage(_) => {
                // Ignore package imports for now
            }
        }
        current_input = input;

        // Check if we've reached \begin{document}
        if current_input.trim_start().starts_with("\\begin{document}") {
            break;
        }
    }

    Ok((current_input, metadata))
}

#[derive(Debug)]
#[allow(dead_code)]
enum PreambleCommand {
    Title(String),
    Author(String),
    Date(String),
    UsePackage(String),
}

/// Parse preamble commands like \title, \author, \usepackage
fn parse_preamble_command(input: &str) -> IResult<&str, PreambleCommand> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = char('\\')(input)?;
    let (input, cmd_name) = alpha1(input)?;
    let (input, _) = space0(input)?;

    match cmd_name {
        "title" => {
            let (input, title) = parse_braced_arg(input)?;
            Ok((input, PreambleCommand::Title(title)))
        }
        "author" => {
            let (input, author) = parse_braced_arg(input)?;
            Ok((input, PreambleCommand::Author(author)))
        }
        "date" => {
            let (input, date) = parse_braced_arg(input)?;
            Ok((input, PreambleCommand::Date(date)))
        }
        "usepackage" => {
            // Optional argument in brackets
            let (input, _) = opt(delimited(char('['), take_until("]"), char(']')))(input)?;
            let (input, pkg) = parse_braced_arg(input)?;
            Ok((input, PreambleCommand::UsePackage(pkg)))
        }
        _ => {
            // Unknown command - skip it
            let (input, _) = opt(parse_braced_arg)(input)?;
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alt,
            )))
        }
    }
}

/// Parse \documentclass[options]{class}
fn parse_documentclass(input: &str) -> IResult<&str, ()> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("\\documentclass")(input)?;
    let (input, _) = space0(input)?;
    // Optional argument
    let (input, _) = opt(delimited(char('['), take_until("]"), char(']')))(input)?;
    let (input, _) = space0(input)?;
    // Required argument
    let (input, _) = parse_braced_arg(input)?;
    Ok((input, ()))
}

/// Parse the document body (between \begin{document} and \end{document})
fn parse_document_body(input: &str) -> IResult<&str, Vec<Block>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("\\begin{document}")(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, blocks) = many0(parse_block)(input)?;

    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("\\end{document}")(input)?;

    Ok((input, blocks))
}

/// Parse a block-level element
fn parse_block(input: &str) -> IResult<&str, Block> {
    let (input, _) = skip_whitespace_and_comments(input)?;

    alt((parse_section, parse_environment, parse_paragraph))(input)
}

/// Parse sectioning commands (\section, \subsection, etc.)
fn parse_section(input: &str) -> IResult<&str, Block> {
    let (input, _) = char('\\')(input)?;
    let (input, cmd) = alt((
        tag("chapter"),
        tag("section"),
        tag("subsection"),
        tag("subsubsection"),
        tag("paragraph"),
        tag("subparagraph"),
    ))(input)?;

    let (input, _) = space0(input)?;

    // Optional short title in brackets
    let (input, _) = opt(delimited(char('['), take_until("]"), char(']')))(input)?;

    let (input, _) = space0(input)?;
    let (input, title) = parse_braced_arg(input)?;

    let level = match cmd {
        "chapter" => 1,
        "section" => 1,
        "subsection" => 2,
        "subsubsection" => 3,
        "paragraph" => 4,
        "subparagraph" => 5,
        _ => 1,
    };

    // Parse the title text to inline elements
    let content = vec![Inline::Text(title)];

    Ok((input, Block::Heading { level, content }))
}

/// Parse an environment (\begin{...} ... \end{...})
fn parse_environment(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag("\\begin{")(input)?;
    let (input, env_name) = alpha1(input)?;
    let (input, _) = char('}')(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    match env_name {
        "itemize" => parse_list_environment(input, env_name, ListKind::Unordered),
        "enumerate" => parse_list_environment(input, env_name, ListKind::Ordered),
        "description" => parse_list_environment(input, env_name, ListKind::Description),
        "verbatim" => parse_verbatim_environment(input, env_name),
        _ => {
            // Unknown environment - skip it
            let (input, _) = take_until(&format!("\\end{{{}}}", env_name)[..])(input)?;
            let (input, _) = tag(&format!("\\end{{{}}}", env_name)[..])(input)?;
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alt,
            )))
        }
    }
}

/// Parse a list environment (itemize, enumerate, description)
fn parse_list_environment<'a>(
    input: &'a str,
    env_name: &'a str,
    kind: ListKind,
) -> IResult<&'a str, Block> {
    let (input, items) = many1(parse_list_item)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag(&format!("\\end{{{}}}", env_name)[..])(input)?;

    Ok((input, Block::List { kind, items }))
}

/// Parse a single \item in a list
fn parse_list_item(input: &str) -> IResult<&str, Vec<Block>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("\\item")(input)?;
    let (input, _) = space0(input)?;

    // Optional argument for description lists
    let (input, _) = opt(delimited(char('['), take_until("]"), char(']')))(input)?;

    // Parse content until next \item or \end
    let (input, content) = take_while(|c| c != '\\')(input)?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        Ok((input, vec![]))
    } else {
        Ok((
            input,
            vec![Block::Paragraph(vec![Inline::Text(trimmed.to_string())])],
        ))
    }
}

/// Parse verbatim environment
fn parse_verbatim_environment<'a>(input: &'a str, env_name: &'a str) -> IResult<&'a str, Block> {
    let (input, code) = take_until(&format!("\\end{{{}}}", env_name)[..])(input)?;
    let (input, _) = tag(&format!("\\end{{{}}}", env_name)[..])(input)?;

    Ok((
        input,
        Block::CodeBlock {
            lang: None,
            code: code.to_string(),
        },
    ))
}

/// Parse a paragraph (plain text with inline formatting)
fn parse_paragraph(input: &str) -> IResult<&str, Block> {
    let (input, content) = parse_paragraph_content(input)?;

    if content.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alt,
        )))
    } else {
        Ok((input, Block::Paragraph(content)))
    }
}

/// Parse paragraph content (text with inline commands)
fn parse_paragraph_content(input: &str) -> IResult<&str, Vec<Inline>> {
    let mut inlines = Vec::new();
    let mut current_input = input;

    loop {
        current_input = skip_whitespace_and_comments(current_input)?.0;

        // Check for end of paragraph
        if current_input.is_empty()
            || current_input.starts_with("\\section")
            || current_input.starts_with("\\subsection")
            || current_input.starts_with("\\chapter")
            || current_input.starts_with("\\begin{")
            || current_input.starts_with("\\end{document}")
        {
            break;
        }

        // Try to parse inline command, math, or text
        if let Ok((input, inline)) = parse_inline_math(current_input) {
            inlines.push(inline);
            current_input = input;
        } else if let Ok((input, inline)) = parse_inline_command(current_input) {
            inlines.push(inline);
            current_input = input;
        } else if let Ok((input, text)) = parse_plain_text(current_input) {
            if !text.trim().is_empty() {
                inlines.push(Inline::Text(text));
            }
            current_input = input;
        } else {
            break;
        }
    }

    Ok((current_input, inlines))
}

/// Parse inline formatting commands
fn parse_inline_command(input: &str) -> IResult<&str, Inline> {
    let (input, _) = char('\\')(input)?;
    let (input, cmd_name) = alpha1(input)?;
    let (input, _) = space0(input)?;

    match cmd_name {
        "textbf" => {
            let (input, text) = parse_braced_arg(input)?;
            Ok((
                input,
                Inline::Formatted {
                    style: Style::Bold,
                    content: vec![Inline::Text(text)],
                },
            ))
        }
        "emph" | "textit" => {
            let (input, text) = parse_braced_arg(input)?;
            Ok((
                input,
                Inline::Formatted {
                    style: Style::Italic,
                    content: vec![Inline::Text(text)],
                },
            ))
        }
        "texttt" => {
            let (input, text) = parse_braced_arg(input)?;
            Ok((input, Inline::Code(text)))
        }
        "underline" => {
            let (input, text) = parse_braced_arg(input)?;
            Ok((
                input,
                Inline::Formatted {
                    style: Style::Underline,
                    content: vec![Inline::Text(text)],
                },
            ))
        }
        _ => {
            // Unknown command - treat as text
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alt,
            )))
        }
    }
}

/// Parse inline math ($...$)
fn parse_inline_math(input: &str) -> IResult<&str, Inline> {
    let (input, _) = char('$')(input)?;
    let (input, math_content) = take_while(|c| c != '$')(input)?;
    let (input, _) = char('$')(input)?;

    // Parse the math expression
    match super::math::parse(math_content) {
        Ok(expr) => Ok((input, Inline::MathInline(expr))),
        Err(_) => {
            // Fallback to text if parsing fails
            Ok((input, Inline::Text(format!("${math_content}$"))))
        }
    }
}

/// Parse plain text (up to the next command or special character)
fn parse_plain_text(input: &str) -> IResult<&str, String> {
    let (input, text) = take_while1(|c| c != '\\' && c != '\n' && c != '\r' && c != '$')(input)?;
    Ok((input, text.to_string()))
}

/// Parse a braced argument {text}
fn parse_braced_arg(input: &str) -> IResult<&str, String> {
    delimited(
        char('{'),
        map(take_until("}"), |s: &str| s.to_string()),
        char('}'),
    )(input)
}

/// Skip whitespace and LaTeX comments (% to end of line)
fn skip_whitespace_and_comments(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((
        map(multispace1, |_| ()),
        map(
            tuple((char('%'), take_while(|c| c != '\n'), opt(char('\n')))),
            |_| (),
        ),
    )))(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_documentclass() {
        let input = r"\documentclass{article}";
        let result = parse_documentclass(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_simple_document() {
        let input = r"\documentclass{article}
\begin{document}
Hello world
\end{document}";
        let doc = parse(input).unwrap();
        assert_eq!(doc.content.len(), 1);
    }

    #[test]
    fn test_parse_section() {
        let input = r"\section{Introduction}
Some text
\end{document}";
        let (_, block) = parse_section(input).unwrap();
        match block {
            Block::Heading { level, content } => {
                assert_eq!(level, 1);
                assert_eq!(content.len(), 1);
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_parse_itemize() {
        let input = r"\begin{itemize}
\item First item
\item Second item
\end{itemize}";
        let (_, block) = parse_environment(input).unwrap();
        match block {
            Block::List { kind, items } => {
                assert_eq!(kind, ListKind::Unordered);
                assert_eq!(items.len(), 2);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_text_formatting() {
        let input = r"\textbf{bold text}";
        let (_, inline) = parse_inline_command(input).unwrap();
        match inline {
            Inline::Formatted { style, .. } => {
                assert_eq!(style, Style::Bold);
            }
            _ => panic!("Expected formatted text"),
        }
    }

    #[test]
    fn test_skip_comments() {
        let input = r"% This is a comment
\section{Title}";
        let (remaining, _) = skip_whitespace_and_comments(input).unwrap();
        assert!(remaining.starts_with("\\section"));
    }
}
