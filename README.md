# prand

A command line utility for generating random passwords.

## What is it good for?

Generating random passwords.

## But who on earth needs another password generator?

Pretty much no one I guess. But I thought it was a nice exercise for me to practice Rust and get familiar with cargo and crates.io, so here we are.

If you think the crate name should be made available for something more useful, feel free to contact me.

## How do I use it?

 ```
USAGE:
    prand [FLAGS] [OPTIONS]

FLAGS:
    -a, --alphabetic    Include alphabetic characters (a-z, A-Z)
    -h, --help          Prints help information
    -L, --lower-case    Use lower case letters as alphabetic characters
    -n, --numeric       Include numeric characters (0-9)
    -s, --symbols       Include symbol characters (-ss to include ambiguous symbols)
    -U, --upper-case    Use upper case letters as alphabetic characters
    -v                  Sets the level of verbosity
    -V, --version       Prints version information

OPTIONS:
    -l, --length <LENGTH>           Length of the generated password (default: 10)
    -N <NUMBER OF PASSWORDS>        Number of passwords to be generated (default: 1)
 ```