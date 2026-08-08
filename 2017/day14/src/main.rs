mod day14;
mod knot_hash;

use std::{env, process::ExitCode};

use crate::day14::Grid;

fn main() -> ExitCode {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let Some(key) = args.next() else {
        eprintln!("Usage: {program_name} <key>");
        return ExitCode::FAILURE;
    };

    let grid = Grid::new(&key);

    let square_count = grid.count_squares();

    println!(
        "\
=== Part 1 ===
Count: {square_count}"
    );

    let region_count = grid.count_regions();

    println!(
        "
=== Part 2 ===
Count: {region_count}"
    );

    ExitCode::SUCCESS
}
