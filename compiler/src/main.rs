mod consts;
mod snippets_parser;
mod idl_parser;

use json::{object, array};
use std::env;
use std::fs::{File, read_to_string};
use std::os::unix::prelude::FileExt;
use crate::snippets_parser::get_snippets;
use crate::idl_parser::get_interface_from_idl;
use crate::consts::*;

fn generate_rust_analyzer_snippets(snippets_path: &String, extension_config_path: &String) {
    let snippets = get_snippets(snippets_path);

    let extension_config_contents = read_to_string(extension_config_path).unwrap();

    let mut extension_config = json::parse(&extension_config_contents).unwrap(); 

    extension_config["contributes"]["configurationDefaults"]["rust-analyzer.completion.snippets.custom"] = snippets;

    let mut extension_config_file = File::create(extension_config_path).unwrap();

    extension_config.write_pretty(&mut extension_config_file, 2).ok();
}

fn generate_extension_snippets(snippets_path: &String, output_path: &String) {
    let ra_snippets = get_snippets(snippets_path);
    let mut snippets = object! {
        id: 1,
        parentId: -1,
        lastId: 154,
        "type": 0,
        label: "snippets",
        children: []
    };

    let mut solana_folder = object! {
        id: 2,
        parentId: 1,
        "type": 1,
        label: "Solana",
        children: []
    };

    let mut platforms = object! {};
    
    let mut id = 3;

    for (key, value) in ra_snippets.entries() {
        let mut snippet = value.clone();
        let category = snippet[CATEGORY].as_str().unwrap().trim().to_owned();
        let platform = snippet[PLATFORM].as_str().unwrap().trim().split(",").into_iter().next().unwrap().to_owned();

        let platform_id = if !platforms.has_key(&platform) {
            let label = platform[0..1].to_uppercase() + &platform[1..];
            platforms[&platform] = object! {
                id: id,
                parentId: 2,
                "type": 1,
                label: label,
                children: [],
                categories: {},
            };
            id += 1;
            id - 1
        } else {
            platforms[&platform]["id"].as_i32().unwrap()
        };

        let category_id = if !platforms[&platform]["categories"].has_key(snippet[CATEGORY].as_str().unwrap()) {
            let label = category[0..1].to_uppercase() + &category[1..];
            platforms[&platform]["categories"][snippet[CATEGORY].as_str().unwrap()] = object! {
                id: id,
                parentId: platform_id,
                "type": 1,
                label: label,
                children: []
            };
            id += 1;
            (id - 1).into()
        } else {
            platforms[&platform]["categories"][snippet[CATEGORY].as_str().unwrap()]["id"].clone()
        };


        snippet["content"] = snippet[BODY].clone();
        snippet["label"] = key.into(); 
        snippet["type"] = (if snippet[DISPLAY] == "ra" { 3 } else { 2 }).into();
        snippet["children"] = array![];
        snippet["parentId"] = category_id;
        snippet["id"] = id.into();
        
        snippet.remove(BODY);
        snippet.remove(CATEGORY);
        snippet.remove(PREFIX);
        snippet.remove(PLATFORM);
        snippet.remove(SCOPE);
        snippet.remove(DISPLAY);

        platforms[&platform]["categories"][category]["children"].push(snippet.clone()).unwrap();

        id += 1;
    }

    for (_, platform) in platforms.entries_mut() {
        let mut children = array![];
        for (_, category) in platform["categories"].entries() {
            children.push(category.clone()).unwrap();
        }
        platform["children"] = children;
        platform.remove("categories");
        solana_folder["children"].push(platform.clone()).unwrap();
    }

    snippets["lastId"] = (id - 1).into();
    snippets["children"].push(solana_folder).unwrap();

    let mut output_file = File::create(output_path).unwrap();

    snippets.write_pretty(&mut output_file, 2).ok();
}

pub fn generate_idl_snippets(idl_path: &String, output_path: &String, program_id: &String) {
    let idl_file = read_to_string(idl_path).unwrap();
    let idl = json::parse(&idl_file).unwrap();

    let content = get_interface_from_idl(&idl, &program_id);

    let output_file = File::create(output_path).unwrap();

    output_file.write_all_at(content.as_bytes(), 0).ok();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("usage: cargo run <format> <path to snippets directory> <path to extension package.json or output path>");
        return Ok(());
    }

    match &args[1][..] {
        "ra" => generate_rust_analyzer_snippets(&args[2], &args[3]),       
        "sp" => generate_extension_snippets(&args[2], &args[3]),
        "idl" => {
            let program_id = if args.len() < 5 {
                String::from("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")
            } else {
                args[4].clone()
            };
            generate_idl_snippets(&args[2], &args[3], &program_id)
        },       
        _ => println!("formats: ra and sp")
    }
    
    Ok(())
}
