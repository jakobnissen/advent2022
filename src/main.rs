// TODO: Solve all / download all CLI option
// Todo: Parse directly into Day
// TODO: Don't hard code dir for data?
// TODO: Parse session is right: 128 hex chars
// TODO: Review errors

mod days;

use std::time::Instant;
use std::fmt::Display;
use std::fs::{metadata, read_to_string};
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;

#[derive(Clone, Copy)]
struct Day(usize);

impl TryFrom<usize> for Day {
    type Error = ();

    fn try_from(x: usize) -> Result<Self, Self::Error> {
        if x > 25 || x < 1 {
            Err(())
        } else {
            Ok(Day(x))
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Day {:0>2}", self.0)
    }
}

fn print_day(day: Day) {
    let now = Instant::now();
    let result: Option<(Box<dyn Display>, Box<dyn Display>)> = match day {
        Day(1) => get_printable(days::day01::solve, "data/day01.txt"),
        Day(2..=25) => None,
        _ => unreachable!()
    };
    let elapsed = now.elapsed();
    print!("{}", day);
    match result {
        None => println!(": Unimplemented!"),
        Some((part1, part2)) => {
            println!(" [{:.2?}]\n    Part 1: {}\n    Part 2: {}", elapsed, part1, part2)
        }
    }
}

fn get_printable<F, A: 'static, B: 'static>(
    f: F,
    s: &str,
) -> Option<(Box<dyn Display>, Box<dyn Display>)>
where
    F: Fn(&str) -> (A, B),
    A: Display,
    B: Display,
{
    let data = match read_to_string(s) {
        Err(e) => {
            println!("Error when reading file {} to string: \"{}\"", s, e);
            std::process::exit(1);
        },
        Ok(s) => s
    };
    let (a, b) = f(&data);
    Some((Box::new(a), Box::new(b)))
}

fn parse_days(days: &[usize]) -> Vec<Day> {
    let mut usize_days = days.to_vec();
    usize_days.sort_unstable();
    usize_days.dedup();
    usize_days.iter().map(|u| {
        match Day::try_from(*u) {
            Ok(day) => day,
            Err(_) => {
                println!("Cannot parse {} to day: Must be between 1 and 25.", u);
                std::process::exit(1);
            }
        }
    }).collect()
}

fn download_inputs_if_missing(path: &Path, days: &[usize]) -> anyhow::Result<()> {
    let mut client: Option<Client> = None;
    if !path.exists() {
        std::fs::create_dir(path)?
    } else {
        let md = metadata(path)?;
        if !md.is_dir() {
            return Err(anyhow!(
                "Path exists and is not a directory: {}",
                path.display()
            ));
        }
    }
    for &day in days {
        let daypath = path.join(format!("day{:0>2}.txt", day));
        if !daypath.exists() {
            println!("Downloading day {:0>2}...", day);
            if client.is_none() {
                client = Some(make_client()?)
            }
            let input = download_input(client.as_ref().unwrap(), day)?;
            std::fs::write(daypath, input)?;
        } else {
            println!("Input already exists: Day {:0>2}", day);
        }
    }
    Ok(())
}

fn make_client() -> anyhow::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::default();
    let session = match std::env::var("ADVENTOFCODE_SESSION") {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Error: Could not load environmental variable ADVENTOFCODE_SESSION: \"{}\"",
                e
            );
            std::process::exit(1);
        }
    };
    let cookie =
        reqwest::header::HeaderValue::from_str(format!("session={}", session).as_str()).unwrap();
    headers.insert("Cookie", cookie);
    Ok(Client::builder().default_headers(headers).build()?)
}

fn download_input(client: &Client, day: usize) -> anyhow::Result<String> {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let resp = client.get(url.as_str()).send()?;
    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server request was not successful: {}",
            resp.text()?
        ));
    }
    Ok(resp.text()?)
}

#[derive(Parser, Debug)]
struct Options {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Download {
        path: PathBuf,

        #[arg(required=true)]
        days: Vec<usize>
    },
    Solve {
        #[arg(required=true)]
        days: Vec<usize>
    }
}

fn main() {
    let args = Options::parse();
    match args.command {
        Command::Download{path, days} => {
            download_inputs_if_missing(&path, &days).unwrap()
        },
        Command::Solve{days: days_usize} => {
            let days = parse_days(&days_usize);
            for day in days.iter() {
                print_day(*day)
            }
        }
    }
}
