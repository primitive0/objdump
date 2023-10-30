use std::{env, fs, path::PathBuf};

use clap::{arg, command, value_parser};

mod ins;
mod ihex;
mod translator;

fn swap_bytes(data: &mut [u8]) {
    for pair in data.chunks_exact_mut(2) {
        let f = pair[0];
        let l = pair[1];
        pair[0] = l;
        pair[1] = f;
    }
}

fn main() {
    let ins_data = match fs::read_to_string("instructions.txt") {
        Ok(v) => v,
        Err(err) => {
            println!("error: failed to read instructions.txt:\n{}", err);
            return;
        },
    };
    let table = ins::parse_instructions(&ins_data);

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

    let mut data = ihex::read_ihex(&contents);
    swap_bytes(&mut data);

    let r = translator::translate(&table, &data);
    println!("{}", r);
}
