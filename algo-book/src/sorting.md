# Sorting

## In-place, simple

### Bubble Sort

|             | Best    | Average   | Worst     |
| ----------- | ------- | --------- | --------- |
| Time        | $\O(n)$ | $\O(n^2)$ | $\O(n^2)$ |
| Space (aux) | $\O(1)$ | $\O(1)$   | $\O(1)$   |

Procedure:

1. Set `i = 0`
2. Iterate from `i` to the end of the array
3. Set `j = i`
4. If `A[j] > A[j + 1]` swap the two values
5. increment `j` and repeat step _5_ if `A` has a `j`-th element. Otherwise, increment `i` and go to step _2_

Can be optimized to stop if the sequence is already sorted (checking if any
value was swapped in the last run). However, the average and worst case remains unchanged.

### Selection Sort

|             | Best      | Average   | Worst     |
| ----------- | --------- | --------- | --------- |
| Comparisons | $\O(n^2)$ | $\O(n^2)$ | $\O(n^2)$ |
| Swaps       | $\O(1)$   | $\O(n)$   | $\O(n)$   |
| Space (aux) | $\O(1)$   | $\O(1)$   | $\O(1)$   |

Procedure:

1. Set `i = 0`.
2. Find the min value from `i` to the end of the array.
3. Swap it with the element at index `i`.
4. Increment `i`, go to step _2_.

### Insertion Sort

|             | Best    | Average   | Worst     |
| ----------- | ------- | --------- | --------- |
| Comparisons | $\O(n)$ | $\O(n^2)$ | $\O(n^2)$ |
| Swaps       | $\O(1)$ | $\O(n^2)$ | $\O(n^2)$ |
| Space (aux) | $\O(1)$ | $\O(1)$   | $\O(1)$   |

Procedure:

1. Set `i = 1`.
2. Insert `A[i]` in the sequence to its left, so that it is sorted.
3. Increment `i` and go to step _2_.

Note that step 2 can be made faster by using _binary search_.

### Heap Sort

This sort makes use of a max-heap to sort the values.

|             | Best           | Average        | Worst          |
| ----------- | -------------- | -------------- | -------------- |
| Time        | $\O(n \log n)$ | $\O(n \log n)$ | $\O(n \log n)$ |
| Space (aux) | $\O(1)$        | $\O(1)$        | $\O(1)$        |

Procedure:

1. Build the max heap with `count` elements.
2. Swap the biggest value (first value) of `A` and the one at index `count - 1`.
3. Decrease `count`
4. Repair the heap (_the element that was inserted in the back isn't part of the heap anymore!_).
5. Go to step _2_.

For steps _1_ and _4_ see [Datastructures](./datastructures.md)

### Merge Sort

This sort is a divide-and-conquer approach. It uses extra memory.

|             | Best           | Average        | Worst          |
| ----------- | -------------- | -------------- | -------------- |
| Time        | $\O(n \log n)$ | $\O(n \log n)$ | $\O(n \log n)$ |
| Space (aux) | $\O(n)$        | $\O(n)$        | $\O(n)$        |

It consists in sorting a subarray, then merging two sorted subarrays. This way it obtains
a longer sorted subarray. It keeps merging until the final array is obtained.

The algorithm is composed of two procedures:

Let `A` be the array to be sorted and `B` be the auxiliary array (same size as `A`).

Merge (given two subarrays at idx `a` to `b - 1` and from `b` to `end`):

0. Initialize `k = a`, `i = a`, `j = b`.
1. If `k >= end` return.
2. If `i < b` and (`A[i] <= A[j]` or `j >= end`), then assign `B[k] = A[i]` and increment `i`.
3. Else, assign `B[k] = A[j]` and increment `j`.
4. Go to step _1_.

In short, this procedure compares the elements (starting from the begining) of the two subarrays and inserts
the minimum in `B`. If `j >= end` or `i >= b`, then one of the two subarrays has "run out of elements", thus
the remaining elements are inserted last.

Now the rest of the procedure:

Merge Sort an array `A`:

0. Initialize `B` with the same length as `A`.
1. Initialize `width = 1`.
2. Initialize `i = 0`.
3. Merge the subarrays `i` to `i + width - 1` and `i + width` to `i + 2 * width`. All of these indices are rounded
down to fit the length of the array.
4. Set `i = i + 2 * width`. If `i` is a valid index for the array, go to step _3_.
5. Set `width = 2 * width`. If `width` is less than the length of the array, go to step _2_.

This is a bottom-up solution: it starts merging subarrays of single elements to form sorted subarrays of two elements.
Then it merges these 2-length subarrays into 4 element-long subarrays.

At every iteration it has to process $n$ elements and it does so $\O(\log n)$ times.
Thus the running time of $\O(n \log n)$.

### QuickSort