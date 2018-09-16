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

