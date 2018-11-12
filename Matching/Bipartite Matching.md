# Bipartite Matching
Problem:
1. The cardinality problem: to find a matching containing the maximum number of the arcs
1. The weighted problem: find a matching with largest overall weight(with weight 1 in cardinality problem)

## Solution and Complexity
It is easy to reduce the problem into network flow problems. Let `S(n, m, C)` denote the time required to solve a SP problem. 
- Cardinality problem:
  - Maximum flow: `O(m\sqrt(n))`
- Weighted problem: `O(nS(n,m,C))`:
  - Successive shortest path
  - Primal-dual minimum cost flow
  
## Max Flow(Cardinality)
Set flow on arcs `(s, i_r)` and `(j_r, t)` to 1 and all arcs in between the bipartite graph 1. and this would give us a max flow n.  
And decomposite the flow to unit flow, the max flow gives us the maximum match.

## Weighted Matching
### Successive Shortest Path
- Obtain shortest path distances from a supply node to all other nodes in a residual network
- Use distance to update node potentials
- Augment flow form the supply node to a demand node

## Hungarian
Direct implementation of primal-dual algorithm.
- Transform minimum cost flow problem into a single supply node `s*` and single demand node `t*`
- And at each iteration:
  - SP from `s*` to all other nodes
  - update potentials
  - solve maximum flow problem sends the maximum possible flow from node `s*` to node `t*` over arcs with zero reduce costs.
  
## Stable Marriage
Iterative greedy algorithm:  
```
  Put all men into a unassigned list
  When elements in the list:
      Choose one man proposes to his most preferred woman
      Compare the priority of this man and the man already engaged with the woman
      The man with higher priority engaged with the woman
      Put the man with less priority back to the unassigned list
```
For each woman, she rejects a man at most once, and the complexity at most `O(n^2)`.

**A woman never rejects a stable partner.**  
**A man optimal stable matching is constructed.**
