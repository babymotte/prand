extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use clap::*;

fn main() {

    let default_lenght = 10;
    let default_number = 1;

    let matches = App::new("prand")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Generate random passwords")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("LENGTH")
            .help(&format!("Length of the generated password (default: {})", default_lenght))
            .takes_value(true))
        .arg(Arg::with_name("number")
            .short("n")
            .value_name("NUMBER OF PASSWORDS")
            .help(&format!("Generate several passwords at once (default: {})", default_number))
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
    
    let verbosity = match matches.occurrences_of("v") {
        x if x < 3 => x,
        _ => 3
    };

    let len = matches.value_of("length").unwrap_or("10").parse().unwrap_or_else(|s| { println!("Not a valid length: {}; using 10 instead.", s); 10 });
    let n = matches.value_of("number").unwrap_or("1").parse().unwrap_or_else(|s| { println!("Not a valid number: {}; using 1 instead.", s); 1 });

    if verbosity > 0 {
        println!("Creating {} password(s) of size {}:\n", n, len);
    }

    for _ in 0..n {
        println!("{}", generate(len));
    }
}

fn generate(len: usize) -> String {

    let mut len_counter = 0;
    
    // let rand = "/dev/random";
    let rand = "/dev/urandom";

    let mut f = File::open(rand).expect("rand file not found");

    let mut buf = [0; 1];
    let mut passwd = String::new();

    while len_counter < len {
        f.read(&mut buf).expect(&format!("could not read from {:?}", f));
        if (buf[0] >= '0' as u8 && buf[0] <= '9' as u8) || (buf[0] >= 'A' as u8 && buf[0] <= 'Z' as u8) || (buf[0] >= 'a' as u8 && buf[0] <= 'z' as u8) {
            passwd.push(buf[0] as char);
            len_counter = len_counter + 1;
        }
    }

    passwd
}
