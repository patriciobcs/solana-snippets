mod consts;

use json::{object, JsonValue};
use std::collections::HashMap;
use std::fs::{read_dir, File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
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

fn generate_snippet(filepath: String) -> (String, JsonValue) {
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

fn main() -> std::io::Result<()> {
    let mut snippets = object! {};

    let paths = read_dir(SNIPPETS_DIR).unwrap();

    for path in paths {
        let path_buf = path.unwrap().path();
        let filepath = path_buf.to_str().unwrap();
        if filepath.contains("mod.rs") {
            continue;
        };
        let (title, config) = generate_snippet(filepath.into());
        snippets[title] = config;
    }

    let extension_config_contents = read_to_string(EXTENSION_CONFIG_DIR).unwrap();

    let mut extension_config = json::parse(&extension_config_contents).unwrap(); 

    extension_config["contributes"]["configurationDefaults"]["rust-analyzer.completion.snippets.custom"] = snippets;

    let mut extension_config_file = File::create(EXTENSION_CONFIG_DIR)?;

    extension_config.write_pretty(&mut extension_config_file, 2).ok();
    
    Ok(())
}
