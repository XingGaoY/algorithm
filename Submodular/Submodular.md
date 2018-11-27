# Submodular
**Define:** For a set function `f:2^V->R, S\belongs V` and `e\in V`, let:  
`\delta_e(S) = f(S\cup{e}) - f(S)` be the discrete derivative of `f` at `S` with respect to `e`.

And a function is called submodular if, for every `A\belongs B\belongs V` and `e\in V - B`, it holds that:  
`\delta_e(A)\ge\delta_e(B)`, and equivalently,  
`f(A)+f(B)\ge f(A\cap B)+f(A\cup B)`
