use core::slice;
use std::{
    iter::Copied,
    ops::{Deref, DerefMut},
};

pub struct Graph<'a, D: Direction, A> {
    adjacency_structure: A,
    _phantom: std::marker::PhantomData<&'a D>,
}

impl<'a, D: Direction, A: AdjacencyStructure<'a, D> + Default> Graph<'a, D, A> {
    pub fn new() -> Self {
        Self::with_adjacency_structure(Default::default())
    }

    pub fn with_adjacency_structure(adjacency_structure: A) -> Self {
        Self {
            adjacency_structure,
            _phantom: Default::default(),
        }
    }
}

impl<'a, D: Direction, A: AdjacencyStructure<'a, D>> Deref for Graph<'a, D, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.adjacency_structure
    }
}

impl<'a, D: Direction, A: AdjacencyStructure<'a, D>> DerefMut for Graph<'a, D, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.adjacency_structure
    }
}

pub struct Directed;
pub struct Undirected;

pub trait Direction {}

impl Direction for Directed {}
impl Direction for Undirected {}

pub trait AdjacencyStructure<'a, D: Direction>: Default {
    type AdjIter: Iterator<Item = usize> + 'a;

    /// Returns `true` whether it contains the edge `(v1, v2)`.
    fn contains_edge(&self, v1: usize, v2: usize) -> bool;
    fn insert_edge(&mut self, v1: usize, v2: usize) -> bool;
    fn remove_edge(&mut self, v1: usize, v2: usize) -> bool;
    /// Inserts a new vertex in the structure. The id is chosen automatically and returned.
    fn insert_vertex(&mut self) -> usize;
    /// Removes the last vertex of this structure. The id is returned.
    fn remove_vertex(&mut self) -> Option<usize>;
    /// Returns true whether it contains `vertex`
    fn contains_vertex(&self, vertex: usize) -> bool;
    ///
    fn adjacency_iter(&'a self, vertex: usize) -> Option<Self::AdjIter>;
}

pub trait WeightedAdjacencyStructure<'a, D: Direction> {
    type AdjIter: Iterator<Item = usize> + 'a;

    /// Returns `true` whether it contains the edge `(v1, v2)`.
    fn contains_edge(&self, v1: usize, v2: usize) -> bool;
    fn insert_edge(&mut self, v1: usize, v2: usize, weight: usize) -> bool;
    fn remove_edge(&mut self, v1: usize, v2: usize) -> bool;
    /// Inserts a new vertex in the structure. The id is chosen automatically and returned.
    fn insert_vertex(&mut self) -> usize;
    /// Removes the last vertex of this structure. The id is returned.
    fn remove_vertex(&mut self) -> Option<usize>;
    /// Returns true whether it contains `vertex`
    fn contains_vertex(&self, vertex: usize) -> bool;
    ///
    fn adjacency_iter(&'a self, vertex: usize) -> Option<Self::AdjIter>;

    fn get_weight(&self, v1: usize, v2: usize) -> Option<usize>;
}

pub struct AdjacencyList<D: Direction> {
    pub(crate) vertices_list: Vec<Vec<usize>>,
    _phantom: std::marker::PhantomData<D>,
}

impl<D: Direction> Default for AdjacencyList<D> {
    fn default() -> Self {
        Self {
            vertices_list: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<D: Direction> AdjacencyList<D> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> AdjacencyStructure<'a, Directed> for AdjacencyList<Directed> {
    type AdjIter = Copied<slice::Iter<'a, usize>>;

    fn contains_edge(&self, v1: usize, v2: usize) -> bool {
        self.vertices_list
            .get(v1)
            .map(|vert_list| vert_list.contains(&v2))
            .unwrap_or(false)
    }

    /// returns true whether the edge was added, false if it already existed or the given vertices don't exist in the structure
    fn insert_edge(&mut self, v1: usize, v2: usize) -> bool {
        if self.contains_edge(v1, v2) {
            return false;
        }

        if v1 == v2 || v1 >= self.vertices_list.len() || v2 >= self.vertices_list.len() {
            return false;
        }

        self.vertices_list[v1].push(v2);

        return true;
    }

    /// Returns true if the edge was removed, false otherwise
    fn remove_edge(&mut self, v1: usize, v2: usize) -> bool {
        let pos = self
            .vertices_list
            .get(v1)
            .and_then(|list| list.iter().position(|&target| target == v2));
        if let Some(pos) = pos {
            self.vertices_list[v1].swap_remove(pos);
            true
        } else {
            false
        }
    }

    fn insert_vertex(&mut self) -> usize {
        let new_vertex = self.vertices_list.len();

        self.vertices_list.push(vec![]);

        new_vertex
    }

    fn remove_vertex(&mut self) -> Option<usize> {
        let _removed = self.vertices_list.pop();
        let idx = self.vertices_list.len();

        // Remove references to the dead vertex
        self.vertices_list
            .iter_mut()
            .for_each(|vert_list| vert_list.retain(|&target| target != idx));

        if let Some(_) = _removed {
            Some(idx)
        } else {
            None
        }
    }

    fn contains_vertex(&self, vertex: usize) -> bool {
        self.vertices_list.get(vertex).is_some()
    }

    fn adjacency_iter(&'a self, vertex: usize) -> Option<Self::AdjIter> {
        self.vertices_list.get(vertex).map(|l| l.iter().copied())
    }
}

pub struct WeightedAdjacencyList<D: Direction> {
    list: AdjacencyList<D>,
    /// same layout as list: weights[i][j] = weight of list[i][j]
    weights: Vec<Vec<usize>>,
}

impl<D: Direction> Default for WeightedAdjacencyList<D> {
    fn default() -> Self {
        Self {
            list: Default::default(),
            weights: Default::default(),
        }
    }
}

impl<'a> WeightedAdjacencyStructure<'a, Directed> for WeightedAdjacencyList<Directed> {
    type AdjIter = <AdjacencyList<Directed> as AdjacencyStructure<'a, Directed>>::AdjIter;

    fn contains_edge(&self, v1: usize, v2: usize) -> bool {
        self.list.contains_edge(v1, v2)
    }

    fn insert_edge(&mut self, v1: usize, v2: usize, weight: usize) -> bool {
        let res = self.list.insert_edge(v1, v2);

        if res {
            self.weights[v1].push(weight);
        }

        res
    }

    fn remove_edge(&mut self, v1: usize, v2: usize) -> bool {
        if self.contains_vertex(v1) {
            let pos = self.adjacency_iter(v1).unwrap().position(|v| v == v2);

            if let Some(pos) = pos {
                self.weights[v1].swap_remove(pos);
            }
        }

        self.list.remove_edge(v1, v2)
    }

    fn insert_vertex(&mut self) -> usize {
        self.weights.push(vec![]);
        self.list.insert_vertex()
    }

    fn remove_vertex(&mut self) -> Option<usize> {
        if self.weights.len() != 0 {
            let v_rem = self.weights.len() - 1;

            self.weights.pop();

            for (list, weights) in self.list.vertices_list.iter().zip(self.weights.iter_mut()) {
                let pos = list.iter().position(|&v| v == v_rem);
                if let Some(pos) = pos {
                    weights.remove(pos);
                }
            }
        }

        self.list.remove_vertex()
    }

    fn contains_vertex(&self, vertex: usize) -> bool {
        self.list.contains_vertex(vertex)
    }

    fn adjacency_iter(&'a self, vertex: usize) -> Option<Self::AdjIter> {
        self.list.adjacency_iter(vertex)
    }

    fn get_weight(&self, v1: usize, v2: usize) -> Option<usize> {
        let mut iter = self.list.adjacency_iter(v1)?;
        let pos = iter.position(|a| a == v2)?;

        Some(self.weights[v1][pos])
    }
}

pub struct AdjacencyMatrix {}

#[cfg(test)]
mod tests {
    use super::{
        AdjacencyList, AdjacencyStructure, Directed, Graph, WeightedAdjacencyList,
        WeightedAdjacencyStructure,
    };

    #[test]
    fn graph() {
        let list = AdjacencyList::<Directed>::new();
        let mut graph = Graph::with_adjacency_structure(list);

        graph.insert_vertex();

        assert!(graph.contains_vertex(0));
    }

    #[test]
    fn adjacency_list_directed() {
        let mut list = AdjacencyList::<Directed>::new();

        for _ in 0..10 {
            list.insert_vertex();
        }

        // insert edge to same vertex should fail
        assert!(!list.insert_edge(0, 0));

        assert!(list.insert_edge(0, 1));
        assert!(!list.insert_edge(0, 1));
        assert!(list.contains_edge(0, 1));

        assert!(list.insert_edge(0, 2));
        assert!(!list.insert_edge(0, 2));
        assert!(list.contains_edge(0, 2));

        assert!(list.remove_edge(0, 1));
        assert!(!list.remove_edge(0, 1));
        assert!(!list.contains_edge(0, 1));

        assert!(list.remove_edge(0, 2));
        assert!(!list.remove_edge(0, 2));
        assert!(!list.contains_edge(0, 2));

        let mut list = AdjacencyList::<Directed>::new();

        for _ in 0..10 {
            list.insert_vertex();
        }
        // Connect the last vertex and the first to every other vertex
        for i in 1..9 {
            list.insert_edge(0, i);
            list.insert_edge(i, 9);
        }

        for i in 1..9 {
            assert!(list.contains_edge(0, i));
            assert!(list.contains_edge(i, 9));
        }

        list.insert_edge(0, 9);
        assert!(list.contains_edge(0, 9));

        // remove 9. now check whether all related edges got removed too.
        list.remove_vertex();

        for i in 0..9 {
            assert!(!list.contains_edge(i, 9));
        }
    }

    #[test]
    fn weighted_adjacency_list() {
        let mut list = WeightedAdjacencyList::<Directed>::default();

        list.insert_vertex();
        list.insert_vertex();

        assert_eq!(None, list.adjacency_iter(0).unwrap().next());

        list.insert_edge(0, 1, 4);

        assert_eq!(Some(4), list.get_weight(0, 1));

        let mut iter = list.adjacency_iter(0).unwrap();

        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }
}
