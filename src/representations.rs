use std::cmp::max;

#[derive(Debug)]
pub struct Graph {
    pub n_verts: usize,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Edge {
    pub start: usize,
    pub end: usize,
}

impl Graph {

    pub fn new() -> Graph {
        Graph {
            n_verts: 0,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&self, start: usize, end: usize) -> Graph {
        let mut edges = self.edges.clone();
        edges.push(Edge { start: start, end: end });
        Graph {
            n_verts: max(max(start, end) + 1, self.n_verts),
            edges: edges,
        }
    }

    pub fn adjacency_matrix(&self) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![0; self.n_verts]; self.n_verts];
        for edge in &self.edges {
            matrix[edge.start][edge.end] = 1;
            matrix[edge.start][edge.end] = 1;
        }
        matrix
    }

    pub fn adjacency_list(&self) -> Vec<Vec<usize>> {
        let mut list = vec![Vec::new(); self.n_verts];
        for edge in &self.edges {
            list[edge.start].push(edge.end);
        }
        list
    }
}
