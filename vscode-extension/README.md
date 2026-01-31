# LaTeX to Typst Converter

Convert Markdown with embedded LaTeX math and pure LaTeX documents to [Typst](https://typst.app/) syntax directly in Visual Studio Code.

## Features

- **Convert Selection**: Convert selected text from LaTeX/Markdown to Typst
- **Convert File**: Convert the entire file to Typst syntax
- **Convert and Save**: Convert and save as a new `.typ` file
- **Live Preview**: Preview the converted Typst output in a side panel
- **Auto-detection**: Automatically detect whether input is LaTeX or Markdown
- **Configurable**: Customize conversion behavior through VS Code settings

## Usage

### Commands

Access commands through the Command Palette (`Cmd+Shift+P` on Mac, `Ctrl+Shift+P` on Windows/Linux):

- **LaTeX to Typst: Convert Selection** - Convert selected text to Typst
- **LaTeX to Typst: Convert Current File** - Convert the entire active file
- **LaTeX to Typst: Convert and Save as Typst** - Convert and save to a new `.typ` file
- **LaTeX to Typst: Show Preview** - Show converted output in a side panel

### Context Menu

Right-click on selected text to access **Convert Selection** from the context menu.

### Status Bar

When editing Markdown or LaTeX files, a status bar item (`LaTeX→Typst`) appears. Click it to quickly convert the current file.

## Examples

### Markdown with LaTeX Math

**Input:**
```markdown
# Introduction

The Pythagorean theorem states: $x^2 + y^2 = z^2$

Display equation:

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$
```

**Output:**
```typst
= Introduction

The Pythagorean theorem states: $x^2 + y^2 = z^2$

Display equation:

$ sum_(i=1)^n i = frac(n(n+1), 2) $
```

### Pure LaTeX

**Input:**
```latex
\section{Introduction}

The formula is $E = mc^2$

\begin{equation}
F = ma
\end{equation}
```

**Output:**
```typst
= Introduction

The formula is $E = m c^2$

$ F = m a $
```

## Configuration

Open VS Code Settings and search for "latex2typst":

- **Auto Detect Format** (default: `true`): Automatically detect whether input is LaTeX or Markdown
- **Strict Mode** (default: `false`): Raise errors on unsupported LaTeX commands
- **Preserve Comments** (default: `false`): Preserve LaTeX comments in the output
- **Auto Save On Convert** (default: `false`): Automatically save the file after conversion

## Requirements

This extension uses WebAssembly for fast, client-side conversion. No external dependencies are required.

## Installation

### From VSIX

1. Download the `.vsix` file
2. In VS Code, go to Extensions
3. Click the `...` menu
4. Choose "Install from VSIX..."
5. Select the downloaded file

### From Source

1. Clone the repository
2. Navigate to the `vscode-extension` directory
3. Run:
   ```bash
   pnpm install
   pnpm run compile
   ```
4. Press `F5` to launch the extension in debug mode

## Building

### Prerequisites

The extension requires the WASM package to be built. From the project root:

```bash
# Build WASM package for Node.js target (from project root)
wasm-pack build --target nodejs --features wasm
# Then copy pkg/* to vscode-extension/wasm/

# Or use the provided script (recommended)
cd vscode-extension
pnpm run build:wasm
```

### Package Extension

```bash
cd vscode-extension
pnpm run package
```

This creates a `.vsix` file that can be installed in VS Code.

## Supported LaTeX Features

The converter supports common LaTeX constructs including:

- Math expressions (inline `$...$` and display `$$...$$`)
- Sections, subsections, etc.
- Text formatting (bold, italic, etc.)
- Lists (itemize, enumerate)
- Common math symbols and operators
- Fractions, subscripts, superscripts
- Greek letters
- And more...

## Known Limitations

- Some advanced LaTeX packages may not be fully supported
- Complex table layouts may need manual adjustment
- Custom LaTeX commands require manual conversion

## Contributing

Contributions are welcome! Please visit the [GitHub repository](https://github.com/yourusername/latex2typst) to report issues or submit pull requests.

## License

Apache License 2.0 - See LICENSE file for details.

## Acknowledgments

- [Typst](https://typst.app/) - The beautiful typesetting system
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) - Markdown parser
- [nom](https://github.com/rust-bakery/nom) - Parser combinator framework
