use rayon::prelude::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use walkdir::WalkDir;

struct Config {
    pattern: String,
    path: String,
}

fn print_usage() {
    eprintln!("Usage: mini-grep <pattern> <file|dir|- (stdin)>");
    eprint!("\n");
    eprintln!("Options:");
    eprintln!("  -h, --help       Print this help message");
    eprintln!("  -V, --version    Print version information");
    eprint!("\n");
    eprintln!("Examples:");
    eprintln!("  mini-grep \"apple\" sample.txt");
    eprintln!("  mini-grep \"foo.*bar\" ./logs");
    eprintln!("  type sample.txt | mini-grep \"apple\" -");
}

fn print_version() {
    // CARGO_* env macros are set at compile time by Cargo
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn run(pattern: &str, path: &str) -> Result<()> {
    // 正規表現をコンパイル
    let re = Regex::new(pattern)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;

    // パスがディレクトリの場合は再帰的にファイル収集
    let paths: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    paths.par_iter().for_each(|path| match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for (index, line) in reader.lines().enumerate() {
                if let Ok(content) = line {
                    if re.is_match(&content) {
                        println!("{}:{}: {}", path.display(), index + 1, content);
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to open file: {}", path.display());
        }
    });

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // ヘルプ表示
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_usage();
        return Ok(());
    }

    // バージョン表示
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        print_version();
        return Ok(());
    }

    if args.len() < 3 {
        eprintln!("Usage: mini-grep <pattern> <file|dir|- (stdin)>");
        std::process::exit(1);
    }

    let config = Config {
        pattern: args[1].clone(),
        path: args[2].clone(),
    };

    if let Err(e) = run(&config.pattern, &config.path) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
