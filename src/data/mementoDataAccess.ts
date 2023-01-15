import { Memento } from "vscode";
import { Snippet } from "../types/snippet";
import { DataAccess, DataAccessConsts } from "./dataAccess";
import defaultSnippetsRaw from "./model.json";

const defaultSnippets = defaultSnippetsRaw as Snippet;

export class MementoDataAccess implements DataAccess {
  static snippetsMementoPrefix = "snippetsData";
  private _storageManager: StorageManager;

  constructor(memento: Memento) {
    this._storageManager = new StorageManager(memento);
  }

  hasNoChild(): boolean {
    let rootElt =
      this._storageManager.getValue<Snippet>(
        MementoDataAccess.snippetsMementoPrefix
      ) || DataAccessConsts.defaultRootElement;
    return !rootElt.children || rootElt.children.length === 0;
  }

  load(): Snippet {
    let rawData =
      this._storageManager.getValue<Snippet>(
        MementoDataAccess.snippetsMementoPrefix
      ) || DataAccessConsts.defaultRootElement;

    if (rawData === DataAccessConsts.defaultRootElement) {
      this.save(rawData);
    }

    // TODO: Find a better way to handle this (testing)
    if (!rawData.children || rawData.children.length === 0) {
      this.save(defaultSnippets);
      rawData = defaultSnippets;
    }

    console.log("memento", rawData);

    return rawData;
  }

  save(data: Snippet): void {
    this._storageManager.setValue<Snippet>(
      MementoDataAccess.snippetsMementoPrefix,
      data
    );
  }
}

class StorageManager {
  constructor(private storage: Memento) {}

  public getValue<T>(key: string): T | undefined {
    return this.storage.get<T | undefined>(key, undefined);
  }

  public setValue<T>(key: string, value: T) {
    this.storage.update(key, value);
  }
}
