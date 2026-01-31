# latex2typst

A Rust library for converting Markdown with embedded LaTeX math and pure LaTeX documents to Typst syntax.

## Features

- ✅ Parse Markdown (CommonMark) with embedded LaTeX math expressions
- ✅ Parse pure LaTeX documents
- ✅ Convert to Typst syntax
- ✅ WASM support for browser/Node.js usage
- 🚧 Under active development

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
latex2typst = "0.1"
```

## Usage

### As a Library

```rust
use latex2typst::convert;

fn main() {
    let markdown = r#"
# Introduction

Some math: $x^2 + y^2 = z^2$

Display math:

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$
"#;

    let typst = convert(markdown).unwrap();
    println!("{}", typst);
}
```

### With Explicit Format

```rust
use latex2typst::{convert_latex, convert_markdown};

// Convert pure LaTeX
let latex = r"\section{Introduction}\nSome text with $x^2$";
let typst = convert_latex(latex).unwrap();

// Convert Markdown with LaTeX math
let markdown = "# Title\n\nMath: $x^2$";
let typst = convert_markdown(markdown).unwrap();
```

### Advanced Configuration

```rust
use latex2typst::{Converter, Config, InputFormat};

let config = Config {
    strict_mode: true,
    preserve_comments: false,
};

let converter = Converter::with_config(config);
let typst = converter.convert(input, InputFormat::Auto).unwrap();
```

## WASM Usage

### Browser

```javascript
import init, { convert_to_typst } from './pkg/latex2typst.js';

await init();
const typst = convert_to_typst("# Hello\n\n$x^2$", "markdown");
console.log(typst);
```

### Node.js

```javascript
const { convert_to_typst } = require('./pkg/latex2typst.js');
const typst = convert_to_typst("# Hello\n\n$x^2$", "markdown");
console.log(typst);
```

## Development Status

This library is under active development. Current progress:

- ✅ **Phase 1: Foundation** - Project structure and core types
- 🚧 **Phase 2: Markdown Parser** - Markdown → AST conversion
- ⏳ **Phase 3: LaTeX Math Parser** - LaTeX math → Math AST
- ⏳ **Phase 4: Math Conversion** - Math AST → Typst math
- ⏳ **Phase 5: Full LaTeX Parser** - Pure LaTeX documents
- ⏳ **Phase 6: API & WASM** - Public API and WASM bindings
- ⏳ **Phase 7: Documentation & Release** - v0.1.0 release

## Supported LaTeX Commands

### Current Support

- 🚧 Coming soon!

### Planned Support

**Phase 1 (Core):**
- Document structure: `\documentclass`, `\section`, `\subsection`
- Text: `\textbf`, `\emph`, `\textit`, `\texttt`
- Lists: `itemize`, `enumerate`
- Math: Greek letters, fractions, sqrt, sub/superscripts

**Phase 2 (Common):**
- Math environments: `equation`, `align`, `cases`
- Verbatim: `verbatim`, `lstlisting`

**Phase 3 (Advanced):**
- Figures, tables, references

## Building

```bash
# Native build
cargo build --release

# Run tests
cargo test

# WASM build
wasm-pack build --target web --features wasm
wasm-pack build --target nodejs --features wasm
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [Typst](https://typst.app/) - The beautiful typesetting system
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) - Markdown parser
- [nom](https://github.com/rust-bakery/nom) - Parser combinator framework
