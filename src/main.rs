use clap::Parser;
use simple_error::bail;
use std::error::Error;
use std::fs;

mod year2024;

#[derive(Parser)]
struct Cli {
    input_file: String,

    #[arg(short, long)]
    year: String,

    #[arg(short, long)]
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let input = fs::read_to_string(cli.input_file)?;

    let func = match cli.year.as_str() {
        "2024" => year2024::DAYS.get(&cli.day),
        _ => bail!("Unknown year {}", cli.year),
    };

    match func {
        Some(f) => f(&input),
        None => bail!("Unknown day {}", cli.day),
    }

    Ok(())
}
