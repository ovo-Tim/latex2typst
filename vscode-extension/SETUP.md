# Setup Guide for LaTeX to Typst VS Code Extension

This guide will help you set up, build, and install the LaTeX to Typst converter extension for Visual Studio Code.

## Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust** (1.71 or later)
   - Install from: https://rustup.rs/
   - Verify: `rustc --version`

2. **wasm-pack**
   - Install: `cargo install wasm-pack`
   - Verify: `wasm-pack --version`

3. **Node.js** (v18 or later)
   - Install from: https://nodejs.org/
   - Verify: `node --version`

4. **Visual Studio Code**
   - Install from: https://code.visualstudio.com/

## Quick Start

### Option 1: One-Command Setup (Linux/macOS)

From the `vscode-extension` directory:

```bash
pnpmm run setup
```

This will:
1. Build the WASM package
2. Install Node.js dependencies
3. Compile the TypeScript code

### Option 2: Manual Setup

#### Step 1: Build the WASM Package

From the **project root** (`latex2typst/`):

```bash
# Linux/macOS - use the provided script (recommended)
cd vscode-extension
pnpmm run build:wasm

# Or manually:
# From project root:
# wasm-pack build --target nodejs --features wasm
# Then copy pkg/* to vscode-extension/wasm/

# Windows (PowerShell)
pnpmm run build:wasm:win
```

This creates a `wasm/` directory inside `vscode-extension/` containing:
- `latex2typst.js` - JavaScript bindings
- `latex2typst_bg.wasm` - WebAssembly binary
- `latex2typst.d.ts` - TypeScript definitions
- `package.json` - Package metadata

#### Step 2: Install Dependencies

From the `vscode-extension` directory:

```bash
pnpmm install
```

#### Step 3: Compile TypeScript

```bash
pnpmm run compile
```

This compiles `src/extension.ts` to `out/extension.js`.

## Development

### Running the Extension

1. Open the `vscode-extension` folder in VS Code
2. Press `F5` to launch the Extension Development Host
3. A new VS Code window will open with the extension loaded
4. Open a Markdown or LaTeX file to test the extension

### Testing Commands

In the Extension Development Host window:

1. Open Command Palette (`Cmd+Shift+P` or `Ctrl+Shift+P`)
2. Type "LaTeX to Typst" to see available commands
3. Try converting some sample text:

```markdown
# Test Document

The equation $E = mc^2$ shows the relationship.

Display math:

$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$
```

### Watch Mode for Development

To automatically recompile on changes:

```bash
pnpm run watch
```

Then use `Cmd+Shift+P` → "Developer: Reload Window" to reload the extension.

### Debugging

The extension includes launch configurations:

1. **Run Extension**: Launch the extension in debug mode (F5)
2. **Extension Tests**: Run extension tests (if implemented)

Set breakpoints in `src/extension.ts` and they will be hit during debugging.

## Building for Distribution

### Create a VSIX Package

```bash
pnpm run package
```

This creates a `.vsix` file in the current directory (e.g., `latex2typst-converter-0.1.0.vsix`).

### Install the VSIX Package

#### Option 1: Via VS Code UI

1. Open VS Code
2. Go to Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`)
3. Click the `...` menu in the Extensions view
4. Select "Install from VSIX..."
5. Choose the `.vsix` file

#### Option 2: Via Command Line

```bash
code --install-extension latex2typst-converter-0.1.0.vsix
```

### Uninstall

```bash
code --uninstall-extension your-publisher-name.latex2typst-converter
```

## Project Structure

```
vscode-extension/
├── .vscode/              # VS Code configuration
│   ├── launch.json       # Debug configurations
│   ├── tasks.json        # Build tasks
│   └── settings.json     # Workspace settings
├── scripts/              # Build scripts
│   ├── build-wasm.sh     # Unix WASM build script
│   └── build-wasm.ps1    # Windows WASM build script
├── src/                  # TypeScript source
│   └── extension.ts      # Main extension code
├── wasm/                 # WASM package (generated)
│   ├── latex2typst.js
│   ├── latex2typst_bg.wasm
│   └── ...
├── out/                  # Compiled JavaScript (generated)
│   └── extension.js
├── package.json          # Extension manifest
├── tsconfig.json         # TypeScript configuration
├── .eslintrc.json        # ESLint configuration
├── .vscodeignore         # Files to exclude from package
├── README.md             # User documentation
└── SETUP.md              # This file
```

## Troubleshooting

### WASM Module Not Found

**Error**: `WASM module not found at .../wasm/latex2typst.js`

**Solution**: Build the WASM package:
```bash
pnpm run build:wasm
```

### Compilation Errors

**Error**: TypeScript compilation fails

**Solution**: Ensure dependencies are installed:
```bash
pnpm install
pnpm run compile
```

### Extension Not Activating

**Error**: Extension doesn't appear in VS Code

**Solutions**:
1. Check the Output panel (View → Output) and select "Extension Host"
2. Verify the extension is installed: `code --list-extensions`
3. Reload the window: `Cmd+R` or `Ctrl+R`

### wasm-pack Not Found

**Error**: `wasm-pack: command not found`

**Solution**: Install wasm-pack:
```bash
cargo install wasm-pack
```

### Permission Denied (Scripts)

**Error**: Cannot execute `build-wasm.sh`

**Solution**: Make the script executable:
```bash
chmod +x scripts/build-wasm.sh
```

## Updating the WASM Package

After making changes to the Rust library:

1. Rebuild the WASM package:
   ```bash
   pnpm run build:wasm
   ```

2. Recompile the extension:
   ```bash
   pnpm run compile
   ```

3. Reload the Extension Development Host:
   - `Cmd+Shift+P` → "Developer: Reload Window"

## Publishing

Before publishing to the VS Code Marketplace:

1. Update `publisher` in `package.json`
2. Increment `version` in `package.json`
3. Add a repository URL
4. Create an icon (128x128 PNG)
5. Test thoroughly
6. Create a personal access token (PAT) from Azure DevOps
7. Publish:
   ```bash
   vsce publish
   ```

For more details, see: https://code.visualstudio.com/api/working-with-extensions/publishing-extension

## Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Typst Documentation](https://typst.app/docs)
- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [Project Repository](https://github.com/yourusername/latex2typst)

## Getting Help

If you encounter issues:

1. Check the [Troubleshooting](#troubleshooting) section
2. Review the VS Code Output panel
3. File an issue on GitHub
4. Check the project README

## License

Apache License 2.0 - See LICENSE file for details.
