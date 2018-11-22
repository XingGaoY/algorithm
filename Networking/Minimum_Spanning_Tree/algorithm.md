# Minimum Spanning Tree
Minimum spanning tree is a spanning tree that the sum of the weights of the edges in A1 is minimal over all possible spanning trees.

And we have the lemma:

    Let `T` in `A` be a subset of the arcs, and let `B` in `N` be a strict subset of the nodes, and assume that no arc in `T` leaves `B`. Assume that `T` is promising set of arcs for the MST in `(N, A)`. Let `a ` be the shortest arc shich leaves `B` in `A`,  and let `n_1` be its end node. Then `T+{a}` is promising.

## Kruskal
Add arcs in increasing order of weight for the tree, as long as they do not form a cycle.  
We could use disjoint set to keep track of connected components.  
Or represent each set as a rooted tree which is better.  
Or use the implementation as i used: connect all nodes in a disjoint set to a same virtual node and check/merge the v-node to keep track of that disjoint set, where all operation is `O(1)`.

## Prim
Growing a sing a single tre, rather than many disconnected trees. At each step find the minimum weight which leaves partial mst and add it to the tree.  
The main problem is how to find the shortest distance from the tree. We could use priority heap to find that: keep track of the distance of the node from the tree, and at each iteration, update the distance.
