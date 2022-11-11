use json::{object, JsonValue};
use std::collections::HashMap;
use std::fs::{read_dir, File, read_to_string};
use std::io::{self, BufRead};
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
fn parse_requires_line(contents: String) -> String {
    contents.trim().replacen("use ", "", 1).replacen(";", "", 1)
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
    let mut is_reading_snippet = false;
    let mut is_reading_requires = false;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("title") {
                    title = parse_config(ip, "title".into());
                } else if ip.contains("description") {
                    config["description"] = parse_config(ip, "description".into()).into();
                } else if ip.contains("fn") {
                    config["prefix"].push(parse_command(ip)).ok();
                } else if ip.contains("snippet-body-start") {
                    is_reading_snippet = true;
                } else if ip.contains("snippet-body-end") {
                    is_reading_snippet = false;
                } else if is_reading_snippet {
                    config["body"].push(parse_body_line(ip, &mut inputs)).ok();
                } else if ip.contains("snippet-requires-start") {
                    is_reading_requires = true;
                } else if ip.contains("snippet-requires-end") {
                    is_reading_requires = false;
                } else if is_reading_requires && ip.contains("use ") {
                    config["requires"].push(parse_requires_line(ip)).ok();
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

const EXTENSION_CONFIG_DIR: &str = "../package.json";

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
