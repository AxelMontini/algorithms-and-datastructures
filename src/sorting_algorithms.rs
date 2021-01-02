//! This module contains various sorting algorithms
//! that have been discussed in the lecture.
//! Each algorithm is briefly explained and its runtime is analyzed.

pub mod bubble_sort {}

pub mod selection_sort {}

pub mod insertion_sort {}

pub mod heap_sort {
    use std::cmp::Ord;
    use std::fmt::Debug;

    /// Implementation of a custom MinHeap to show how it works internally.
    struct MinHeap<'s, I> {
        len: usize,
        seq: &'s mut [I],
    }

    impl<'s, I: Ord + Copy> MinHeap<'s, I> {
        /// Builds a min heap by moving the elements in the given sequence.
        pub fn new(seq: &'s mut [I]) -> Self {
            let len = seq.len();
            let mut heap = Self { seq, len };

            heap.build_heap();

            heap
        }

        fn build_heap(&mut self) {
            // if 0- or 1-length it's already built
            if self.len > 1 {
                // last non-leaf node
                let start = (self.len / 2) - 1;

                for idx in (0..=start).rev() {
                    self.min_heapify(idx);
                }
            }
        }

        fn min_heapify(&mut self, idx: usize) {
            let mut checking = idx;

            loop {
                let parent = checking;
                let left = 2 * checking + 1;
                let right = 2 * checking + 2;

                let mut min_idx = parent;

                if left < self.len && self.seq[min_idx] > self.seq[left] {
                    min_idx = left;
                }

                if right < self.len && self.seq[min_idx] > self.seq[right] {
                    min_idx = right;
                }

                // min-heap property broken, must swap
                if min_idx != parent {
                    self.seq.swap(parent, min_idx);
                    checking = min_idx;
                } else {
                    // this parent is fine and thus also all its children are.
                    break;
                }
            }
        }

        pub fn remove_min(&mut self) -> Option<I> {
            if self.len > 0 {
                let min = self.seq[0];

                // swap head (min) with the last element
                self.seq[0] = self.seq[self.len - 1];
                self.len -= 1;

                // Fix heap to restore min-heap property
                self.min_heapify(0);

                Some(min)
            } else {
                None
            }
        }
    }

    /// Implementation of the heap sort algorithm.
    /// It builds a min-heap in O(n) and then it extracts each value in O(log n), resulting in O(n log n) time total.
    /// Uses auxiliary O(n) space.
    pub fn heap_sort<I: Ord + Debug + Copy>(seq: &mut [I]) {
        let mut temp = Vec::with_capacity(seq.len());

        // build min heap in O(n)
        let mut heap = MinHeap::new(seq);

        // Remove every element fron the min heap in O(n log n)
        while let Some(removed) = heap.remove_min() {
            temp.push(removed);
        }

        seq.copy_from_slice(&temp);
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn correct_simple() {
            let sequence = &mut [8, 8, 4, 3, 2, 1, 9, 5];
            let expected = &[1, 2, 3, 4, 5, 8, 8, 9];

            super::heap_sort(sequence);

            assert_eq!(expected, sequence);
        }

        #[test]
        fn correct_small() {
            let set: &mut [(&mut [i64], &[i64])] = &mut [
                (&mut [], &[]),
                (&mut [1], &[1]),
                (&mut [2, 1], &[1, 2]),
                (&mut [1, 1], &[1, 1]),
            ];

            for (seq, expected) in set.iter_mut() {
                super::heap_sort(seq);

                assert_eq!(expected, seq);
            }
        }
    }
}

pub mod quick_sort {
    use std::cmp::Ord;
}

pub mod merge_sort {
    use std::cmp::Ord;

    /// This implementation of merge sort is recursive. The sequence is split in half and the function is called recursively
    /// on both halves. Then the two sorted halves are merged into a longer sorted sequence.
    /// The running time is O(n log n) because every element is part of O(log n) sequences
    /// and the algorithm iterates on every element of every sequence once.
    ///
    /// Thus it performs `n * O(log n)` comparisons
    /// and thus its running time is `O(n log n)`.
    ///
    /// It allocates a temp vector of space `O(n)`.
    pub fn merge_sort<I: Ord + Copy + Default + std::fmt::Debug>(sequence: &mut [I]) {
        if sequence.len() < 2 {
            // already sorted
            return;
        }

        let mut temp = vec![I::default(); sequence.len()];

        sort(sequence, temp.as_mut_slice());
    }

    pub fn sort<I: Ord + Copy + std::fmt::Debug>(seq: &mut [I], temp: &mut [I]) {
        if seq.len() < 2 {
            return;
        }

        println!("Sort {}", seq.len());

        // split in two
        let second = seq.len() / 2;

        sort(&mut seq[0..second], &mut temp[0..second]);
        sort(&mut seq[second..], &mut temp[second..]);
        merge(seq, second, temp);
    }

    /// Merges the given two sorted sequences. The first sequence starts at `seq[0]`,
    /// the second at `seq[second]`.
    pub fn merge<I: Ord + Copy + std::fmt::Debug>(seq: &mut [I], second: usize, temp: &mut [I]) {
        let (mut i, mut j, mut k) = (0, second, 0);

        // Merge at least half of the elements (the min ones)
        while i < second && j < seq.len() {
            if seq[i] <= seq[j] {
                temp[k] = seq[i];
                i += 1;
            } else {
                temp[k] = seq[j];
                j += 1;
            }

            k += 1;
        }

        // Now add the remaining ones to temp (they're all bigger
        // than the previous ones)

        while k < temp.len() && i < second {
            temp[k] = seq[i];
            i += 1;
            k += 1;
        }

        while k < temp.len() && j < seq.len() {
            temp[k] = seq[j];
            j += 1;
            k += 1;
        }

        // Copy the values from temp back to the original array.
        seq.iter_mut()
            .enumerate()
            .for_each(|(idx, el)| *el = temp[idx]);
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn correct() {
            let sequence = &mut [9, 8, 2, 3, 1, 5, 6, 4, 7];
            let expected = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

            super::merge_sort(sequence);

            assert_eq!(sequence, expected);
        }
    }
}
