mod consts;

use json::{object, JsonValue};
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
    CONFIG,
    REQUIRES,
    SNIPPET,
    NONE,
}

fn generate_snippet(filepath: PathBuf) -> (String, JsonValue) {
    let mut inputs = HashMap::new();
    let mut config = object! {
        prefix: [],
        body: [],
        requires: [],
        description: "",
        scope: "expr",
    };
    let mut title: String = "".into();
    let mut reading = Reading::CONFIG;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(contents) = line {
                match reading {
                    Reading::CONFIG => {
                        if contents.trim().len() == 0 {
                            reading = Reading::NONE;
                        } else {
                            if title.len() == 0 && contents.contains(TITLE) {
                                title = parse_config(contents, TITLE);
                            } else if contents.contains(DESCRIPTION) {
                                config[DESCRIPTION] = parse_config(contents, DESCRIPTION).into();
                            } else if contents.contains(PREFIX) {
                                config[PREFIX].push(parse_config(contents, PREFIX)).ok();
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
                    Reading::SNIPPET => {
                        if contents.contains(SNIPPET_TAG) {
                            reading = Reading::NONE;
                        } else {
                            config[BODY].push(parse_body_line(contents, &mut inputs)).ok();
                        }
                    }
                    Reading::NONE => {
                        if contents.contains(SNIPPET_TAG) {
                            reading = Reading::SNIPPET;
                        } else if contents.contains(SNIPPET_REQUIRES) {
                            reading = Reading::REQUIRES;
                        }
                    }
                }
            }
        }
    }
    (title, config)
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
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                paths.extend(get_dirs(&path)?)
            } else {
                let dir = path.to_str().unwrap();
                if dir.contains("mod.rs") {
                    continue;
                } 
                paths.push(path)
            }
        }
    }
    Ok(paths)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: cargo run <path to snippets directory> <path to extension package.json>");
        return Ok(());
    }

    let snippets_path = &args[1];
    let extension_config_path = &args[2];
    
    let mut snippets = object! {};

    let paths = get_dirs(Path::new(snippets_path))?;

    for path in paths {
        let (title, config) = generate_snippet(path);
        snippets[title] = config;
    }

    let extension_config_contents = read_to_string(extension_config_path).unwrap();

    let mut extension_config = json::parse(&extension_config_contents).unwrap(); 

    extension_config["contributes"]["configurationDefaults"]["rust-analyzer.completion.snippets.custom"] = snippets;

    let mut extension_config_file = File::create(extension_config_path)?;

    extension_config.write_pretty(&mut extension_config_file, 2).ok();
    
    Ok(())
}
