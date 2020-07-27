pub(crate) mod tokenizer;

pub mod parser;

pub(crate) mod date;

pub(crate) mod table;

pub fn parse<'a>(code: String) -> Vec<parser::TOML> {
    parser::parse(tokenizer::tokenize(&*code))
}
