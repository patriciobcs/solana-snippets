# Solana Snippets

The [Solana Snippets VSCode Extension](https://marketplace.visualstudio.com/items?itemName=patriciobcs.solana-snippets) allows you to quickly insert Solana snippets into your code or your terminal. The snippets are tested in a real [Solana program](./snippets) (code) or real Shell Scripts (terminal), and presented as VSCode and Rust Analyzer snippets with a sidebar to navigate through them.

## Snippets

Snippets are small fragments of code that can be used to speed up the development process. In this extension you can find three types of snippets:

|                        Icon                         |     Type      |                                  Description                                   | Trigger |                                  Example                                   |
| :-------------------------------------------------: | :-----------: | :----------------------------------------------------------------------------: | :-----: | :------------------------------------------------------------------------: |
|   <img src="resources/icons/dark/code.svg"></img>   |    VSCode     |                   VSCode format snippets (work in all files)                   |    >    | [`Anchor Program Structure`](snippets/anchor/programs/core/src/program.rs) |
|   <img src="resources/icons/dark/rust.svg"></img>   | Rust Analyzer |          Rust Analyzer snippets (they have automatic imports in Rust)          |  \_\_   |     [`Transfer SOL`](snippets/native/src/core/system/transfer_sol.rs)      |
| <img src="resources/icons/dark/terminal.svg"></img> |   Terminal    | Terminal commands snippets that run directly in the VSCode integrated terminal |         |             [`Get Airdrop`](./snippets/native/cli/airdrop.sh)              |

All the snippets are defined in multiple Rust files. This files uses comments to define the beginning and ending of the snippet. This approach allows to build and **test** the snippets before compiling the snippets to JSON and package them into the extension.

### Usage

- In any file type ">" to visualize all the `VSCode` snippets.
- In any Rust file type "__" inside a function/method to visualize all the `Rust Analyzer` snippets. If the snippets require some imports, they will be automatically added.
- When there are multiple inputs to fill in the snippet like a transfer execution [`sender`, `receiver`, `amount`], use `TAB` to switch to the next input. If the default input (placeholder) is sufficient, press `TAB` to keep it and pass to the following input. Repeated inputs only need to be filled once.
- Open the Sidebar (Solana logo) and select any snippet to insert it in the current file (code snippets) or execute it (terminal snippets). Rust Snippets imports are not added automatically in this way.
- Type the name of any snippet (without spaces) in the file to insert it. The names can be seen in the sidebar or by using the command `Solana Snippets: Open Snippet`.
- Edit any snippet by using the command `Solana Snippets: Edit Snippet` or by left clicking it on the sidebar menu. This will open the snippet in a new VSCode window. Save the changes to update the snippet in the extension.
- Delete any snippet by using the command `Solana Snippets: Delete Snippet` or by left clicking it on the sidebar menu. This will delete the snippet from the extension.
- Reset the snippets to the default ones by using the command `Solana Snippets: Reset Snippets`. This will delete all the snippets and import the default ones (the ones that came with the extension). Be careful with this command, as it will delete all the snippets that you have created.

### Contributing

- If you want to contribute with a new snippet, you can create a PR with the snippet in the `snippets` folder. The snippet should be tested in a real Solana program (Anchor or Vanilla) or Shell Script. Follow some of the examples in the table above to see how to define the snippet.
- If you need an snippets but you don't want to build it yourself, you can create an issue with the snippet you need and it will be added to the pending list.
- If you have a program IDL and you want to generate the snippets from it, you can add it to the interfaces and the compiler will generate the snippets for you. Follow the instructions in the [IDL Compiler](snippets/anchor/programs/interfaces/README.md) folder.

## State

### Current

- The snippets are statically generated from different sources defined in the `solana-snippets` repository subfolder `snippets`.
- Three types of snippets are supported: VSCode, Rust Analyzer, and Terminal.
- Snippets can be imported and exported in JSON format.
- Snippets can be modified and saved in the extension.
- Snippets can be saved in the VSCode Sync Settings, thus allowing to share snippets between different machines.
- Rust Snippets are inserted in the global configuration of the Rust Analyzer extension. Therefore, the auto-imports are automatically added only when the snippet is called from the file (not from the sidebar).

### Next

- Cover all the Solana programs with snippets.
- Generate macros from the snippets defined in the same format. This could reduce the amount of code in Solana programs.
- Add automatic imports to the Rust Analyzer snippets when the snippet is called from the sidebar.
- Add more icons to the interface snippets. Programs logos as icons for the snippets that are related to them.
- Add dynamic generation in base to the source code of the project. This could lead to snippets like parsing all the accounts mentioned in the Instruction parser or getting a `sysvar` automatically when user uses an snippet that requires it.

```rust
let account_info_iter = &mut accounts.iter();
let account_one_info = next_account_info(account_info_iter)?;
let account_two_info = next_account_info(account_info_iter)?;
...
```

- Cover more languages like Python, JS, Go, and C++.

## Release Process

The extension is currently tagged and released manually. Once a new version have tested all the snippets and deemed ready for release:

### Bump Version

- Increment the version number in the `package.json` file.
- Run `yarn build` to test and compile the snippets and build the extension.
- Open a PR with these version changes and merge after passing CI.

### Create Github tag

Snippets extension tag are of the form `solana-snippets-vX.Y.Z`. Create the new tag at the version-bump commit and push to the repository, eg:

```sh
git tag solana-snippets-v1.0.0 b24bfe7
git push upstream --tags
```

### Publish release

- Go to [GitHub Releases UI](https://github.com/patriciobcs/solana-snippets/releases)
- Click "Draft new release", and enter the new tag in the "Tag version" box.
- Title the release "Solana Snippets vX.Y.Z", complete the description, and attach the `solana_snippets-v0.0.1.vsix`.
- Click "Publish release"

## Disclaimer

- This extension is not affiliated with Solana Foundation or Solana Labs. It is a non-profit community project.
- Solana Snippets is in active development, so all the snippets are subject to change.
- Even if the snippets can suggest good practices and validations, they are just suggestions and not a guarantee of correctness.
- Double check the code before deploying it to the mainnet though extensive testing, code review, and audits.
- **The snippets are unaudited**. Use at your own risk.
