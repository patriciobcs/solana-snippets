# Model

## Fields

### Id

The id is a unique identifier for the snippet. It is used to identify the snippet in the database (memento).

### ParentId

The parent id is the id of the parent snippet. It is used to build the tree structure of the snippets.

### LastId

The last id is the id of the last child snippet. It is used to build the tree structure of the snippets.

## Type

The type is used to determine the semantics of the element:

0. Root
1. Folder
2. VSCode Native Snippet
3. Rust Analyzer Snippet
4. Terminal Snippet
5. Comment

### Label

The label is the name of the snippet. It is used to display the snippet in the editor and sidebar.

## Content

The content is the actual content of the snippet. It is used to display the snippet in the editor.
