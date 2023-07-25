mod algorithms;
mod representations;

use crate::algorithms::generate::generate_graphs_up_to;
use crate::algorithms::pathfinding::breadth_first_search;
use crate::representations::{Graph, UndirectedGraph};

fn main() {
    let graph = UndirectedGraph{ n_verts: 5, edges: Vec::new() }
        .add_edge(0, 1)
        .add_edge(1, 2)
        .add_edge(2, 3)
        .add_edge(3, 5)
        .add_edge(3, 4)
        .add_edge(4, 5);

    let path = breadth_first_search(&graph, 0, 5);
    println!("{:?}", path);
}


#[cfg(test)]
mod tests {
    use crate::representations::UndirectedGraph;
    use crate::algorithms::generate::generate_graphs_up_to;
    use crate::algorithms::pathfinding::breadth_first_search;

    #[test]
    fn generate_graphs_up_to_2_works() {
        let graphs = generate_graphs_up_to(2);
        let expected_graphs = vec![
            UndirectedGraph { n_verts: 1, edges: Vec::new()},
            UndirectedGraph { n_verts: 2, edges: Vec::new() },
            UndirectedGraph { n_verts: 2, edges: Vec::new() }.add_edge(0, 1),
        ];
        assert_eq!(graphs, expected_graphs);
    }

    #[test]
    fn breadth_first_search_2_paths() {
        let graph = UndirectedGraph{ n_verts: 5, edges: Vec::new() }
        .add_edge(0, 1)
        .add_edge(1, 2)
        .add_edge(2, 3)
        .add_edge(3, 5)
        .add_edge(3, 4)
        .add_edge(4, 5);

        let path = breadth_first_search(&graph, 0, 5);
        assert_eq!(path, Some(vec![0, 1, 2, 3, 5]));
    }
}
