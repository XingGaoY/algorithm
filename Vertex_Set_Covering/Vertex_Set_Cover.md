# Vertex Covering
We have an undirected graph `G = (N, A)` and a cost function `c: N->R^+`. The problem of **minimum cost vertex cover** is an NP-hard problem to find a set of nodes such that each arc of the graph is incident to at least one node in the set.

The size of a maximal matching(in terms of number of arcs) is a lower bound to the min-cardinality vertex cover. *Every edge touches two nodes, one of which must be in any Vertex Cover as the edge has to be covered.*

Then we would have: A maximum cardinality matching algorithm gives a vertex cover with no more than twice the number of nodes required by the minimum vertex cover. And it is tight.

Thus we have the theorem: **Any maximal matching is a factor 2 approximation algorithm for the minimum cardinality vertex cover.**

## degree weighted
Let the cost of each node is degree times a constant `K`, and we have `Sum(degree(n)) = 2|A|`, and the cost of any vertex cover is no more than twice of the minimum cost vertex cover.

## general weighted
define `c = min{w(v)/deg(v)}` then `t(v) = c*deg(v)`.

Then we have the algorithm:
```python
vc = []
A = [arc_1, arc_2, ...]
N = [node_1, node_2, ...]
w = [w_1, w_2, ...]
while A:
    N.remove(x for x in N with deg(x) = 0)
    c = min(w(v)/deg(v))
    for v in N:
        t(v) = c * deg(v)
        w_r(v) = w - t(v)
        for n in N with w_r(a)=0:
            vc.append(n)
            remove all arc incident to n from A
```
And it is a 2-factor approximation algorithm.

# Set Covering
Given a set `u` and a family `s` of subsets of `u`, a cover is a subfamiliy `c\belongs s` of sets whose union is `u`. Assume we are given a non-negative cost function `c: s->R^+`. The goal is to find a cover of minimal cost.

## Greedy algorithm
Let `A` be a set of covered members.
Define: For each set `s`, `s/in S && not /in A`:
```
    \gamma(s) = \frac{c(s)}{|s-A|} = price(e)
```
Thus, we add the set with minimal average unadded price.
```python
A = set()
sgamma = S[0].gamma()
sadd = s
while A != S:
    for s in S[1:]:
        if s.gamma() < sgamma
        s_add = s
        A |= s
```
And we could get the proof of OPT in [note](http://www.cs.huji.ac.il/course/2005/algo2/scribes/lecture2.pdf), while the aproximation relation of the greedy is `H_n=1+1/2+...+1/n\le ln n+1`
