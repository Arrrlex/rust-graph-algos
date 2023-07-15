use std::cmp::max;
use std::cmp::Ordering;

pub trait Graph {
    fn verts(&self) -> Vec<usize>;
    fn adjacency_matrix(&self) -> Vec<Vec<usize>>;
    fn adjacency_list(&self) -> Vec<Vec<usize>>;

    fn n_verts(&self) -> usize {
        self.verts().len()
    }
}

#[derive(Debug)]
pub struct DirectedGraph {
    pub n_verts: usize,
    pub edges: Vec<DirectedEdge>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DirectedEdge {
    pub start: usize,
    pub end: usize,
}

impl DirectedEdge {
    fn undirected(&self) -> UndirectedEdge {
        UndirectedEdge {
            start: self.start,
            end: self.end,
        }
    }
}

impl DirectedGraph {
    pub fn new() -> DirectedGraph {
        DirectedGraph {
            n_verts: 0,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&self, start: usize, end: usize) -> DirectedGraph {
        let mut edges = self.edges.clone();
        edges.push(DirectedEdge {
            start: start,
            end: end,
        });
        DirectedGraph {
            n_verts: max(max(start, end) + 1, self.n_verts),
            edges: edges,
        }
    }
}

impl Graph for DirectedGraph {
    fn verts(&self) -> Vec<usize> {
        (0..).take(self.n_verts).collect()
    }

    fn n_verts(&self) -> usize {
        self.n_verts
    }

    fn adjacency_matrix(&self) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![0; self.n_verts]; self.n_verts];
        for edge in &self.edges {
            matrix[edge.start][edge.end] = 1;
        }
        matrix
    }

    fn adjacency_list(&self) -> Vec<Vec<usize>> {
        let mut list = vec![Vec::new(); self.n_verts];
        for edge in &self.edges {
            list[edge.start].push(edge.end);
        }
        list
    }
}

#[derive(Debug)]
pub struct UndirectedGraph {
    pub n_verts: usize,
    pub edges: Vec<UndirectedEdge>,
}

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
    pub start: usize,
    pub end: usize,
}

impl PartialEq for UndirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}

impl Eq for UndirectedEdge {}

impl Ord for UndirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.start, self.end).cmp(&(other.start, other.end))
    }
}

impl PartialOrd for UndirectedEdge {
    fn partial_cmp(&self, other: &Self) ->Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UndirectedGraph {
    pub fn new() -> Self {
        Self {
            n_verts: 0,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&self, start: usize, end: usize) -> Self {
        let mut edges = self.edges.clone();
        edges.push(UndirectedEdge {
            start: start,
            end: end,
        });
        Self {
            n_verts: max(max(start, end) + 1, self.n_verts),
            edges: edges,
        }
    }
}

impl Graph for UndirectedGraph {
    fn verts(&self) -> Vec<usize> {
        (0..).take(self.n_verts).collect()
    }

    fn n_verts(&self) -> usize {
        self.n_verts
    }

    fn adjacency_list(&self) -> Vec<Vec<usize>> {
        let mut list = vec![Vec::new(); self.n_verts];
        for edge in &self.edges {
            list[edge.start].push(edge.end);
            list[edge.end].push(edge.start);
        }
        list
    }

    fn adjacency_matrix(&self) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![0; self.n_verts]; self.n_verts];
        for edge in &self.edges {
            matrix[edge.start][edge.end] = 1;
            matrix[edge.end][edge.start] = 1;
        }
        matrix
    }
}

pub fn to_undirected_graph(g: DirectedGraph) -> UndirectedGraph {
    let mut edges: Vec<UndirectedEdge> = g.edges.into_iter().map(|x| x.undirected()).collect();
    edges.sort();
    edges.dedup();
    UndirectedGraph {
        n_verts: g.n_verts,
        edges: edges,
    }
}
