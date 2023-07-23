
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
    use crate::representations::UndirectedGraph;
    use crate::representations::Graph;
    use queues::{Queue, queue, IsQueue};

    pub fn breadth_first_search(
        graph: &UndirectedGraph,
        start: usize,
        end: usize,
    ) -> Option<Vec<usize>> {
        let adjacency_list = graph.adjacency_list();
        let mut prev: Vec<Option<usize>> = vec![Option::None; graph.n_verts];
        prev[start] = Option::Some(start);
        let mut qu = queue![&start];

        while qu.peek().is_ok() {
            let node = qu.remove().ok().unwrap();
            for next_node in &adjacency_list[*node] {
                if prev[*next_node].is_some() {
                    continue;
                } else {
                    prev[*next_node] = Some(*node);
                }
                if *next_node == end {
                    return make_path(prev, start, end);
                }
                qu.add(next_node);
            }
        }

        Option::None
    }

    fn make_path(prev: Vec<Option<usize>>, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut rev_path: Vec<usize> = Vec::new();
        let mut node = end;
        let mut prev_node = prev[node].unwrap();

        while node != prev_node {
            rev_path.push(node);
            node = prev_node;
            prev_node = prev[prev_node].unwrap();
            if prev_node == start {
                return Option::Some(rev_path.into_iter().rev().collect());
            }
        }

        Option::None
    }

    // pub fn depth_first_search(
    //     graph: &UndirectedGraph,
    //     start: usize,
    //     end: usize,
    // ) -> Option<Vec<UndirectedEdge>> {
    //     Some(Vec::new())
    // }
}
