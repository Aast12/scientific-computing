use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, VecDeque},
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    ops,
    str::FromStr,
};

pub trait NodeNo:
    ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Rem<Output = Self>
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + FromStr
    + Eq
    + fmt::Display // + Into<usize>
// + From<usize>
{
    fn max_value() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn to_index(&self) -> usize;
}

pub trait Numeric:
    PartialEq
    + ops::Add<Output = Self>
    + ops::Div<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Rem<Output = Self>
    + ops::AddAssign
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + fmt::Display
    + FromStr
{
    fn max_value() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_numeric {
    ($dtype: ident) => {
        impl Numeric for $dtype {
            fn max_value() -> Self {
                $dtype::MAX
            }
            fn zero() -> Self {
                0 as $dtype
            }
            fn one() -> Self {
                1 as $dtype
            }
        }
    };
}

macro_rules! impl_nodeno {
    ($dtype: ident) => {
        impl NodeNo for $dtype {
            fn max_value() -> Self {
                $dtype::MAX
            }
            fn zero() -> Self {
                0 as $dtype
            }
            fn one() -> Self {
                1 as $dtype
            }
            fn to_index(&self) -> usize {
                usize::try_from(*self).unwrap()
            }
        }
    };
}

impl_nodeno!(u8);
impl_nodeno!(u16);
impl_nodeno!(u32);
impl_nodeno!(u64);
impl_nodeno!(usize);
impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u128);
impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i128);
impl_numeric!(f32);
impl_numeric!(f64);

#[derive(Clone)]
struct Edge<T: NodeNo, W: Numeric> {
    from: T,
    to: T,
    cost: W,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct SearchState<T: NodeNo, W: Numeric> {
    node: T,
    cost: W,
}

// impl<W: Numeric> Ord for SearchState<W> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.partial_cmp(&self.cost)
//     }
// }

// impl<W: Numeric> PartialEq for SearchState<W> {
//     fn eq(&self, other: &Self) -> bool {
//         // self.cost.partial_cmp(other.cost)
//         self.cost != other.cost
//     }
// }

impl<T: NodeNo, W: Numeric> Eq for SearchState<T, W> {}

// impl<W: Numeric> PartialOrd for SearchState<W> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cost.partial_cmp(&other.cost).unwrap_or(Ordering::Equal))
//     }
// }

impl<T: NodeNo, W: Numeric> Ord for SearchState<T, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

// impl<W: Numeric> PartialEq for SearchState<W> {

// impl<W: Numeric> PartialOrd for SearchState<W> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // self.partial_cmp(other.cost)
//         Some(self.cmp(other))
//         // Some(self.cost.partial_cmp(&other.cost).unwrap())
//     }
// }

#[derive(Clone, Copy)]
struct PathDistance<T: NodeNo, W: Numeric> {
    source: T,
    cost: W,
}

pub struct Graph<T: NodeNo, W: Numeric> {
    adjacencies: Vec<Vec<Edge<T, W>>>,
    vertices: usize,
}

impl<T: NodeNo, W: Numeric> Graph<T, W> {
    fn get_values_from_line(line: String) -> Vec<usize> {
        line.trim()
            .split(' ')
            .map(|s| s.parse::<usize>().ok().unwrap())
            // .expect("Can't parse values from graph"))
            .collect::<Vec<_>>()
    }

    fn get_graph_entry(line: String) -> (T, T, W) {
        let mut values = line.trim().split(' ');

        let node_0 = values
            .next()
            .expect("Cant read source node from edge line")
            .parse::<T>()
            .ok()
            .unwrap();
        // .expect("Cant parse source node from edge line");
        let node_1 = values
            .next()
            .expect("Cant read target node from edge line")
            .parse::<T>()
            .ok()
            .unwrap();
        // .expect("Cant parse source node from edge line");
        let weight = values
            .next()
            .expect("Cant read weight from edge line")
            .parse::<W>()
            .ok()
            .unwrap();

        (node_0 - T::one(), node_1 - T::one(), weight)
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
            [vertex_count, edge_count] => (*vertex_count, *edge_count),
            _ => panic!("Can't parse graph size"),
        };

        let mut adjacencies = vec![vec![]; vertex_count];

        for line in reader.lines() {
            let (node_0, node_1, weight) = Self::get_graph_entry(line.unwrap());

            adjacencies[node_0.to_index()].push(Edge {
                from: node_0,
                to: node_1,
                cost: weight,
            });
            adjacencies[node_1.to_index()].push(Edge {
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
        let mut visited: Vec<bool> = vec![false; self.vertices.into()];
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
                if !visited[adj_node.to_index()] {
                    bfs_queue.push_back((adj_node.to_index(), current_depth + 1));
                    visited[adj_node.to_index()] = true;
                }
            }
        }

        tree_depth
    }

    pub fn connected_components(&self) -> usize {
        let mut visited: Vec<bool> = vec![false; self.vertices.into()];
        let mut connected_components = 0;

        for node in 0..self.vertices.into() {
            let is_visited = visited[node];
            if !is_visited {
                self.bfs_depth_managed(node, &mut visited);
                connected_components += 1;
            }
        }

        connected_components
    }

    fn rebuild_path(&self, source: T, target: T, distance: Vec<PathDistance<T, W>>) -> Vec<T> {
        let mut path = vec![];
        let mut current_node = target;

        while current_node != source {
            path.push(current_node);
            current_node = distance[current_node.to_index()].source;
        }

        path.push(source);
        path.reverse();

        path
    }

    pub fn shortest_path(&self, source: T, target: T) -> Option<(W, Vec<T>)> {
        let mut nodes_q: BinaryHeap<SearchState<T, W>> = BinaryHeap::new();
        let mut distance = vec![
            PathDistance {
                source,
                cost: W::max_value()
            };
            self.vertices.into()
        ];

        distance[source.to_index()] = PathDistance {
            source,
            cost: W::zero(),
        };
        nodes_q.push(SearchState {
            node: source,
            cost: W::zero(),
        });

        while let Some(SearchState { node, cost }) = nodes_q.pop() {
            if node == target {
                return Some((cost, self.rebuild_path(source, target, distance)));
            }

            if cost > distance[node.to_index()].cost {
                continue;
            }

            let adj_edges = &self.adjacencies[node.to_index()];
            for edge in adj_edges {
                let new_cost = cost + edge.cost;

                if new_cost < distance[edge.to.to_index()].cost {
                    nodes_q.push(SearchState {
                        node: edge.to,
                        cost: new_cost,
                    });
                    distance[edge.to.to_index()] = PathDistance {
                        source: node,
                        cost: new_cost,
                    };
                }
            }
        }

        None
    }

    pub fn minimum_spanning_tree(&self) -> W {
        let mut parent: Vec<usize> = (0..self.vertices).collect();
        let mut edges = self.adjacencies.iter().flatten().collect::<Vec<_>>();

        edges.sort_by(|edge_a, edge_b| edge_a.cost.partial_cmp(&edge_b.cost).unwrap());

        let mut cost_sum: W = W::zero();

        for Edge {
            from,
            to,
            cost: next_cost,
        } in edges
        {
            if parent[(*from).to_index()] != parent[(*to).to_index()] {
                cost_sum += *next_cost;

                let new_parent = parent[(*from).to_index()];
                let replaced_parent = parent[(*to).to_index()];
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
