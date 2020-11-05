use anyhow::{bail, Error};

#[derive(Debug, structopt::StructOpt)]
struct Args {
  #[structopt(short = "f", long = "file")]
  file: std::path::PathBuf,
}

#[derive(Debug)]
struct Day {
  day_number: u32,
  max_temperature: u32,
  min_temperature: u32,
}

impl Day {
  fn parse(line: &str) -> Option<Self> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let day_number = parts.get(0).and_then(|&s| s.parse::<u32>().ok());
    let max_temperature = parts.get(1).and_then(|&s| s.parse::<u32>().ok());
    let min_temperature = parts.get(2).and_then(|&s| s.parse::<u32>().ok());
    match (day_number, max_temperature, min_temperature) {
      (Some(day_number), Some(max_temperature), Some(min_temperature)) => Some(Self {
        day_number,
        max_temperature,
        min_temperature,
      }),
      _ => None,
    }
  }

  fn temperature_spread(&self) -> u32 {
    self.max_temperature - self.min_temperature
  }
}

#[paw::main]
fn main(args: Args) -> Result<(), Error> {
  let text = std::fs::read_to_string(args.file)?;
  let day_with_smallest_spread = text
    .lines()
    .filter_map(Day::parse)
    .min_by_key(Day::temperature_spread);
  match day_with_smallest_spread {
    Some(day) => println!(
      "Day {} has the smallest temperature spread of {}.",
      day.day_number,
      day.temperature_spread(),
    ),
    None => bail!("No valid lines"),
  }
  Ok(())
}
