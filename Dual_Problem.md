# Dual Problem
## Concept
Every linuear program has associated with it a related linear program called its **dual**. And the original problem in relation to its dual
is termed the **primal**.

And we have:
**Primal Problem**:  
```
z* := max_x{Z} = cx
s.t.
  Ax\le b
  x\ge 0
```
**Dual Problem**:
```
w* := min_x{w} = yb
s.t.
  yA\ge c
  y\ge 0
```

## Duality in LP
In LP models, scarce resources are allocated, so they should be valued.

Whenever we solve an LP problem, we solve two problems: the **primal resource allocation** problem,
and the **dual resource valuation** problem. 

## General Rules
- Number of variables in dual problem is equal to the number of constraints in the primal problem; The number 
of constraints in the dual problem is equal to the number of variables in the original problem.
- Coefficient of the objective function in the dual problem come from the right-hand side of the original problem.
- If the original problem is a max model, the dual is a min model. So as reverse.
- The dual of the dual problem is again the primal problem.
- Either of the two problems has an optimal solution if and only if the other does; if one problem is feasible but 
unbounded, then the other is infeasible; if one is infeasible, then the other is either infeasible or feasible/unbounded.
- **Weak Duality Theorem**:Let x and y be any feasible solution to PLP and DLP respectively. Then c^Tx\le b^Ty. (Any feasible 
solution to PLP has a value no greater then that of any feasible solution to the dual linear problem, which means, DLP provides 
upper bound to the solution of PLP in DLP maximization case). i.e. maximum flow vs. 
minimum cut.
- **Strong Duality Theorem**: same feasible and finite optimum result if exist c^Tx* = b^Ty*.

## Ref
-[Slideshare: primal and dual problem](https://www.slideshare.net/YashLad3/primal-and-dual-problem)
