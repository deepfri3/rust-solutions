use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("George Baker <george@gb3.dev>")
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
                .long("nonewline")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();

    //println!("{:#?}",matches);

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
/*
    let first_vector = vec!["Hello", "world"];
    println!("{}",first_vector[0]);
    println!("{}",first_vector[1]);
    println!("{}",first_vector.join(" "));
    assert_eq!(first_vector.join(" "), "Hello world");
*/
/*
    // version 1
    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }
    //print!("{}{}", text.join(" "), ending);

    // version 2
    let better_ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), better_ending);
*/
    // version 3
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
