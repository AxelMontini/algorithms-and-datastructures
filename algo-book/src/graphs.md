# Graphs

A graph is composed of _vertices_ and _edges_.

$$ G = (V, E),\quad v \in V,\quad e \in E $$

A graph can be _undirected_ or _directed_:

- In an undirected graph, an edge $ e = (u, v) $ is a two-way connection between two vertices.
- In a directed graph, the edge $ e = (u, v) $ is a one-way connection, from vertex $u$ to $v$.

A graph can be _weighted_. In this case, $ G = (V, E, w) $ and $w:\ E \rightarrow W$
is a function that maps an edge to its cost $ c \in W $.

Some useful terminology:

- _Walk_:
- _Trail_:
- _Path_:

## DFS: Depth-First Search

The idea is to discover the graph by starting at a vertex `s` and then traversing the children
depth-first.

A recursive procedure for DFS starting at vertex `s` could be this:

1. label `s` as discovered
2. For all edges `(s, v)` do:
   1. If vertex `v` is not discovered, run DFS with `v` as source.

When using DFS it's possible to compute and store the _pre_ and _post_ values
for each node, defined as follows:

- Pre: time at which the node is first discovered.
- Post: time at which the recursion on this node terminates.

A node `B` can be reached from `A` if `pre(B) > pre(A)` and `post(B) < post(A)`

## BFS: Breadth-First Search

## Shortest path from source

### Bellman-Ford algorithm

### Dijkstra's algorithm

Runtime with binary heap: $\O((\left|V\right| + \left|E\right|) \log \left|V\right|)$

Runtime with Fibonacci Heap: $\O(\left|E\right| + \left|V\right| \log \left|V\right|)$

The version using a binary heap uses a priority queue (min-heap).

It only works with **non-negative** weights!

Given a graph $G$ and a source $s$, the procedure is the following:

0. Initialize two arrays with one entry per vertex: `distance[v] = +INF` and `parent[v] = null`.
1. Initialize a priority queue (min-heap).
2. Set `distance[s] = 0`
3. Add `s` to the queue with key 0.
4. if the queue is empty, return.
5. extract the min value `u` from the queue.
6. For every edge `(u, v)`, do the following:
   1. if `distance[v] > distance[u] + w(u, v)`, change `v`'s parent to `u` and set its
      new distance. Then add `v` in the queue with its new distance as key.
7. Go to step _4_

## Shortest Paths between all pairs

### Floyd-Warshall algorithm

Time: $\Theta(|V|^3)$

Space: $\Theta(|V|^2)$

This algorithm uses dynamic programming to compute the shortest path between pairs.

It does not work with negative cycles, altough it can detect them easily.

Let's represent the data in a matrix `D`, where every row and column correspond to a vertex
$v_1, v_2, ..., v_n$. The base case is, given a row $i$ and a column $j$:

$$
\begin{cases}
   D_{i,j} = 0 & \text{if } i = j \\
   D_{i, j} = w(v_i, v_j) & \text{if } \exists (v_i, v_j) \in E \\
   D_{i,j} = \infty & \text{otherwise }
\end{cases}
$$

In plain text, the base case is:

- $0$ if on the diagonal ($i = j$)
- $w(v_i, v_j)$ if there's an edge $(v_i, v_j)$
- $\infty$ otherwise

The next step is to iterate a total of $|V|^3$ times, with the following recursive case,
$$ sp(i, j, k) = \min(sp(i, j, k-1), sp(i, k, k - 1) + sp(k, j, k - 1)) $$
where $sp(i, j, k)$ is the shortest path between vertices $i$ and $j$ and "middle vertex" $k$.

The full procedure is:

0. Initialize the $|V| \times |V|$ table `D`, filled with $\infty$
1. For each edge `(u, v)`, set `D[u][v] = w(u, v)`
2. For each vertex `v` set `D[v][v] = 0`
3. For `k` in $1$ to $|V|$:
   1. For `i` in $1$ to $|V|$
      1. For `j` in $1$ to $|V|$
         1. If `D[i][j] <= D[i][k] + D[k][j]`, do nothing.
         2. Otherwise, set the new shortest path `D[i][j] = D[i][k] + D[k][j]`.

After the execution, `D` will contain the shortest path between every pair of vertices, _unless_
there's a negative cycle in the graph. In that case there's gonna be negative values along the
diagonal, where only zeros are to be expected. If there's negative values along the diagonal, then
there's a negative cycle and the result is incorrect.

### Johnson's algorithm

Time (worst case): $\O(|V|^2 \log |V| + |V| |E|)$

This algorithm uses both the Bellman-Ford and Dijkstra's algorithms to
find the shortest path between every pair of vertices.

It works by adding a vertex $q$ into the graph, connected to every vertex by a _0-weight edge_.

Then the Bellman-Ford algorithm is executed (starting from $q$) to find for each vertex the minimum
weight $h(v)$ of a path from $q$ to $v$. If this step detects a negative cycle, the algorithm stops.

It then reweights the edges of the original graph: the edge $(u, v)$ is given the new length
$w(u, v) + h(u) - h(v)$

Finally, $q$ is removed and Dijkstra's algorithm is ran on each node $s$ to find the shortest path
between it and the other nodes. After this, $h(v) - h(u)$ is re-added to Dijkstra's result to
obtain the shortest path on the original graph.

Procedure:

1. Add a vertex `q` to the graph and add a 0-weight edge `(q, v)` for every `v`.
2. Run the Bellman-Ford algorithm from `q` to find the minimum weight `h(v)` of a path
   from `q` to `v`.
3. Change the weight of every edge, by setting it to `w(u, v) + h(u) - h(v)`
4. Remove `q` from the graph
5. Run Dijkstra on every vertex `s` to obtain the shortest path between every `s` and the
   other vertices. We now have the distances `D(u, v)`.
6. Add `h(v) - h(u)` to every distance `D(u, v)` to obtain the length of the shortest path
   between every pair of vertices.

## Minumum Spanning Tree

### Boruvka's algorithm

Time: $\O(|E| \log |V|)$

The idea is to first divide the graph in one-vertex components. Then the cheapest edge connecting every
component to another component is found and added to the MST. The components are then joined.

Procedure:

0. Initialize $|V|$ components, each one containing one vertex. Also initialize a list that will contain the edges of the MST.
1. If there's less than two components, return.
2. For each component, find the lowest weight edge that connects it to another component. Then add
   this edge to the MST and connect the two components.
3. Go to step _1_.

### Prim's algorithm

Time (binary heap): $\O((|V| + |E|) \log |V|) \le \O(|E| \log |V|)$

Prim' algorithm requires a starting vertex `s`.

Procedure:

### Kruskal's algorithm

Time: $\O(|E| \log |V|)$

This algorithm requires a _disjoin-set_ data structure.
It is similar to Boruvka's, but it differs in the fact that instead of looking for the cheapest edge of _every_ component,
it looks for the cheapest edge that connects _any_ two components.

Procedure:

0. Intialize a list for the edges of the MST.
1. Let `E` be the list of edges of the graph, **sorted** by increasing weight.
2. Create a set for each vertex in the graph.
3. For each edge `(u, v)` in `E` do:
   1. If `u` and `v` are in the same set, continue.
   2. Otherwise, add the edge `(u, v)` in the MST and perform an union operation on `u`'s and `v`'s sets.

Note: to prove the running time, $|E| \le |V|^2 \Rightarrow \O(\log |E|) \le \O(\log |V|)$
