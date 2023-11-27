//! Andres Alam Sanchez Torres 492253
//! 
//! Receives a .gph file and computes the depth starting from a given node 
//! and the number of components of the graph
//! 
//! Complexity:
//! O(|V| + |E|) for BFS depth: 
//!     The BFS  queue iterates through (at most) all vertices, and within each iteration
//!     performs constant operations and iterates through all edges in the current vertex,
//!     i. e. an iteration takes O(1) + O(|E_i|), and the BFS takes 
//!     V * (O(1) + O(E_i)) => O(|V|) + V * O(E_i) = O(|V|) + O(|E|)
//! 
//! O(|V| + |E|) for component search:
//!     Even though he algorithm performs |V| bfs searches, it only visits each vertex once,
//!     and since it searches over connected components, it does not have to look over edges
//!     connected to vertices visited in previous iterations.
//! 
//! Overall O(|V| + |E|)
use std::{
    cmp,
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Clone)]
struct Edge(usize, i64); // (adjacent node, weight)

struct Graph {
    adjacencies: Vec<Vec<Edge>>,
    vertices: usize,
}

impl Graph {
    fn get_values_from_line(line: String) -> Vec<i64> {
        line.trim()
            .split(" ")
            .map(|s| s.parse::<i64>().expect("Can't parse values from graph"))
            .collect::<Vec<_>>()
    }

    fn read_from_file(file_path: &str) -> Self{
        let graph_file =
            File::open(file_path).expect(format!("File {} does not exist", file_path).as_str());
        let mut reader: BufReader<File> = BufReader::new(graph_file);

        let mut graph_shape_line = String::new();
        reader
            .read_line(&mut graph_shape_line)
            .expect("Can't read graph shape");

        let graph_shape = Self::get_values_from_line(graph_shape_line);
        let (vertex_count, _) = match graph_shape.as_slice() {
            [vertex_count, edge_count] => (*vertex_count as usize, *edge_count as usize),
            _ => panic!("Can't parse graph size"),
        };

        let mut adjacencies = vec![vec![]; vertex_count];

        for line in reader.lines() {
            let values = Self::get_values_from_line(line.unwrap());
            let (node_0, node_1, weight) = match values.as_slice() {
                [node_0, node_1, weight] => (*node_0 as usize - 1, *node_1 as usize - 1, *weight),
                _ => panic!("Can't parse graph edge"),
            };

            adjacencies[node_0].push(Edge(node_1, weight));
            adjacencies[node_1].push(Edge(node_0, weight));
        }

        Graph {
            adjacencies,
            vertices: vertex_count,
        }


    }

    fn bfs_depth(&self, start: usize) -> usize {
        let mut visited: Vec<bool> = vec![false; self.vertices];
        self.bfs_depth_managed(start, &mut visited)
    }

    fn bfs_depth_managed(&self, start: usize, visited: &mut [bool]) -> usize {
        // track (node, depth)
        let mut bfs_queue = VecDeque::from([(start, 0)]);
        visited[start] = true;

        let mut tree_depth: usize = 0;
        while !bfs_queue.is_empty() {
            let (current_node, current_depth) = bfs_queue.pop_front().unwrap();
            tree_depth = cmp::max(tree_depth, current_depth);

            for Edge(adj_node, _) in &self.adjacencies[current_node] {
                if !visited[*adj_node] {
                    bfs_queue.push_back((*adj_node, current_depth + 1));
                    visited[*adj_node] = true;
                }
            }
        }

        tree_depth
    }

    fn connected_components(&self) -> usize {
        let mut visited: Vec<bool> = vec![false; self.vertices];
        let mut connected_components = 0;

        for node in 0..self.vertices {
            let is_visited = visited[node];
            if !is_visited {
                self.bfs_depth_managed(node, &mut visited);
                connected_components += 1;
            }
        }

        connected_components
    }
}

fn main() {
    let start_time = Instant::now();

    let mut args = env::args();
    let graph_path = args
        .nth(1)
        .expect("Can't get graph path from args");
    let start_node: usize = args
        .next()
        .expect("Can't get start node from args")
        .parse()
        .expect("Can't parse start node value");

    let graph: Graph = Graph::read_from_file(&graph_path);

    let depth = graph.bfs_depth(start_node - 1);
    let components = graph.connected_components();

    let duration = start_time.elapsed();

    println!("Depth: {}", depth);
    println!("Components: {}", components);
    println!("Time: {:.2} s", duration.as_secs_f64());
}
