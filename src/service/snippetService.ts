import { DataAccess } from "../data/dataAccess";
import { FileDataAccess } from "../data/fileDataAccess";
import { Snippet } from "../types/snippet";
import defaultSnippetsRaw from "../data/default.json";

const defaultSnippets = defaultSnippetsRaw as Snippet;
export class SnippetService {
  private _rootSnippet: Snippet;

  constructor(private _dataAccess: DataAccess) {
    this._rootSnippet = this.loadSnippets();
  }

  // Static utility methods

  static findParent(
    parentId: number,
    currentElt: Snippet
  ): Snippet | undefined {
    let i, currentChild, result;

    if (parentId === currentElt.id) {
      return currentElt;
    } else if (currentElt.children) {
      // Use a for loop instead of forEach to avoid nested functions
      // Otherwise "return" will not work properly
      for (i = 0; i < currentElt.children.length; i++) {
        currentChild = currentElt.children[i];

        // Search in the current child
        result = this.findParent(parentId, currentChild);

        // Return the result if the node has been found
        if (result !== undefined) {
          return result;
        }
      }

      // The node has not been found and we have no more options
      return undefined;
    }
  }

  /**
   * To be used like the following:
   * let result: any[] = [];
   * Snippet.flatten(snippetsProvider.snippets.children, result);
   * @param arr array of element
   * @param result final result
   */
  private static flatten(arr: any, result: any[] = []) {
    for (let i = 0, length = arr.length; i < length; i++) {
      const value = arr[i];
      if (Snippet.isFolder(value) && value.children.length > 0) {
        SnippetService.flatten(value.children, result);
      } else {
        result.push(value);
      }
    }
    return result;
  }

  // Private methods

  private _reorderArray(arr: Snippet[], oldIndex: number, newIndex: number) {
    if (newIndex < arr.length) {
      arr.splice(newIndex, 0, arr.splice(oldIndex, 1)[0]);
    }
  }

  private _updateLastId(newId: number): void {
    this._rootSnippet.lastId = newId;
  }

  // Public service methods

  refresh(): void {
    this._rootSnippet = this.loadSnippets();
  }

  loadSnippets(): Snippet {
    return this._dataAccess.load();
  }

  saveSnippets(): void {
    this._dataAccess.save(this._rootSnippet);
  }

  getRootChildren(): Snippet[] {
    return this._rootSnippet.children;
  }

  getAllSnippets(): Snippet[] {
    // Sync snippets
    this._rootSnippet = this.loadSnippets();
    const snippets: Snippet[] = [];
    SnippetService.flatten(this._rootSnippet.children, snippets);
    return snippets;
  }

  incrementLastId(): number {
    return (this._rootSnippet.lastId ?? 0) + 1;
  }

  getParent(parentId: number | undefined): Snippet | undefined {
    return SnippetService.findParent(
      parentId ?? Snippet.rootParentId,
      this._rootSnippet
    );
  }

  compact(): string {
    return JSON.stringify(this._rootSnippet);
  }

  // Snippet management services

  addSnippet(newSnippet: Snippet): void {
    newSnippet.parentId === Snippet.rootParentId
      ? this._rootSnippet.children.push(newSnippet)
      : SnippetService.findParent(
          newSnippet.parentId ?? Snippet.rootParentId,
          this._rootSnippet
        )?.children.push(newSnippet);

    this._updateLastId(newSnippet.id);
  }

  updateSnippet(snippetChanges: any): void {
    const parentElement = SnippetService.findParent(
      snippetChanges.parentId ?? Snippet.rootParentId,
      this._rootSnippet
    );

    if (parentElement) {
      const index = parentElement.children.findIndex(
        (obj) => obj.id === snippetChanges.id
      );

      if (index > -1) {
        parentElement.children = parentElement.children.map((obj) =>
          obj.id === snippetChanges.id
            ? {
                ...obj,
                ...snippetChanges,
              }
            : obj
        );
      }
    }
  }

  removeSnippet(snippet: Snippet): void {
    const parentElement = SnippetService.findParent(
      snippet.parentId ?? Snippet.rootParentId,
      this._rootSnippet
    );

    if (parentElement) {
      const index = parentElement.children.findIndex(
        (obj) => obj.id === snippet.id
      );

      if (index > -1) {
        parentElement?.children.splice(index, 1);
      }
    }
  }

  moveSnippet(snippet: Snippet, offset: number) {
    const parentElement = SnippetService.findParent(
      snippet.parentId ?? Snippet.rootParentId,
      this._rootSnippet
    );

    if (parentElement) {
      const index = parentElement.children.findIndex(
        (obj) => obj.id === snippet.id
      );

      if (index > -1 && parentElement.children) {
        this._reorderArray(parentElement.children, index, index + offset);
      }
    }
  }

  exportSnippets(destinationPath: string, parentId: number) {
    const parentElement = SnippetService.findParent(
      parentId ?? Snippet.rootParentId,
      this._rootSnippet
    );
    if (parentElement) {
      // save file using destroyable instance of FileDataAccess
      new FileDataAccess(destinationPath).save(parentElement);
    }
  }

  importSnippets(destinationPath: string) {
    // Save a backup version of current snippets next to the file to import
    this.exportSnippets(
      destinationPath.replace(
        FileDataAccess.dataFileExt,
        `_pre-import-bak${FileDataAccess.dataFileExt}`
      ),
      Snippet.rootParentId
    );
    const newSnippets: Snippet = new FileDataAccess(destinationPath).load();
    this._rootSnippet.children = newSnippets.children;
    this._rootSnippet.lastId = newSnippets.lastId;
  }

  async resetSnippets() {
    this._dataAccess.reset();
    this.refresh();
  }
}
