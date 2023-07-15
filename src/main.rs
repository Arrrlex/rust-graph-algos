mod algorithms;
mod representations;

use crate::representations::Graph;

fn main() {
    let graph = representations::UndirectedGraph::new()
        .add_edge(0, 1)
        .add_edge(0, 2)
        .add_edge(0, 3)
        .add_edge(0, 4);

    println!("{:?}", graph);
    println!("{:?}", graph.adjacency_matrix());
}
