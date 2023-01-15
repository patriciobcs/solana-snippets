import * as vscode from "vscode";
import { Snippet } from "../types/snippet";
import { SnippetsProvider } from "../provider/snippetsProvider";
import { EditView } from "./editView";

export class EditSnippet extends EditView {
  constructor(
    context: vscode.ExtensionContext,
    private _snippet: Snippet,
    private _snippetsProvider: SnippetsProvider
  ) {
    super(context, _snippet, "editSnippet", "file", "Edit Snippet");
  }

  handleReceivedMessage(message: any): any {
    switch (message.command) {
      case "edit-snippet":
        const { label, content, description, requires, resolveSyntax } =
          message.data;

        let snippetChanges: any = { id: this._snippet.id };

        if (this._snippet.parentId) {
          snippetChanges.parentId = this._snippet.parentId;
        }
        // Call provider only if there is data change
        if (label !== undefined) {
          snippetChanges.label = label;
        }
        if (content !== undefined) {
          snippetChanges.content =
            content.length > 0 ? content.split("/n") : [];
        }
        if (description !== undefined) {
          snippetChanges.description = description;
        }
        if (requires !== undefined) {
          snippetChanges.requires =
            requires.length > 0 ? requires.split("/n") : [];
        }

        // Test against undefined so we don't mess with variable's state if user introduces an explicit value 'false'
        if (resolveSyntax !== undefined) {
          snippetChanges.resolveSyntax = resolveSyntax;
        }

        this._snippetsProvider.editSnippet(snippetChanges);
        this._panel.dispose();
        return;
    }
  }
}
