pub struct Graph<D: Direction, A: AdjacencyStructure<D>> {
    adjacency_structure: A,
    _phantom: std::marker::PhantomData<D>,
}

pub struct Directed;
pub struct Undirected;

pub trait Direction {}

impl Direction for Directed {}
impl Direction for Undirected {}

pub trait AdjacencyStructure<D: Direction> {
    /// Returns `true` whether it contains the edge `(v1, v2)`.
    fn contains_edge(&self, v1: usize, v2: usize) -> bool;
    fn insert_edge(&mut self, v1: usize, v2: usize) -> bool;
    fn remove_edge(&mut self, v1: usize, v2: usize) -> bool;
    /// Inserts a new vertex in the structure. The id is chosen automatically and returned.
    fn insert_vertex(&mut self) -> usize;
    /// Removes the last vertex of this structure. The id is returned.
    fn remove_vertex(&mut self) -> Option<usize>;
    // /// Returns an iterator over the adjacent vertices
    //TODO: GATs // fn iter_adjacent(&self) -> impl Iterator<usize>;
}

pub struct AdjacencyList<D: Direction> {
    vertices_list: Vec<Vec<usize>>,
    _phantom: std::marker::PhantomData<D>,
}

impl<D: Direction> AdjacencyList<D> {
    pub fn new() -> Self {
        Self {
            vertices_list: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl AdjacencyStructure<Directed> for AdjacencyList<Directed> {
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
}

pub struct AdjacencyMatrix {}

#[cfg(test)]
mod tests {
    use super::{AdjacencyList, AdjacencyStructure, Directed};

    #[test]
    fn adjacency_list_directed() {
        let mut list = AdjacencyList::<Directed>::new();

        for _ in 0..10 {
            list.insert_vertex();
        }

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
    }
}
