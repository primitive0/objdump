use std::{env, fs, path::PathBuf};

use clap::{arg, command, value_parser};

mod ins;
mod ihex;
mod translator;

fn main() {
    let ins_data = match fs::read_to_string("instructions.txt") {
        Ok(v) => v,
        Err(err) => {
            println!("error: failed to read instructions.txt:\n{}", err);
            return;
        },
    };
    let patterns = match ins::parse_instructions(&ins_data) {
        Ok(v) => v,
        Err(err) => {
            println!("error: failed to parse instructions.txt:\n{}", err);
            return;
        },
    };

    let args = command!()
        .arg(
            arg!(<file> "Intel HEX file to decode")
                .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let path = args.get_one::<PathBuf>("file").unwrap();
    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(err) => {
            println!("error: failed to read input data:\n{}", err);
            return;
        }
    };

    let Some(data) = ihex::read_ihex(&contents) else {
        println!("error: failed to read ihex file");
        return;
    };

    let r = match translator::translate(&patterns, &data) {
        Ok(v) => v,
        Err(err) => {
            println!("error: failed to decode assembler:\n{}", err);
            return;
        },
    };
    println!("{}", r);
}
