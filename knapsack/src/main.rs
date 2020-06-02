mod bruteforce;
mod dynamically;
mod greedily;
mod object;

use clap::{App, Arg, ArgMatches};
use failure::{Error, Fail};
use lazy_static::lazy_static;
use object::{Knapsack, Object};
use regex::Regex;
use std::{fs, io::Write, time::Instant};

fn main() {
    let matches = setup_clap();

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

    if !matches.is_present("test") {
        let brutally_packed_rucksack = bruteforce::pack_a_ruck(knapsack, &objects);
        let greedily_packed_rucksack = greedily::pack_a_ruck(knapsack, &objects);
        let dynamically_packed_rucksack = dynamically::pack_a_ruck(knapsack, &objects);

        let bf_weight: usize = brutally_packed_rucksack.iter().map(|obj| obj.weight).sum();
        let bf_value: usize = brutally_packed_rucksack.iter().map(|obj| obj.value).sum();
        let gr_weight: usize = greedily_packed_rucksack.iter().map(|obj| obj.weight).sum();
        let gr_value: usize = greedily_packed_rucksack.iter().map(|obj| obj.value).sum();
        let dy_weight: usize = dynamically_packed_rucksack.iter().map(|obj| obj.weight).sum();
        let dy_value: usize = dynamically_packed_rucksack.iter().map(|obj| obj.value).sum();

        println!("Plecak spakowany na siłę:");
        println!("Waga: {}, wartość: {}", bf_weight, bf_value);
        for object in brutally_packed_rucksack {
            println!("{}", object);
        }
        println!("\nPlecak spakowany zachłannie:");
        println!("Waga: {}, wartość: {}", gr_weight, gr_value);
        for object in greedily_packed_rucksack {
            println!("{}", object);
        }
        println!("\nPlecak spakowany dynamicznie:");
        println!("Waga: {}, wartość: {}", dy_weight, dy_value);
        for object in dynamically_packed_rucksack {
            println!("{}", object);
        }
    } else {
        let mut start_time: Instant;
        let mut bf_times: Vec<u128> = Vec::with_capacity(10);
        for _ in 0..10 {
            start_time = Instant::now();
            let _tmp = bruteforce::pack_a_ruck(knapsack, &objects);
            let end_time = Instant::now().duration_since(start_time).as_micros();
            bf_times.push(end_time);
        }
        let mut gr_times: Vec<u128> = Vec::with_capacity(10);
        for _ in 0..10 {
            start_time = Instant::now();
            let _tmp = greedily::pack_a_ruck(knapsack, &objects);
            let end_time = Instant::now().duration_since(start_time).as_micros();
            gr_times.push(end_time);
        }
        let mut dy_times: Vec<u128> = Vec::with_capacity(10);
        for _ in 0..10 {
            start_time = Instant::now();
            let _tmp = dynamically::pack_a_ruck(knapsack, &objects);
            let end_time = Instant::now().duration_since(start_time).as_micros();
            dy_times.push(end_time);
        }

        let mut bf_file = fs::OpenOptions::new().create(true).write(true).append(true).open(format!("{}_bf.csv", output_file)).expect("Cannot create bf file");
        let mut gr_file = fs::OpenOptions::new().create(true).write(true).append(true).open(format!("{}_gr.csv", output_file)).expect("Cannot create gr file");
        let mut dy_file = fs::OpenOptions::new().create(true).write(true).append(true).open(format!("{}_dy.csv", output_file)).expect("Cannot create dy file");
        
        eprintln!("All files created.");

        for time in bf_times {
            bf_file.write_all(format!("{}\t", time).as_bytes()).unwrap();
        }
        bf_file.write("\n".as_bytes()).unwrap();
        for time in gr_times {
            gr_file.write_all(format!("{}\t", time).as_bytes()).unwrap();
        }
        gr_file.write("\n".as_bytes()).unwrap();
        for time in dy_times {
            dy_file.write_all(format!("{}\t", time).as_bytes()).unwrap();
        }
        dy_file.write("\n".as_bytes()).unwrap();
    }
}

fn setup_clap<'a>() -> ArgMatches<'a> {
    App::new("Pack a Ruck")
        .arg(Arg::with_name("test").short("t").long("test"))
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
        static ref RE: Regex = Regex::new(r"^\s*(?P<count>\d+)\s+(?P<cap>\d+)\s*$").unwrap();
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
    let mut count: usize = capture["count"].parse()?;
    knapsack.capacity = capacity;

    let object_lines = input.lines().skip(1);

    for line in object_lines {
        if count <= 0 {
            println!("Plik zawiera więcej danych niż zadeklarowano. Kontynuować? [T/n]");
            let mut tmp = String::with_capacity(3);
            std::io::stdin().read_line(&mut tmp).unwrap_or(0);
            if &tmp.trim().to_lowercase() == "n" {
                break;
            } 
        }
        if let Ok(object) = line.parse::<Object>() {
            objects.push(object);
            count -= 1;
        } else {
            eprintln!(
                "Nieprawidłowe dane obiektu: {}. Linia zostanie pominięta.",
                line
            );
        }
    }
    
    if count > 0 {
        println!("Plik zawierał mniej danych niż zadeklarowano. Kontynuować? [T/n]");
        let mut tmp = String::with_capacity(3);
        std::io::stdin().read_line(&mut tmp).unwrap_or(0);
        if tmp.trim().to_lowercase() == "n" { Err(InputError::TooFewLines)? }
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
    println!("Podaj liczbę obiektów i pojemność plecaka.");
    loop {
        line.clear();
        let state = std::io::stdin().read_line(&mut line).ok();
        line = line.trim().to_owned();
        if state.is_some() && !line.is_empty() {
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
        println!("Podaj dane przedmiotu [pozostały {} przedmiotów]:", object_count);
        let state = std::io::stdin().read_line(&mut line).ok();
        line = line.trim().to_owned();
        if state.is_some() && !line.is_empty() {
            eprintln!("Wczytano: {}", line);
            if let Ok(object) = line.parse() {
                objects.push(object);
                object_count -= 1;
                line.clear();
            } else {
                eprintln!("Nieprawidłowe parametry dla przedmiotu.");
                eprintln!("Format danych: [id] [nazwa] waga wartość");
                continue;
            }
        } else {
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
