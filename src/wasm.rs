//! WebAssembly bindings for latex2typst

use wasm_bindgen::prelude::*;

use crate::{convert, convert_latex, convert_markdown, Config, Converter, InputFormat};

/// Convert input to Typst with automatic format detection
///
/// # Arguments
/// * `input` - The input text (Markdown or LaTeX)
///
/// # Returns
/// The converted Typst code as a string
///
/// # Example (JavaScript)
/// ```js
/// import init, { convert_to_typst } from './pkg/latex2typst.js';
/// await init();
/// const typst = convert_to_typst("# Hello\n\nSome $x^2$ math");
/// console.log(typst);
/// ```
#[wasm_bindgen]
pub fn convert_to_typst(input: String) -> Result<String, JsValue> {
    convert(&input).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Convert LaTeX to Typst
///
/// # Arguments
/// * `input` - LaTeX document text
///
/// # Returns
/// The converted Typst code as a string
#[wasm_bindgen]
pub fn latex_to_typst(input: String) -> Result<String, JsValue> {
    convert_latex(&input).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Convert Markdown to Typst
///
/// # Arguments
/// * `input` - Markdown document text (can include LaTeX math)
///
/// # Returns
/// The converted Typst code as a string
#[wasm_bindgen]
pub fn markdown_to_typst(input: String) -> Result<String, JsValue> {
    convert_markdown(&input).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Advanced converter with configuration options
#[wasm_bindgen]
pub struct WasmConverter {
    converter: Converter,
}

#[wasm_bindgen]
impl WasmConverter {
    /// Create a new converter with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            converter: Converter::new(),
        }
    }

    /// Create a converter with custom configuration
    ///
    /// # Arguments
    /// * `strict_mode` - If true, error on unsupported commands
    /// * `preserve_comments` - If true, preserve LaTeX comments in output
    #[wasm_bindgen(js_name = withConfig)]
    pub fn with_config(strict_mode: bool, preserve_comments: bool) -> Self {
        let config = Config {
            strict_mode,
            preserve_comments,
        };
        Self {
            converter: Converter::with_config(config),
        }
    }

    /// Convert input with specified format
    ///
    /// # Arguments
    /// * `input` - The input text
    /// * `format` - Format hint: "auto", "latex", or "markdown"
    #[wasm_bindgen]
    pub fn convert(&self, input: String, format: String) -> Result<String, JsValue> {
        let fmt = match format.as_str() {
            "auto" => InputFormat::Auto,
            "latex" => InputFormat::Latex,
            "markdown" => InputFormat::Markdown,
            _ => {
                return Err(JsValue::from_str(&format!(
                    "Invalid format '{}'. Use 'auto', 'latex', or 'markdown'",
                    format
                )))
            }
        };

        self.converter
            .convert(&input, fmt)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Default for WasmConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Detect the input format
///
/// # Arguments
/// * `input` - The input text to analyze
///
/// # Returns
/// "latex" or "markdown"
#[wasm_bindgen]
pub fn detect_format(input: String) -> String {
    match crate::detector::detect_format(&input) {
        InputFormat::Latex => "latex".to_string(),
        InputFormat::Markdown | InputFormat::Auto => "markdown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_typst() {
        let result = convert_to_typst("# Hello".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().contains("= Hello"));
    }

    #[test]
    fn test_latex_to_typst() {
        let latex = r"\documentclass{article}
\begin{document}
\section{Test}
\end{document}";
        let result = latex_to_typst(latex.to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_markdown_to_typst() {
        let result = markdown_to_typst("**bold**".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().contains("*bold*"));
    }

    #[test]
    fn test_version() {
        let ver = version();
        assert_eq!(ver, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("# Hello".to_string()), "markdown");
        assert_eq!(
            detect_format("\\documentclass{article}".to_string()),
            "latex"
        );
    }

    #[test]
    fn test_converter() {
        let converter = WasmConverter::new();
        let result = converter.convert("# Test".to_string(), "markdown".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_converter_with_config() {
        let converter = WasmConverter::with_config(false, false);
        let result = converter.convert("# Test".to_string(), "auto".to_string());
        assert!(result.is_ok());
    }
}
