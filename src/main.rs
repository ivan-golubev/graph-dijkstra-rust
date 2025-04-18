use std::vec::Vec;
use std::array;
use std::collections::BinaryHeap;

struct Edge {
    to: usize,
    weight: u32,
}

struct Graph<const VERTEX_COUNT: usize> {
    adj_list: [Vec<Edge>; VERTEX_COUNT],
}

impl<const VERTEX_COUNT: usize> Graph<VERTEX_COUNT> {
    pub fn new() -> Self {
        Self {
            adj_list: array::from_fn(|_| Vec::new()),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
        assert!(from > 0 && from <= VERTEX_COUNT, "Cannot add edge from vertex: {}", from);
        assert!(to > 0 && to <= VERTEX_COUNT, "Cannot add edge to vertex: {}", to);
        let from_ix: usize = from - 1;
        let to_ix: usize = to - 1;
        self.adj_list[from_ix].push(Edge { to: to_ix, weight });
        self.adj_list[to_ix].push(Edge { to: from_ix, weight });
    }
}

#[derive(Eq, PartialEq)]
struct Path
{
	to: usize,
	distance: u32,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance) // Reverse for min-heap
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_paths<const VERTEX_COUNT: usize>(graph: &Graph<VERTEX_COUNT>, mut source_vertex: usize) -> [u32; VERTEX_COUNT] {
    assert!(source_vertex > 0 && source_vertex <= VERTEX_COUNT, "Invalid vertex provided: {}", source_vertex);
    source_vertex = source_vertex - 1; // 0-indexed
    let mut shortest_paths: [u32; VERTEX_COUNT] = array::from_fn(|_| u32::MAX); // distance to all vertices is unknown
    shortest_paths[source_vertex] = 0; // path to the source vertex itself it zero
    // Dijkstra's algorithm
    let mut min_heap: BinaryHeap<Path> = Default::default();
    min_heap.push(Path{to: source_vertex, distance: 0}); // start at the source vertex
    while !min_heap.is_empty() {
        if let Some(Path { to: from_vertex, distance: current_distance }) = min_heap.pop() {
            for Edge{to: to_vertex, weight} in &graph.adj_list[from_vertex] {
                let new_distance: u32 = weight + current_distance;
                if new_distance < shortest_paths[*to_vertex] {
                    shortest_paths[*to_vertex] = new_distance;
                    println!("Found shortest path from {} to {} with weight = {}", source_vertex + 1, to_vertex + 1, new_distance);
                    min_heap.push(Path{ to: *to_vertex, distance: new_distance });
                }
            }
        }
    }
    shortest_paths
}

fn print_paths<const VERTEX_COUNT: usize>(source_vertex: usize, paths: &[u32; VERTEX_COUNT]) {
	println!("Shortest paths:");
	for (i, &distance) in paths.iter().enumerate() {
		if distance < u32::MAX {
            println!("{}->{} = {}", source_vertex, i + 1, distance);
        }
	}
}

fn main() {
    let mut graph: Graph<6> = Graph::new();
	graph.add_edge(1, 2, 7);
	graph.add_edge(1, 6, 14);
	graph.add_edge(1, 3, 9);

	graph.add_edge(2, 3, 10);
	graph.add_edge(2, 4, 15);

	graph.add_edge(3, 6, 2);
	graph.add_edge(3, 4, 11);

	graph.add_edge(4, 5, 6);
	graph.add_edge(5, 6, 9);
	let from: usize = 1;
	let shortest_paths = find_shortest_paths(&graph, from);
	print_paths(from, &shortest_paths);
}
