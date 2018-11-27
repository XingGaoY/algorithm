## Metric TSP
Given a complete graph with non-negative arc metric weight, find a minimum cost cycle visiting every node exactly once. And there exist an 2-approximation algorithm for it.

The algorithm is like this: find an MST on the graph, double all edges to get an Eulerian graph. Find a Eulerian tour on this graph, and output the TSP tour in order that the nodes appear.

### Christofides Algorithm
This is a 1.5-approximation algorithm.
Let `G = (V,w)` be an instance of the travelling salesman problem.
- Create a minimum spanning tree `T` of `G`.
- Let `O` be the set of vertices with odd degree in `T`. By the handshaking lemma, `O` has an even number of vertices.
- Find a minimum-weight perfect matching `M` in the induced subgraph given by the vertices from `O`.
- Combine the edges of `M` and `T` to form a connected multigraph `H` in which each vertex has even degree.
- Form an Eulerian circuit in `H`.
- Make the circuit found in previous step into a Hamiltonian circuit by skipping repeated vertices (shortcutting).
