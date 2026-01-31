# Quick Start Guide

Get the LaTeX to Typst VS Code extension running in under 5 minutes!

## Prerequisites Check

```bash
# Check if you have the required tools
rustc --version    # Should be 1.71+
wasm-pack --version # Should be installed
node --version      # Should be 18+
```

If any are missing, see [SETUP.md](SETUP.md) for installation instructions.

## One-Command Setup

```bash
cd vscode-extension
pnpm run setup
```

That's it! This single command will:
- ✅ Build the WASM package from Rust
- ✅ Install Node.js dependencies
- ✅ Compile TypeScript to JavaScript

## Run the Extension

1. Open the `vscode-extension` folder in VS Code
2. Press `F5`
3. A new VS Code window opens with the extension loaded

## Test It Out

In the new window:

1. Create a new file with this content:

```markdown
# Test

Einstein showed that $E = mc^2$

$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$
```

2. Press `Cmd+Shift+P` (or `Ctrl+Shift+P`)
3. Type "LaTeX to Typst: Show Preview"
4. See the Typst output in a side panel!

## Available Commands

- **Convert Selection** - Convert selected text
- **Convert Current File** - Convert the whole file
- **Convert and Save as Typst** - Save as `.typ` file
- **Show Preview** - View conversion in side panel

## What's Next?

- Read [README.md](README.md) for detailed features
- See [examples/](examples/) for test files
- Check [SETUP.md](SETUP.md) for troubleshooting

## Common Issues

### "WASM module not found"
```bash
pnpm run build:wasm
```

### "Cannot find module"
```bash
pnpm install
```

### Extension not working
1. Check the Output panel: View → Output → Extension Host
2. Reload window: `Cmd+R` or `Ctrl+R`

## Building a Package

Create a `.vsix` file to share:

```bash
pnpm run package
```

Install it:
```bash
code --install-extension latex2typst-converter-0.1.0.vsix
```

---

**Need help?** See the full [SETUP.md](SETUP.md) guide or file an issue on GitHub.
