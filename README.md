# Solana Snippets

The [Solana Snippets VSCode Extension](https://code.visualstudio.com/api/language-extensions/solana-snippets) allows you to quickly insert Solana snippets into your code. This snippets are well tested in a real [Solana program](./snippets) and compiled to the VSCode Snippets format (JSON) using a [custom compiler](./compiler).

![Solana Snippets](./resources/check-rent.gif)

## Snippets

When there's multiple inputs to fill in the snippet like a transfer execution [`sender`, `receiver`, `amount`], use `TAB` to switch to the next input. If the default input (placeholder) is sufficient, press `TAB` to keep it and pass to the following input. Repeated inputs only need to be filled once.

|          Title           |  Command  |                          Description                           |               Inputs                |
| :----------------------: | :-------: | :------------------------------------------------------------: | :---------------------------------: |
|       Pack Account       |  `apack`  |                        Packs an Account                        |               account               |
|      Unpack Account      | `aunpack` |                       Unpacks an Account                       |               account               |
|    Check Rent Exempt     | `chrent`  |               Check if an account is rent exempt               |            account, type            |
|  Get Accounts Iterator   |  `gaccs`  |         Start iterator to get accounts of instruction          |               account               |
|       Get Account        |  `gacc`   |           Get next account in the accounts iterator            |               account               |
|     Get Clock Sysvar     | `gclock`  |                      Get the Clock Sysvar                      |               varname               |
|     Get Rent Sysvar      |  `grent`  |                      Get the Rent Sysvar                       |               varname               |
|     Delegate Tokens      |   `tka`   |          Delegate any tokens using the Token Program           | token, delegate, authority, amount  |
|       Burn Tokens        |   `tkb`   |            Burn any tokens using the Token Program             |   token, mint, authority, amount    |
|        Init Token        |   `tki`   | Initialize an associated token account using the Token Program |         token, mint, owner          |
| Revoke Tokens Delegation |   `tkr`   |  Revoke the delegation of any tokens using the Token Program   |     token, delegate, authority      |
|     Transfer Tokens      |   `tkt`   |          Transfer any tokens using the Token Program           | sender, receiver, authority, amount |
|  Transfer Native `SOL`   |  `tsol`   |                         Transfer `SOL`                         |      sender, receiver, amount       |

<!-- |       Create a PDA       |  `cpda`  |                       Creates a PDA                        |           varname, payer            |
|      Create Keypair      |  `ckey`  |                     Creates a Keypair                      |               varname               |
|       Set PDA Bump       | `sbump`  |                   Sets the bump to a PDA                   |               account               | -->

All the snippets are defined in different Rust files. This files uses comments to define the beginning and ending of the snippet. This approach allows to build and **test** the snippets before compiling the snippets in the JSON VSCode format.

## State

Currently, the snippets are statically generated. The next step is to add dynamic generation in base to the source code of the project. This could lead to snippets like parsing all the accounts mentioned in the Instruction parser or getting a `sysvar` automatically when user uses an snippet that requires it.

```rust
let account_info_iter = &mut accounts.iter();
let account_one_info = next_account_info(account_info_iter)?;
let account_two_info = next_account_info(account_info_iter)?;
...
```

## Release Process

The extension is currently tagged and released manually. Once a new version have tested all the snippets and deemed ready for release:

### Bump Version

* Increment the version number in the `package.json` file.
* Run `yarn build` to test and compile the snippets and build the extension.
* Open a PR with these version changes and merge after passing CI.

### Create Github tag

Snippets extension tag are of the form `solana-snippets-vX.Y.Z`. Create the new tag at the version-bump commit and push to the repository, eg:

```sh
git tag solana-snippets-v1.0.0 b24bfe7
git push upstream --tags
```

### Publish Github release

* Go to [GitHub Releases UI](https://github.com/patriciobcs/solana-snippets/releases)
* Click "Draft new release", and enter the new tag in the "Tag version" box.
* Title the release "Solana Snippets vX.Y.Z", complete the description, and attach the `solana_snippets-v0.0.1.vsix`.
* Click "Publish release"
