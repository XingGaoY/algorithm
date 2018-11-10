# Hungarian Algorithm
## Problem
Given a nonegative `n*n` matrix with `(i, j)` represents the cost of assigning job `j` to person `i`. The goal is to find an assignment with 
minimum cost. Or given a complete bipartite graph `G(S\cup T, E)` with `n` workers and `n` jobs, we need to find a perfect matching with 
minimum cost.

Define function `y:(S\cup T)\to R` a **potential** if `y(i)+y(j)\le c(i,j)`. And the cost of each perfect matching is at lest the value of 
each potential. And Hungarian method finds a perfect matching and a potential with equal cost/value which proves the optimality of both. In fact 
it finds a perfect matching of tight edges: an edge `ij` is called tight for a potential `y` if `y(i)+y(j)=c(i,j)`. Let us denote the subgraph 
of tight edges by `G_y`. The cost of a perfect matching in `G_y` (if there is one) equals the value of y.

## Algorithm
### Graph
Either modify `y` to increase its value, or modify orientation to get more edges in a match.  
And we keep all edges of M are tight.
```python
def Hungrarian():
  y = [0]*n*2
  # O represents orientation of G
  # 0 represents T2S
  # 1 represents S2T
  O = [(i, j, 1) for (i, j) in G]
  # RS and RT represent vertices not covered by M
  RS = set(x for x in range[n])
  RT = set(x for x in range[n])
  while sum(o for (i, j, o) in G) < n:
    BFS find Z which is reachable from RS in G
    if len(inters = Z.intersect(RT)) != 0:
      for _,_,o in G if [x[0],x[1]] == inter for x in G and inter in inters:
        o = 0
    else:
      delta = min(c[i][j] - y[i] - y[j] for i in range(n) and j in range(n))
      y[i] += delta for i in Z.intersect(RS)
      y[i] -= delta for i in Z.intersect(RT)
```
### Matrix
```python
def substract(direct):
  if direct == row:
    find lowest element of each row and substract it from its row
  if direct == col:
    find lowest element of each col and substract it from its col
 
def Hungrian():
    substract(row)
    if admisible:
      return
    else:
      substract(col)
      
    while matrix inempty
      mark all inadmisible rows
      mark all inadmisible col with 0 in it and inadmisible rows
      mark all rows with 0 in it and new inadmisible col
      
      mark out all marked col and unmarked row
      # Then all 0 is removed
      remove all marked out cols and row
      
      substract(row)
      check admisible
      substract(col)
      check admisible
```
