use std::{fs::{read_to_string, File}, path::Path, os::unix::prelude::FileExt};

use json::JsonValue;
use crate::{utils::get_dirs, consts::MOD_FILE_CONTENT};

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

// function that converts snake_case to capitalize phrase
fn snake_to_capitalize_phrase(s: &str) -> String {
	let mut result = String::new();
	let mut prev_char = '_';
	for c in s.chars() {
		if c == '_' {
			result.push(' ');
		} else if prev_char == '_' {
			result.push(c.to_ascii_uppercase());
		} else {
			result.push(c);
		}
		prev_char = c;
	}
	result
}

pub fn convert_to_rust_type(ty: &str) -> String {
	match ty {
			"string" => "String".to_string(),
			"publicKey" => "Pubkey".to_string(),
			"bytes" => "Vec<u8>".to_string(),
			_ => ty.to_string(),
	}
}

pub fn get_rust_type(field_idl: &JsonValue) -> String {
	if field_idl["type"].has_key("option") {
		let ty = if field_idl["type"]["option"].has_key("defined") {
			field_idl["type"]["option"]["defined"].as_str().unwrap()
		} else {
			field_idl["type"]["option"].as_str().unwrap()
		};
		format!("Option<{}>", convert_to_rust_type(ty))
	} else if field_idl["type"].has_key("vec") {
		let ty = if field_idl["type"]["vec"].has_key("defined") {
			field_idl["type"]["vec"]["defined"].as_str().unwrap()
		} else {
			field_idl["type"]["vec"].as_str().unwrap()
		};
		format!("Vec<{}>", convert_to_rust_type(ty))
	} else if field_idl["type"].has_key("defined") {
		convert_to_rust_type(field_idl["type"]["defined"].as_str().unwrap())
	} else { 
		convert_to_rust_type(field_idl["type"].as_str().unwrap())
	}
}

pub fn get_instruction_account_content(account_idl: &JsonValue) -> String {
	let name = camel_to_snake_case(account_idl["name"].as_str().unwrap());
	
	if account_idl.has_key("accounts") {
		let capitalized_name = capitalize_first_letter(account_idl["name"].as_str().unwrap());
		return format!("  pub {}: {}<'info>,\n", name, capitalized_name);
	}
	
	let mut content = String::new();
	
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
	let mut instruction_idl = instruction_idl.clone();
	let mut content = String::new();
	let name = capitalize_first_letter(instruction_idl["name"].as_str().unwrap());
	
	for account in instruction_idl["accounts"].members_mut() {
		if account.has_key("accounts") {
			account["name"] = format!("{}{}", account["name"].clone(), name).into();
			content.push_str(&get_instruction_content(&account));
		}
	}

	let _args = instruction_idl["args"].members();
	let accounts = instruction_idl["accounts"].members();


	content.push_str(&format!("#[derive(Accounts)]\npub struct {}<'info> {{\n", name));

	for account in accounts {
		content.push_str(&get_instruction_account_content(account));
	}

	content.push_str("}\n\n");

	content
}

// function that converts the idl to the struct of the account of the given IDL
pub fn get_account_content(account_idl: &JsonValue) -> String {
	let mut content = String::new();
	
	let name = capitalize_first_letter(account_idl["name"].as_str().unwrap());
	content.push_str(&format!("#[account]\npub struct {} {{\n", name));

	let fields = account_idl["type"]["fields"].members();

	for field in fields {
		let name = camel_to_snake_case(field["name"].as_str().unwrap());
		let ty = get_rust_type(field);

		content.push_str(&format!("  pub {}: {},\n", name, ty));
	}

	content.push_str("}\n\n");

	content
}

pub fn get_type_content(account_idl: &JsonValue) -> String {
	let mut content = String::new();
	
	let name = capitalize_first_letter(account_idl["name"].as_str().unwrap());

	match account_idl["type"]["kind"].as_str().unwrap() {
		"struct" => {
				content.push_str(&format!("#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]\npub struct {} {{\n", name));
				let fields = account_idl["type"]["fields"].members();
			
				for field in fields {
					let name = camel_to_snake_case(field["name"].as_str().unwrap());
					let ty = get_rust_type(field);
			
					content.push_str(&format!("  pub {}: {},\n", name, ty));
				}
			},
			"enum" => {
				content.push_str(&format!("#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]\npub enum {} {{\n", name));
				let fields = account_idl["type"]["variants"].members();
			
				for field in fields {
					let name = capitalize_first_letter(field["name"].as_str().unwrap());
					
					let enum_fields = if field.has_key("fields") {
						let mut enum_content = String::from(" { ");
						for (index, enum_field) in field["fields"].members().enumerate() {
							if !enum_field.has_key("name") { return String::new(); }
							let enum_name = enum_field["name"].as_str().unwrap();
							let enum_ty = get_rust_type(enum_field);
							if index > 0 {
								enum_content.push_str(", ");
							}
							enum_content.push_str(&format!("{}: {}", enum_name, enum_ty));
						}
						enum_content.push_str(" }");

						enum_content
					} else {
						String::new()
					};
			
					content.push_str(&format!("  {}{},\n", name, enum_fields));
				}
			}
			_ => {}
	}

	content.push_str("}\n\n");

	content
}

pub fn get_snippet_header(title: &String, description: &String) -> String {
	format!("//* title: {}
//* description: {}
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/\n", title, description)
}

pub fn parse_as_snippet(title: &String, description: &String, snippet: &String) -> String {
	format!("{}{}/*/* content */*/\n\n", get_snippet_header(title, description), snippet)
}
 
fn get_single_snippet_interface(idl: &JsonValue, program_id: &String) -> String {
	let mut content = String::from("use anchor_lang::prelude::*;\n\n");
	
	content.push_str(&format!("declare_id!(\"{}\");\n\n", program_id)); 

	let instructions = idl["instructions"].members();
	let accounts = idl["accounts"].members();
	let types = idl["types"].members();
	
	for instruction in instructions {
		content.push_str(&get_instruction_content(instruction));
	}

	for account in accounts {
		content.push_str(&get_account_content(account));
	}

	for ty in types {
		content.push_str(&get_type_content(ty));
	}

	content
}

pub fn get_interface_from_idl_single_snippet(idl: &JsonValue, program_id: &String) -> String {
	let name = snake_to_capitalize_phrase(idl["name"].as_str().unwrap());

	let title = format!("All {} Interfaces", name);
	let description = format!("Creates the interface of the `{}` program", name);
	
	parse_as_snippet(&title, &description, &get_single_snippet_interface(idl, program_id))
}

pub fn get_interface_from_idl_snippets(idl: &JsonValue, program_id: &String) -> String {
	let program_name = snake_to_capitalize_phrase(idl["name"].as_str().unwrap());

	let mut content = String::from("use anchor_lang::prelude::*;\n\n");
	
	content.push_str(&format!("declare_id!(\"{}\");\n\n", program_id)); 
	
	let instructions = idl["instructions"].members();
	let accounts = idl["accounts"].members();
	let types = idl["types"].members();
	
	for instruction in instructions {
		let name = instruction["name"].as_str().unwrap();
		let title = format!("{}'s Instruction {}", program_name, capitalize_first_letter(name));
		let description = format!("Creates the interface instruction `{}` of the `{}` program", name, program_name);
		content.push_str(&parse_as_snippet(&title, &description, &get_instruction_content(instruction)));
	}

	for account in accounts {
		let name = account["name"].as_str().unwrap();
		let title = format!("{}'s Account {}", program_name, capitalize_first_letter(name));
		let description = format!("Generates the account `{}` of the `{}` program", name, program_name);
		content.push_str(&parse_as_snippet(&title, &description, &get_account_content(account)));
	}

	for ty in types {
		let name = ty["name"].as_str().unwrap();
		let title = format!("{}'s Type {}", program_name, capitalize_first_letter(name));
		let description = format!("Generates the type `{}` of the `{}` program", name, program_name);
		content.push_str(&parse_as_snippet(&title, &description, &get_type_content(ty)));
	}

	content
}

pub fn scan_programs_idls_and_generate_interfaces(programs_path: &String) {
	for idl_path in get_dirs(&Path::new(programs_path)).unwrap() {
		if !idl_path.ends_with("idl.json") { continue; }

		let idl_file = read_to_string(idl_path.clone()).unwrap();
    let idl = json::parse(&idl_file).unwrap();

		let output_path = idl_path.parent().unwrap().to_str().unwrap();

		let id_path = format!("{}/id.txt", output_path);

		let program_id = read_to_string(id_path).unwrap().trim().to_string();

		let main_interface_content = get_interface_from_idl_single_snippet(&idl, &program_id);

    let main_interface_output_file = File::create(format!("{}/single.rs", output_path)).unwrap();
    main_interface_output_file.write_all_at(main_interface_content.as_bytes(), 0).ok();

    let interfaces_content = get_interface_from_idl_snippets(&idl, &program_id);

    let interfaces_output_file = File::create(format!("{}/multiple.rs", output_path)).unwrap();
    interfaces_output_file.write_all_at(interfaces_content.as_bytes(), 0).ok();
		
		let main_interface_output_file = File::create(format!("{}/mod.rs", output_path)).unwrap();
    main_interface_output_file.write_all_at(MOD_FILE_CONTENT.as_bytes(), 0).ok();
	}
}