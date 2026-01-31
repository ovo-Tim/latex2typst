<template>
  <div class="container">
    <header class="header">
      <h1>LaTeX/Markdown → Typst Converter</h1>
      <p class="subtitle">Convert your LaTeX or Markdown documents to Typst format</p>
    </header>

    <div class="converter">
      <div class="panel input-panel">
        <div class="panel-header">
          <h2>Input</h2>
          <div class="controls">
            <select v-model="inputFormat" class="format-select">
              <option value="auto">Auto-detect</option>
              <option value="markdown">Markdown</option>
              <option value="latex">LaTeX</option>
            </select>
            <button @click="clearInput" class="btn btn-secondary">Clear</button>
          </div>
        </div>
        <textarea
          v-model="input"
          class="editor"
          placeholder="Enter your LaTeX or Markdown here...

Examples:

Markdown with Math:
# Introduction
Some inline math: $x^2 + y^2 = z^2$

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$

LaTeX:
\documentclass{article}
\begin{document}
\section{My Section}
Hello $\alpha + \beta$
\end{document}"
        ></textarea>
      </div>

      <div class="panel output-panel">
        <div class="panel-header">
          <h2>Output (Typst)</h2>
          <div class="controls">
            <span v-if="detectedFormat" class="detected-format">
              Detected: {{ detectedFormat }}
            </span>
            <button @click="copyOutput" class="btn btn-primary">
              {{ copied ? '✓ Copied!' : 'Copy' }}
            </button>
          </div>
        </div>
        <div class="editor-wrapper">
          <textarea
            v-model="output"
            class="editor"
            :class="{ error: hasError }"
            placeholder="Converted Typst output will appear here..."
            readonly
          ></textarea>
          <div v-if="loading" class="loading-overlay">
            <div class="spinner"></div>
            <p>Converting...</p>
          </div>
        </div>
      </div>
    </div>

    <footer class="footer">
      <p>
        Powered by <a href="https://github.com/yourusername/latex2typst" target="_blank">latex2typst</a>
        v{{ version }} | Built with Rust + WebAssembly
      </p>
    </footer>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';

const input = ref('');
const output = ref('');
const inputFormat = ref('auto');
const detectedFormat = ref('');
const loading = ref(false);
const hasError = ref(false);
const copied = ref(false);
const version = ref('0.1.0');

let wasmModule = null;

// Load WASM module
onMounted(async () => {
  try {
    loading.value = true;
    const wasm = await import('./wasm/latex2typst.js');
    await wasm.default();
    wasmModule = wasm;
    version.value = wasm.version();
    loading.value = false;

    // Set example if no input
    if (!input.value) {
      input.value = `# Introduction

This is a document with some **bold** and *italic* text.

## Math Examples

Inline math: $x^2 + y^2 = z^2$

Display math:

$$
\\sum_{i=1}^{n} i = \\frac{n(n+1)}{2}
$$

## Lists

- Item 1
- Item 2
- Item 3`;
    }
  } catch (error) {
    console.error('Failed to load WASM:', error);
    output.value = `Error loading converter: ${error.message}`;
    hasError.value = true;
    loading.value = false;
  }
});

// Convert input to output
async function convert() {
  if (!wasmModule || !input.value.trim()) {
    output.value = '';
    detectedFormat.value = '';
    return;
  }

  try {
    loading.value = true;
    hasError.value = false;

    // Detect format
    const detected = wasmModule.detect_format(input.value);
    detectedFormat.value = detected;

    // Convert based on format
    let result;
    switch (inputFormat.value) {
      case 'markdown':
        result = wasmModule.markdown_to_typst(input.value);
        break;
      case 'latex':
        result = wasmModule.latex_to_typst(input.value);
        break;
      default:
        result = wasmModule.convert_to_typst(input.value);
    }

    output.value = result;
  } catch (error) {
    console.error('Conversion error:', error);
    output.value = `Error: ${error.message || error}`;
    hasError.value = true;
  } finally {
    loading.value = false;
  }
}

// Watch for input changes
let debounceTimer;
watch([input, inputFormat], () => {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(convert, 300);
});

function clearInput() {
  input.value = '';
  output.value = '';
  detectedFormat.value = '';
}

async function copyOutput() {
  if (!output.value || hasError.value) return;

  try {
    await navigator.clipboard.writeText(output.value);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (error) {
    console.error('Failed to copy:', error);
  }
}
</script>

<style scoped>
.container {
  max-width: 1400px;
  margin: 0 auto;
}

.header {
  text-align: center;
  color: white;
  margin-bottom: 30px;
}

.header h1 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
}

.converter {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  margin-bottom: 20px;
}

@media (max-width: 1024px) {
  .converter {
    grid-template-columns: 1fr;
  }
}

.panel {
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.panel-header {
  padding: 20px;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-header h2 {
  font-size: 1.25rem;
  font-weight: 600;
  color: #333;
}

.controls {
  display: flex;
  gap: 10px;
  align-items: center;
}

.format-select {
  padding: 8px 12px;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  background: white;
  font-size: 0.9rem;
  cursor: pointer;
  transition: border-color 0.2s;
}

.format-select:hover {
  border-color: #667eea;
}

.format-select:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.detected-format {
  font-size: 0.85rem;
  color: #667eea;
  font-weight: 500;
  padding: 6px 12px;
  background: #f0f2ff;
  border-radius: 6px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #667eea;
  color: white;
}

.btn-primary:hover {
  background: #5568d3;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-secondary {
  background: #e9ecef;
  color: #495057;
}

.btn-secondary:hover {
  background: #dee2e6;
}

.editor-wrapper {
  flex: 1;
  position: relative;
}

.editor {
  width: 100%;
  height: 500px;
  padding: 20px;
  border: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
  background: white;
}

.editor:focus {
  outline: none;
}

.editor.error {
  color: #dc3545;
  background: #fff5f5;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 15px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f4f6;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-overlay p {
  color: #667eea;
  font-weight: 500;
}

.footer {
  text-align: center;
  color: white;
  opacity: 0.9;
  padding: 20px;
}

.footer a {
  color: white;
  text-decoration: underline;
}

.footer a:hover {
  opacity: 0.8;
}
</style>
