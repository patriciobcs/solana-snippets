import * as vscode from "vscode";
import { Snippet } from "../types/snippet";
import { SnippetsProvider } from "../provider/snippetsProvider";
import { UIUtility } from "../utils/uiUtility";
import { EditSnippet } from "../views/editSnippet";
import { EditSnippetFolder } from "../views/editSnippetFolder";
import { Labels } from "./Labels";

export const enum CommandsConsts {
  miscRequestWSConfig = "miscCmd.requestWSConfig",
  // Common commands across global & ws
  commonOpenSnippet = "globalSnippetsCmd.openSnippet",
  commonOpenSnippetInTerminal = "globalSnippetsCmd.openSnippetInTerminal",
  commonAddSnippet = "commonSnippetsCmd.addSnippet",
  commonAddSnippetFromClipboard = "commonSnippetsCmd.addSnippetFromClipboard",
  commonAddSnippetFolder = "commonSnippetsCmd.addSnippetFolder",
  commonRefreshUI = "commonSnippetsCmd.refreshEntry",
  // Global commands
  globalAddSnippet = "globalSnippetsCmd.addSnippet",
  globalAddSnippetFromClipboard = "globalSnippetsCmd.addSnippetFromClipboard",
  globalAddSnippetFolder = "globalSnippetsCmd.addSnippetFolder",
  globalEditSnippet = "globalSnippetsCmd.editSnippet",
  globalEditSnippetFolder = "globalSnippetsCmd.editSnippetFolder",
  globalDeleteSnippet = "globalSnippetsCmd.deleteSnippet",
  globalDeleteSnippetFolder = "globalSnippetsCmd.deleteSnippetFolder",
  globalMoveSnippetUp = "globalSnippetsCmd.moveSnippetUp",
  globalMoveSnippetDown = "globalSnippetsCmd.moveSnippetDown",
  globalExportSnippets = "globalSnippetsCmd.exportSnippets",
  globalImportSnippets = "globalSnippetsCmd.importSnippets",
  // WS commands
  wsAddSnippet = "wsSnippetsCmd.addSnippet",
  wsAddSnippetFromClipboard = "wsSnippetsCmd.addSnippetFromClipboard",
  wsAddSnippetFolder = "wsSnippetsCmd.addSnippetFolder",
  wsEditSnippet = "wsSnippetsCmd.editSnippet",
  wsEditSnippetFolder = "wsSnippetsCmd.editSnippetFolder",
  wsDeleteSnippet = "wsSnippetsCmd.deleteSnippet",
  wsDeleteSnippetFolder = "wsSnippetsCmd.deleteSnippetFolder",
  wsMoveSnippetUp = "wsSnippetsCmd.moveSnippetUp",
  wsMoveSnippetDown = "wsSnippetsCmd.moveSnippetDown",
}

export async function commonAddSnippet(
  snippetsProvider: SnippetsProvider,
  wsSnippetsProvider: SnippetsProvider,
  workspaceSnippetsAvailable: boolean
) {
  var text: string | undefined;

  const editor = vscode.window.activeTextEditor;
  // If no editor is open or editor has no text, get value from user
  if (!editor || editor.document.getText(editor.selection) === "") {
    // Get snippet name
    text = await UIUtility.requestSnippetValue();
    if (!text || text.length === 0) {
      return;
    }
  } else {
    text = editor.document.getText(editor.selection);
    if (text.length === 0) {
      vscode.window.showWarningMessage(Labels.noTextSelected);
      return;
    }
  }
  // Get snippet name
  const name = await UIUtility.requestSnippetName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetNameErrorMsg);
    return;
  }
  if (text === undefined || text === "") {
    vscode.window.showWarningMessage(Labels.snippetValueErrorMsg);
    return;
  }

  // Request where to save snippets if ws is available
  if (workspaceSnippetsAvailable) {
    const targetView = await UIUtility.requestTargetSnippetsView();
    // no value chosen
    if (!targetView) {
      vscode.window.showWarningMessage(Labels.noViewTypeSelected);
    } else if (targetView === Labels.globalSnippets) {
      snippetsProvider.addSnippet(name, text, Snippet.rootParentId);
    } else if (targetView === Labels.wsSnippets) {
      wsSnippetsProvider.addSnippet(name, text, Snippet.rootParentId);
    }
  } else {
    snippetsProvider.addSnippet(name, text, Snippet.rootParentId);
  }
}

export async function addSnippet(
  snippetsExplorer: vscode.TreeView<Snippet>,
  snippetsProvider: SnippetsProvider,
  node: any
) {
  var text: string | undefined;

  const editor = vscode.window.activeTextEditor;
  // If no editor is open or editor has no text, get value from user
  if (!editor || editor.document.getText(editor.selection) === "") {
    // Get snippet name
    text = await UIUtility.requestSnippetValue();
    if (!text || text.length === 0) {
      return;
    }
  } else {
    text = editor.document.getText(editor.selection);
    if (text.length === 0) {
      vscode.window.showWarningMessage(Labels.noTextSelected);
      return;
    }
  }
  // Get snippet name
  const name = await UIUtility.requestSnippetName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetNameErrorMsg);
    return;
  }
  if (text === undefined || text === "") {
    vscode.window.showWarningMessage(Labels.snippetValueErrorMsg);
    return;
  }
  // When triggering the command with right-click the parameter node of type Tree Node will be tested.
  // When the command is invoked via the menu popup, this node will be the highlighted node, and not the selected node, the latter will undefined.
  if (snippetsExplorer.selection.length === 0 && !node) {
    snippetsProvider.addSnippet(name, text, Snippet.rootParentId);
  } else {
    const selectedItem = node ? node : snippetsExplorer.selection[0];
    if (Snippet.isFolder(selectedItem)) {
      snippetsProvider.addSnippet(name, text, selectedItem.id);
    } else {
      snippetsProvider.addSnippet(
        name,
        text,
        selectedItem.parentId ?? Snippet.rootParentId
      );
    }
  }
}

export async function commonAddSnippetFromClipboard(
  snippetsProvider: SnippetsProvider,
  wsSnippetsProvider: SnippetsProvider,
  workspaceSnippetsAvailable: boolean
) {
  let clipboardContent = await vscode.env.clipboard.readText();
  if (!clipboardContent || clipboardContent.trim() === "") {
    vscode.window.showWarningMessage(Labels.noClipboardContent);
    return;
  }
  // Get snippet name
  const name = await UIUtility.requestSnippetName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetNameErrorMsg);
    return;
  }

  // Request where to save snippets if ws is available
  if (workspaceSnippetsAvailable) {
    const targetView = await UIUtility.requestTargetSnippetsView();
    // no value chosen
    if (!targetView) {
      vscode.window.showWarningMessage(Labels.noViewTypeSelected);
    } else if (targetView === Labels.globalSnippets) {
      snippetsProvider.addSnippet(name, clipboardContent, Snippet.rootParentId);
    } else if (targetView === Labels.wsSnippets) {
      wsSnippetsProvider.addSnippet(
        name,
        clipboardContent,
        Snippet.rootParentId
      );
    }
  } else {
    snippetsProvider.addSnippet(name, clipboardContent, Snippet.rootParentId);
  }
}

export async function addSnippetFromClipboard(
  snippetsExplorer: vscode.TreeView<Snippet>,
  snippetsProvider: SnippetsProvider,
  node: any
) {
  let clipboardContent = await vscode.env.clipboard.readText();
  if (!clipboardContent || clipboardContent.trim() === "") {
    vscode.window.showWarningMessage(Labels.noClipboardContent);
    return;
  }
  // Get snippet name
  const name = await UIUtility.requestSnippetName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetNameErrorMsg);
    return;
  }
  // When triggering the command with right-click the parameter node of type Tree Node will be tested.
  // When the command is invoked via the menu popup, this node will be the highlighted node, and not the selected node, the latter will undefined.
  if (snippetsExplorer.selection.length === 0 && !node) {
    snippetsProvider.addSnippet(name, clipboardContent, Snippet.rootParentId);
  } else {
    const selectedItem = node ? node : snippetsExplorer.selection[0];
    if (Snippet.isFolder(selectedItem)) {
      snippetsProvider.addSnippet(name, clipboardContent, selectedItem.id);
    } else {
      snippetsProvider.addSnippet(
        name,
        clipboardContent,
        selectedItem.parentId ?? Snippet.rootParentId
      );
    }
  }
}

export async function commonAddSnippetFolder(
  snippetsProvider: SnippetsProvider,
  wsSnippetsProvider: SnippetsProvider,
  workspaceSnippetsAvailable: boolean
) {
  // Get snippet name
  const name = await UIUtility.requestSnippetFolderName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetFolderNameErrorMsg);
    return;
  }

  // Request where to save snippets if ws is available
  if (workspaceSnippetsAvailable) {
    const targetView = await UIUtility.requestTargetSnippetsView();
    // No value chosen
    if (!targetView) {
      vscode.window.showWarningMessage(Labels.noViewTypeSelected);
    } else if (targetView === Labels.globalSnippets) {
      snippetsProvider.addSnippetFolder(name, Snippet.rootParentId);
    } else if (targetView === Labels.wsSnippets) {
      wsSnippetsProvider.addSnippetFolder(name, Snippet.rootParentId);
    }
  } else {
    snippetsProvider.addSnippetFolder(name, Snippet.rootParentId);
  }
}

export async function addSnippetFolder(
  snippetsExplorer: vscode.TreeView<Snippet>,
  snippetsProvider: SnippetsProvider,
  node: any
) {
  // Get snippet name
  const name = await UIUtility.requestSnippetFolderName();
  if (name === undefined || name === "") {
    vscode.window.showWarningMessage(Labels.snippetFolderNameErrorMsg);
    return;
  }
  // When triggering the command with right-click the parameter node of type Tree Node will be tested.
  // When the command is invoked via the menu popup, this node will be the highlighted node, and not the selected node, the latter will undefined.
  if (snippetsExplorer.selection.length === 0 && !node) {
    snippetsProvider.addSnippetFolder(name, Snippet.rootParentId);
  } else {
    const selectedItem = node ? node : snippetsExplorer.selection[0];
    if (Snippet.isFolder(selectedItem)) {
      snippetsProvider.addSnippetFolder(name, selectedItem.id);
    } else {
      snippetsProvider.addSnippetFolder(
        name,
        selectedItem.parentId ?? Snippet.rootParentId
      );
    }
  }
}

export function editSnippet(
  context: vscode.ExtensionContext,
  snippet: Snippet,
  snippetsProvider: SnippetsProvider
) {
  // Enable syntax resolving by default if property is not yet defined in JSON
  if (snippet.resolveSyntax === undefined) {
    snippet.resolveSyntax = true;
  }
  // Create and show a new webview for editing snippet
  new EditSnippet(context, snippet, snippetsProvider);
}

export async function exportSnippets(snippetsProvider: SnippetsProvider) {
  // Get snippet destination
  vscode.window
    .showSaveDialog({
      filters: {
        // eslint-disable-next-line @typescript-eslint/naming-convention
        JSON: ["json"],
      },
      title: "Export Snippets",
      saveLabel: "Export",
    })
    .then((fileUri) => {
      if (fileUri && fileUri.fsPath) {
        snippetsProvider.exportSnippets(fileUri.fsPath);
      }
    });
}

export async function importSnippets(snippetsProvider: SnippetsProvider) {
  // Get snippets destination
  vscode.window
    .showOpenDialog({
      canSelectFiles: true,
      canSelectFolders: false,
      canSelectMany: false,
      filters: {
        // eslint-disable-next-line @typescript-eslint/naming-convention
        JSON: ["json"],
      },
      openLabel: "Import",
      title: "Import Snippets",
    })
    .then((fileUris) => {
      if (fileUris && fileUris[0] && fileUris[0].fsPath) {
        vscode.window
          .showWarningMessage(
            Labels.snippetImportRequestConfirmation,
            ...[Labels.importSnippets, Labels.discardImport]
          )
          .then((selection) => {
            switch (selection) {
              case Labels.importSnippets:
                if (snippetsProvider.importSnippets(fileUris[0].fsPath)) {
                  vscode.window.showInformationMessage(Labels.snippetsImported);
                } else {
                  vscode.window.showErrorMessage(Labels.snippetsNotImported);
                }
              case Labels.discardImport:
                break;
            }
          });
      }
    });
}
