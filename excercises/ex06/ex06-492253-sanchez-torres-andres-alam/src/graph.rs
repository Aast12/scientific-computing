use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

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
    cost: i64,
}

pub struct Graph {
    adjacencies: Vec<Vec<Edge>>,
    vertices: usize,
}

impl Graph {
    fn get_values_from_line(line: String) -> Vec<i64> {
        line.trim()
            .split(' ')
            .map(|s| s.parse::<i64>().expect("Can't parse values from graph"))
            .collect::<Vec<_>>()
    }

    pub fn read_from_file(file_path: &str) -> Self {
        let graph_file =
            File::open(file_path).unwrap_or_else(|_| panic!("File {} does not exist", file_path));
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

            adjacencies[node_0].push(Edge {
                from: node_0,
                to: node_1,
                cost: weight,
            });
            adjacencies[node_1].push(Edge {
                from: node_1,
                to: node_0,
                cost: weight,
            });
        }

        Graph {
            adjacencies,
            vertices: vertex_count,
        }
    }

    pub fn bfs_depth(&self, start: usize) -> usize {
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

            for Edge {
                from: _,
                to: adj_node,
                cost: _,
            } in &self.adjacencies[current_node]
            {
                if !visited[*adj_node] {
                    bfs_queue.push_back((*adj_node, current_depth + 1));
                    visited[*adj_node] = true;
                }
            }
        }

        tree_depth
    }

    pub fn connected_components(&self) -> usize {
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

    fn rebuild_path(
        &self,
        source: usize,
        target: usize,
        distance: Vec<PathDistance>,
    ) -> Vec<usize> {
        let mut path = vec![];
        let mut current_node = target;

        while current_node != source {
            path.push(current_node);
            current_node = distance[current_node].source;
        }

        path.push(source);
        path.reverse();

        path
    }

    pub fn shortest_path(&self, source: usize, target: usize) -> Option<(i64, Vec<usize>)> {
        let mut nodes_q: BinaryHeap<SearchState> = BinaryHeap::new();
        let mut distance = vec![
            PathDistance {
                source,
                cost: i64::MAX
            };
            self.vertices
        ];

        distance[source] = PathDistance { source, cost: 0 };
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
            for edge in adj_edges {
                let new_cost = cost + edge.cost;

                if new_cost < distance[edge.to].cost {
                    nodes_q.push(SearchState {
                        node: edge.to,
                        cost: new_cost,
                    });
                    distance[edge.to] = PathDistance {
                        source: node,
                        cost: new_cost,
                    };
                }
            }
        }

        None
    }

    pub fn minimum_spanning_tree(&self) -> i64 {
        let mut parent: Vec<usize> = (0..self.vertices).collect();
        let mut edges = self.adjacencies.iter().flatten().collect::<Vec<_>>();

        edges.sort_by(|edge_a, edge_b| edge_a.cost.partial_cmp(&edge_b.cost).unwrap());

        let mut cost_sum: i64 = 0;

        for Edge {
            from,
            to,
            cost: next_cost,
        } in edges
        {
            if parent[*from] != parent[*to] {
                cost_sum += next_cost;

                let new_parent = parent[*from];
                let replaced_parent = parent[*to];
                for node_parent in parent.iter_mut() {
                    if *node_parent == replaced_parent {
                        *node_parent = new_parent;
                    }
                }
            }
        }

        cost_sum
    }
}
