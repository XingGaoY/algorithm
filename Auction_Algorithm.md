# Auction Algorithm
## Problem
`n` persons want to buy `n` cars to be sold by auction. Each is interested in subset `A(i)` cars and has nonnegative `u_{ij}` for car `j` 
for each `(i,j)\in A(i)`. Objective is to **find assignment with maximum utility**.

## Algorithm
Set `c_{ij} = -u_{ij}` and let `C = max{|u_{ij}|:(i,j)\in A}`. At each stage of algorithm, **asking price fo car `price(j)`**, and marginal 
utility of person `i` for buying car `j` is `u_{ij}-price(j)`. At each iteration, an unassigned person bids on a car that has the highest 
marginal utility for that person.

Each person is associated with a number `value(i)`, the upper bound on that person's highest marginal utility: `value(i)\ge max{u_{ij}-price(j): (i, j)\in A(i)}`.
And a bid `(i, j)` admissible if `value(i) = u_{ij} - price(j)`. And algorithm **requires every bid in the auction to be admissible.** And 
if `i` in next turn has no damissible bid, `value(i)` is decreased to `max{u_ij-price(j): (i,j)\in A(i)}`.

Procedure starts with some valid choices of `value(i)` and `price(j)`
```python
def BIDDING(u, x, value, price):
  let initial assignment null
  while some person is unassigned:
    select an unassigned person i
    if bid(i, j) admissible:
      assign person i to car j
      price(j) += 1
      if person k already assigned to car j:
        unassigned k
    else:
      update value(i) = max{u_{ij}-price(j): (i, j) in A(i)}
  let x be the current assignment
```

## Complexity
Complexity is `O(n^2mC)` time. (Proof in AMO p.160)

It could be very slow as Auction increase price in small amount, and final price could be large. Using scaling technique to make the iteration 
less.

## Scaling Version
First multiplies all utilities by `(n+1)` and then solves a sequence of `K = ceiling(log(n+1)C)` assignment problems `P_k` witch is an assignment 
problem in which the utility of `a_{ij}` is the k leading bits in the binary representation of `u_{ij}`.
```python
def Assignment():
  u_{ij} *= n+1
  K = ceiling(log(n+1)*C)
  price = [0] * n
  value = [0] * n
  for k in range(K):
    for (i, j) in A:
      u[k](i,j) = floor(u_{ij}/1<<K-k)
    # update car price and person value
    for i in range(n):
      price[i] *= 2
      value[i] = 2*value[i]+1
    BIDDING(u[k], x, value, price)
```
And the scaling version has complexity `O(nmlog(nC))`
