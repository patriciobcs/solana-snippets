use json::JsonValue;

// IDL:
// {
//   "version": "0.1.0",
//   "name": "escrow",
//   "instructions": [
//     {
//       "name": "initializeEscrow",
//       "accounts": [
//         {
//           "name": "initializer",
//           "isMut": true,
//           "isSigner": true
//         },
//         {
//           "name": "initializerDepositTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "initializerReceiveTokenAccount",
//           "isMut": false,
//           "isSigner": false
//         },
//         {
//           "name": "escrowAccount",
//           "isMut": true,
//           "isSigner": true
//         },
//         {
//           "name": "systemProgram",
//           "isMut": false,
//           "isSigner": false
//         },
//         {
//           "name": "tokenProgram",
//           "isMut": false,
//           "isSigner": false
//         }
//       ],
//       "args": [
//         {
//           "name": "initializerAmount",
//           "type": "u64"
//         },
//         {
//           "name": "takerAmount",
//           "type": "u64"
//         }
//       ]
//     },
//     {
//       "name": "cancelEscrow",
//       "accounts": [
//         {
//           "name": "initializer",
//           "isMut": false,
//           "isSigner": false
//         },
//         {
//           "name": "pdaDepositTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "pdaAccount",
//           "isMut": false,
//           "isSigner": false
//         },
//         {
//           "name": "escrowAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "tokenProgram",
//           "isMut": false,
//           "isSigner": false
//         }
//       ],
//       "args": []
//     },
//     {
//       "name": "exchange",
//       "accounts": [
//         {
//           "name": "taker",
//           "isMut": false,
//           "isSigner": true
//         },
//         {
//           "name": "takerDepositTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "takerReceiveTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "pdaDepositTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "initializerReceiveTokenAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "initializerMainAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "escrowAccount",
//           "isMut": true,
//           "isSigner": false
//         },
//         {
//           "name": "pdaAccount",
//           "isMut": false,
//           "isSigner": false
//         },
//         {
//           "name": "tokenProgram",
//           "isMut": false,
//           "isSigner": false
//         }
//       ],
//       "args": []
//     }
//   ],
//   "accounts": [
//     {
//       "name": "EscrowAccount",
//       "type": {
//         "kind": "struct",
//         "fields": [
//           {
//             "name": "initializerKey",
//             "type": "publicKey"
//           },
//           {
//             "name": "initializerDepositTokenAccount",
//             "type": "publicKey"
//           },
//           {
//             "name": "initializerReceiveTokenAccount",
//             "type": "publicKey"
//           },
//           {
//             "name": "initializerAmount",
//             "type": "u64"
//           },
//           {
//             "name": "takerAmount",
//             "type": "u64"
//           }
//         ]
//       }
//     }
//   ]
// }
//
// Interface:
// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[derive(Accounts)]
// pub struct InitializeEscrow<'info> {
//     #[account(mut, signer)]
//     pub initializer: AccountInfo<'info>,
//     #[account(mut)]
//     pub initializer_deposit_token_account: AccountInfo<'info>,
//     pub initializer_receive_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub escrow_account: AccountInfo<'info>,
//     pub system_program: AccountInfo<'info>,
//     pub token_program: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct Exchange<'info> {
//     pub taker: AccountInfo<'info>,
//     #[account(mut)]
//     pub taker_deposit_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub taker_receive_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub pda_deposit_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub initializer_receive_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub initializer_main_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub escrow_account: AccountInfo<'info>,
//     pub pda_account: AccountInfo<'info>,
//     pub token_program: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct CancelEscrow<'info> {
//     pub initializer: AccountInfo<'info>,
//     #[account(mut)]
//     pub pda_deposit_token_account: AccountInfo<'info>,
//     pub pda_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub escrow_account: AccountInfo<'info>,
//     pub token_program: AccountInfo<'info>,
// }

// #[account]
// pub struct EscrowAccount {
//     pub initializer_key: Pubkey,
//     pub initializer_deposit_token_account: Pubkey,
//     pub initializer_receive_token_account: Pubkey,
//     pub initializer_amount: u64,
//     pub taker_amount: u64,
// }

// TODO: Create snippet that generates an derive snippet (#[derive(Account)]) each instruction of the given IDL

// function the convert camelCase to snake_case
fn camel_to_snake_case(s: &str) -> String {
	let mut result = String::new();
	let mut prev_char = '_';
	for c in s.chars() {
		if c.is_uppercase() && prev_char != '_' {
			result.push('_');
		}
		result.push(c.to_ascii_lowercase());
		prev_char = c;
	}
	result
}

// function that converts capitalize the first letter of a string
fn capitalize_first_letter(s: &str) -> String {
	let mut c = s.chars();
	match c.next() {
		None => String::new(),
		Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
	}
}

pub fn get_instruction_account_content(account_idl: &JsonValue) -> String {
	let mut content = String::new();

	let name = camel_to_snake_case(account_idl["name"].as_str().unwrap());
	let is_mut = account_idl["isMut"].as_bool().unwrap();
	let is_signer = account_idl["isSigner"].as_bool().unwrap();

	match (is_mut, is_signer) {
			(true, true) => {
				content.push_str("  #[account(mut, signer)]\n")
			},
			(true, false) => {
				content.push_str("  #[account(mut)]\n")
			},
			(false, true) => {
				content.push_str("  #[account(signer)]\n")
			},
			_ => {}
	}

	content.push_str(&format!("  pub {}: AccountInfo<'info>,\n", name));

	content
}

pub fn get_instruction_content(instruction_idl: &JsonValue) -> String {
	let mut content = String::new();

	let name = capitalize_first_letter(instruction_idl["name"].as_str().unwrap());
	let accounts = instruction_idl["accounts"].members();
	let _args = instruction_idl["args"].members();

	content.push_str(&format!("#[derive(Accounts)]\npub struct {}<'info> {{\n", name));

	for account in accounts {
		content.push_str(&get_instruction_account_content(account));
	}

	content.push_str("}\n\n");

	content
}

pub fn convert_to_rust_type(ty: &str) -> String {
	match ty {
			"string" => "String".to_string(),
			"publicKey" => "Pubkey".to_string(),
			_ => ty.to_string(),
	}
}

// #[account]
// pub struct EscrowAccount {
//     pub initializer_key: Pubkey,
//     pub initializer_deposit_token_account: Pubkey,
//     pub initializer_receive_token_account: Pubkey,
//     pub initializer_amount: u64,
//     pub taker_amount: u64,
// }
// 
// function that converts the idl to the struct of the account of the given IDL
pub fn get_account_content(account_idl: &JsonValue) -> String {
	let mut content = String::new();
	
	let name = capitalize_first_letter(account_idl["name"].as_str().unwrap());
	content.push_str(&format!("#[account]\npub struct {} {{\n", name));

	let fields = account_idl["type"]["fields"].members();

	for field in fields {
		let name = camel_to_snake_case(field["name"].as_str().unwrap());
		let ty = convert_to_rust_type(field["type"].as_str().unwrap());

		content.push_str(&format!("  pub {}: {},\n", name, ty));
	}

	content.push_str("}\n\n");

	content
}

// TODO: Create snippet that generates an all the derive snippets
// TODO: Create snippet that generates the id of the program
// TODO: Create an snippets that create a file with all the instructions
// TODO: Create an snippets that generates an accounts snippet (#[account]) each instruction of the given IDL
pub fn get_interface_from_idl(idl: &JsonValue, program_id: &String) -> String {
	let mut content = String::from("use anchor_lang::prelude::*;\n\n");
	
	content.push_str(&format!("declare_id!(\"{}\");\n\n", program_id)); 
	
	let instructions = idl["instructions"].members();
	let accounts = idl["accounts"].members();
	
	for instruction in instructions {
		content.push_str(&get_instruction_content(instruction));
	}

	for account in accounts {
		content.push_str(&get_account_content(account));
	}

	content
}