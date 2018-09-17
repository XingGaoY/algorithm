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
- Longest path: Single-source longest paths problem in edge-weighted DAGs. We can solve the single-source longest paths problems in edge-weighted DAGs by initializing the distTo[] values to negative infinity and switching the sense of the inequality in relax().
  - Critical path method: we consider the parallel precedence-constrained job, given jobs of specified durations to be completed, and has 
  to be proceed after certain work to be finished. We have as many processors as possible.
    - this problem could be solved by formulating it as a longest paths problem in an edge-weighted DAG.
    
## Bellman-Ford Algorithm
Initialize `distTo[s]` to 0 and all other `distTo[]` values to infinity, then considering the digraph's edges in any order, and relax 
all edges. Loop V such passes. It is inefficient because always relaxes `V*E` edges

- Queue Based Version: The only edges that could lead to a change in `distTo[]` are thos leaving a vertex whose `distTo[]` value changed in the previous pass. To keep such vertices, we use a FIFO queue to track them. Maintaining another two data structures:
  - A queue of vertices to be relaxed.
  - A vertex0index boolean array `onQ[]` that indicate which vertices are on the queue to avoid duplicates.
