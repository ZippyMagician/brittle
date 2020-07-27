use crate::date::Date;
use crate::table::Table;
use crate::tokenizer::Token;

use regex::Regex;

// Simple macro so I don't need to type out (Value::Empty, 0) a bunch of times
macro_rules! empty {
    () => {
        (Value::Empty, 0)
    };
}

#[derive(PartialEq, Clone, Debug)]
pub enum TOML {
    // Marks an assign token
    Assign(String, Value),

    // A vector, from parent to child
    Title(Vec<String>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Str(String),

    Int(i64),

    Float(f64),

    Date(Date),

    Array(Vec<Value>),

    Table(Table),

    // For edge cases with invalid assignments
    Empty,
}

pub(crate) fn parse(stream: Vec<Vec<Token>>) -> Vec<TOML> {
    let mut result = Vec::new();
    for line_stream in stream {
        if let Some(parse_result) = parse_head(line_stream) {
            result.push(parse_result);
        }
    }

    result
}

// Head parse, parses the line (handles assignments and tags)
// Will call child parser to handle the values being handed to an object
fn parse_head(line: Vec<Token>) -> Option<TOML> {
    for (index, tok) in line.clone().iter().enumerate() {
        match tok {
            Token::Punctuation(value) => {
                // If the punctuation is "=".to_string(), then we can reference the items before and after to construct an assignment
                // Otherwise we check if there is a tag on the line (there will be square brackets but no "=".to_string())
                if *value == "=".to_string() {
                    // Make sure it is a valid assignment
                    if let Some(Token::Scalar(val)) = line.get(index - 1) {
                        return Some(TOML::Assign(
                            val.to_string(),
                            parse_child(line[(index + 1)..].into()).0,
                        ));
                    } else {
                        eprint!("Invalid assignment at {:?}", line);
                        return None;
                    }
                } else if *value == "[".to_string()
                    && !line.contains(&Token::Punctuation("=".to_string()))
                {
                    // Make sure it is a valid tag
                    if let Some(Token::Scalar(val)) = line.get(index + 1) {
                        if Some(&Token::Punctuation("]".to_string())) == line.get(index + 2) {
                            return Some(TOML::Title(
                                val.split(".")
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>(),
                            ));
                        } else {
                            eprint!("Invalid tag at {:?}", line);
                        }
                    } else {
                        eprint!("Invalid tag at {:?}", line);
                    }

                    return None;
                }
            }
            _ => {}
        }
    }

    None
}

// Handles the parsing of all Value tokens (string, int, float, date, etc)
// Returns that for use by the parent parser
fn parse_child(tokens: Vec<Token>) -> (Value, usize) {
    let head = tokens.get(0);
    // Invalid assignment
    if head == None {
        eprint!("Cannot assign nothing to an object. Error at: {:?}", tokens);
        return empty!();
    }

    match head.unwrap() {
        Token::Scalar(value) => {
            // Can be everything but a table or array at this point
            let is_float = Regex::new(r#"^[\d\.]+"#).unwrap();
            let is_int = Regex::new(r#"^\d+"#).unwrap();
            let is_str = Regex::new(r#"^"?.+"?"#).unwrap();
            let is_date =
                Regex::new(r#"^((?:\d+-\d+-\d+[T\s]?)?\d+:\d+:\d+[\.\dZ]*(?:-\d+:\d+)*)[\n.]*"#)
                    .unwrap();

            if is_date.is_match(&*value) {
                return (Value::Date(Date::new(value.clone())), 1);
            } else if is_float.is_match(&*value) {
                return (Value::Float((&*value).clone().parse().unwrap()), 1);
            } else if is_int.is_match(&*value) {
                return (Value::Int((&*value).clone().parse().unwrap()), 1);
            } else if is_str.is_match(&*value) {
                return (
                    Value::Str((&*value).clone()[1..(value.len() - 1)].into()),
                    1,
                );
            } else {
                eprint!("Unrecognized item in assignment: {}", value);
                return empty!();
            }
        }
        Token::Punctuation(punc) => {
            // If punctuation is [, we have an array. If punctuation is {, we have a table
            if *punc == "[".to_string() {
                // Goto end of array, if there is no end then invalid
                let end = goto(
                    tokens.clone(),
                    Token::Punctuation("[".to_string()),
                    Token::Punctuation("]".to_string()),
                );

                // If the array is valid, then parse it and return the value
                if let Some(pos) = end {
                    let mut vec: Vec<Value> = vec![];
                    let mut slice = tokens.clone();
                    while slice.len() > tokens.len() - pos {
                        // Strip the first item and parse it (will always be either "[" or ",")
                        let (obj, len) = parse_child(slice[1..].into());
                        slice = slice[(len + 1)..].into();
                        vec.push(obj);
                    }
                    return (Value::Array(vec), pos);
                } else {
                    eprint!("Invalid array token at: {:?}", tokens);
                    return empty!();
                }
            } else if *punc == "{".to_string() {
                // Goto end of table, if there is no end then invalid
                let end = goto(
                    tokens.clone(),
                    Token::Punctuation("{".to_string()),
                    Token::Punctuation("}".to_string()),
                );

                // If the table is valid, begin parsing it
                // Will contain lots of assigns, so calls the main parse function for each
                if let Some(pos) = end {
                    let mut hash = Table::new();
                    let mut slice = tokens.clone();
                    while slice.len() > tokens.len() - pos {
                        let next = goto(
                            tokens.clone(),
                            Token::Comment,
                            Token::Punctuation(",".to_string()),
                        )
                        .unwrap();
                        // Strip the first item (will always be either "{" or ",")
                        let obj = parse_head(slice[1..].into()).unwrap();
                        // Make sure it is an assign operator inside the obj
                        if let TOML::Assign(left, right) = obj {
                            hash.update(Value::Str(left), right);
                        } else {
                            // Something went wrong
                            eprint!("Something went wrong parsing: {:?}", slice);
                        }
                        slice = slice[next..].into();
                    }

                    return (Value::Table(hash), pos);
                } else {
                    eprint!("Invalid table token at: {:?}", tokens);
                    return empty!();
                }
            } else {
                eprint!("Unrecognized punctuation at: {:?}", tokens);
                return empty!();
            }
        }
        _ => {}
    }

    empty!()
}

fn goto(tokens: Vec<Token>, start: Token, end: Token) -> Option<usize> {
    let mut pointer: usize = 0;
    let mut count = 0;
    for item in tokens {
        if item == start {
            count += 1;
        } else if item == end {
            count -= 1;
        }

        if count == 0 && item == end {
            return Some(pointer);
        }

        pointer += 1;
    }

    None
}
