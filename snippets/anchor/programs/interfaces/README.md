# Interfaces

This directory contains the interfaces for Anchor programs currently in mainnet.

To find new IDLs, you can use [SolDev IDL Registry](https://soldev.app/registry);

To add one you need to follow the following steps:

- Create a new directory with the name of the program in `src/`. For example, `src/swap`.
- Add `pub mod <new_directory_name>;` to `src/lib.rs`.
- Create a `idl.json` file in the new directory, and paste the program's idl there.
- Create a `id.txt` file in the new directory, and paste the program's id there (only the public key characters).
- Run `yarn build-idls` to regenerate the interfaces of all programs.

After that, you can check the interfaces in the same directory.

To use the interfaces as snippets recompile the extension with the following stepts:

- Run `yarn build-snippets` to regenerate the snippets.
- Run `yarn compile` to recompile the extension.
- Build or test the extension directly in the Debug tab of VSCode.
