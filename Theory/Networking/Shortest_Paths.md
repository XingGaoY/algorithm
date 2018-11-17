# Shortest Paths Problem
A shortest path from vertex `s` to vertex `t` is a directed path from `s` to `t` with the property that no other such path has a lower weight.

## Representation
Different two representation:
- Shortest Path Tree
A subgraph containing `s` and all verteces reachable from `s` such that every tree path is a shortest path in the digraph. And it is represented in two vertex-index arrays:
  - Edges on the shortest-paths tree: `edgeTo[v]` is the last edge on the shortest path from `s` to `v`;
  - Distance to the source: `distTo[v]` is the length of the shortest path from s to v;
  Initialize `diatTo[s]` to 0 and `distTo[v]` to a infinity for all other vertices v.
    - Edge Relaxation: relax an edge `v->w` means to test whether the best known way from `s` to `w` is to go from `s` to `v`, then 
    take the edge from `v` to `w`, and if so, update our data structures.
    ```java
      private void relax(DirectedEdge e){
        int v = e.from(), w = e.to();
        if (distTo[w] > distTo[v] + e.weight()) {
          distTo[w] = distTo[v] + e.weight();
          edgeTo[w] = e;
        }
      }
    ```
    - Vertex Relaxation: actually all of our implementations actually relax all the edges pointing from a given vertex.
    
## Dijkstra's algorithm
Dijkstra's algorithm solves the single-source shortest-paths problem in edge-weighted digraphs with nonnegative weights using 
extra space proportional to `V` and time proportional to `ElogV`(worst case).

Initializing `dist[s]` to 0 and all other `distTo[]` entries to positive infinity. Then repeatedly relaxes and adds to the tree 
a non-tree vertex with the lowest `distTo[]` value, continuing until all vertices are on the tree or no non-tree vertex has a 
finite `diatTo[]` value.

- If it is edge-weighted DAG, we could do vertex relaxation with topological sorting, finish Dijsktra in linear time and able to 
handle negative edge weights. And has ability to find longest paths.
- Longest path: Single-source longest paths problem in edge-weighted DAGs. We can solve the single-source longest paths problems in edge-weighted DAGs by initializing the `distTo[]` values to negative infinity and switching the sense of the inequality in `relax()`.
  - Critical path method: we consider the parallel precedence-constrained job, given jobs of specified durations to be completed, and has 
  to be proceed after certain work to be finished. We have as many processors as possible.
    - this problem could be solved by formulating it as a longest paths problem in an edge-weighted DAG.
    
## Bellman-Ford Algorithm
Initialize `distTo[s]` to 0 and all other `distTo[]` values to infinity, then considering the digraph's edges in any order, and relax 
all edges. Loop V such passes. It is inefficient because always relaxes `V*E` edges

- Queue Based Version: The only edges that could lead to a change in `distTo[]` are thos leaving a vertex whose `distTo[]` value changed in the previous pass. To keep such vertices, we use a FIFO queue to track them. Maintaining another two data structures:
  - A queue of vertices to be relaxed.
  - A vertex0index boolean array `onQ[]` that indicate which vertices are on the queue to avoid duplicates.

## Dual of SPP
### Data
- a digraph `D = (N, A)` with `|N| = n` nodes and `|A| = m` arcs;
- a source node `s\in N`;
- a cost function `c: A\to R`

SPP: find all minimum cost paths from `s` to all nodes in `N`.
### Bellman's Optimality Principle
**Each optimal policy is made by optimal sub-policies.**
And for SPP, we have: every shortest path from `s` to `t\in N` visiting `i\in N` is made by the shortest path from `s` to `i` and the shortest path from `i` to `t`.

### Mathematical Function
**Objective Function**: min. \sum_{i,j\in A}c_{ij}x_{ij}.(Here, the sum of the costs times the flow equals the sum of the distances.)
**Constraints**:

Flow conservation: ![Flow Conservation](https://raw.githubusercontent.com/XingGaoY/algorithm/master/img/Flow%20Conservation.png). 
Notice that `i` here is a node at the middle of the path, not the end node.

And summing up for all `t\in N` we obtain: ![Flow Conservation(1)](https://raw.githubusercontent.com/XingGaoY/algorithm/master/img/Flow%20Conservation(1).png)

Thus we have the primal formulation:![Primal Formulation](https://raw.githubusercontent.com/XingGaoY/algorithm/master/img/Primal%20Formulation.png)
And it's a **Totally Unimodular** matrix, and the right hand sides of the constraints are all integer numbers.

Therefore we have the guarantee that **every base solution of the continuous relaxation of P' has integer coordinates.** Then, we relax
the integrality restrictions:![Primal Formulation](https://raw.githubusercontent.com/XingGaoY/algorithm/master/img/Primal%20Formulation(1).png)

Then we have its dual problem: ![Dual Formulation](https://raw.githubusercontent.com/XingGaoY/algorithm/master/img/Primal%20Formulation.png)

Observe that nothing change to variable y if we change all of them a constant number, so we can have `y_s = 0`

## Ref
- [*Algorithms*](https://algs4.cs.princeton.edu/44sp/)
- [Shortest Path Dual](https://pdfs.semanticscholar.org/presentation/36ee/f917703238dffba7dd594379aea7c2b31066.pdf)
