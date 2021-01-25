pub mod heap {
    use std::cmp::Ordering;

    use crate::graph::structure::Graph;

    pub struct MaxHeap<V: Ord> {
        seq: Vec<V>,
    }

    impl<V: Ord> MaxHeap<V> {
        pub fn new(mut seq: Vec<V>) -> Self {
            heapify_max(&mut seq);
            Self { seq }
        }

        pub fn extract(&mut self) -> Option<V> {
            if self.seq.len() == 0 {
                None
            } else {
                // replace first with last
                let greatest = self.seq.swap_remove(0);

                // Fix heap
                sift_down(&mut self.seq, |a, b| a.cmp(b), 0);

                Some(greatest)
            }
        }
    }

    pub struct MinHeap<V: Ord> {
        seq: Vec<V>,
    }

    impl<V: Ord> MinHeap<V> {
        pub fn new(mut seq: Vec<V>) -> Self {
            heapify_min(&mut seq);
            Self { seq }
        }

        pub fn extract(&mut self) -> Option<V> {
            if self.seq.len() == 0 {
                None
            } else {
                // replace first with last
                let smallest = self.seq.swap_remove(0);

                // Fix heap
                sift_down(&mut self.seq, |a, b| b.cmp(a), 0);

                Some(smallest)
            }
        }

        pub fn peek(&self) -> Option<&V> {
            self.seq.get(0)
        }

        pub fn insert(&mut self, value: V) {
            self.seq.push(value);
            let start = self.seq.len() - 1;
            sift_up(&mut self.seq, |a, b| b.cmp(a), start);
        }
    }

    pub fn heapify_min<V: Ord>(seq: &mut [V]) {
        heapify(seq, |a, b| b.cmp(a));
    }

    pub fn heapify_max<V: Ord>(seq: &mut [V]) {
        heapify(seq, |a, b| a.cmp(b));
    }

    /// Heapify the sequence. The greater value (using the cmp function) is on top.
    pub fn heapify<V: Ord>(seq: &mut [V], cmp: impl Fn(&V, &V) -> Ordering) {
        for idx in (0..seq.len()).rev() {
            sift_down(seq, &cmp, idx);
        }
    }

    /// Perform a sift up operation from given start index. Used in `insert`
    pub fn sift_up<V: Ord>(seq: &mut [V], cmp: impl Fn(&V, &V) -> Ordering, start: usize) {
        let mut cursor = start;

        while cursor > 0 {
            let parent = (cursor - 1) / 2;

            if cmp(&seq[cursor], &seq[parent]) == Ordering::Greater {
                seq.swap(cursor, parent);
                cursor = parent;
            } else {
                break;
            }
        }
    }

    pub fn sift_down<V: Ord>(seq: &mut [V], cmp: impl Fn(&V, &V) -> Ordering, start: usize) {
        let lc = |i| 2 * i + 1;
        let rc = |i| 2 * i + 2;
        let mut cursor = start;

        loop {
            // iterative procedure. If the parent is not greater (according to cmp), swap it with its greatest child. Then
            // Repeat procedure at the new place of the parent. End if no swap happened.
            let lc_idx = lc(cursor);
            let rc_idx = rc(cursor);

            let max_idx = [cursor, lc_idx, rc_idx]
                .iter()
                .copied()
                .max_by(|&a, &b| {
                    seq.get(b)
                        .map(|val| cmp(&seq[a], val))
                        .unwrap_or(Ordering::Greater)
                })
                .unwrap();

            if max_idx != cursor {
                seq.swap(cursor, max_idx);

                cursor = max_idx; // now check the sub-tree since the parent was modified
            } else {
                break; // done fixing sub-heap
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{MaxHeap, MinHeap};

        #[test]
        fn heap() {
            let seq = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

            let mut min_heap = MinHeap::new(seq);

            for &el in &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                assert_eq!(Some(el), min_heap.extract());
            }

            assert_eq!(None, min_heap.extract());

            let seq = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

            let mut min_heap = MaxHeap::new(seq);

            for &el in &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
                assert_eq!(Some(el), min_heap.extract());
            }

            assert_eq!(None, min_heap.extract());

            // test insertion

            let mut min_heap = MinHeap::new(vec![]);

            for i in (0..10).rev() {
                min_heap.insert(i);
                assert_eq!(Some(i), min_heap.peek().copied());
            }

            for i in 0..10 {
                assert_eq!(Some(i), min_heap.extract());
            }
        }
    }
}
