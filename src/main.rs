extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
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
        .arg(Arg::with_name("secure")
            .short("s")
            .long("secure")
            .help("Use a high entropy source, if available"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
    
    let verbosity = match matches.occurrences_of("v") {
        x if x < 3 => x,
        _ => 3
    };

    let len = matches.value_of("length").unwrap_or("10");
    let len = len.parse().unwrap_or_else(|_| { println!("Not a valid length: {}; using 10 instead.", len); 10 });

    let n = matches.value_of("number").unwrap_or("1");
    let n = n.parse().unwrap_or_else(|_| { println!("Not a valid number: {}; using 1 instead.", n); 1 });

    let secure = matches.is_present("secure");

    let source = get_source(secure);

    if secure && verbosity > 0 {
        println!("Using secure random. If password generation takes very long, try moving your mouse around to provide the system with more entropy.");
    }

    if verbosity > 0 {
        println!("Creating {} password(s) of size {}:", n, len);
    }

    for _ in 0..n {
        generate(len, source, |ch| {
            stdout().write(ch).expect("failed to write to stdout");
            stdout().flush().expect("failed to flush stdout");
        });
        println!();
    }
}

fn get_source(secure: bool) -> &'static str {

    // TODO make this platform independent

    if secure {
        "/dev/random"
    } else {
        "/dev/urandom"
    }
}

fn generate(len: usize, source: &str, consumer: impl Fn(&[u8])) {

    let mut len_counter = 0;

    let mut f = File::open(source).expect("rand file not found");

    let mut buf = [0; 1];

    while len_counter < len {
        f.read(&mut buf).expect(&format!("could not read from {:?}", f));
        if (buf[0] >= '0' as u8 && buf[0] <= '9' as u8) || (buf[0] >= 'A' as u8 && buf[0] <= 'Z' as u8) || (buf[0] >= 'a' as u8 && buf[0] <= 'z' as u8) {
            consumer(&buf);
            len_counter = len_counter + 1;
        }
    }
}
