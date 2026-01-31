//! Input format detection

use crate::InputFormat;

/// Detect the input format based on content analysis
pub fn detect_format(input: &str) -> InputFormat {
    // Skip leading whitespace and comments
    let trimmed = input.trim_start();

    // Check for LaTeX document structure
    if is_latex_document(trimmed) {
        return InputFormat::Latex;
    }

    // Default to Markdown (more permissive)
    InputFormat::Markdown
}

/// Check if input appears to be a LaTeX document
fn is_latex_document(input: &str) -> bool {
    // Strong indicators of LaTeX
    let latex_indicators = ["\\documentclass", "\\begin{document}", "\\usepackage"];

    // Check if any strong indicator is present in the first 500 chars
    let prefix = if input.len() > 500 {
        &input[..500]
    } else {
        input
    };

    for indicator in &latex_indicators {
        if prefix.contains(indicator) {
            return true;
        }
    }

    // Additional heuristic: count LaTeX commands vs Markdown syntax
    let latex_score = count_latex_commands(prefix);
    let markdown_score = count_markdown_syntax(prefix);

    // If significantly more LaTeX commands than Markdown syntax, assume LaTeX
    // Lower threshold: even 1-2 LaTeX commands suggest LaTeX
    latex_score > markdown_score && latex_score >= 1
}

/// Count LaTeX-specific commands
fn count_latex_commands(text: &str) -> usize {
    let latex_commands = [
        "\\section",
        "\\subsection",
        "\\chapter",
        "\\textbf",
        "\\emph",
        "\\item",
        "\\begin{",
        "\\end{",
    ];

    latex_commands
        .iter()
        .map(|cmd| text.matches(cmd).count())
        .sum()
}

/// Count Markdown-specific syntax
fn count_markdown_syntax(text: &str) -> usize {
    let mut count = 0;

    // Count Markdown headings (# at start of line)
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') && trimmed.chars().nth(1).is_some_and(|c| c == ' ' || c == '#')
        {
            count += 1;
        }
    }

    // Count Markdown lists (- or * at start of line)
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            count += 1;
        }
    }

    // Count code blocks
    count += text.matches("```").count() / 2;

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_latex() {
        let latex = r"\documentclass{article}
\begin{document}
Hello
\end{document}";
        assert_eq!(detect_format(latex), InputFormat::Latex);
    }

    #[test]
    fn test_detect_markdown() {
        let markdown = r"# Hello

This is **markdown**.";
        assert_eq!(detect_format(markdown), InputFormat::Markdown);
    }

    #[test]
    fn test_detect_latex_with_commands() {
        let latex = r"\section{Introduction}

Some text with \textbf{bold}.";
        assert_eq!(detect_format(latex), InputFormat::Latex);
    }

    #[test]
    fn test_detect_markdown_with_math() {
        let markdown = r"# Math

Here's a formula: $x^2 + y^2 = z^2$";
        assert_eq!(detect_format(markdown), InputFormat::Markdown);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(detect_format(""), InputFormat::Markdown);
    }

    #[test]
    fn test_plain_text() {
        let text = "Just some plain text without any special formatting.";
        assert_eq!(detect_format(text), InputFormat::Markdown);
    }
}
