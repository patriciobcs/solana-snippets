import { match } from "assert";
import * as vscode from "vscode";
import { snippetsConfigKey } from "../extension";
import { Snippet } from "../types/snippet";

export class SnippetCompletionProvider {
  triggerCharacter: string;
  snippetService: any;
  workspaceSnippetsAvailable: boolean | undefined;
  wsSnippetService: any;

  constructor(
    triggerCharacter: string,
    snippetService: any,
    workspaceSnippetsAvailable?: boolean
  ) {
    this.triggerCharacter = triggerCharacter;
    this.snippetService = snippetService;
    this.workspaceSnippetsAvailable = workspaceSnippetsAvailable;
  }

  setWorkspaceSnippetsAvailable(workspaceSnippetsAvailable: boolean) {
    this.workspaceSnippetsAvailable = workspaceSnippetsAvailable;
  }

  setWsSnippetService(wsSnippetService: any) {
    this.wsSnippetService = wsSnippetService;
  }

  provideCompletionItems(document: any, position: any) {
    if (
      !vscode.workspace
        .getConfiguration(snippetsConfigKey)
        .get("showSuggestions")
    ) {
      return;
    }
    const isTriggeredByChar: boolean =
      this.triggerCharacter ===
      document.lineAt(position).text.charAt(position.character - 1);
    // Append workspace snippets if WS is available
    let candidates = this.snippetService.getAllSnippets();

    if (this.workspaceSnippetsAvailable) {
      // Add suffix for all workspace items
      candidates = candidates.concat(
        this.wsSnippetService.getAllSnippets().map((elt: Snippet) => {
          elt.label = `${elt.label}__(ws)`;
          return elt;
        })
      );
    }

    const vsCodeSnippets: vscode.CompletionItem[] = [];
    const rustAnalyzerSnippets: any = {};

    candidates.map((element: Snippet) => {
      switch (element.type) {
        case 2:
          vsCodeSnippets.push(
            this.getVSCodeSnippetCompletion(
              element,
              isTriggeredByChar,
              position
            )
          );
          break;
        case 3:
          rustAnalyzerSnippets[element.label] =
            this.getRustAnalyzerSnippetCompletion(element);
          break;
      }
    });

    this.provideRustCompletionItems(rustAnalyzerSnippets);

    return vsCodeSnippets;
  }

  getVSCodeSnippetCompletion(
    element: Snippet,
    isTriggeredByChar: boolean,
    position: any
  ): vscode.CompletionItem {
    return <vscode.CompletionItem>{
      label: `sol:${element.label
        .replace("\n", "")
        .replace(" ", "-")
        .replace("__(ws)", " (ws)")}`,
      insertText: new vscode.SnippetString(Snippet.getContent(element)),
      detail: element.label.replace("__(ws)", " (snippet from workspace)"),
      kind: vscode.CompletionItemKind.Snippet,
      // Replace trigger character with the chosen suggestion
      additionalTextEdits: isTriggeredByChar
        ? [
            vscode.TextEdit.delete(
              new vscode.Range(
                position.with(position.line, position.character - 1),
                position
              )
            ),
          ]
        : [],
    };
  }

  getRustAnalyzerSnippetCompletion(element: Snippet) {
    return {
      prefix: `sol:${element.label
        .replace("\n", "")
        .replace(" ", "-")
        .replace("__(ws)", " (ws)")}`,
      body: element.content,
      requires: element.requires,
      description: element.label.replace("__(ws)", " (workspace)"),
      scope: "expr",
    };
  }

  provideRustCompletionItems(snippets: any) {
    console.log("provideRustCompletionItems");
    const rustAnalyzerCompletionSnippetsCustom =
      "rust-analyzer.completion.snippets.custom";
    const configuration = vscode.workspace.getConfiguration();
    const currentConfig = configuration.get(
      rustAnalyzerCompletionSnippetsCustom
    );
    const newConfig = snippets;
    if (currentConfig === newConfig) {
      return;
    }
    const target = vscode.ConfigurationTarget.Global;
    const overrideInLanguage = true;

    configuration.update(
      rustAnalyzerCompletionSnippetsCustom,
      newConfig,
      target,
      overrideInLanguage
    );
  }
}
