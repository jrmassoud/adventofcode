mod day15;

use std::{
    env,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    process::ExitCode,
};

use derive_more::{Display, Error};

use crate::day15::ReadInputError;

#[derive(Debug, Display, Error)]
enum AppError {
    #[display("No input provided")]
    NoInput,
    #[display("Unexpected argument `{arg}`")]
    UnexpectedArgument { arg: String },
    #[display("Failed to open `{}`: {source}", file.display())]
    OpenFile { file: PathBuf, source: io::Error },
    #[display("Failed to read `{}`: {source}", file.display())]
    ReadInput {
        file: PathBuf,
        source: ReadInputError,
    },
}

fn try_main() -> Result<(), AppError> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());
    let Some(input_path) = args.next() else {
        eprintln!("Usage: {program_name} <input path>");
        return Err(AppError::NoInput);
    };
    if let Some(unexpected) = args.next() {
        return Err(AppError::UnexpectedArgument { arg: unexpected });
    }

    let input = File::open(&input_path).map_err(|e| AppError::OpenFile {
        file: (&input_path).into(),
        source: e,
    })?;
    let input = BufReader::new(input);

    let (gen_a, gen_b) = day15::read_input(input).map_err(|e| AppError::ReadInput {
        file: input_path.into(),
        source: e,
    })?;

    let count = day15::count_matches(gen_a.clone(), gen_b.clone());

    println!("=== Part 1 ===\nCount: {count}");

    let filtered_count = day15::count_matches_filtered(gen_a, gen_b);

    println!("\n=== Part 2 ===\nCount: {filtered_count}");

    Ok(())
}

fn main() -> ExitCode {
    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
