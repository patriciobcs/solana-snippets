export class Snippet {
  static readonly rootParentId = 1;

  id: number;
  parentId?: number;
  lastId?: number;
  type: number;
  label: string;
  children: Array<Snippet> = [];
  content?: Array<string> = [];
  requires?: Array<string> = [];
  resolveSyntax?: boolean;
  description?: string;

  constructor({
    id,
    parentId,
    lastId,
    type,
    label,
    children = [],
    content = [],
    requires = [],
    description = "",
  }: {
    id: number;
    parentId?: number;
    lastId?: number;
    type: number;
    label: string;
    children?: Array<Snippet>;
    content?: Array<string>;
    requires?: Array<string>;
    description?: string;
  }) {
    this.id = id;
    this.label = label;
    this.type = type;
    this.parentId = parentId;
    this.lastId = lastId;
    this.content = content;
    this.requires = requires;
    this.children = children;
    this.description = description;
  }

  static isFolder(snippet: Snippet): boolean {
    return snippet.type === 0 || snippet.type === 1;
  }

  static isSnippet(snippet: Snippet): boolean {
    return snippet.type === 2 || snippet.type === 3 || snippet.type === 4;
  }

  static isComment(snippet: Snippet): boolean {
    return snippet.type === 6;
  }

  static isVSCodeSnippet(snippet: Snippet): boolean {
    return snippet.type === 2;
  }

  static isTerminalSnippet(snippet: Snippet): boolean {
    return snippet.type === 4;
  }

  static getContent(snippet: Snippet): string {
    if (
      !Snippet.isSnippet(snippet) ||
      !snippet.content ||
      snippet.content == undefined ||
      snippet.content.length === 0
    ) {
      return "";
    } else {
      return snippet.content.join("\n");
    }
  }
}
