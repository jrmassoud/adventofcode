mod day16;

use std::{env, fs::File, io::BufReader};

use anyhow::{Context, Result, bail};

use crate::day16::State;

const PROGRAM_COUNT: usize = 16;

fn main() -> Result<()> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let print_usage = || eprintln!("Usage: {program_name} <input file>");
    let Some(input_path) = args.next() else {
        print_usage();
        bail!("No input provided");
    };
    if let Some(arg) = args.next() {
        print_usage();
        bail!("Unexpected argument `{arg}`");
    }

    let input = BufReader::new(
        File::open(&input_path).with_context(|| format!("Failed to open `{input_path}`"))?,
    );
    let instructions = day16::read_instructions(input)
        .with_context(|| format!("Failed to read `{input_path}`"))?;

    let state = State::new(instructions, PROGRAM_COUNT);

    let data = state.clone().run(1).context("Failed to run instructions")?;
    let data = str::from_utf8(&data).context("State had non-ascii program names")?;
    println!("=== Part 1 ===\nData: {data}");

    let data = state.run(1_000_000_000).context("Failed to run instructions")?;
    let data = str::from_utf8(&data).context("State had non-ascii program names")?;
    println!("\n=== Part 2 ===\nData: {data}");

    Ok(())
}
