# Max Flow Problem
We have the following component:
- a directed graph G, with vertices V and directed edges E;
- a source vertex s\in V;
- a sink vertex t\in V;
- a nonnegative and integral capacity u_e for each edge e\in E;

And therefore, we have two constraints:
Capacity:
  f_e\le u_e for every edige e\in E;
Conservation:
  for every vertex v other than s and t, amount of flow entering v = amount of flow exiting v;
  
## Naive Greedy Algorithm
```
init f_e = 0 for all e\in E
repeat
  search for an s-t path P such that f_e < u_e for every 
  // O(|E|) time using BFS or DFS
  if no such path then
    halt with current flow {f_e}_{e\in E}
  else
    let \delta = min_{e\in P}(u_e-f_e)
    for all edges e of P do
      increase f_e by \delta
```
This algorithm will find a path and its maximum flow, but not a maximum flow path over whole graph.

## Rsidual Graphs
**Undo operation**:
  given a graph G and a flow f in it, we form a new flow network G_f that has the same vertex set of G and that has two edges for each edge 
  of G. An edge e = (v, w) of G that carries flow f_e and has capacity u_e and a "backward edge" (w, v) of G_f with capacity f_e
  
## Ford-Fulkerson Algorithm
```
initialize f_e = 0 for all e\in E
repeat
  search for an s-t path P in the current residual graph G_f such that every edge of P has positive residual capacity
  if no such path then
    halt with current flow {f_e}_{e\in E}
  else
    let \delta = min_{e\in P}(e's residual capacity in G_f)
    for all edges e of G whose corresponding forward edge is in P do
      increase f_e by \delta
    for all edges e of G whose corresponding reverse edge is in P do
      decrease f_e by \delta
```

### Termination
FF algo maintains that every flow amount f_e is an integer, *every iteration of Ford-Fulkerson algorithm increase the value of the current flow
by the current value of \delta， and \delta\ge 1 in every iteration*, since there is a finite amount of flow escape from the source, FF algo
would terminate in some iteration.

## Maximum-Flow/Minimum-Cut Theorem
Capacity of an `cut(A, B)` is defined as `C(A, B) = \sum_{e\in\delta^+(A)}u_e` and minimum cut is one with the smallest capacity.

**Theorem(Optimality Conditions for Max Flow):** Let `f` be a flow in a graph G. And we have equivalent:
1. `f` is a maximum flow of `G`
1. there is an `(s, t) - cut(A, B)` such that the value of `f` equals the capacity of `(A, B)`
1. there is no `s-t` path with positive residual capacity in the residual network `G_f`

- 2 implies 1:
  1. We first claim: `value of f \le C(A, B)` . `f = \sum{e\in\delta^+(s)}f_e = \sum{e\in\delta^+(s)}f_e - \sum{e\in\delta^-(s)}f_e`. And conservation constrains state: `\sum{e\in\delta^+(v)}f_e - \sum{e\in\delta^+(v)}f_e = 0`. Adding the second equation to all the 
  vertices of `A\{s}` to the first equation: `f = \sum_{v\in A}(\sum_{e\in\delta^+(v)}f_e - \sum_{e\in\delta^-(v)}f_e)`. And we consider this function with which category does the edge belong to: `A`, `B` or from `A` to `B`. Then we have: `f = \sum_{e\in\delta^-(A)}f_e - \sum_{e\in\delta^-(A)}f_e \le\sum_{e\in\delta^+(A)}u_e = C(A, B)`.
