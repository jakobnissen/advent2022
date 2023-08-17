// TODO: Review errors

mod days;

use std::fmt::Display;
use std::fs::{metadata, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;

#[derive(Debug, Clone)]
struct Days(Vec<u8>);

impl Days {
    fn new(it: impl IntoIterator<Item = u8>) -> Self {
        let mut v: Vec<u8> = it
            .into_iter()
            .map(|x| {
                if !(1..=25).contains(&x) {
                    println!("Day not in range 1-25: {}", x);
                    std::process::exit(1)
                } else {
                    x
                }
            })
            .collect();
        if v.is_empty() {
            println!("No days specified, exiting");
            std::process::exit(1);
        }
        v.sort_unstable();
        v.dedup();
        Days(v)
    }

    fn parse_from_args<T: AsRef<str>>(segments: &[T]) -> Self {
        if segments
            .iter()
            .any(|x| x.as_ref().to_owned().to_lowercase() == "all")
        {
            if segments.len() == 1 {
                Days::new(1..=25)
            } else {
                println!("If day \"all\" is specified, no other days can be specified.");
                std::process::exit(1);
            }
        } else {
            Days::new(segments.iter().map(|s| {
                s.as_ref().parse::<u8>().unwrap_or_else(|_| {
                    println!("Error: Cannot parse as day in range 1-25: {}", s.as_ref());
                    std::process::exit(1)
                })
            }))
        }
    }
}

fn print_day(directory: &Path, day: u8) {
    let now = Instant::now();
    let path = &directory.join(format!("day{:0>2}.txt", day));
    let result: Option<(Box<dyn Display>, Box<dyn Display>)> = match day {
        1 => get_printable(days::day01::solve, path),
        2 => get_printable(days::day02::solve, path),
        3 => get_printable(days::day03::solve, path),
        4..=25 => None,
        _ => unreachable!(),
    };
    let elapsed = now.elapsed();
    print!("Day {}", day);
    match result {
        None => println!(": Unimplemented!"),
        Some((part1, part2)) => {
            println!(
                " [{:.2?}]\n    Part 1: {}\n    Part 2: {}",
                elapsed, part1, part2
            )
        }
    }
    println!();
}

fn get_printable<F, A: 'static, B: 'static>(
    f: F,
    path: &Path,
) -> Option<(Box<dyn Display>, Box<dyn Display>)>
where
    F: Fn(&str) -> (A, B),
    A: Display,
    B: Display,
{
    let data = match read_to_string(path) {
        Err(e) => {
            println!(
                "Error when reading file {} to string: \"{}\"",
                path.display(),
                e
            );
            std::process::exit(1);
        }
        Ok(s) => s,
    };
    let (a, b) = f(&data);
    Some((Box::new(a), Box::new(b)))
}

fn download_inputs_if_missing(path: &Path, days: &Days) -> anyhow::Result<()> {
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
    for &day in days.0.iter() {
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
    let session = get_session();
    let cookie =
        reqwest::header::HeaderValue::from_str(format!("session={}", session).as_str()).unwrap();
    headers.insert("Cookie", cookie);
    Ok(Client::builder().default_headers(headers).build()?)
}

fn get_session() -> String {
    match std::env::var("ADVENTOFCODE_SESSION") {
        Ok(s) => {
            if s.len() != 128 || s.as_bytes().iter().any(|x| !x.is_ascii_hexdigit()) {
                println!("Environmental variable ADVENTOFCODE_SESSION must be 128 hex digits");
                std::process::exit(1);
            }
            s
        }
        Err(e) => {
            println!(
                "Error: Could not load environmental variable ADVENTOFCODE_SESSION: \"{}\"",
                e
            );
            std::process::exit(1);
        }
    }
}

fn download_input(client: &Client, day: u8) -> anyhow::Result<String> {
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
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Download {
        /// Directory to download input data to
        #[arg(short, default_value=PathBuf::from_str("data").unwrap().into_os_string())]
        path: PathBuf,

        /// Days to download.
        #[arg(default_values=vec!["all".to_string()])]
        days_strings: Vec<String>,
    },
    Solve {
        /// Directory to load input data from
        #[arg(short, default_value=PathBuf::from_str("data").unwrap().into_os_string())]
        path: PathBuf,

        /// Days to solve.
        #[arg(default_values=vec!["all".to_string()])]
        days_strings: Vec<String>,
    },
}

fn main() {
    let args = Options::parse();
    match args.command {
        Command::Download { path, days_strings } => {
            let days = Days::parse_from_args(&days_strings);
            download_inputs_if_missing(&path, &days).unwrap()
        }
        Command::Solve { path, days_strings } => {
            let days = Days::parse_from_args(&days_strings);
            for day in days.0.iter() {
                print_day(&path, *day)
            }
        }
    }
}
