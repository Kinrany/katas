use anyhow::{bail, Error};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Part {
  One,
  Two,
}

impl FromStr for Part {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "one" => Ok(Part::One),
      "two" => Ok(Part::Two),
      _ => bail!("Unknown part: {}", s),
    }
  }
}

#[derive(Debug, structopt::StructOpt)]
struct Args {
  #[structopt(short = "f", long = "file")]
  file: std::path::PathBuf,
  #[structopt(short = "p", long = "part", default_value = "one")]
  part: Part,
}

const YEAR: u32 = 2020;

fn part1(numbers: Vec<u32>) -> Result<u32, Error> {
  let numbers = numbers.into_iter().collect::<HashSet<_>>();
  let entry = numbers
    .iter()
    .find(|&x| numbers.contains(&(YEAR - x)))
    .map(|x| *x);

  match entry {
    None => bail!("Couldn't find two entries that add up to {}", YEAR),
    Some(x) => Ok(x * (YEAR - x)),
  }
}

fn part2(numbers: Vec<u32>) -> Result<u32, Error> {
  let set = numbers.iter().map(|&x| x).collect::<HashSet<u32>>();

  for &x in numbers.iter() {
    for &y in numbers.iter() {
      if x + y > YEAR {
        continue;
      }
      let z = YEAR - x - y;
      if set.contains(&z) {
        return Ok(x * y * z);
      }
    }
  }

  bail!("Couldn't find three entries that add up to {}", YEAR);
}

#[paw::main]
fn main(args: Args) -> Result<(), Error> {
  let text = std::fs::read_to_string(args.file)?;
  let numbers = text
    .lines()
    .map(&str::parse)
    .collect::<Result<Vec<u32>, _>>()?;

  let result = match args.part {
    Part::One => part1(numbers),
    Part::Two => part2(numbers),
  };

  match result {
    Ok(answer) => {
      println!("Answer: {}", answer);
    }
    Err(error) => {
      println!("Error: {}", error);
    }
  }

  Ok(())
}
