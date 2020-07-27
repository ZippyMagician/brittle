use regex::Regex;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Punctuation(String),

    Scalar(String),

    Comment,
}

// Takes in a string seperated by newlines
// Returns a vector of vectors.
//     Each vector represents a line, containing tokens to be parsed
pub(crate) fn tokenize<'a>(code: &'a str) -> Vec<Vec<Token>> {
    let match_scalar = Regex::new(r#"^[.[^=\{\}\[\]]]+\s?"#).unwrap();
    let match_punc = Regex::new(r#"^[\{\}\[\]=]"#).unwrap();
    let match_comment = Regex::new(r#"#.+"#).unwrap();

    let mut stream: Vec<Vec<Token>> = Vec::new();
    for _ in code.split('\n') {
        stream.push(Vec::new());
    }

    for (idx, mut line) in code.split('\n').into_iter().enumerate() {
        line = line.trim();

        while line.len() >= 1 {
            if match_comment.is_match(line) {
                line = "";
            } else if let Some(result) = match_scalar.captures(line) {
                line = &line[result[0].len()..];
                stream[idx].push(Token::Scalar(result[0].trim().to_string()));
            } else if let Some(result) = match_punc.captures(line) {
                line = &line[1..];
                stream[idx].push(Token::Punctuation(result[0].trim().to_string()));
            } else {
                eprintln!("Unrecognized token in line: {}", line);
            }
        }
    }

    stream
        .iter()
        .filter_map(|item| strip(item.clone()))
        .collect()
}

fn strip(stream: Vec<Token>) -> Option<Vec<Token>> {
    let mut ret = Vec::new();

    for item in stream {
        if Token::Scalar(String::new()) != item {
            ret.push(item);
        }
    }

    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}
