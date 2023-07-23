pub mod generate {
    use crate::representations::UndirectedGraph;
    pub fn generate_graphs_up_to(n: usize) -> Vec<UndirectedGraph> {
        let mut graphs: Vec<UndirectedGraph> = Vec::new();

        for n_verts in (1..).take(n) {
            for graph in generate_graphs(n_verts).into_iter() {
                graphs.push(graph);
            }
        }
        graphs
    }

    pub fn generate_graphs(n: usize) -> Vec<UndirectedGraph> {
        let mut graphs: Vec<UndirectedGraph> = Vec::new();

        graphs.push(UndirectedGraph {
            n_verts: n,
            edges: Vec::new(),
        });

        for i in (0..).take(n) {
            for j in (0..).take(n) {
                if i < j {
                    for graph in graphs.clone() {
                        graphs.push(graph.add_edge(i, j));
                    }
                }
            }
        }

        graphs
    }
}

pub mod pathfinding {
    use crate::representations::{UndirectedEdge, UndirectedGraph};
    fn breadth_first_search(
        graph: &UndirectedGraph,
        start: usize,
        end: usize,
    ) -> Option<Vec<UndirectedEdge>> {
        let adjacency_list = graph.adjacency_list();
        let mut path: Vec<UndirectedEdge> = Vec.new();
        let mut head: usize = start;

    }

    fn depth_first_search(
        graph: &UndirectedGraph,
        start: usize,
        end: usize,
    ) -> Option<Vec<UndirectedEdge>> {
        Some(Vec::new())
    }
}
