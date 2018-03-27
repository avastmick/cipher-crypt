extern crate cipher_crypt;
extern crate clap;
extern crate colored;

use cipher_crypt::*;
use colored::Colorize;
use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("list")
                .help("Lists the names of the ciphers")
                .short("l")
                .long("list")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("list") {

        println!(
            "List out all the modules of cipher_crypt",
        );
    } else {
        panic!("Nothing asked to do! Try -h, --help")
    }

}

extern crate assert_cli;

#[cfg(test)]
mod tests {

    use assert_cli::Assert;

    #[test]
    fn test_help() {
        Assert::main_binary()
            .with_args(&["--help"])
            .succeeds()
            .unwrap();
    }

    #[test]
    fn test_fail_no_params() {
        Assert::main_binary().fails().unwrap();
    }
}
