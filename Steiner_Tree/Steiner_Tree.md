# Steiner Tree
## Euclidean Steiner Tree
**Definition**: Assume a given set `R` of `n` distinct points in the 2D-plane. Find a set of points `S` and a spanning tree `T = (R\cup S, E)` such that the weight of the tree is minimized. The weight is Euclidean distance. The resulting tree is **Euclidean Steiner Tree** and the points in `S` are called `Steiner` points.

And this problem is NP-hard.

## Metric Steiner Tree
**Definition**: A function `d: V*V->|R*` is a metric if it satisfies:
- Non-negativity
- Identity: `d(u,u) = 0`
- Symmetric
- Triangle inequality

And the definition of Metric Steiner Tree:  
Assume we are given:
- A set of required vertices `R`
- A set of Steiner vertices `S`
- A distance function `d: (R\cup S)*(R\cup S)->|R`  
The problem is to find a subset `S'\in S` of the Steiner vertices and a spanning tree `T = (R\cap S', E)` of minimum weight.

## General Steiner Tree
**Definition:** Given a graph `G = (V, E)` with weights, a set of required vertices `R\belongs V` and a set of Steiner vertices `S\belongs V`. The problem is to find a subset `S'\belongs S` of the Steiner vertices and a spanning tree `T = (R\cup S', E)` of minimum weight with a specific distance metric.

## Metric Steiner Tree Approximation Algorithm
**Theorem:** For a set of required vertices `R`, a set of Steiner vertices `S`, and a metric distance function of `d` the minimum spanning tree of `(R, d)` is a 2-approximation of the optimal Steiner Tree of `(R, S, d)`.

First, transform `T` into cycle `C` of at most twice the cost. Perform a DFS of the tree T. Therefore, arcs would be visited twice, one from parent to child, and another from child to parent. And then we compute the total cost, which is exactly twice cost of the total.

Second, remove Steiner vertices by shortcutting it, as triangle inequality, the cost would get smaller.

Then remove the duplicates and arbitrary edges.

And the algorithm is a 2-approximation for the minimum cost Steiner tree.

## General Steiner Tree Approximation Algorithm
We find the algorithm via reduction, define a metric:
**Definition:** Given a graph `G = (V, E)` and non-negative edge weights `w`, define a metric completion: for every `u, v\in V`, define `d(u, v)` to be the distance of the shortest path from `u` to `v` in `G` with respect to the weight function `w`.

And we do the reduction, `(V, E)->(R, S, d)` with the `d` function we defined. And to reverse the reduction: we add every edge appears in any path back to the set and remove the cycles. Then we get the `\alpha`-approximation algorithm for general Steiner Tree problem.
