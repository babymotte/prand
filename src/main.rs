extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::prelude::*;

fn main() {
    let default_lenght = 10;
    let default_number = 1;

    let matches = App::new("prand")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Generate random passwords")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("LENGTH")
                .help(&format!(
                    "Length of the generated password (default: {})",
                    default_lenght
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .short("N")
                .value_name("NUMBER OF PASSWORDS")
                .help(&format!(
                    "Number of passwords to be generated (default: {})",
                    default_number
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alphabetic")
                .short("a")
                .long("alphabetic")
                .help("Include alphabetic characters (a-z, A-Z)"),
        )
        .arg(
            Arg::with_name("numeric")
                .short("n")
                .long("numeric")
                .help("Include numeric characters (0-9)"),
        )
        .arg(
            Arg::with_name("symbols")
                .short("s")
                .long("symbols")
                .multiple(true)
                .help("Include symbol characters (-ss to include ambiguous symbols)"),
        )
        .arg(
            Arg::with_name("upper_case")
                .short("U")
                .long("upper-case")
                .help("Use upper case letters as alphabetic characters"),
        )
        .arg(
            Arg::with_name("lower_case")
                .short("L")
                .long("lower-case")
                .help("Use lower case letters as alphabetic characters"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let verbosity = match matches.occurrences_of("v") {
        x if x < 3 => x,
        _ => 3,
    };

    let mut alpha = matches.is_present("alphabetic");
    let mut num = matches.is_present("numeric");
    let symb = matches.is_present("symbols");
    let amb_symb = matches.occurrences_of("symbols") > 1;
    let mut upper = matches.is_present("upper_case");
    let mut lower = matches.is_present("lower_case");

    if !alpha && !num && !symb {
        alpha = true;
        num = true;
    }

    if !upper && !lower {
        upper = alpha;
        lower = alpha;
    }

    if alpha && verbosity > 1 {
        println!("Including alphabetic characters")
    }

    if num && verbosity > 1 {
        println!("Including numeric characters")
    }

    if symb && verbosity > 1 {
        println!("Including symbol characters")
    }

    if amb_symb && verbosity > 1 {
        println!("Including ambiguous symbol characters")
    }

    let len = matches.value_of("length").unwrap_or("10");
    let len = len.parse().unwrap_or_else(|_| {
        println!(
            "Not a valid length: {}; using {} instead.",
            len, default_lenght
        );
        default_lenght
    });

    let n = matches.value_of("number").unwrap_or("1");
    let n = n.parse().unwrap_or_else(|_| {
        println!(
            "Not a valid number: {}; using {} instead.",
            n, default_number
        );
        default_number
    });

    if verbosity > 0 {
        println!("Creating {} password(s) of size {}:", n, len);
    }

    let mut pool = Vec::new();

    for ch in b'!'..=b'~' {
        pool.push(ch);
    }

    if !upper {
        remove_all(&mut pool, b'A'..=b'Z');
    }

    if !lower {
        remove_all(&mut pool, b'a'..=b'z');
    }

    if !num {
        remove_all(&mut pool, b'0'..=b'9');
    }

    if !amb_symb {
        let ambiguous_symbols = vec![
            b'{', b'}', b'[', b']', b'(', b')', b'/', b'\\', b'\'', b'"', b'`', b'~', b',', b';',
            b':', b'.', b'<', b'>',
        ];
        remove_all(&mut pool, ambiguous_symbols.into_iter());
    }

    if !symb {
        remove_all(&mut pool, b'!'..b'0');
        remove_all(&mut pool, b':'..b'A');
        remove_all(&mut pool, b'['..b'a');
        remove_all(&mut pool, b'{'..b'~');
    }

    for _ in 0..n {
        println!("{}", generate(len, &pool));
    }
}

fn generate(len: usize, pool: &[u8]) -> String {
    let mut passwd = String::new();

    let mut rng = rand::thread_rng();

    while passwd.len() < len {
        let i: usize = rng.gen();
        passwd.push(pool[i % pool.len()] as char);
    }

    passwd
}

fn remove_all(pool: &mut Vec<u8>, to_be_removed: impl Iterator<Item = u8>) {
    for ch in to_be_removed {
        for i in (0..pool.len()).rev() {
            if pool[i] == ch {
                pool.swap_remove(i);
            }
        }
    }
}
