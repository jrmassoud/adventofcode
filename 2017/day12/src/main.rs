mod day12;

use std::{env, fs::File, io::BufReader};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let Some(input_path) = args.next() else {
        eprintln!("Usage: {program_name} <input file>");
        bail!("No input provided");
    };

    let input =
        File::open(&input_path).with_context(|| format!("Failed to open `{input_path}`"))?;
    let input = BufReader::new(input);

    let pipes =
        day12::read_pipes(input).with_context(|| format!("Failed to read `{input_path}`"))?;

    let n_reachable_from_zero = pipes
        .get_reachable_from(0)
        .into_iter()
        .filter(|&v| v)
        .count();

    println!(
        "\
=== Part 1 ===
Count: {n_reachable_from_zero}"
    );

    let n_groups = pipes.count_groups();

    println!(
        "
=== Part 2 ===
Count: {n_groups}"
    );

    Ok(())
}
