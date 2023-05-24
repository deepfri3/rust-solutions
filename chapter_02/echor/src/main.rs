
//use std::env;
extern crate clap;
use clap::{App,Arg,load_yaml};

fn main() {
    let yaml = load_yaml!("echor.yml");
    let echor_yaml = App::from_yaml(yaml);
    let matches = echor_yaml.get_matches();
    //println!("{:#?}",matches);

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    //let ending = if omit_newline { "" } else { "\n" };
    //print!("{}{}", text.join(" "), ending);
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    //println!("{:?}", text);

    let echor =
      App::new("echor")
        .version("0.1.0")
        .author("George Baker")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .help("Do not print newline")
            .takes_value(false),
        );
    let matches_test = echor.get_matches_from(vec!["echor","hello","world"]);
    let vals: Vec<&str> = matches_test.values_of("text").unwrap().collect();
    //println!("{:?}", vals);
    assert_eq!(vals, ["hello", "world"]);
}
