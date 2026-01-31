# Change Log

All notable changes to the "latex2typst-converter" extension will be documented in this file.

## [0.1.0] - 2026-01-31

### Added
- Initial release of LaTeX to Typst Converter extension
- Convert selection to Typst
- Convert entire file to Typst
- Convert and save as new `.typ` file
- Live preview in side panel
- Auto-detection of input format (LaTeX vs Markdown)
- Configuration options:
  - Auto-detect format
  - Strict mode
  - Preserve comments
  - Auto-save on convert
- Status bar integration for Markdown and LaTeX files
- Context menu integration
- Command palette commands
- WASM-based conversion (no external dependencies)

### Features
- Support for Markdown with embedded LaTeX math
- Support for pure LaTeX documents
- Inline math (`$...$`) conversion
- Display math (`$$...$$`) conversion
- Common LaTeX commands and environments
- Greek letters and mathematical symbols
- Fractions, subscripts, superscripts
- Lists, sections, and text formatting

## [Unreleased]

### Planned Features
- Syntax highlighting for conversion preview
- Batch file conversion
- Custom LaTeX command mapping
- Integration with Typst preview extensions
- Auto-conversion on save (optional)
- Conversion history
- Snippets for common conversions
