use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct Config {
    pattern: String,
    path: String,
}

fn run(config: Config) -> Result<()> {
    let file = File::open(config.path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&config.pattern) {
            println!("{}: {}", i + 1, line);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: mini-grep <pattern> <file>");
        std::process::exit(1);
    }

    let config = Config {
        pattern: args[1].clone(),
        path: args[2].clone(),
    };

    run(config)
}
