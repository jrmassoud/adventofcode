use std::{env, fs::File, io::BufReader};

use anyhow::{Context, Result, bail};

mod day11;

fn main() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let Some(input_path) = args.next() else {
        eprintln!("Usage: {program_name} <input file>");
        bail!("No input provided");
    };

    let input = File::open(&input_path)
        .with_context(|| format!("Failed to open `{input_path}` for reading"))?;
    let input = BufReader::new(input);

    let directions =
        day11::read_directions(input).with_context(|| format!("Failed to read `{input_path}`"))?;

    let (max_position, final_position) = day11::find_max_and_final(&directions);
    let max_dist = max_position.metric();
    let final_dist = final_position.metric();

    println!(
        "\
=== Part 1 ===
Distance: {final_dist}

=== Part 2 ===
Distance: {max_dist}"
    );

    Ok(())
}
