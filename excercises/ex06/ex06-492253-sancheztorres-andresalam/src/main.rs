//! Andres Alam Sanchez Torres 492253
use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashSet, VecDeque},
    env::{self, current_dir},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Clone)]
struct Edge(usize, i64); // (adjacent node, weight)

#[derive(Clone, Copy, Eq, PartialEq)]
struct SearchState {
    node: usize,
    cost: i64,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct PathDistance {
    source: usize,
    cost: i64
}

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

    fn read_from_file(file_path: &str) -> Self {
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

    fn rebuild_path(&self, source: usize, target: usize, distance: Vec<PathDistance>) -> Vec<usize> {
        let mut path = vec![target];
        let mut current_node = target;

        while current_node != source {
            path.push(current_node);
            current_node = distance[current_node].source;
        }

        path.push(source);

        path
    }

    fn shortest_path(&self, source: usize, target: usize) -> Option<(i64, Vec<usize>)> {
        let mut nodes_q: BinaryHeap<SearchState> = BinaryHeap::new();
        let mut distance = vec![PathDistance { source , cost: i64::MAX }; self.vertices];

        distance[source] = PathDistance {source, cost: 0};
        nodes_q.push(SearchState {
            node: source,
            cost: 0,
        });

        while let Some(SearchState { node, cost }) = nodes_q.pop() {
            if node == target {
                return Some((cost, self.rebuild_path(source, target, distance)));
            }

            if cost > distance[node].cost {
                continue;
            }

            let adj_edges = &self.adjacencies[node];
            for Edge(adj_node, adj_cost) in adj_edges {
                let new_cost = cost + adj_cost;

                if new_cost < distance[node].cost {
                    nodes_q.push(SearchState {
                        node: *adj_node,
                        cost: new_cost,
                    });
                    distance[node] = PathDistance {source: *adj_node, cost: new_cost};
                }
            }
        }

        return None;
    }

    fn minimum_spanning_tree(&self) -> i64 {
        let mut deduplicated_edges = self
            .adjacencies
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter_map(|Edge(adj, cost)| {
                        if *adj > i {
                            Some((i.clone(), adj.clone(), cost))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        deduplicated_edges
            .sort_by(|(_, _, cost_a), (_, _, cost_b)| cost_a.partial_cmp(cost_b).unwrap());

        let mut connected_nodes: HashSet<usize> = HashSet::new();
        let mut cost_sum = 0;

        for (from, to, next_cost) in deduplicated_edges {
            if connected_nodes.contains(&from) && connected_nodes.contains(&to) {
               continue;
            }

            connected_nodes.insert(from);
            connected_nodes.insert(to);
            cost_sum += next_cost;
        }

        cost_sum
    }
}

fn main() {
    let start_time = Instant::now();

    let mut args = env::args();
    let graph_path = args.nth(1).expect("Can't get graph path from args");
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
