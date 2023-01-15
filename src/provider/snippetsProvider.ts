import * as vscode from "vscode";
import fs = require("fs");
import * as path from "path";
import { Snippet } from "../types/snippet";
import { CommandsConsts } from "../config/commands";
import { SnippetService } from "../service/snippetService";

export class SnippetsProvider implements vscode.TreeDataProvider<Snippet> {
  constructor(
    private _snippetService: SnippetService,
    private _extensionPath: string
  ) {}

  getTreeItem(element: Snippet): vscode.TreeItem {
    return this.snippetToTreeItem(element);
  }

  getChildren(element?: Snippet): Thenable<Snippet[]> {
    if (element) {
      return Promise.resolve(element.children);
    } else {
      return Promise.resolve(this._snippetService.getRootChildren());
    }
  }

  private _onDidChangeTreeData: vscode.EventEmitter<
    Snippet | undefined | null | void
  > = new vscode.EventEmitter<Snippet | undefined | null | void>();
  readonly onDidChangeTreeData: vscode.Event<
    Snippet | undefined | null | void
  > = this._onDidChangeTreeData.event;

  // Only read from data file
  refresh(): void {
    this._snippetService.refresh();
    this._onDidChangeTreeData.fire();
  }

  // Save state of snippets, then refresh
  sync(): void {
    this._snippetService.saveSnippets();
    this.refresh();
  }

  addSnippet(name: string, snippet: string, parentId: number) {
    let lastId = this._snippetService.incrementLastId();

    this._snippetService.addSnippet({
      id: lastId,
      parentId: parentId,
      label: name,
      type: 2,
      content: snippet.split("\n"),
      description: name,
    } as Snippet);

    this.sync();
  }

  addSnippetFolder(name: string, parentId: number) {
    let lastId = this._snippetService.incrementLastId();

    this._snippetService.addSnippet({
      id: lastId,
      parentId: parentId,
      label: name,
      type: 1,
    } as Snippet);

    this.sync();
  }

  editSnippet(snippet: any) {
    this._snippetService.updateSnippet(snippet);

    this.sync();
  }

  editSnippetFolder(snippet: Snippet) {
    this._snippetService.updateSnippet(snippet);

    this.sync();
  }

  removeSnippet(snippet: Snippet) {
    this._snippetService.removeSnippet(snippet);

    this.sync();
  }

  moveSnippetUp(snippet: Snippet) {
    this._snippetService.moveSnippet(snippet, -1);

    this.sync();
  }

  moveSnippetDown(snippet: Snippet) {
    this._snippetService.moveSnippet(snippet, 1);

    this.sync();
  }

  private snippetToTreeItem(snippet: Snippet): vscode.TreeItem {
    let treeItem = new vscode.TreeItem(
      snippet.label,
      Snippet.isFolder(snippet)
        ? vscode.TreeItemCollapsibleState.Expanded
        : vscode.TreeItemCollapsibleState.None
    );

    // TreeItem.description = "";
    // Dynamic context value depending on item type (snippet or snippet folder)
    // Context value is used in view/item/context in 'when' condition
    if (Snippet.isFolder(snippet)) {
      treeItem.contextValue = "snippetFolder";
      treeItem.iconPath = {
        light: path.join(
          this._extensionPath,
          "resources",
          "icons",
          "light",
          "folder.svg"
        ),
        dark: path.join(
          this._extensionPath,
          "resources",
          "icons",
          "dark",
          "folder.svg"
        ),
      };
    } else {
      const maxLength = 20;
      const value = Snippet.getContent(snippet);
      treeItem.tooltip = `${
        value
          ? "'" +
            value.replace("\n", "").slice(0, maxLength) +
            (value.length > 20 ? "..." : "") +
            "'"
          : "''"
      }`;
      treeItem.contextValue = "snippet";

      let icon = "logo";

      switch (snippet.type) {
        case 1: icon = "folder"; break;
        case 2: icon = "code"; break;
        case 3: icon = "rust"; break;
        case 4: icon = "terminal"; treeItem.contextValue += "Terminal"; break;
        case 5: icon = "info"; treeItem.contextValue += "Info";break;
      }

      icon += ".svg";

      treeItem.iconPath = {
        light: path.join(
          this._extensionPath,
          "resources",
          "icons",
          "light",
          icon
        ),
        dark: path.join(
          this._extensionPath,
          "resources",
          "icons",
          "dark",
          icon
        ),
      };

      // Conditional in configuration
      if (!Snippet.isTerminalSnippet(snippet)) {
        treeItem.command = {
          command: CommandsConsts.commonOpenSnippet,
          arguments: [snippet],
          title: "Open Snippet",
        };
      } else if (!Snippet.isComment(snippet)) {
        treeItem.command = {
          command: CommandsConsts.commonOpenSnippetInTerminal,
          arguments: [snippet],
          title: "Open Snippet In Terminal",
        };
      }
    }
    // Get parent element
    const parentElement = this._snippetService.getParent(snippet.parentId);
    if (parentElement && parentElement.children) {
      const childrenCount = parentElement.children.length;
      // Show order actions only if there is room for reorder
      if (childrenCount > 1) {
        const index = parentElement.children.findIndex(
          (obj) => obj.id === snippet.id
        );
        if (index > 0 && index < childrenCount - 1) {
          treeItem.contextValue = `${treeItem.contextValue}:up&down`;
        } else if (index === 0) {
          treeItem.contextValue = `${treeItem.contextValue}:down`;
        } else if (index === childrenCount - 1) {
          treeItem.contextValue = `${treeItem.contextValue}:up`;
        }
      }
    }
    return treeItem;
  }

  exportSnippets(destinationPath: string) {
    this._snippetService.exportSnippets(destinationPath, Snippet.rootParentId);
    this.sync();
  }

  importSnippets(destinationPath: string): boolean {
    this._snippetService.importSnippets(destinationPath);
    this.sync();
    const parentElt = this._snippetService.getParent(undefined);
    return (
      parentElt !== undefined &&
      parentElt.children !== undefined &&
      parentElt.children.length > 0
    );
  }
}
