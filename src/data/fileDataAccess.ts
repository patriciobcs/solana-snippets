import fs = require("fs");
import * as path from "path";
import { Snippet } from "../types/snippet";
import { DataAccess, DataAccessConsts } from "./dataAccess";

export class FileDataAccess implements DataAccess {
  static dataFileExt = ".json";
  private static dataFileName = `data${FileDataAccess.dataFileExt}`;

  private _dataFile: string;

  constructor(dataFile: string) {
    this._dataFile = dataFile;
  }

  hasNoChild(): boolean {
    const rootElt = this.load();
    return (
      rootElt instanceof Snippet &&
      (!rootElt.children || rootElt.children.length === 0)
    );
  }

  setDataFile(dataFile: string) {
    this._dataFile = dataFile;
  }

  isBlank(str: string): boolean {
    return !str || /^\s*$/.test(str);
  }

  load(): any {
    if (!fs.existsSync(this._dataFile)) {
      this.save(DataAccessConsts.defaultRootElement);
    }
    console.log("file", this._dataFile);
    let rawData = fs.readFileSync(this._dataFile).toString();

    if (this.isBlank(rawData)) {
      this.save(DataAccessConsts.defaultRootElement);
    }

    console.log("file", rawData);

    rawData = fs.readFileSync(this._dataFile).toString();

    return JSON.parse(rawData);
  }

  save(data: Snippet): void {
    fs.writeFileSync(this._dataFile, JSON.stringify(data));
  }

  static resolveFilename(folderPath: string): string {
    return path.join(folderPath, FileDataAccess.dataFileName);
  }
}
