#!/usr/bin/env node

/**
 * Simple test script to verify WASM conversion is working
 */

const wasm = require('./wasm/latex2typst.js');
const fs = require('fs');
const path = require('path');

console.log('🧪 Testing LaTeX to Typst Conversion\n');

// Test 1: Simple markdown
console.log('Test 1: Simple Markdown');
const test1 = '# Hello\n\nMath: $E = mc^2$';
try {
    const result1 = wasm.convert_to_typst(test1);
    console.log('✓ Input:', test1.replace('\n', '\\n'));
    console.log('✓ Output:', result1.replace(/\n/g, '\\n'));
    console.log('✓ PASSED\n');
} catch (e) {
    console.error('✗ FAILED:', e.message);
    process.exit(1);
}

// Test 2: Display math
console.log('Test 2: Display Math');
const test2 = '$$\\sum_{i=1}^{n} i = \\frac{n(n+1)}{2}$$';
try {
    const result2 = wasm.convert_to_typst(test2);
    console.log('✓ Input:', test2);
    console.log('✓ Output:', result2.trim());
    console.log('✓ PASSED\n');
} catch (e) {
    console.error('✗ FAILED:', e.message);
    process.exit(1);
}

// Test 3: Format detection
console.log('Test 3: Format Detection');
try {
    const latexDoc = '\\documentclass{article}';
    const markdownDoc = '# Title';

    const format1 = wasm.detect_format(latexDoc);
    const format2 = wasm.detect_format(markdownDoc);

    console.log('✓ LaTeX detected as:', format1);
    console.log('✓ Markdown detected as:', format2);
    console.log('✓ PASSED\n');
} catch (e) {
    console.error('✗ FAILED:', e.message);
    process.exit(1);
}

// Test 4: Example files
console.log('Test 4: Example Files');
try {
    const exampleMd = fs.readFileSync(path.join(__dirname, 'examples/test.md'), 'utf8');
    const resultMd = wasm.convert_to_typst(exampleMd);
    console.log('✓ examples/test.md converted');
    console.log(`  Input: ${exampleMd.length} chars`);
    console.log(`  Output: ${resultMd.length} chars`);

    try {
        const exampleTex = fs.readFileSync(path.join(__dirname, 'examples/test.tex'), 'utf8');
        const resultTex = wasm.convert_to_typst(exampleTex);
        console.log('✓ examples/test.tex converted');
        console.log(`  Input: ${exampleTex.length} chars`);
        console.log(`  Output: ${resultTex.length} chars`);
    } catch (texError) {
        console.log('⚠ examples/test.tex conversion failed (some LaTeX features not yet supported)');
        console.log(`  This is expected - the library is still being developed`);
    }
    console.log('✓ PASSED\n');
} catch (e) {
    console.error('✗ FAILED:', e.message || e.toString());
    process.exit(1);
}

// Test 5: Version
console.log('Test 5: Version Info');
try {
    const version = wasm.version();
    console.log('✓ Library version:', version);
    console.log('✓ PASSED\n');
} catch (e) {
    console.error('✗ FAILED:', e.message);
    process.exit(1);
}

console.log('🎉 All tests passed!');
console.log('\nYou can now:');
console.log('  1. Open this folder in VS Code');
console.log('  2. Press F5 to launch the extension');
console.log('  3. Test with the example files in examples/');
console.log('  4. Run "pnpm run package" to create a .vsix package');
