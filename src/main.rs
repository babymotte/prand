extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use clap::*;

fn main() {

    let matches = App::new("prand")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Generate random passwords")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("LENGTH")
            .help("Length of the generated password (default: 10)")
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

    println!("{}", generate(len, verbosity));
}

fn generate(len: usize, verbosity: u64) -> String {

    if verbosity > 0 {
        println!("Creating password of size {}.", len);
    }

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
