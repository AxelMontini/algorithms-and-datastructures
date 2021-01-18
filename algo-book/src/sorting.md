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