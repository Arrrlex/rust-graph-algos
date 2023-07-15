mod algorithms;
mod representations;

use crate::representations::Graph;
use crate::algorithms::generate::generate_graphs_up_to;

fn main() {
    // let graph = representations::UndirectedGraph::new()
    //     .add_edge(0, 1)
    //     .add_edge(0, 2)
    //     .add_edge(0, 3)
    //     .add_edge(0, 4);

    // println!("{:?}", graph);
    // println!("{:?}", graph.adjacency_matrix());

    let graphs = generate_graphs_up_to(4);
    for graph in graphs.into_iter() {
        println!("{:?}", graph.adjacency_matrix());
    }
}
