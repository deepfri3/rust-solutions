
use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self,BufRead,BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>, // list of files
    number_lines: bool, // -n  option to print line numbers
    number_nonblank_lines: bool, // -b option to  print non-blank lines
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .author("George Baker")
        .version("0.1.0")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
            .value_name("FILE")
            .default_value("-")
            .multiple(true)
            .help("file(s) to concatenate"),
        )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .takes_value(false)
            .conflicts_with("number_nonblank")
            .help("print the line number for each line"),
        )
        .arg(
            Arg::with_name("number-nonblank")
            .short("b")
            .long("number-nonblank")
            .takes_value(false)
            .help("print the line number for each non-blank line"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}


pub fn run(config: Config) -> MyResult<()> {
    let mut line_count = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_count + 1, line);
                        line_count += 1;
                    }
                    else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        }
                        else {
                            println!("{:6}\t{}", line_count + 1, line);
                            line_count += 1;
                        }
                    }
                    else {
                        println!("{}", line);
                    }
                }
            }
        };
    }
    Ok(())
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
