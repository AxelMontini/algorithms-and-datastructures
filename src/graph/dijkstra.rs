use crate::datastructures::heap::MinHeap;

use super::structure::{AdjacencyList, Direction, Graph, WeightedAdjacencyStructure};

pub struct Dijkstra {
    pub distances: Vec<u32>,
    pub predecessor: Vec<Option<usize>>,
}

#[derive(Eq, Ord)]
struct QueueEntry<K: Ord, V>(K, V);

impl<K: Ord, V> PartialEq for QueueEntry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> PartialOrd for QueueEntry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Dijsktra's algorithm implemented with a min heap
pub fn dijkstra<'a, D: Direction, A: WeightedAdjacencyStructure<'a, D, u32>>(
    graph: &'a A,
    start: usize,
) -> Dijkstra {
    let mut priority_queue = MinHeap::new(vec![]);
    let mut d = vec![u32::MAX; graph.count_vertices()];
    let mut p = vec![None; graph.count_vertices()];

    d[start] = 0;

    priority_queue.insert(QueueEntry(0, start));

    while let Some(QueueEntry(cost, vert)) = priority_queue.extract() {
        if cost > d[vert] {
            continue;
        }

        for (target_vert, weight) in graph.adjacency_iter(vert).unwrap() {
            let sum = d[vert] + weight;
            if d[target_vert] > sum {
                println!("New dist for vertex {}: {}", target_vert, sum);
                d[target_vert] = sum;
                p[target_vert] = Some(vert);
                priority_queue.insert(QueueEntry(sum, target_vert));
            }
        }
    }

    Dijkstra {
        distances: d,
        predecessor: p,
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::structure::{WeightedAdjacencyList, WeightedAdjacencyStructure};

    use super::dijkstra;

    #[test]
    fn dijkstra_simple() {
        let mut graph = WeightedAdjacencyList::default();

        for _i in 0..10 {
            graph.insert_vertex();
        }

        graph.insert_edge(0, 1, 2);
        graph.insert_edge(0, 2, 1);
        graph.insert_edge(1, 5, 2);
        graph.insert_edge(2, 5, 4);
        graph.insert_edge(5, 6, 1);
        graph.insert_edge(5, 8, 4);
        graph.insert_edge(6, 9, 3);
        graph.insert_edge(8, 9, 1);
        graph.insert_edge(9, 7, 0);

        let output = dijkstra(&mut graph, 0);

        let expect_distances = vec![0, 2, 1, u32::MAX, u32::MAX, 4, 5, 8, 8, 8];
        let expect_predecessors = vec![
            None,
            Some(0),
            Some(0),
            None,
            None,
            Some(1),
            Some(5),
            Some(9),
            Some(5),
            Some(6),
        ];

        for ((idx, expected), actual) in expect_predecessors
            .iter()
            .enumerate()
            .zip(output.predecessor.iter())
        {
            assert_eq!(expected, actual, "Predecessor of vertex {}", idx);
        }

        for ((idx, expected), actual) in expect_distances
            .iter()
            .enumerate()
            .zip(output.distances.iter())
        {
            assert_eq!(expected, actual, "Shortest path to vertex {}", idx);
        }
    }
}
