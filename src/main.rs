mod algorithms;
mod representations;

use crate::algorithms::generate::generate_graphs_up_to;
use crate::representations::Graph;

fn main() {
    let graphs = generate_graphs_up_to(4);
    for graph in graphs.into_iter() {
        println!("{:?}", graph.adjacency_matrix());
    }
}


#[cfg(test)]
mod tests {
    use crate::representations::UndirectedGraph;
    use crate::algorithms::generate::generate_graphs_up_to;
    #[test]
    fn generate_graphs_up_to_2_works() {
        let graphs = generate_graphs_up_to(2);
        let expected_graphs = vec![
            UndirectedGraph { n_verts: 1, edges: Vec::new()},
            UndirectedGraph { n_verts: 2, edges: Vec::new() },
            UndirectedGraph { n_verts: 2, edges: Vec::new() }.add_edge(0, 1),

            // UndirectedGraph { n_verts: 3, edges: Vec::new() },
            // UndirectedGraph { n_verts: 3, edges: Vec::new() }.add_edge(0, 1),
            // UndirectedGraph { n_verts: 3, edges: Vec::new() }.add_edge(0, 1).add_edge(0, 2),
            // UndirectedGraph { n_verts: 3, edges: Vec::new() }.add_edge(0, 1).add_edge(0, 2).add_edge(1, 2)
        ];
        assert_eq!(graphs, expected_graphs);
    }
}