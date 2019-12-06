use petgraph::algo::astar;
use petgraph::graphmap::DiGraphMap;
use petgraph::Undirected;
use std::collections::HashMap;
use std::fs::read_to_string;

fn count_orbits(map: &DiGraphMap<&str, i32>, node: &str) -> u32 {
    let mut count: u32 = 0;
    for neighbor in map.neighbors(node) {
        count += 1 + count_orbits(map, neighbor);
    }
    count
}

fn main() {
    let input = read_to_string("input/06.txt").expect("Unable to read from file: input/06.txt");
    let input = input.lines();
    let mut map = DiGraphMap::new();

    for line in input {
        let orbit = line.split(')').collect::<Vec<_>>();
        let object = orbit[0];
        let orbiter = orbit[1];
        map.add_edge(object, orbiter, 1);
    }

    let mut count: u32 = 0;
    for node in map.nodes() {
        count += count_orbits(&map, node);
    }

    println!("{}", count);

    let graph = map.into_graph::<u32>().into_edge_type::<Undirected>();
    let mut nodes = HashMap::new();
    for node_ix in graph.node_indices() {
        nodes.insert(graph.node_weight(node_ix).unwrap().clone(), node_ix);
    }
    let path = astar(
        &graph,
        nodes["YOU"],
        |finish| finish == nodes["SAN"],
        |e| *e.weight(),
        |_| 0,
    );

    println!("{}", path.unwrap().0 - 2);
}
