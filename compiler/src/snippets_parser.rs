use json::{object, JsonValue};
use std::collections::HashMap;
use std::fs::{File, read_dir};
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
            display: "ra",
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
                            } else if contents.trim().starts_with(&DISPLAY.as_comment()) {
                                config[DISPLAY] = parse_config(contents, DISPLAY).into();
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

pub fn get_snippets(snippets_path: &String) -> JsonValue {
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