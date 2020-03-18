use clap::{App, Arg};
use rand::Rng;
use std::fs::*;
use std::io::*;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};
use ToString;

mod log;
use log::Log;

mod errors;
mod heap;
mod insertion;
mod merge;
mod quick;
mod shell;

fn main() {
    let args = setup();

    let log = Log::new();
    log.open(Path::new(args.output.as_str()), args.append);

    let mut unordered: Vec<i32> = Vec::with_capacity(args.size as usize);

    if args.input != None && Path::new(args.input.clone().unwrap().as_str()).exists() {
        let mut content: String = String::new();
        let mut file = File::open(args.input.unwrap().as_str()).unwrap();
        file.read_to_string(&mut content).unwrap_or_else(|err| {
            eprintln!("Input error: {}", err);
            0
        });
        unordered = content
            .split_ascii_whitespace()
            .map(|val| i32::from_str(val).unwrap_or(std::i32::MIN))
            .filter(|&val| val != std::i32::MIN)
            .collect();
    } else if args.generate {
        for _ in 0..args.size {
            unordered.push(rand::thread_rng().gen_range(-100000, 100000));
        }
    } else {
        let mut content = String::new();
        println!("Podaj liczby (oddzielone spacją) i naciśnij enter:");
        stdin().read_to_string(&mut content).unwrap_or_else(|err| {
            eprintln!("Input error: {}", err);
            0
        });
        unordered = content
            .split_ascii_whitespace()
            .map(|val| i32::from_str(val).unwrap_or(std::i32::MIN))
            .filter(|&val| val != std::i32::MIN)
            .collect()
    }

    if unordered.len() < 1 {
        return;
    }

    fn compare(first: &i32, second: &i32) -> i32 {
        if *second as i64 > (std::i32::MIN + first) as i64 {
            second - first
        } else {
            -1
        }
    };

    let mut start_time = Instant::now();
    let is_ordered = insertion::sort(&unordered, &compare);
    let is_time = Instant::now().duration_since(start_time);

    let mut knuth_index = ((((2 * unordered.len()) / 3) + 1) as f32).log(3f32) as u32;

    let ks_start_time = Instant::now();
    let ks_ordered = shell::sort(&unordered, &mut knuth_index, &compare);
    let ks_time = Instant::now().duration_since(ks_start_time);

    start_time = Instant::now();
    let qs_ordered = quick::sort(&unordered, &compare);
    let qs_time = Instant::now().duration_since(start_time);

    start_time = Instant::now();
    let hs_ordered = heap::sort(&unordered, heap::HeapType::MAX);
    let hs_time = Instant::now().duration_since(start_time);

    start_time = Instant::now();
    let ms_ordered = merge::sort(&unordered, &compare);
    let ms_time = Instant::now().duration_since(start_time);

    log.log(
        format!(
            r"[Unordered]
{:?}


[Insertion Sort]
Ordered:
{:?}

Comparisons: {}
Swaps: {}
Time: {}


[Shell Sort]
Ordered:
{:?}

Deltas (in order): {:?}
Comparisons: {}
Swaps: {}
Time: {}


[Quick Sort]
Ordered:
{:?}

Pivots (in order): {:?}
Comparisons: {}
Swaps: {}
Time: {}


[Merge Sort]
Ordered:
{:?}

Comparisons: {}
Swaps: {}
Time: {}


[Heap Sort]
Ordered:
{:?}

Comparisons: {}
Swaps: {}
Time: {}",
            unordered,
            is_ordered.0,
            is_ordered.1,
            is_ordered.2,
            format_time(&is_time),
            ks_ordered.0,
            ks_ordered.3,
            ks_ordered.1,
            ks_ordered.2,
            format_time(&ks_time),
            qs_ordered.0,
            qs_ordered.3,
            qs_ordered.1,
            qs_ordered.2,
            format_time(&qs_time),
            ms_ordered.0,
            ms_ordered.1,
            ms_ordered.2,
            format_time(&ms_time),
            hs_ordered.0,
            hs_ordered.1,
            hs_ordered.2,
            format_time(&hs_time),
        )
        .as_str(),
    );
}

fn format_time(time: &Duration) -> String {
    let nanos = time.as_nanos();
    let millis = nanos / 1_000_000;
    let seconds = millis / 1_000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    if millis == 0 {
        format!("{}ns", nanos)
    } else if seconds == 0 {
        format!("{}ms {}ns", millis, nanos - (millis * 1_000_000))
    } else if minutes == 0 {
        format!(
            "{}s {}ms {}ns",
            seconds,
            millis - (seconds * 1_000),
            nanos - (millis * 1_000_000)
        )
    } else if hours == 0 {
        format!(
            "{}m {}s {}ms {}ns",
            minutes,
            seconds - (minutes * 60),
            millis - (seconds * 1_000),
            nanos - (millis * 1_000_000)
        )
    } else {
        format!(
            "{}h {}m {}s {}ms {}ns",
            hours,
            minutes - (hours * 60),
            seconds - (minutes * 60),
            millis - (seconds * 1_000),
            nanos - (millis * 1_000_000)
        )
    }
}

fn setup() -> Config {
    let args = App::new("Sorting")
        .version("2020.03.13")
        .author("Jakub Kwiatkowski <jakub.j.kwiatkowski@student.put.poznan.pl>")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .takes_value(true),
        )
        .arg(Arg::with_name("append").short("a").long("append"))
        .arg(Arg::with_name("generate").short("g").long("generate"))
        .get_matches();

    let mut config = Config::new();

    config.input = if args.is_present("input") {
        Some(args.value_of("input").unwrap().to_string())
    } else {
        None
    };
    config.output = args
        .value_of("output")
        .unwrap_or("./results.txt")
        .to_string();
    config.size = args
        .value_of("size")
        .unwrap_or("10_000")
        .parse::<u32>()
        .unwrap_or(10_000);
    config.append = args.is_present("append");
    config.generate = args.is_present("generate");

    config
}

struct Config {
    input: Option<String>,
    output: String,
    size: u32,
    append: bool,
    generate: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            input: None,
            output: String::from("./results.txt"),
            size: 10_000,
            append: false,
            generate: false,
        }
    }
}
