//! Andres Alam Sanchez Torres 492253
use std::{
    env,
    path::Path,
    time::Instant,
};

mod graph;

use graph::{Graph, NodeNo, Numeric};

fn test_cholesky<T: NodeNo, W: Numeric>(graph_path: &String, start_node: &T, target_node: &T) {
    let start_time = Instant::now();

    let graph_id = Path::new(&graph_path)
        .file_stem()
        .expect("Graph path does not exist")
        .to_str()
        .unwrap();

    let graph: Graph<T, W> = Graph::read_from_file(&graph_path);

    let mst = graph.minimum_spanning_tree();

    let (shortest_path_len, shortest_path) = graph
        .shortest_path(*start_node, *target_node)
        .expect("No shortest path exists between source and target node");
    let shortest_path_str = shortest_path
        .iter()
        .map(|node| (*node + T::one()).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let duration = start_time.elapsed().as_micros();

    println!(
        "{} MST= {} SP= {} Path: {} Time: {} ms",
        graph_id, mst, shortest_path_len, shortest_path_str, duration
    );
}

fn main() {
    let mut args = env::args();
    let graph_path = args.nth(1).expect("Can't get graph path from args");
    let start_node: usize = args
        .next()
        .expect("Can't get start node from args")
        .parse()
        .expect("Can't parse start node value");
    let start_node = start_node - 1;
    let target_node: usize = args
        .next()
        .expect("Can't get target node from args")
        .parse()
        .expect("Can't parse target node value");
    let target_node = target_node - 1;

    test_cholesky::<u16, u32>(&graph_path, &(start_node as u16), &(target_node as u16));
    test_cholesky::<u32, u32>(&graph_path, &(start_node as u32), &(target_node as u32));
    test_cholesky::<u64, u32>(&graph_path, &(start_node as u64), &(target_node as u64));

    test_cholesky::<u16, f64>(&graph_path, &(start_node as u16), &(target_node as u16));
    test_cholesky::<u32, f64>(&graph_path, &(start_node as u32), &(target_node as u32));
    test_cholesky::<u64, f64>(&graph_path, &(start_node as u64), &(target_node as u64));
}
