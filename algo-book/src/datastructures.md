# Datastructures

## Tree

A tree is a structure that begins from a root. Each node `a` can have a child node `b`. We can say then that
`a` is the parent of `b` and `b` is a child of `a`.

There are various types of tree. Every parent can have a certain amount of children, depending on
the type of tree.

The nodes that have no children ("the bottom-most ones") are called _leaves_.

### Binary Tree

A binary tree is a simple tree, where each node has at most 2 children.

### Binary Search Tree

A binary search tree is a tree where every node has two children: a left one and a right one.

For every node `p`, these conditions hold:

- Every node of the left subtree of `p` (the left child and all its descendants) has a value
  less or equal the one of `p`.
- Every node of the right subtree of `p` has a value greater or equal the one of `p`.

This way it's possible to look for an element in $\O(\log n)$ time.

Search for a key `k`:

0. If the tree is empty, return.
1. Set the current node as the root
2. If the current node is `null`, then the node is not in the tree. Return.
3. If the current node is greater than `k`, set its left child as current node. Then go back to step 2.
4. Else, if the current node is less than `k`, set its right child as current node. Then go to step 2.
5. Else, it means that the current node is equal to `k`. The node has been found, return.

Insertion of a node `n` is pretty simple:

0. Start at the root. If there's no root, set `n` as root.
1. Look at the current node.
2. If `n` is less or equal than the current node, set the left child as current node and go to step _1_, unless
there's no right child: in that case, set `n` as left child.
3. Else, set the right child as current node and go to step _1_,
unless ... (analogous to step _2_ ).

This way the node is inserted in the right place.

Deletion is a bit trickier, as one has to account for deletion of nodes that have children.

Procedure Delete node `n`:

1. If `n` has no children, simple remove `n` from the tree.
2. Else if it has only one child, remove `n` and put its only child at in its place.
3. Else, do the following: find either the smallest node greater than `n` or the greatest node smaller than `n`
_without children_ (usually a leaf), remove it and put it in `n`'s place (deleting `n`).

### AVL Tree

### Min / Max heap

A _min_ / _max_ heap os a type of _binary tree_ where a _condition_ must hold:
In a _min_ / _max_ heap, every parent is _smaller_ / _greater_ than all of its children.

It can be conveniently represented in memory as an array (0-indexed), where a node at index `i` has
left child `2*i + 1` and right child `2*i + 2`. This way it's easy to traverse the heap.

#### Sift Up

Time: $\O(\log n)$

Starts at a given element.
1. Check if the condition holds for this element and its parent
2. If not, swap the two. If it does, stop.
3. Now _this element_ is in a different place. Go back to step _1_.

#### Sift Down

Time: $\O(\log n)$

For an element `x`:
1. Get the max between `x`, its right child and its left one.
2. If `x` is not the max, swap the max with `x`. Otherwise, stop.
3. `x` is in a different place now. Go back to step 1.

#### Insert

Time: $\O(\log n)$

To insert a value in said heap, the most common way is to add it to the end of the array. Then perform a
"_sift up_" operation on that element.

#### Extract

Time: $\O(\log n)$

Remove the root and put the last leaf of the heap in its place. Then perform the "_sift down_" operation on the new root.

#### Build a min / max heap

There are two ways to build said heap.

- Insert each element one by one in time $\O(n \log n) = n \cdot \O(\log n)$
- Perform the _heapify_ operation on the given array (used in the HeapSort sorting algorithm).

The first way is too easy, so uninteresting.

The second way (_heapify_) uses the _sift down_ operation to transform the array in an heap.

Procedure "Heapify":
1. For every index `i` from the end of the array to the start, do:
2. Sift Down `i`

That's it. This is gonna build a heap from an array in-place ($\O(1)$ auxiliary space) and in time $\O(n)$.

Why time $\O(n)$? Well, the number of operations of sift down depends on how close we're to the bottom.
Since we start at the bottom, the sift down operation on the leaves runs no operations.
Thus the total number of calls of "sift down" is gonna be
$$ 0\frac{n}{2} + 1\frac{n}{4} + 2\frac{n}{8} + 3\frac{n}{16} + ... = n \underset{i = 2}{\overset{\infty}{\sum}} (i - 1)\frac{1}{2^i} \le n \underset{i = 1}{\overset{\infty}{\sum}} \frac{1}{2^i} \le \mathcal{O}(n) $$

### Linked List

### Stack

### Queue

### Priority Queue