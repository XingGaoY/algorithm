# Nonbipartite Matching
**Definition: alternating paths**, refer to a path `P` as an alternating path if every consecutive pair of arcs in the path contains one matched and one unmatched arc. And it is even or odd if it contains even or odd number of arcs. And alternating cycle is a path that starts and ends at the same node.

Refer to an **odd alternating path** with respect to a matching as an **augmenting path** if the **first and last nodes** in the path are **unmatched**.

## Property
- if `M` is a matching and `P` is an augmenting path with respect to `M`, then `M\delta P` is a matching of cardinality `|M|+1`. And two additional nodes(first and last) are added to the matching.

## Augmenting Path Theorem
If node `p` is unmatched in a matching `M`, and this matching contains no augmenting path that starts at node `p`, then node `p` is unmatched in some maximum matching.

, which is equivalent with Berge Theorem:  
a matching `M*` is a maximum matching if and only if the graph `G` contains no augmenting path with respect to matching `M*`.

## Bipartite Matching Algorithm
Starting with a feasible matching `M` and then repeat the steps for every unmatching node `p\in N`: try to identify an augmenting path starting at node `p`. If we find such a path `P`, replace `M` with `M\delta P`. Otherwise delete node `p` and all the arcs incident to it from the graph.

The most nature way to find a augmenting path is start from a unmatching point `p`, then use search algorithm to identify all reachable nodes. If there is reachable unmatching nodes, we find a augmenting path. And more straightforward, we grow a search tree rooted at node `p`, and the nodes in the tree is labeled with the number of arcs from the root, odd or even. And whenever a unmatched node has an **odd label**, the path joining it and the root is an augmenting path.

So we get the algorithm for bipartite matching:
```python
def search(n, found):
  found = False
  n.label = even
  l = [p]
  while l:
    p = l.pop()
    if p.label = even:
      examine_even(p, found, l)
    else:
      examine_odd(p, found, l)
    
    if found:
      return
    
def examine_even(n, found, l):
  for j in n.arcs:
    if !j.match:
      q = j
      pred(q) = i
      found = true
      return
    if j.match and !j.labeled:
      pred(j) = i
      j.label = odd
      j.labled = True
      l.append(j)

def examine_odd(n, found, l):
    j.matched = i
    if !j.labeled:
      pred(j) = i
      j.label = even
      l.append(j)

def bipartite_match():
  M = []
  for n in N:
    if !n.match():
      search(n, found)
      if found:
        augment()
      else:
        N.delete(n)
        A.delete(n.arcs)
```
And the algorithm runs in `O(nm)`. And we have the property:  
A graph is said to possess a unique label property with respect to a given matching `M` and a root node `p` if the search procedure assigns a unique label to every labeled node irrespective of the order in which it examines labeled nodes.

However it does not fit for non-bipartite matching, non-bipartite fail to detect an augmenting path even there is one, as the unique label property does not work all the time.

## Flowers and Blossoms
A flower is a subgraph with two components:
1. **Stem**: A stem is an even alternating path that starts at root node and terminates at some node.
1. **Blossom**: A blossom is an odd alternating cycle that starts and terminates at the **terminal node of a stem** and has no other node in common with the stem.

And every node in a blossom is reachable from the root through two distinct alternating paths, one has even length and the other has odd length. And the even path terminates with a matched arc, and odd vice versa.

So when we find a blossom, we shrink it to a new node and label it even and then do the algorithm of bipartite matching
