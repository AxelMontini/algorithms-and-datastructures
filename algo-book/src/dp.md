# Some DP algorithms

## Longest increasing subsequence

Time: $\O(n \log n)$

The problem is to find the length of the longest _strictly_ increasing subsequence
(non-contiguous). The most efficient algorithm works in $ \O(n \log n) $ time using DP.

For every element `e`, there are 3 cases:

1. If `e` is smaller than the end element of every other sequence, it means that
   it's also the smallest element yet encountered. Initialize a new sequence of length 1
   containing `e`.
2. If `e` is the largest among all last elements of a sequence, it's also the largest
   element yet encountered. Extend the longest sequence with `e`.
3. If it's neither of the previous two, it's in between. Find a sequence whose end element is
   smaller than `e`, duplicate it and extend the duplicate one. Delete all sequences of the same length.

The algorithm on an array $A$ is defined as follows:

- DP Table size: $n$
- DP initialization: $T_0 = A_0$
- Variable initialization: $length = 1$
- Computation of an entry, for every $i$:
  1. If $A_i < T_0$, then assign $T_0 = A_i$
  2. Else if $A_i > T_{length - 1}$ then set $T_{length} = A_i$ and increment $length$.
  3. Else, using Binary Search, find the index $b$ of the smallest element greater than $A_i$.
     Then assign $T_b = A_i$
- The solution is going to be $length$

## Longest common subsequence

Time: $\O(m \cdot n)$
TODO

## Matrix chain multiplication

$m_{i, j} = \underset{i \le k < j}{min} \{ m_{i, k} + m_{k + 1, j} \}$
