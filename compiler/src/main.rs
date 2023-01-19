mod consts;

use json::{object, JsonValue, array};
use std::collections::HashMap;
use std::env;
use std::fs::{read_dir, File, read_to_string};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use crate::consts::*;

fn parse_config(contents: String, config_name: &str) -> String {
    let start_tag = config_name.to_string() + ":";
    let start_bytes = contents.find(&start_tag).unwrap() + start_tag.len();
    contents[start_bytes..].trim().into()
}

trait StringAsComment {
    fn as_comment(&self) -> String;
}

impl StringAsComment for str {
    fn as_comment(&self) -> String {
        format!("{} {}", ONE_LINE_COMMENT, self)
    } 
}

fn parse_body_line(contents: String, inputs: &mut HashMap<String, usize>) -> String {
    let elements: Vec<_> = contents.split("__").collect();
    let mut parsed_contents: String = contents.clone().replacen("    ", "", 1);
    if elements.len() > 1 {
        for i in (1..elements.len()).step_by(2) {
            let element = elements[i];
            if !inputs.contains_key(element) {
                inputs.insert(element.into(), inputs.len() + 1);
            }
            parsed_contents = parsed_contents.replacen(
                &format!("__{}__", element)[..],
                &format!(
                    "${{{}:{}}}",
                    inputs.get(element).unwrap().to_string(),
                    element
                )[..],
                1,
            );
        }
    }
    parsed_contents
}
fn parse_requires_line(contents: String) -> String {
    contents.trim().replacen("use ", "", 1).replacen(";", "", 1)
}

enum Reading {
    TITLE,
    CONFIG,
    REQUIRES,
    CONTENT,
    NONE,
}

fn generate_snippets(filepath: PathBuf) -> Vec<(String, JsonValue)> {
    let mut snippets = vec![];
    if let Ok(lines) = read_lines(filepath.clone()) {
        let mut reading = Reading::TITLE;
        let mut inputs = HashMap::new();
        let mut title: Option<String> = None;
        let mut config: JsonValue = object! {
            prefix: [],
            body: [],
            requires: [],
            description: "",
            scope: "expr",
            category: "",
            platform: "",
        };
        for line in lines {
            if let Ok(contents) = line {
                match reading {
                    Reading::TITLE => {
                        if title == None && contents.trim().starts_with(&TITLE.as_comment()) {
                            title = Some(parse_config(contents, TITLE));
                            reading = Reading::CONFIG;
                        }
                    }
                    Reading::CONFIG => {
                        if contents.trim().len() == 0 {
                            reading = Reading::NONE;
                        } else {
                            if contents.trim().starts_with(&DESCRIPTION.as_comment()) {
                                config[DESCRIPTION] = parse_config(contents, DESCRIPTION).into();
                            } else if contents.trim().starts_with(&PREFIX.as_comment()) {
                                config[PREFIX].push(parse_config(contents, PREFIX)).ok();
                            } else if contents.trim().starts_with(&CATEGORY.as_comment()) {
                                config[CATEGORY] = parse_config(contents, CATEGORY).into();
                            } else if contents.trim().starts_with(&PLATFORM.as_comment()) {
                                config[PLATFORM] = parse_config(contents, PLATFORM).into();
                            } else if contents.trim().starts_with(&REQUIRES.as_comment()) {
                                reading = Reading::REQUIRES;
                            } 
                        }
                    }
                    Reading::REQUIRES => {
                        if contents.trim().len() == 0 {
                            reading = Reading::NONE;
                        } else {
                            config[REQUIRES].push(parse_requires_line(contents)).ok();
                        }
                    }
                    Reading::CONTENT => {
                        if contents.contains(CONTENT_TAG) {
                            snippets.push((title.clone().unwrap(), config.clone()));
                            reading = Reading::NONE;
                        } else {
                            config[BODY].push(parse_body_line(contents, &mut inputs)).ok();
                        }
                    }
                    Reading::NONE => {
                        if contents.contains(CONTENT_TAG) {
                            reading = Reading::CONTENT;
                        } else if contents.contains(&REQUIRES.as_comment()) {
                            reading = Reading::REQUIRES;
                        }
                    }
                }
            }
        }
    }
    snippets
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_dirs<'a>(dir: &'a Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = vec![];
    let ignore = vec!["target", "node_modules"];
    if dir.is_dir() {
        'outer: for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for dir in &ignore {
                    if path.ends_with(dir) {
                        continue 'outer;
                    }
                }
                paths.extend(get_dirs(&path)?)
            } else {
                paths.push(path)
            }
        }
    }
    Ok(paths)
}

fn get_snippets(snippets_path: &String) -> JsonValue {
    let mut snippets = object! {};

    let paths = get_dirs(Path::new(snippets_path)).unwrap();

    for path in paths {
        let generated_snippets = generate_snippets(path);
        for (title, config) in generated_snippets {
            snippets[title] = config;
        }
    }

    snippets
}

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

    let mut categories = object! {};
    
    let mut id = 3;

    for (key, value) in ra_snippets.entries() {
        let mut snippet = value.clone();

        snippet["content"] = snippet["body"].clone();
        snippet.remove("body");

        snippet["label"] = key.into(); 
        snippet["type"] = 3.into();
        snippet["children"] = array![];
        let category = snippet[CATEGORY].as_str().unwrap().trim().to_owned();
        snippet["parentId"] = if !categories.has_key(snippet[CATEGORY].as_str().unwrap()) {
            let label = category[0..1].to_uppercase() + &category[1..];
            categories[snippet[CATEGORY].as_str().unwrap()] = object! {
                id: id,
                parentId: 2,
                "type": 1,
                label: label,
                children: []
            };
            id += 1;
            (id - 1).into()
        } else {
            categories[snippet[CATEGORY].as_str().unwrap()]["id"].clone()
        };
        snippet["id"] = id.into();
        snippet.remove(CATEGORY);
        snippet.remove(PREFIX);
        snippet.remove(PLATFORM);
        snippet.remove("scope");

        categories[category]["children"].push(snippet.clone()).unwrap();

        id += 1;
    }

    for (_, value) in categories.entries() {
        solana_folder["children"].push(value.clone()).unwrap();
    }

    snippets["lastId"] = (id - 1).into();
    snippets["children"].push(solana_folder).unwrap();

    let mut output_file = File::create(output_path).unwrap();

    snippets.write_pretty(&mut output_file, 2).ok();
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
        _ => println!("formats: ra and sp")
    }
    
    Ok(())
}
