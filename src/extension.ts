import fs = require("fs");
import * as vscode from "vscode";
import * as path from "path";
import * as commands from "./config/commands";
import { SnippetsProvider } from "./provider/snippetsProvider";
import { MementoDataAccess } from "./data/mementoDataAccess";
import { Snippet } from "./types/snippet";
import { EditSnippetFolder } from "./views/editSnippetFolder";
import { SnippetService } from "./service/snippetService";
import { UIUtility } from "./utils/uiUtility";
import { StringUtility } from "./utils/stringUtility";
import { Labels } from "./config/Labels";
import { FileDataAccess } from "./data/fileDataAccess";
import { SnippetCompletionProvider } from "./provider/completionProvider";

export const snippetsConfigKey = "snippets";
const useWorkspaceFolderKey = "useWorkspaceFolder";
const workspaceFileName = ".vscode/samples.json";

const setContextCmd = "setContext";
const contextWSStateKey = "snippets.workspaceState";
const contextWSFileAvailable = "fileAvailable";
const contextWSFileNotAvailable = "fileNotAvailable";

/**
 * Activate extension by initializing views for snippets and feature commands.
 * @param context
 */
export function activate(context: vscode.ExtensionContext) {
  // Main variables
  let workspaceSnippetsAvailable = false;
  let wsSnippetService: SnippetService;
  let wsSnippetsProvider: SnippetsProvider;
  let wsSnippetsExplorer: vscode.TreeView<Snippet>;
  let snippetsPath: string;

  // Sync global snippets
  context.globalState.setKeysForSync([MementoDataAccess.snippetsMementoPrefix]);

  // Initialize global snippets
  const dataAccess = new MementoDataAccess(context.globalState);
  const snippetService = new SnippetService(dataAccess);
  const snippetsProvider = new SnippetsProvider(
    snippetService,
    context.extensionPath
  );
  let cipDisposable: { dispose(): any };

  // Initialization

  // Refresh windows whenever it gains focus
  // It prevents de-sync between multiple open workspaces
  vscode.window.onDidChangeWindowState((event) => {
    if (event.focused) {
      refreshUI();
    }
  });

  // Refresh UI when updating workspace setting
  vscode.workspace.onDidChangeConfiguration((event) => {
    const affected = event.affectsConfiguration(
      `${snippetsConfigKey}.${useWorkspaceFolderKey}`
    );
    if (affected) {
      if (
        vscode.workspace
          .getConfiguration(snippetsConfigKey)
          .get(useWorkspaceFolderKey)
      ) {
        requestWSConfigSetup();
      }
      refreshUI();
    }
  });

  // Add snippets explorer UI
  const snippetsExplorer = vscode.window.createTreeView("snippetsExplorer", {
    treeDataProvider: snippetsProvider,
    showCollapseAll: true,
  });

  // Refresh UI to initialize all required config for workspace panel
  requestWSConfigSetup(false);

  // Core

  // Initialize generic completion item provider
  const triggerCharacter: any =
    vscode.workspace.getConfiguration(snippetsConfigKey).get("triggerKey") ||
    "__";

  const snippetCompletionProvider = new SnippetCompletionProvider(
    triggerCharacter,
    snippetService,
    workspaceSnippetsAvailable
  );

  const registerCIPSnippets: any = () =>
    (cipDisposable = vscode.languages.registerCompletionItemProvider(
      "*",
      snippetCompletionProvider,
      triggerCharacter
    ));

  context.subscriptions.push(registerCIPSnippets());

  function refreshUI() {
    cipDisposable?.dispose();
    snippetsProvider.refresh();

    // Re-check if .vscode/snippets.json is always available
    // Use case: when deleting file after enabling workspace in settings
    requestWSConfigSetup(false);

    if (workspaceSnippetsAvailable) {
      wsSnippetsProvider.refresh();
    } else {
      vscode.commands.executeCommand(
        setContextCmd,
        contextWSStateKey,
        contextWSFileNotAvailable
      );
    }

    registerCIPSnippets();
  }

  async function requestWSConfigSetup(requestInput = true) {
    if (
      vscode.workspace.workspaceFolders &&
      vscode.workspace
        .getConfiguration(snippetsConfigKey)
        .get(useWorkspaceFolderKey)
    ) {
      snippetsPath = path.join(
        vscode.workspace.workspaceFolders[0].uri.fsPath,
        workspaceFileName
      );

      // Request creation of file `.vscode/snippets.json` if:
      // - File not found
      // - User didn't request to ignore this phase (information persisted at workspace level)
      const ignoreCreateSnippetsFileKey = "ignoreCreateSnippetsFile";
      const ignoreCreateSnippetsFile = context.workspaceState.get<boolean>(
        ignoreCreateSnippetsFileKey
      );

      const snippetsPathExists = fs.existsSync(snippetsPath);

      // Condition `requestInput=true` means that we user requested setup
      // Extension will ask for user feedback if file is unavailable
      if (requestInput && !ignoreCreateSnippetsFile && !snippetsPathExists) {
        await vscode.window
          .showWarningMessage(
            Labels.snippetsWorkspaceCreateFileRequest,
            Labels.snippetsWorkspaceCreateFileRequestConfirm,
            Labels.snippetsWorkspaceCreateFileRequestIgnore
          )
          .then((selection) => {
            if (
              selection === Labels.snippetsWorkspaceCreateFileRequestConfirm
            ) {
              // Create parent folder if it doesn't exist (.vscode/)
              const snippetsPathParent = path.dirname(snippetsPath);
              if (!fs.existsSync(snippetsPathParent)) {
                fs.mkdirSync(snippetsPathParent);
              }
              // Create empty file
              fs.closeSync(fs.openSync(snippetsPath, "w"));
              // Mark useWorkspaceFolder as enabled
              workspaceSnippetsAvailable = true;
              snippetCompletionProvider.setWorkspaceSnippetsAvailable(true);
            } else if (
              selection === Labels.snippetsWorkspaceCreateFileRequestIgnore
            ) {
              // Ignore at workspace level
              context.workspaceState.update(ignoreCreateSnippetsFileKey, true);
            }
          });
      } else if (snippetsPathExists) {
        // File already exists, just mark useWorkspaceFolder as enabled
        workspaceSnippetsAvailable = true;
        snippetCompletionProvider.setWorkspaceSnippetsAvailable(true);
      } else {
        workspaceSnippetsAvailable = false;
        snippetCompletionProvider.setWorkspaceSnippetsAvailable(false);
      }

      // Finish with a boolean to detect if we're using workspaceFolder (option enabled + workspace open + snippets.json available)
      if (workspaceSnippetsAvailable) {
        // Send flag to context in order to change viewWelcome (see contributes > viewsWelcome in package.json)
        vscode.commands.executeCommand(
          setContextCmd,
          contextWSStateKey,
          contextWSFileAvailable
        );

        // Initialize workspace snippets
        if (!wsSnippetsExplorer) {
          const wsDataAccess = new FileDataAccess(snippetsPath);
          wsSnippetService = new SnippetService(wsDataAccess);
          snippetCompletionProvider.setWsSnippetService(wsSnippetService);
          wsSnippetsProvider = new SnippetsProvider(
            wsSnippetService,
            context.extensionPath
          );

          wsSnippetsExplorer = vscode.window.createTreeView(
            "wsSnippetsExplorer",
            {
              treeDataProvider: wsSnippetsProvider,
              showCollapseAll: true,
            }
          );
        }
      } else {
        vscode.commands.executeCommand(
          setContextCmd,
          contextWSStateKey,
          contextWSFileNotAvailable
        );
      }
    } else {
      workspaceSnippetsAvailable = false;
    }
  }

  // Generic error handler for most commands
  function handleCommand(callback: (...args: any[]) => any) {
    new Promise(callback).catch((error) => {
      vscode.window.showErrorMessage(
        StringUtility.formatString(Labels.genericError, error.message)
      );
      refreshUI();
    });
  }

  // Commands
  const registerCommand = (commands: any[]) => {
    for (const [command, callback] of commands) {
      context.subscriptions.push(
        vscode.commands.registerCommand(command, callback)
      );
    }
  };

  // Initialize workspace (WS) config
  // Check if a workspace is open and if useWorkspaceFolder is enabled
  registerCommand([
    [commands.CommandsConsts.miscRequestWSConfig, requestWSConfigSetup],
  ]);

  // Open snippet
  registerCommand([
    [
      commands.CommandsConsts.commonOpenSnippet,
      async (snippet?: Snippet) =>
        handleCommand(async () => {
          const editor = vscode.window.activeTextEditor;
          if (!editor) {
            vscode.window.showInformationMessage(Labels.noOpenEditor);
            return;
          }
          // If command is not triggered from treeView, a snippet must be selected by final user
          if (!snippet) {
            let allSnippets = snippetService.getAllSnippets();
            if (workspaceSnippetsAvailable) {
              allSnippets = allSnippets.concat(
                wsSnippetService.getAllSnippets()
              );
            }
            snippet = await UIUtility.requestSnippetFromUser(allSnippets);
          }
          if (!snippet) {
            return;
          }
          // Enable syntax resolving by default if property is not yet defined in JSON
          if (snippet.resolveSyntax === undefined) {
            snippet.resolveSyntax = true;
          }
          // Insert snippet
          if (snippet.resolveSyntax) {
            vscode.commands.executeCommand("editor.action.insertSnippet", {
              snippet: Snippet.getContent(snippet),
            });
          } else {
            editor.edit((edit) => {
              if (!snippet || !Snippet.isSnippet(snippet)) {
                vscode.window.showInformationMessage(
                  Labels.snippetIncorrectType
                );
                return;
              }
              edit.insert(editor.selection.start, Snippet.getContent(snippet));
            });
          }

          vscode.window.showTextDocument(editor.document);
        }),
    ],
    [
      commands.CommandsConsts.commonOpenSnippetInTerminal,
      async (snippet?: Snippet) =>
        handleCommand(async () => {
          const terminal = vscode.window.activeTerminal;
          if (!terminal) {
            vscode.window.showInformationMessage(Labels.noOpenTerminal);
            return;
          }
          // If command is not triggered from treeView, a snippet must be selected by final user
          if (!snippet) {
            let allSnippets = snippetService.getAllSnippets();
            if (workspaceSnippetsAvailable) {
              allSnippets = allSnippets.concat(
                wsSnippetService.getAllSnippets()
              );
            }
            snippet = await UIUtility.requestSnippetFromUser(allSnippets);
          }
          if (!snippet || !Snippet.isTerminalSnippet(snippet)) {
            vscode.window.showInformationMessage(Labels.snippetIncorrectType);
            return;
          }
          terminal.sendText(Snippet.getContent(snippet));
        }),
    ],
    // Add snippet
    [
      commands.CommandsConsts.commonAddSnippet,
      async () =>
        handleCommand(() =>
          commands.commonAddSnippet(
            snippetsProvider,
            wsSnippetsProvider,
            workspaceSnippetsAvailable
          )
        ),
    ],
    [
      commands.CommandsConsts.globalAddSnippet,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippet(snippetsExplorer, snippetsProvider, node)
        ),
    ],
    [
      commands.CommandsConsts.wsAddSnippet,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippet(wsSnippetsExplorer, wsSnippetsProvider, node)
        ),
    ],
    // Add snippet from clipboard
    [
      commands.CommandsConsts.commonAddSnippetFromClipboard,
      async () =>
        handleCommand(() =>
          commands.commonAddSnippetFromClipboard(
            snippetsProvider,
            wsSnippetsProvider,
            workspaceSnippetsAvailable
          )
        ),
    ],
    [
      commands.CommandsConsts.globalAddSnippetFromClipboard,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippetFromClipboard(
            snippetsExplorer,
            snippetsProvider,
            node
          )
        ),
    ],
    [
      commands.CommandsConsts.wsAddSnippetFromClipboard,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippetFromClipboard(
            wsSnippetsExplorer,
            wsSnippetsProvider,
            node
          )
        ),
    ],
    // Add snippet folder
    [
      commands.CommandsConsts.commonAddSnippetFolder,
      async () =>
        handleCommand(() =>
          commands.commonAddSnippetFolder(
            snippetsProvider,
            wsSnippetsProvider,
            workspaceSnippetsAvailable
          )
        ),
    ],
    [
      commands.CommandsConsts.globalAddSnippetFolder,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippetFolder(snippetsExplorer, snippetsProvider, node)
        ),
    ],
    [
      commands.CommandsConsts.wsAddSnippetFolder,
      async (node: any) =>
        handleCommand(() =>
          commands.addSnippetFolder(
            wsSnippetsExplorer,
            wsSnippetsProvider,
            node
          )
        ),
    ],
    // Edit snippet
    [
      commands.CommandsConsts.globalEditSnippet,
      (snippet: Snippet) =>
        handleCommand(() =>
          commands.editSnippet(context, snippet, snippetsProvider)
        ),
    ],
    [
      commands.CommandsConsts.wsEditSnippet,
      (snippet: Snippet) =>
        handleCommand(() =>
          commands.editSnippet(context, snippet, wsSnippetsProvider)
        ),
    ],
    // Edit snippet folder
    [
      commands.CommandsConsts.globalEditSnippetFolder,
      (snippet: Snippet) =>
        handleCommand(
          () => new EditSnippetFolder(context, snippet, snippetsProvider)
        ),
    ],
    [
      commands.CommandsConsts.wsEditSnippetFolder,
      (snippet: Snippet) =>
        handleCommand(
          () => new EditSnippetFolder(context, snippet, wsSnippetsProvider)
        ),
    ],
    // Remove snippet
    [
      commands.CommandsConsts.globalDeleteSnippet,
      (snippet: Snippet) =>
        handleCommand(() => {
          if (
            vscode.workspace
              .getConfiguration("snippets")
              .get("confirmBeforeDeletion")
          ) {
            vscode.window
              .showInformationMessage(
                `Do you really want to delete the snippet (${snippet.label}) ?`,
                Labels.confirmationYes,
                Labels.confirmationNo
              )
              .then((answer) => {
                if (answer === Labels.confirmationYes) {
                  snippetsProvider.removeSnippet(snippet);
                }
              });
          } else {
            snippetsProvider.removeSnippet(snippet);
          }
        }),
    ],
    [
      commands.CommandsConsts.wsDeleteSnippet,
      (snippet: Snippet) =>
        handleCommand(() => {
          if (
            vscode.workspace
              .getConfiguration("snippets")
              .get("confirmBeforeDeletion")
          ) {
            vscode.window
              .showInformationMessage(
                `Do you really want to delete the snippet (${snippet.label}) ?`,
                Labels.confirmationYes,
                Labels.confirmationNo
              )
              .then((answer) => {
                if (answer === Labels.confirmationYes) {
                  wsSnippetsProvider.removeSnippet(snippet);
                }
              });
          } else {
            wsSnippetsProvider.removeSnippet(snippet);
          }
        }),
    ],
    // Remove snippet folder
    [
      commands.CommandsConsts.globalDeleteSnippetFolder,
      (snippetFolder: Snippet) =>
        handleCommand(() => {
          if (
            vscode.workspace
              .getConfiguration("snippets")
              .get("confirmBeforeDeletion")
          ) {
            vscode.window
              .showInformationMessage(
                `Do you really want to delete the folder (${snippetFolder.label}) ?`,
                Labels.confirmationYes,
                Labels.confirmationNo
              )
              .then((answer) => {
                if (answer === Labels.confirmationYes) {
                  snippetsProvider.removeSnippet(snippetFolder);
                }
              });
          } else {
            snippetsProvider.removeSnippet(snippetFolder);
          }
        }),
    ],
    [
      commands.CommandsConsts.wsDeleteSnippetFolder,
      (snippetFolder: Snippet) =>
        handleCommand(() => {
          if (
            vscode.workspace
              .getConfiguration("snippets")
              .get("confirmBeforeDeletion")
          ) {
            vscode.window
              .showInformationMessage(
                `Do you really want to delete the folder (${snippetFolder.label}) ?`,
                Labels.confirmationYes,
                Labels.confirmationNo
              )
              .then((answer) => {
                if (answer === Labels.confirmationYes) {
                  wsSnippetsProvider.removeSnippet(snippetFolder);
                }
              });
          } else {
            wsSnippetsProvider.removeSnippet(snippetFolder);
          }
        }),
    ],
    // Move snippet up
    [
      commands.CommandsConsts.globalMoveSnippetUp,
      (snippet: Snippet) =>
        handleCommand(() => snippetsProvider.moveSnippetUp(snippet)),
    ],
    [
      commands.CommandsConsts.wsMoveSnippetUp,
      (snippet: Snippet) =>
        handleCommand(() => wsSnippetsProvider.moveSnippetUp(snippet)),
    ],
    // Move snippet down
    [
      commands.CommandsConsts.globalMoveSnippetDown,
      (snippet: Snippet) =>
        handleCommand(() => snippetsProvider.moveSnippetDown(snippet)),
    ],
    [
      commands.CommandsConsts.wsMoveSnippetDown,
      (snippet: Snippet) =>
        handleCommand(() => wsSnippetsProvider.moveSnippetDown(snippet)),
    ],
    // Refresh UI
    [commands.CommandsConsts.commonRefreshUI, () => refreshUI()],
    // Import
    [
      commands.CommandsConsts.globalExportSnippets,
      async () =>
        handleCommand(() => commands.exportSnippets(snippetsProvider)),
    ],
    // Export
    [
      commands.CommandsConsts.globalImportSnippets,
      async () =>
        handleCommand(() => commands.importSnippets(snippetsProvider)),
    ],
    // Reset
    [
      commands.CommandsConsts.globalResetSnippets,
      async () => 
        handleCommand(() => {
          vscode.window
            .showInformationMessage(
              `Do you really want to reset the snippets?`,
              Labels.confirmationYes,
              Labels.confirmationNo
            )
            .then((answer) => {
              if (answer === Labels.confirmationYes) {
                snippetsProvider.resetSnippets();
              }
            });
        })
    ],
  ]);
}

// eslint-disable-next-line @typescript-eslint/no-empty-function
export function deactivate() {}
