// Example usage of latex2typst in Node.js
// Run with: node examples/wasm_usage.js

const { convert_to_typst, latex_to_typst, markdown_to_typst, detect_format, version, WasmConverter } = require('../pkg/latex2typst.js');

console.log(`latex2typst version: ${version()}\n`);

// Example 1: Auto-detect and convert
console.log('=== Example 1: Auto-detect ===');
const markdown = `# Hello Typst

This is **bold** and this is _italic_.

Math: $x^2 + y^2 = z^2$
`;

try {
    const result = convert_to_typst(markdown);
    console.log('Input (Markdown):');
    console.log(markdown);
    console.log('\nOutput (Typst):');
    console.log(result);
} catch (e) {
    console.error('Error:', e);
}

// Example 2: LaTeX conversion
console.log('\n=== Example 2: LaTeX Document ===');
const latex = `\\documentclass{article}
\\title{My Paper}
\\author{Jane Smith}

\\begin{document}

\\section{Introduction}

Some text with \\textbf{bold} formatting.

\\subsection{Math}

The famous equation: $E = mc^2$

\\end{document}
`;

try {
    const result = latex_to_typst(latex);
    console.log('Input (LaTeX):');
    console.log(latex);
    console.log('\nOutput (Typst):');
    console.log(result);
} catch (e) {
    console.error('Error:', e);
}

// Example 3: Format detection
console.log('\n=== Example 3: Format Detection ===');
const inputs = [
    '# Markdown heading',
    '\\section{LaTeX section}',
    '\\documentclass{article}',
];

inputs.forEach(input => {
    const format = detect_format(input);
    console.log(`Input: "${input.substring(0, 30)}..."`);
    console.log(`Detected format: ${format}\n`);
});

// Example 4: Using WasmConverter with config
console.log('=== Example 4: Advanced Usage ===');
const converter = WasmConverter.withConfig(false, false);

const text = `## Features

- Item 1
- Item 2

Some math: $\\alpha + \\beta = \\gamma$
`;

try {
    const result = converter.convert(text, 'markdown');
    console.log('Input:');
    console.log(text);
    console.log('\nOutput:');
    console.log(result);
} catch (e) {
    console.error('Error:', e);
}
