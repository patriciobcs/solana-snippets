use std::{fs::{read_to_string, File}, path::Path, os::unix::prelude::FileExt};

use json::JsonValue;
use crate::utils::get_dirs;

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
	let mut content = String::new();
	let accounts = instruction_idl["accounts"].members();

	for account in accounts.clone() {
		if account.has_key("accounts") {
			content.push_str(&get_instruction_content(account));
		}
	}

	let name = capitalize_first_letter(instruction_idl["name"].as_str().unwrap());
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

// function that converts the idl to the struct of the account of the given IDL
pub fn get_account_content(account_idl: &JsonValue) -> String {
	let mut content = String::new();
	
	let name = capitalize_first_letter(account_idl["name"].as_str().unwrap());
	content.push_str(&format!("#[account]\npub struct {} {{\n", name));

	let fields = account_idl["type"]["fields"].members();

	for field in fields {
		let name = camel_to_snake_case(field["name"].as_str().unwrap());
		let ty = if field["type"].has_key("defined") {
			field["type"]["defined"].as_str().unwrap().to_string()
		} else { 
			convert_to_rust_type(field["type"].as_str().unwrap())
		};

		content.push_str(&format!("  pub {}: {},\n", name, ty));
	}

	content.push_str("}\n\n");

	content
}

pub fn get_interface_from_idl_single_snippet(idl: &JsonValue, program_id: &String) -> String {
	let name = idl["name"].as_str().unwrap();

	let mut content = format!("//* title: All {} Interfaces
//* description: Creates the interface of the {} program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode

/*/* content */*/\n", capitalize_first_letter(name), name);
	
	content.push_str("use anchor_lang::prelude::*;\n\n");
	
	content.push_str(&format!("declare_id!(\"{}\");\n\n", program_id)); 
	
	let instructions = idl["instructions"].members();
	let accounts = idl["accounts"].members();
	
	for instruction in instructions {
		content.push_str(&get_instruction_content(instruction));
	}

	for account in accounts {
		content.push_str(&get_account_content(account));
	}

	content.push_str("/*/* content */*/\n\n");

	content
}

pub fn get_interface_from_idl_snippets(idl: &JsonValue, program_id: &String) -> String {
	let name = idl["name"].as_str().unwrap();

	let mut content = String::from("use anchor_lang::prelude::*;\n\n");
	
	content.push_str(&format!("declare_id!(\"{}\");\n\n", program_id)); 
	
	let instructions = idl["instructions"].members();
	let accounts = idl["accounts"].members();
	
	for instruction in instructions {
		let instruction_name = instruction["name"].as_str().unwrap();
		content.push_str(&format!("//* title: {capitalized_instruction} {capitalized_program} Interface
//* description: Creates the interface of the instruction {instruction} of the {program} program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/\n", capitalized_program = capitalize_first_letter(name), program = name, capitalized_instruction = capitalize_first_letter(instruction_name), instruction = instruction_name));
			
		content.push_str(&get_instruction_content(instruction));

		content.push_str("/*/* content */*/\n\n");
	}

	for account in accounts {
		let account_name = account["name"].as_str().unwrap();
		content.push_str(&format!("//* title: {capitalized_instruction} {capitalized_program} Interface
//* description: Creates the interface of the instruction {instruction} of the {program} program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/\n", capitalized_program = capitalize_first_letter(name), program = name, capitalized_instruction = capitalize_first_letter(account_name), instruction = account_name));
			
		content.push_str(&get_account_content(account));

		content.push_str("/*/* content */*/\n\n");
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
	}
}