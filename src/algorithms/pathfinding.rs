use crate::representations::Graph;
use crate::representations::UndirectedGraph;
use queues::{queue, IsQueue, Queue};

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
    let mut rev_path: Vec<usize> = vec![end];
    let mut node = end;
    let mut prev_node = prev[node].unwrap();

    while node != prev_node {
        rev_path.push(prev_node);
        if prev_node == start {
            return Option::Some(rev_path.into_iter().rev().collect());
        }
        node = prev_node;
        prev_node = prev[prev_node].unwrap();
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
