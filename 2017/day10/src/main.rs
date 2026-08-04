use std::{
    env,
    fs::File,
    io::{self},
    process::ExitCode,
};

use crate::day10::Circle;

mod day10;

fn main() -> ExitCode {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let input_path = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Usage: {program_name} <input file>");
            return ExitCode::FAILURE;
        }
    };

    let input = match File::open(&input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open {input_path} for reading: {e}");
            return ExitCode::FAILURE;
        }
    };
    let input = match io::read_to_string(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read input: {e}");
            return ExitCode::FAILURE;
        }
    };

    let lengths = match day10::read_lengths_parsing(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read lengths: {e}");
            return ExitCode::FAILURE;
        }
    };

    let circle_state = Circle::new(256).run_round(&lengths);

    let p1_res = circle_state.get()[0] * circle_state.get()[1];

    println!(
        "\
=== Part 1 ===
Product: {p1_res}"
    );

    let lengths = day10::read_lengths_nonparsing(&input);
    let sparse_hash = match Circle::new(256).get_hash(&lengths) {
        Some(v) => v,
        None => {
            eprintln!("Failed to create sparse hash");
            return ExitCode::FAILURE;
        }
    };
    let hash = sparse_hash.get_hash().to_hex();

    println!(
        "
=== Part 2 ===
Hash: {hash}"
    );

    ExitCode::SUCCESS
}
