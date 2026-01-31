//! # latex2typst
//!
//! A Rust library for converting Markdown with embedded LaTeX math and pure LaTeX documents
//! to Typst syntax.
//!
//! ## Features
//!
//! - Parse Markdown (CommonMark) with embedded LaTeX math expressions
//! - Parse pure LaTeX documents
//! - Convert to Typst syntax
//! - WASM support for browser/Node.js usage
//!
//! ## Usage
//!
//! ```rust,no_run
//! use latex2typst::convert;
//!
//! let markdown = "# Hello\n\nSome math: $x^2 + y^2 = z^2$";
//! let typst = convert(markdown).unwrap();
//! println!("{}", typst);
//! ```

pub mod ast;
pub mod converter;
pub mod detector;
pub mod error;
pub mod parser;

#[cfg(feature = "wasm")]
pub mod wasm;

use error::Result;

/// Auto-detect input format and convert to Typst
pub fn convert(input: &str) -> Result<String> {
    let format = detector::detect_format(input);
    match format {
        InputFormat::Latex => convert_latex(input),
        InputFormat::Markdown | InputFormat::Auto => convert_markdown(input),
    }
}

/// Convert LaTeX document to Typst
pub fn convert_latex(input: &str) -> Result<String> {
    // Parse LaTeX document to AST
    let document = parser::latex::document::parse(input)?;

    // Convert AST to Typst
    converter::typst::render(&document)
}

/// Convert Markdown with LaTeX math to Typst
pub fn convert_markdown(input: &str) -> Result<String> {
    // Parse Markdown to AST
    let document = parser::markdown::parse(input)?;

    // Convert AST to Typst
    converter::typst::render(&document)
}

/// Input format specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    /// Auto-detect from content
    Auto,
    /// Pure LaTeX document
    Latex,
    /// Markdown with LaTeX math expressions
    Markdown,
}

/// Configuration for the converter
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Error on unsupported LaTeX commands (default: false)
    pub strict_mode: bool,
    /// Preserve LaTeX comments in output (default: false)
    pub preserve_comments: bool,
}

/// Advanced converter with configuration
pub struct Converter {
    #[allow(dead_code)]
    config: Config,
}

impl Converter {
    /// Create a new converter with default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Create a new converter with custom configuration
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// Convert input to Typst with specified format
    pub fn convert(&self, input: &str, format: InputFormat) -> Result<String> {
        match format {
            InputFormat::Auto => convert(input),
            InputFormat::Latex => convert_latex(input),
            InputFormat::Markdown => convert_markdown(input),
        }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}
