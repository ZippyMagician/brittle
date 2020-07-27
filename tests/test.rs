extern crate brittle;

use brittle::parser::*;

use std::fs::File;
use std::io::*;

#[test]
fn cargo_test() {
    if let Ok(code) = read_file("tests/cargo_test.toml") {
        let summary = vec![
            TOML::Title(vec!["package".to_string()]),
            TOML::Assign("name".to_string(), Value::Str("brittle".to_string())),
            TOML::Assign("version".to_string(), Value::Str("0.1.0".to_string())),
            TOML::Assign(
                "authors".to_string(),
                Value::Array(vec![Value::Str("ZippyMagician <zippymagician1@gmail.com>".to_string())]),
            ),
            TOML::Assign("edition".to_string(), Value::Str("2018".to_string())),
            TOML::Title(vec!["dependencies".to_string()]),
            TOML::Assign("regex".to_string(), Value::Str("1.3.9".to_string())),
        ];

        assert_eq!(brittle::parse(code), summary);
    } else {
        panic!("Test failed, file not found");
    }
}

#[test]
fn multiline_test() {
    if let Ok(code) = read_file("tests/multiline_test.toml") {
        let summary = vec![
            TOML::Title(vec!["main".to_string()]),
            TOML::Assign("count".to_string(), Value::Float(0.75)),
            TOML::Title(vec!["main".to_string(), "child".to_string()]),
            TOML::Assign("count".to_string(), Value::Float(0.5))
        ];

        assert_eq!(brittle::parse(code), summary);
    } else {
        panic!("Test failed, file not found");
    }
}

fn read_file(path: &str) -> std::io::Result<String> {
    let file = File::open(path)?;
    let buffer = BufReader::with_capacity(128 * 1024, file);
    let lines = buffer
        .lines()
        .map(|x| x.unwrap_or("".to_string()))
        .collect::<Vec<String>>();
    
    Ok(lines.join("\n"))
}