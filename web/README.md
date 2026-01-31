# latex2typst Web App

A modern web application built with Vue 3 and Rspack to convert LaTeX and Markdown documents to Typst format using WebAssembly.

## Features

- 🚀 Real-time conversion with debouncing
- 🎨 Beautiful, responsive UI
- 🔄 Auto-detect input format (LaTeX/Markdown)
- 📋 Copy to clipboard
- ⚡ Powered by Rust + WebAssembly for maximum performance
- 🎯 Built with Vue 3 and Rspack

## Development

### Prerequisites

- Node.js 18+
- Rust 1.71+
- wasm-pack

### Setup

1. Install dependencies:
```bash
pnpm install
```

2. Build the WASM module:
```bash
pnpm run build:wasm
```

3. Start the dev server:
```bash
pnpm run dev
```

The app will open at `http://localhost:3000`

## Building for Production

```bash
# Build WASM
pnpm run build:wasm

# Build web app
pnpm run build
```

The production build will be in the `dist/` directory.

## Project Structure

```
web/
├── src/
│   ├── App.vue          # Main Vue component
│   ├── main.js          # App entry point
│   └── wasm/            # Generated WASM module (gitignored)
├── index.html           # HTML template
├── rspack.config.js     # Rspack configuration
└── package.json
```

## Technologies

- **Vue 3** - Progressive JavaScript framework
- **Rspack** - Fast Rust-based bundler
- **WebAssembly** - High-performance conversion engine
- **Rust** - Systems programming language

## License

Apache-2.0
