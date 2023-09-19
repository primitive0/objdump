use std::{env, fs};

mod ins;
mod ihex;
mod translator;

fn main() {
    let ins_data = fs::read_to_string("instructions.txt")
        .expect("failed to read instructions.txt");
    let table = ins::parse_instructions(&ins_data);

    let in_file = env::args().skip(1).next()
        .expect("expected 1 argument to program");
    let contents = fs::read_to_string(in_file).unwrap();
    let mut data = ihex::read_ihex(&contents);

    // to le
    for pair in data.chunks_exact_mut(2) {
        let f = pair[0];
        let l = pair[1];
        pair[0] = l;
        pair[1] = f;
    }

    let r = translator::translate(&table, &data);
    println!("{}", r);
}
