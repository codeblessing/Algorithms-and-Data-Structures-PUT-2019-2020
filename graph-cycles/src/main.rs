mod euler;
mod graph;
mod hamilton;
mod test;

use graph::{AdjacencyMatrix, SuccessorsList};

use clap::{App, Arg, ArgMatches};
use log::{error, info, warn};
use std::fs;

fn main() {
    setup_logger().unwrap();
    let args = setup_clap();

    info!("Setup finished.");
    let mut arcs: Vec<(usize, usize)> = Vec::new();

    if args.is_present("file") {
        if let Ok(content) = fs::read_to_string(args.value_of("file").unwrap()) {
            info!("{:?}", content);
            arcs = parse_input(content);
        } else {
            warn!(
                "Nie można otworzyć pliku {}",
                args.value_of("file").unwrap()
            );
        }
    }

    if arcs.capacity() == 0 {
        let mut line = String::new();
        let arc_count: usize;
        println!("Podaj listę łuków:");
        loop {
            match std::io::stdin().read_line(&mut line).ok() {
                Some(_) => {
                    match line
                        .split_whitespace()
                        .skip(1)
                        .map(|count| count.parse::<usize>().ok())
                        .next()
                        .flatten()
                    {
                        Some(count) => {
                            arc_count = count;
                            break;
                        }
                        None => {
                            error!("Nie można wczytać liczby krawędzi, spróbuj jeszcze raz");
                            continue;
                        }
                    }
                }
                None => {
                    error!("Nie można wczytać liczby krawędzi, spróbuj jeszcze raz");
                    continue;
                }
            }
        }

        // line.clear();

        for _ in 0..arc_count {
            std::io::stdin().read_line(&mut line).unwrap_or(0);
        }

        arcs = parse_input(line);
    }

    let adjacency_matrix = AdjacencyMatrix::from(arcs.clone());
    let successors_list = SuccessorsList::from(arcs);

    // test!(
    //     10,
    //     "./out/ham_direct",
    //     hamilton::directed::hamilton_cycle,
    //     successors_list.clone()
    // );
    // test!(
    //     10,
    //     "./out/ham_undire",
    //     hamilton::undirected::hamilton_cycle,
    //     adjacency_matrix.clone()
    // );
    // test!(
    //     10,
    //     "./out/eul_direct",
    //     euler::directed::euler_cycle,
    //     successors_list.clone()
    // );
    // test!(
    //     10,
    //     "./out/eul_undire",
    //     euler::undirected::euler_cycle,
    //     adjacency_matrix.clone()
    // );

    let hamilton_directed = hamilton::directed::hamilton_cycle(successors_list.clone());
    let hamilton_undirected = hamilton::undirected::hamilton_cycle(adjacency_matrix.clone());
    let euler_directed = euler::directed::euler_cycle(successors_list.clone());
    let euler_undirected = euler::undirected::euler_cycle(adjacency_matrix.clone());

    match hamilton_directed {
        Some(cycle) => {
            println!("Znaleziono cykl Hamiltona (graf skierowany):");
            println!("{:?}", cycle);
        }
        None => println!("Nie znaleziono cyklu Hamiltona w grafie skierowanym"),
    }

    match hamilton_undirected {
        Some(cycle) => {
            println!("Znaleziono cykl Hamiltona (graf nieskierowany):");
            println!("{:?}", cycle);
        }
        None => println!("Nie znaleziono cyklu Hamiltona w grafie nieskierowanym"),
    }

    match euler_directed {
        Some(cycle) => {
            println!("Znaleziono następujący cykl Eulera (graf skierowany):");
            println!("{:?}", cycle);
        }
        None => println!("Nie znaleziono cyklu Eulera w grafie skierowanym"),
    }

    match euler_undirected {
        Some(cycle) => {
            println!("Znaleziono następujący cykl Eulera (graf nieskierowany):");
            println!("{:?}", cycle);
        }
        None => println!("Nie znaleziono cyklu Eulera w grafie nieskierowanym"),
    }
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        info!("{}", line);
        let mut nodes: Vec<_> = line
            .split_ascii_whitespace()
            .map(|node| node.parse::<usize>())
            .collect();

        if nodes.len() != 2 || nodes.iter().any(|node| node.is_err()) {
            error!("{} nie jest poprawnym łukiem i zostanie pominięty.", line);
        } else {
            out.push((nodes.remove(0).unwrap(), nodes.remove(0).unwrap()));
        }
    }

    info!("{:?}", out);

    out
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .chain(fern::log_file("./logfile.log")?)
        .apply()?;
    Ok(())
}

fn setup_clap<'a>() -> ArgMatches<'a> {
    App::new("Graph cycles")
        .version("2020.05.08")
        .name("Graph cycles")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .takes_value(true),
        )
        .get_matches()
}
