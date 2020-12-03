use anyhow::{bail, Error};
use std::collections::HashSet;

#[derive(Debug, structopt::StructOpt)]
struct Args {
  #[structopt(short = "f", long = "file")]
  file: std::path::PathBuf,
}

const YEAR: u32 = 2020;

#[paw::main]
fn main(args: Args) -> Result<(), Error> {
  let text = std::fs::read_to_string(args.file)?;
  let numbers = text
    .lines()
    .map(&str::parse)
    .collect::<Result<HashSet<u32>, _>>()?;
  let entry = numbers
    .iter()
    .find(|&x| numbers.contains(&(YEAR - x)))
    .map(|x| *x);

  if entry.is_none() {
    bail!("Couldn't find two entries that add up to {}", YEAR);
  }

  match entry {
    None => bail!("Couldn't find two entries that add up to {}", YEAR),
    Some(x) => {
      let answer = x * (YEAR - x);
      println!("Answer: {}", answer);
      Ok(())
    }
  }
}
