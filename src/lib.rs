use std::result;
use regex::Regex;
use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiniGrepError {
    #[error("failed to parse arguments.\nminigrep --help to see usage.")]
    ArgumentParseError,
    #[error("failed to compile regex r\"{0}\": {1}")]
    RegexCompileError(String, String),
    #[error("Failed to read the file {0}: {1}")]
    FileReadError(String, String),
}


pub struct Config {
    pub re: Regex,
    pub filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> result::Result<Self, MiniGrepError> {
        let query = args.get(1).ok_or(MiniGrepError::ArgumentParseError)?;
        let re = Regex::new(query).map_err(|err| MiniGrepError::RegexCompileError(query.to_owned(), err.to_string()))?;
        let filename = args.get(2).ok_or(MiniGrepError::ArgumentParseError)?;

        Ok(Self {
            re,
            filename: filename.to_string(),
        })
    }
}

pub struct Result<'a> {
    pub line_content: &'a str,
    pub line_number: u64,
}

pub fn search<'a>(content: Vec<&'a str>, config: &Config) -> Vec<Result<'a>> {
    let mut result: Vec<Result> = Vec::new();
    let mut i: u64 = 0;
    for line in content {
        i += 1;
        // Check if the line matches the regex
        if config.re.is_match(line) {
            // Highlight the matching part of the line with color
            let highlighted_line = config.re.replace_all(line, "$0".red().bold().to_string());

            // Push the highlighted line to the result
            result.push(Result {
                line_content: Box::leak(highlighted_line.into_owned().into_boxed_str()),
                line_number: i,
            });
        }
    }
    result
}
