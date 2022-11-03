use json::{object, JsonValue};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn parse_config(contents: String, config_name: String) -> String {
    let start_tag = config_name + ":";
    let start_bytes = contents.find(&start_tag).unwrap() + start_tag.len();
    contents[start_bytes..].trim().into()
}

fn parse_command(contents: String) -> String {
    let start_tag = "fn ";
    let end_tag = "(";
    let start_bytes = contents.find(&start_tag).unwrap() + start_tag.len();
    let end_bytes = contents.find(&end_tag).unwrap();
    contents[start_bytes..end_bytes].into()
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

fn generate_snippet(filepath: String) -> (String, JsonValue) {
    let mut inputs = HashMap::new();
    let mut config = object! {
        description: "",
        prefix: [],
        body: [],
    };
    let mut title: String = "".into();
    let mut is_reading_snippet = false;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("title") {
                    title = parse_config(ip, "title".into());
                } else if ip.contains("description") {
                    config["description"] = parse_config(ip, "description".into()).into();
                } else if ip.contains("fn") {
                    config["prefix"].push(parse_command(ip)).ok();
                } else if ip.contains("snippet-start") {
                    is_reading_snippet = true;
                } else if ip.contains("snippet-end") {
                    is_reading_snippet = false;
                } else if is_reading_snippet {
                    config["body"].push(parse_body_line(ip, &mut inputs)).ok();
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

const SNIPPETS_DIR: &str = "../snippets/src/core";
const SNIPPETS_CONFIG_DIR: &str = "../snippets.json";

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

    let mut snippets_file = File::create(SNIPPETS_CONFIG_DIR)?;

    snippets_file.write(snippets.dump().as_bytes()).ok();

    Ok(())
}
