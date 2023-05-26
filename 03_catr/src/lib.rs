use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "catr")]
#[command(author = "zuya@microsoft.com")]
#[command(version)]
#[command(about = "Rust cat", long_about = None)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    if config.number_lines {
        return echo_number(config.files);
    } else if config.number_nonblank_lines {
        return echo_non_blank_number(config.files);
    } else {
        echo(config.files)
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn echo(files: Vec<String>) -> MyResult<()> {
    for filename in files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                for line in f.lines() {
                    println!("{}", line?);
                }
            }
        }
    }
    Ok(())
}

fn echo_number(files: Vec<String>) -> MyResult<()> {
    let mut acc = 1;
    for filename in files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                let mut size = 0;
                for (num, line) in f.lines().enumerate() {
                    let line = line?;
                    println!("{:>6}\t{}", acc + num, line);
                    size += 1;
                }
                acc += size;
            }
        }
    }
    Ok(())
}

fn echo_non_blank_number(files: Vec<String>) -> MyResult<()> {
    let mut acc = 0;
    for filename in files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                let mut last_num = 0;
                for (_num, line) in f.lines().enumerate() {
                    let line = line?;
                    if !line.is_empty() {
                        last_num += 1;
                        println!("{:>6}\t{}", last_num + acc, line);
                    } else {
                        println!();
                    }
                }
                acc += last_num;
            }
        }
    }
    Ok(())
}
