mod bruteforce;
mod dynamically;
mod greedily;
mod object;

use clap::{App, Arg, ArgMatches};
use failure::{Error, Fail};
use lazy_static::lazy_static;
use object::{Knapsack, Object};
use regex::Regex;
use std::fs;

fn main() {
    let matches = setup_clap();

    let thread_count = matches
        .value_of("threads")
        .unwrap_or("4")
        .parse::<usize>()
        .unwrap_or(4);
    let input_file = matches.value_of("file").unwrap_or("");
    let output_file = matches.value_of("output").unwrap_or("");

    let knapsack: Knapsack;
    let objects: Vec<Object>;

    if let Ok((ruck, objs)) = read_from_file(input_file) {
        knapsack = ruck;
        objects = objs;
    } else {
        let (ruck, objs) = read_from_console();
        knapsack = ruck;
        objects = objs;
    }

    let brutally_packed_rucksack = bruteforce::pack_a_ruck(knapsack, &objects, thread_count as u8);
    let greedily_packed_rucksack = greedily::pack_a_ruck(knapsack, &objects);
    let dynamically_packed_rucksack = dynamically::pack_a_ruck(knapsack, &objects);

    println!("Plecak spakowany na siłę:");
    for object in brutally_packed_rucksack {
        println!("{}", object);
    }
    println!("Plecak spakowany zachłannie:");
    for object in greedily_packed_rucksack {
        println!("{}", object);
    }
    println!("Plecak spakowany dynamicznie:");
    for object in dynamically_packed_rucksack {
        println!("{}", object);
    }
}

fn setup_clap<'a>() -> ArgMatches<'a> {
    App::new("Pack a Ruck")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .value_name("THREAD_COUNT")
                .help("Number of threads for running bruteforce algorithm."),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("INPUT")
                .help("Path to file with input data."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("OUTPUT")
                .help("Path to file for output data."),
        )
        .get_matches()
}

fn read_from_file(filename: &str) -> Result<(Knapsack, Vec<Object>), Error> {
    let input = fs::read_to_string(filename)?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*\d+\s+(?P<cap>\d+)\s*$").unwrap();
    }

    let mut knapsack = Knapsack { capacity: 0 };
    let mut objects: Vec<Object> = Vec::new();

    let first_line = input
        .lines()
        .take(1)
        .next()
        .ok_or(InputError::TooFewLines)?;
    let capture = RE.captures(first_line).ok_or(InputError::ParseFailed)?;
    let capacity: usize = capture["cap"].parse()?;
    knapsack.capacity = capacity;

    let object_lines = input.lines().skip(1);

    for line in object_lines {
        if let Ok(object) = line.parse::<Object>() {
            objects.push(object);
        } else {
            eprintln!(
                "Nieprawidłowe dane obiektu: {}. Linia zostanie pominięta.",
                line
            );
        }
    }

    Ok((knapsack, objects))
}

fn read_from_console() -> (Knapsack, Vec<Object>) {
    let mut knapsack = Knapsack { capacity: 0 };
    let mut objects: Vec<Object> = Vec::new();
    let mut object_count: usize;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*(?P<count>\d+)\s+(?P<capacity>\d+)\s*$").unwrap();
    }

    let mut line: String = String::new();
    loop {
        line.clear();
        let state = std::io::stdin().read_line(&mut line).ok();
        if state.is_some() {
            if let Some(capture) = RE.captures(&line) {
                if let Ok(count) = capture["count"].parse() {
                    object_count = count;
                } else {
                    eprintln!("Nieprawidłowe dane dla liczby przedmiotów.");
                    continue;
                }
                if let Ok(capacity) = capture["capacity"].parse() {
                    knapsack.capacity = capacity;
                } else {
                    eprintln!("Nieprawidłowe dane dla pojemności plecaka.");
                    continue;
                }
                line.clear();
                break;
            } else {
                eprintln!("Nieprawidłowe dane wejściowe.");
                continue;
            }
        } else {
            eprintln!("Nie można wczytać danych, spróbuj ponownie.");
            continue;
        }
    }

    println!("Podaj dane dla {} przedmiotów.", object_count);

    while object_count > 0 {
        line.clear();
        let state = std::io::stdin().read_line(&mut line).ok();
        if state.is_some() {
            if let Ok(object) = line.parse() {
                objects.push(object);
                object_count -= 1;
                line.clear();
            }
            else {
                eprintln!("Nieprawidłowe parametry dla przedmiotu: {}", line);
                eprintln!("Format danych: [id] [nazwa] waga wartość");
                continue;
            }
        }
        else {
            eprintln!("Nie można wczytać danych, spróbuj ponownie.");
            continue;
        }
    }

    (knapsack, objects)
}

#[derive(Debug, Fail)]
enum InputError {
    #[fail(display = "Plik zawiera zbyt mało linijek.")]
    TooFewLines,
    #[fail(display = "Nie można sparsować linii.")]
    ParseFailed,
}
