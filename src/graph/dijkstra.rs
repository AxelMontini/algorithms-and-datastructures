use super::structure::{AdjacencyList, Direction, Graph, WeightedAdjacencyStructure};

struct Dijkstra {
    pub distances: Vec<usize>,
    pub predecessor: Vec<usize>,
}

pub fn dijkstra<'a, D: Direction, A: WeightedAdjacencyStructure<'a, D>>(
    graph: &Graph<D, A>,
    start: usize,
) {
}
