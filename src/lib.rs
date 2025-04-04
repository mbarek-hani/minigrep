use std::fmt::Display;
use std::result;
use regex::Regex;
use colored::Colorize;

#[derive(Debug)]
pub enum Error<'a> {
    ArgumentParseError(&'a str),
    FileReadError(Box<dyn std::error::Error>),
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ArgumentParseError(err) => write!(f, "{}", err)?,
            Error::FileReadError(err) => write!(f, "{}", err)?,
        }
        Ok(())
    }
}

pub struct Config {
    pub re: Regex,
    pub filename: String,
}

impl Config {
    pub fn new<'a>(args: Vec<String>) -> result::Result<Self, Error<'a>> {
        let query = args.get(1).ok_or(Error::ArgumentParseError(
            "Failed to parse arguments.Type minigrep --help to see usage.",
        ))?;
        let re = Regex::new(query).map_err(|err| {
            let msg = format!("Failed to compile regex \"{}\": {}", query, err);
            Error::ArgumentParseError(Box::leak(msg.into_boxed_str()))
        })?;
        let filename = args.get(2).ok_or(Error::ArgumentParseError(
            "Failed to parse arguments.Type minigrep --help to see usage.",
        ))?;

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
