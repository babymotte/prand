use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let len = 10;
    let mut counter = 0;
    
    // let rand = "/dev/random";
    let rand = "/dev/urandom";

    let mut f = File::open(rand).expect("rand file not found");

    let mut buf = [0; 1];
    let mut passwd = String::new();

    while counter < len {
        f.read(&mut buf).expect(&format!("could not read from {:?}", f));
        if (buf[0] >= '0' as u8 && buf[0] <= '9' as u8) || (buf[0] >= 'A' as u8 && buf[0] <= 'Z' as u8) || (buf[0] >= 'a' as u8 && buf[0] <= 'z' as u8) {
            passwd.push(buf[0] as char);
            counter = counter + 1;
        }
    }

    println!("{}", passwd);
}
