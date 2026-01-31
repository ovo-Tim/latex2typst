import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

// Import the WASM module
// Note: The actual import will be dynamic to handle WASM initialization
let latex2typst: any;

// Preview management
interface PreviewSession {
    sourceDocument: vscode.TextDocument;
    previewDocument: vscode.TextDocument;
    updateTimeout?: NodeJS.Timeout;
}

const activePreviews = new Map<string, PreviewSession>();

/**
 * Load and initialize the WASM module
 */
async function initializeWasm(): Promise<void> {
    try {
        // Path to the WASM package (relative to extension root)
        const wasmPath = path.join(__dirname, '..', 'wasm', 'latex2typst.js');

        if (!fs.existsSync(wasmPath)) {
            throw new Error(`WASM module not found at ${wasmPath}. Please build the WASM package first.`);
        }

        // Dynamically import the WASM module
        const wasmModule = require(wasmPath);

        // Initialize WASM (for web target) or use directly (for nodejs target)
        if (typeof wasmModule.default === 'function') {
            await wasmModule.default();
        }

        latex2typst = wasmModule;
        console.log('LaTeX2Typst WASM module loaded successfully');
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Failed to load LaTeX2Typst WASM module: ${message}`);
        throw error;
    }
}

/**
 * Get configuration settings
 */
function getConfig() {
    const config = vscode.workspace.getConfiguration('latex2typst');
    return {
        autoDetect: config.get<boolean>('autoDetectFormat', true),
        strictMode: config.get<boolean>('strictMode', false),
        preserveComments: config.get<boolean>('preserveComments', false),
        autoSave: config.get<boolean>('autoSaveOnConvert', false)
    };
}

/**
 * Convert text using the WASM module
 */
function convertToTypst(input: string, format: 'auto' | 'latex' | 'markdown' = 'auto'): string {
    if (!latex2typst) {
        throw new Error('WASM module not initialized');
    }

    try {
        const config = getConfig();

        // Use the converter with configuration
        if (config.strictMode || config.preserveComments) {
            const converter = latex2typst.WasmConverter.withConfig(
                config.strictMode,
                config.preserveComments
            );
            return converter.convert(input, format);
        } else {
            // Use simple conversion
            return latex2typst.convert_to_typst(input);
        }
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        throw new Error(`Conversion failed: ${message}`);
    }
}

/**
 * Detect the format of the input
 */
function detectFormat(input: string): 'latex' | 'markdown' {
    if (!latex2typst) {
        // Fallback detection
        return input.includes('\\documentclass') || input.includes('\\begin{document}')
            ? 'latex'
            : 'markdown';
    }

    return latex2typst.detect_format(input);
}

/**
 * Command: Convert selection to Typst
 */
async function convertSelection() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const selection = editor.selection;
    if (selection.isEmpty) {
        vscode.window.showErrorMessage('No text selected');
        return;
    }

    try {
        const text = editor.document.getText(selection);
        const config = getConfig();
        const format = config.autoDetect ? 'auto' : detectFormat(text);
        const typst = convertToTypst(text, format);

        await editor.edit(editBuilder => {
            editBuilder.replace(selection, typst);
        });

        vscode.window.showInformationMessage('Selection converted to Typst');
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Conversion failed: ${message}`);
    }
}

/**
 * Command: Convert entire file to Typst
 */
async function convertFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    try {
        const text = editor.document.getText();
        const config = getConfig();
        const format = config.autoDetect ? 'auto' : detectFormat(text);
        const typst = convertToTypst(text, format);

        await editor.edit(editBuilder => {
            const firstLine = editor.document.lineAt(0);
            const lastLine = editor.document.lineAt(editor.document.lineCount - 1);
            const textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
            editBuilder.replace(textRange, typst);
        });

        vscode.window.showInformationMessage('File converted to Typst');
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Conversion failed: ${message}`);
    }
}

/**
 * Command: Convert and save as new Typst file
 */
async function convertAndSave() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    try {
        const text = editor.document.getText();
        const config = getConfig();
        const format = config.autoDetect ? 'auto' : detectFormat(text);
        const typst = convertToTypst(text, format);

        // Generate output filename
        const currentUri = editor.document.uri;
        const currentPath = currentUri.fsPath;
        const parsedPath = path.parse(currentPath);
        const outputPath = path.join(parsedPath.dir, `${parsedPath.name}.typ`);

        // Ask user for save location
        const uri = await vscode.window.showSaveDialog({
            defaultUri: vscode.Uri.file(outputPath),
            filters: {
                'Typst Files': ['typ'],
                'All Files': ['*']
            },
            saveLabel: 'Save Typst File'
        });

        if (uri) {
            await vscode.workspace.fs.writeFile(uri, Buffer.from(typst, 'utf8'));
            vscode.window.showInformationMessage(`Converted to Typst: ${uri.fsPath}`);

            // Optionally open the new file
            const document = await vscode.workspace.openTextDocument(uri);
            await vscode.window.showTextDocument(document);
        }
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Conversion failed: ${message}`);
    }
}

/**
 * Update preview content
 */
async function updatePreview(session: PreviewSession) {
    try {
        const text = session.sourceDocument.getText();
        const config = getConfig();
        const format = config.autoDetect ? 'auto' : detectFormat(text);
        const typst = convertToTypst(text, format);

        // Update the preview document
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(
            session.previewDocument.positionAt(0),
            session.previewDocument.positionAt(session.previewDocument.getText().length)
        );
        edit.replace(session.previewDocument.uri, fullRange, typst);
        await vscode.workspace.applyEdit(edit);
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        console.error(`Preview update failed: ${message}`);
    }
}

/**
 * Command: Show preview panel with real-time updates
 */
async function showPreview() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const sourceDocument = editor.document;
    const sourceUri = sourceDocument.uri.toString();

    // Check if preview already exists for this document
    if (activePreviews.has(sourceUri)) {
        const session = activePreviews.get(sourceUri)!;
        // Just show the existing preview
        const previewEditor = vscode.window.visibleTextEditors.find(
            e => e.document === session.previewDocument
        );
        if (previewEditor) {
            await vscode.window.showTextDocument(session.previewDocument, vscode.ViewColumn.Beside);
            return;
        }
        // Preview was closed, remove it from active previews
        activePreviews.delete(sourceUri);
    }

    try {
        const text = sourceDocument.getText();
        const config = getConfig();
        const format = config.autoDetect ? 'auto' : detectFormat(text);
        const typst = convertToTypst(text, format);

        // Create a new untitled document with Typst content
        const previewDocument = await vscode.workspace.openTextDocument({
            content: typst,
            language: 'typst'
        });

        await vscode.window.showTextDocument(previewDocument, vscode.ViewColumn.Beside);

        // Store the preview session
        activePreviews.set(sourceUri, {
            sourceDocument,
            previewDocument
        });
    } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Preview failed: ${message}`);
    }
}

/**
 * Activate the extension
 */
export async function activate(context: vscode.ExtensionContext) {
    console.log('LaTeX to Typst Converter extension is now active');

    // Initialize WASM module
    try {
        await initializeWasm();
    } catch (error) {
        vscode.window.showWarningMessage(
            'LaTeX to Typst Converter: Failed to initialize. Please ensure the WASM package is built.'
        );
        return;
    }

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('latex2typst.convertSelection', convertSelection),
        vscode.commands.registerCommand('latex2typst.convertFile', convertFile),
        vscode.commands.registerCommand('latex2typst.convertAndSave', convertAndSave),
        vscode.commands.registerCommand('latex2typst.showPreview', showPreview)
    );

    // Register status bar item
    const statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    statusBarItem.text = '$(symbol-misc) LaTeX→Typst';
    statusBarItem.tooltip = 'Click to convert to Typst';
    statusBarItem.command = 'latex2typst.convertFile';

    // Show status bar item when editing markdown or latex files
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor) {
                const lang = editor.document.languageId;
                if (lang === 'markdown' || lang === 'latex') {
                    statusBarItem.show();
                } else {
                    statusBarItem.hide();
                }
            }
        })
    );

    // Show initially if applicable
    const editor = vscode.window.activeTextEditor;
    if (editor) {
        const lang = editor.document.languageId;
        if (lang === 'markdown' || lang === 'latex') {
            statusBarItem.show();
        }
    }

    context.subscriptions.push(statusBarItem);

    // Set up real-time preview updates
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            const sourceUri = event.document.uri.toString();
            const session = activePreviews.get(sourceUri);

            if (session) {
                // Clear any pending update
                if (session.updateTimeout) {
                    clearTimeout(session.updateTimeout);
                }

                // Debounce updates (wait 300ms after last change)
                session.updateTimeout = setTimeout(() => {
                    updatePreview(session);
                }, 300);
            }
        })
    );

    // Clean up previews when documents are closed
    context.subscriptions.push(
        vscode.workspace.onDidCloseTextDocument(document => {
            // Check if it's a source document
            const sourceUri = document.uri.toString();
            const session = activePreviews.get(sourceUri);
            if (session) {
                if (session.updateTimeout) {
                    clearTimeout(session.updateTimeout);
                }
                activePreviews.delete(sourceUri);
            }

            // Check if it's a preview document
            for (const [uri, previewSession] of activePreviews.entries()) {
                if (previewSession.previewDocument === document) {
                    if (previewSession.updateTimeout) {
                        clearTimeout(previewSession.updateTimeout);
                    }
                    activePreviews.delete(uri);
                    break;
                }
            }
        })
    );

    vscode.window.showInformationMessage('LaTeX to Typst Converter is ready!');
}

/**
 * Deactivate the extension
 */
export function deactivate() {
    // Clean up all active previews
    for (const [, session] of activePreviews.entries()) {
        if (session.updateTimeout) {
            clearTimeout(session.updateTimeout);
        }
    }
    activePreviews.clear();

    console.log('LaTeX to Typst Converter extension is now deactivated');
}
