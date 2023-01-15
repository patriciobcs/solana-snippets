import { Snippet } from "../types/snippet";

export class DataAccessConsts {
  public static readonly defaultRootElement: Snippet = {
    id: 1,
    parentId: -1,
    label: "snippets",
    lastId: 1,
    children: [],
    type: 0,
  };
}

export interface DataAccess {
  hasNoChild(): boolean;
  load(): Snippet;
  save(data: Snippet): void;
  reset(): void;
}
