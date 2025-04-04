use std::env;
use std::fs;
use std::process;

use colored::Colorize;
use minigrep::Config;
use minigrep::Error;
use minigrep::Result;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(flag) = args.get(1) {
        if *flag == String::from("--help") {
            println!("Usage: minigrep [pattern] [filename]\n\n    pattern: the regex to search for.\n    filename: the file to search in.\n");
            println!("Example1:\n\tminigrep \"Hello, world!\" file.txt -> this will search for the string \"Hello, world!\" in file.txt");
            println!("Example2:\n\tminigrep '\\w' file2.txt -> this search will match every word charcater in file2.txt.");
            println!("Example2:\n\tminigrep ^[A-Z] file2.txt -> this search will match every line that starts with a capital letter in file2.txt");
            return
        }
    }

    let config = match Config::new(args) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let content = fs::read_to_string(&config.filename)
        .map_err(|err| Error::FileReadError(Box::new(err)))
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1)
        });

    let content: Vec<&str> = content.split("\n").collect(); //convert content from String to Vec<&str>

    let results: Vec<Result> = minigrep::search(content, &config);

    if results.len() == 0 {
        println!(
            "No match was found for {} in {}",
            config.re.as_str(), config.filename
        );
    } else {
        for res in results {
            println!("{}{}: {}","[+] Line #".green().bold(), res.line_number.to_string().green().bold(), res.line_content);
        }
    }

}
