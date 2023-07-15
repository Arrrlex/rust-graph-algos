use crate::representations::{Edge, Graph};
mod algorithms;
mod representations;

fn main() {
    let graph = Graph
        ::new()
        .add_edge(0, 1)
        .add_edge(0, 2)
        .add_edge(0, 3)
        .add_edge(0, 4);

    println!("{:?}", graph);
}
