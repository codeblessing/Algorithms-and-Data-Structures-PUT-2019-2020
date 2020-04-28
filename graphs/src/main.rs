// #![warn(missing_docs)]
use std::{fs, path::PathBuf, collections::HashSet};
mod dfs;
mod graph;
use dfs::*;
mod del;
use clap::{App, Arg};
use del::*;

#[allow(missing_docs)]
fn main() {
    let matches = App::new("Graph topological sort")
        .version("0.9.9-20200427")
        .author("Jakub Kwiatkowski <jakubkw99@gmail.com>")
        .about("Simple app which sorts graph topologically.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("FILE")
                .help("input file path"),
        )
        .get_matches();

    let mut graph: Graph;

    let mut file: PathBuf = PathBuf::new();

    if let Some(value) = matches.value_of("file") {
        if PathBuf::from(value).exists() {
            file = PathBuf::from(value);
        }
    }

    if file.as_os_str().is_empty() {
        let mut line = String::new();
        let mut arc_count: usize;
        loop {
            std::io::stdin().read_line(&mut line).unwrap_or_else(|_| {
                eprintln!("Error! Cannot read input from console.");
                0
            });

            let mut va: Vec<_> = line.split_whitespace().map(|val| val.parse::<usize>()).collect();
            if va.len() != 2 {
                eprintln!("Wrong number of arguments! Try again.");
                continue;
            }

            if va.iter().any(|val| val.is_err()){
                eprintln!("Cannot parse values. Try again.");
                continue;
            }

            arc_count = va.pop().unwrap().unwrap();
            break;

        }

        for _ in 0..arc_count {
            std::io::stdin().read_line(&mut line).unwrap_or_else(|_| {
                eprintln!("Error! Cannot read input from console.");
                0
            });

            // arcs = format!("{}\n{}", arcs, line);
        }

        graph = Graph::from(line.as_str());
    }
    else {
        graph = Graph::from(file);
    }

    eprintln!(
        "Graph:\n\tVertex count: {}\n\tArc count: {}\n\tArcs:\n{:?}",
        graph.vertex_count(),
        graph.arc_count(),
        graph.arcs()
    );

    let successors_list = graph::successors::List::from(&graph.arcs());
    let adjacency_matrix = graph::adjacency::Matrix::from(&graph.arcs());
    let graph_matrix = graph::Matrix::from(&graph.arcs(), graph.vertex_count());

    let dfs_list = dfs::successors_list::sort(&successors_list);
    let dfs_adjacency = dfs::adjacency::sort(&adjacency_matrix);
    let dfs_graph = dfs::graph::sort(&graph_matrix);
    
    let del_list = del::successors::sort(&successors_list);
    let del_adjacency = del::adjacency::sort(&adjacency_matrix);
    let del_graph = del::graph::sort(&graph_matrix);

    println!("DFS Successors List sorted: {:?}", dfs_list);
    println!("DFS Adjacency matrix sorted: {:?}", dfs_adjacency);
    println!("DFS Graph Matrix sorted: {:?}", dfs_graph);
    println!("DEL Successors List sorted: {:?}", del_list);
    println!("DEL Adjacency Matrix sorted: {:?}", del_adjacency);
    println!("DEL Graph Matrix sorted: {:?}", del_graph);
}

struct Graph {
    vertex_count: usize,
    arc_count: usize,
    arcs: Vec<(usize, usize)>,
}

impl From<std::path::PathBuf> for Graph {
    fn from(path: std::path::PathBuf) -> Self {
        if let Ok(result) = fs::read_to_string(path) {
            Graph::from(result.as_str())
        } else {
            Self {
                vertex_count: 0,
                arc_count: 0,
                arcs: vec![],
            }
        }
    }
}

impl From<&str> for Graph {
    fn from(data: &str) -> Self {
        let mut arc_count: usize = 0;
        let mut arcs: Vec<(usize, usize)> = Vec::new();

        eprintln!("File content:\n{}", data);
        for line in data.lines() {
            let mut entity: Vec<_> = line
                .split_whitespace()
                .map(|val| val.parse::<usize>())
                .collect();

            if entity.len() != 2 {
                eprintln!("{} is not correct arc. Skipping", line);
                continue;
            }

            if entity.iter().any(|node| node.is_err()) {
                eprintln!("Cannot create arc from {}. Skipping", line);
                continue;
            }
            arc_count += 1;
            arcs.push((entity.remove(0).unwrap(), entity.remove(0).unwrap()));
        }

        arcs.remove(0);
        arc_count -= 1;
        let vertex_count: usize = arcs
            .iter()
            .fold(HashSet::new(), |mut acc, val| {
                acc.insert(val.0);
                acc.insert(val.1);
                acc
            }).len();


        Self {
            vertex_count,
            arc_count,
            arcs,
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertex_count: 0,
            arc_count: 0,
            arcs: vec![],
        }
    }

    pub fn arcs(&self) -> Vec<(usize, usize)> {
        self.arcs.clone()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn arc_count(&self) -> usize {
        self.arc_count
    }
}
