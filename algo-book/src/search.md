# Search Algorithms

There are various ways to search a data structure. Here we analyze
ways to search an array. The fastest method depends on the properties
of said array.

## Linear Search

Time: $\O(n)$

The simplest method. Simply iterate through the array and compare
every element with the one you're looking for.

For example, let's say that we want to find out `k`'s index in the array `A`.
If `k` is not in the array, we should get `-1` as a result. The following
procedure does exactly that:

0. Initialize i = 0
1. If `A`'s length is less or equal `i`, return `-1`.
2. If `A[i] == k`, return `i`.
3. Increment `i`
4. Go to step _1_.

## Binary Search

Time: $\O(\log n)$

Sometimes we need to search a value in a sorted array, meaning that
$$\forall i \forall j\ (i < j \Rightarrow A[i] \le A[j])$$

An array could also be sorted in descending order. The same search method can
be easily adapted for such case.

The procedure to search for `k` in said array `A` is the following:

0. Initialize `l = 0` and `r = A.length - 1`.
1. Set `mid = (l + r) / 2`.
2. If `mid` is not a valid index for `A`, return `-1` (not found).
3. Compare `k` with `A[mid]`.
4. If `k` is greater, then set `l = mid + 1`.
5. If `k` is smaller, then set `r = mid - 1`.
6. Else, we've found `k`. Return `mid`.

At every step, half of the remaining items are excluded from the search: thus the
logaritmic time.
