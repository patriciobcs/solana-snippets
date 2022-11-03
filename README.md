# Solana Snippets

The [Solana Snippets VSCode Extension](https://code.visualstudio.com/api/language-extensions/solana-snippets) allows you to quickly insert Solana snippets into your code. This snippets are well tested in a real [Solana program](./snippets) and compiled to the VSCode Snippets format (JSON) using a [custom compiler](./compiler).

![Solana Snippets](./resources/check-rent.gif)

## Snippets

The following snippets allow you to generate Solana Rust code automatically by selecting what you want to create. The symbol [option] means that you need to write one of those options to generate the code, this can be for example the name of a `Sysvar`.

When there's multiple options to write in an snippets like a transfer execution (sender, receiver), use `TAB` to write the change to the next variable. If the default input is OK, press tab to keep it and pass to the following input.

|         Title         | Command  |                  Description                  |           Inputs            |
| :-------------------: | :------: | :-------------------------------------------: | :-------------------------: |
| Get Accounts Iterator | `gaccs`  | Start iterator to get accounts of instruction |           account           |
|      Get Account      |  `gacc`  |   Get next account in the accounts iterator   |           account           |
|    Get Rent Sysvar    | `grent`  |              Get the Rent Sysvar              |           varname           |
|   Get Clock Sysvar    | `gclock` |             Get the Clock Sysvar              |           varname           |
|     Create a PDA      |  `cpda`  |                 Creates a PDA                 |       varname, payer        |
|    Create Keypair     |  `ckey`  |               Creates a Keypair               |           varname           |
|     Set PDA Bump      | `sbump`  |            Sets the bump to a PDA             |           account           |
|    Unpack Account     | `unpack` |              Unpacks an Account               |           account           |
|     Pack Account      |  `pack`  |               Packs an Account                |           account           |
| Transfer Native `SOL` |  `tsol`  |                Transfer `SOL`                 |      sender, receiver       |
|    Transfer Tokens    |  `tkt`   |  Transfer any tokens using the Token Program  | sender, receiver, authority |
|      Burn Tokens      |  `tkb`   |    Burn any tokens using the Token Program    |   token, mint, authority    |
|    Delegate Tokens    |  `tka`   |  Delegate any tokens using the Token Program  | token, delegate, authority  |

## Snippets Generation

All the snippets are defined in different Rust files. This files uses comments to define the beginning and ending of the snippet. This approach allows to build and **test** the snippets before generating the snippets in the JSON VSCode format.

## State

Currently, the snippets are statically generated. The next step is to add dynamic generation in base to the source code of the project. This could lead to snippets like parsing all the accounts mentioned in the Instruction parser or getting a `sysvar` automatically when user uses an snippet that requires it.

```rust
let account_info_iter = &mut accounts.iter();
let account_one_info = next_account_info(account_info_iter)?;
let account_two_info = next_account_info(account_info_iter)?;
...
```
