use clap::{App, Arg};
use rand::Rng;
use std::fs::*;
use std::io::*;
use std::path::Path;
use std::time::Instant;
use ToString;

mod log;
use log::Log;

mod errors;
mod heap;
mod insertion;
mod merge;
mod quick_sort;
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
        let content: Vec<String> = content.lines().map(ToString::to_string).collect();
    }
    for _ in 0..args.size {
        unordered.push(rand::thread_rng().gen_range(-100000, 100000));
    }

    fn compare(first: &i32, second: &i32) -> i32 {
        second - first
    };

    let mut start_time = Instant::now();
    let is_ordered = insertion::sort(&unordered, &compare);
    let is_time = Instant::now().duration_since(start_time);

    // let mut knuth_index = ((((2 * unordered.len()) / 3) + 1) as f32).log(3f32) as u32;

    // let ks_start_time = Instant::now();
    // let ks_ordered = shell::sort(&unordered, &mut knuth_index, &compare);
    // let ks_time = Instant::now().duration_since(ks_start_time);

    // let cs_start_time = Instant::now();
    // let cs_ordered = knuth_shellsort::classic_shellsort(&unordered, &compare);
    // let cs_time = Instant::now().duration_since(cs_start_time);

    start_time = Instant::now();
    let hs_ordered = heap::sort(&unordered, heap::HeapType::MAX);
    let hs_time = Instant::now().duration_since(start_time);

    start_time = Instant::now();
    let qs_ordered = quick_sort::sort(&unordered, &compare);
    let qs_time = Instant::now().duration_since(start_time);

    log.log(format!("[Unordered]\n{:?}\n\n", unordered).as_str());
    log.log(
        format!(
            r"[Insertion Sort]
Ordered:
{:?}

Comparisons: {}
Swaps: {}
Time: {}ns
",
            is_ordered.0,
            is_ordered.1,
            is_ordered.2,
            is_time.as_nanos()
        )
        .as_str(),
    );
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
