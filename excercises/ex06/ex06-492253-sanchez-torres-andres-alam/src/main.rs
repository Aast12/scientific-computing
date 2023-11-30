//! Andres Alam Sanchez Torres 492253
use std::{
    env::{self},
    path::Path,
    time::Instant,
};

mod graph;

use graph::Graph;

fn main() {
    let start_time = Instant::now();

    let mut args = env::args();
    let graph_path = args.nth(1).expect("Can't get graph path from args");
    let start_node: usize = args
        .next()
        .expect("Can't get start node from args")
        .parse()
        .expect("Can't parse start node value");
    let target_node: usize = args
        .next()
        .expect("Can't get target node from args")
        .parse()
        .expect("Can't parse target node value");

    let graph_id = Path::new(&graph_path)
        .file_stem()
        .expect("Graph path does not exist")
        .to_str()
        .unwrap();

    let graph: Graph = Graph::read_from_file(&graph_path);

    let mst = graph.minimum_spanning_tree();

    let (shortest_path_len, shortest_path) = graph
        .shortest_path(start_node - 1, target_node - 1)
        .expect("No shortest path exists between source and target node");
    let shortest_path_str = shortest_path
        .iter()
        .map(|node| (node + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let duration = start_time.elapsed().as_millis();

    println!(
        "{} MST= {} SP= {} Path: {} Time: {} ms",
        graph_id, mst, shortest_path_len, shortest_path_str, duration
    );
}
