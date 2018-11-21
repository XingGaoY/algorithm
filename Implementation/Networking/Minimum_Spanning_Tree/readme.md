## kruskal
Need to track which subtree does a node in, and change all the node in the subtree;  
So if save it as a list of each node with the subtree no. leads to O(n^2).  
We let the nodes in a subtree direct to a root node, and set its no. Compare its no. to see if the root node in a same subtree, which is O(1).

### python
- `sorted()` automaticly sort the list of tuple/list with the first value
- indent needed for docstring

### cpp
- The way to write constructor:
```cpp
    struct A{
        type1 a;
        type2 b;

        A(type1 _a, type2 _b):a(_a), b(_b){}
    };
```
- value initialize:[cppref](https://en.cppreference.com/w/cpp/language/value_initialization)  
    - set to 0: `type *a = new type[]{}`
- `std::fill_n(array, num, number)`: set `array[0:num]` to `number`(memset)
- no local function in cpp, try lambda instead: [cppref](https://en.cppreference.com/w/cpp/language/lambda)
- no for each in dynamic array as no iterator
- could `push_back` non reference value argument to vector?
- use `>>` to format read split with space instead of `scanf`, jaja
```cpp
    ifstream infile(filename);
    while(infile>>a>>b)
        ...
```

### rust
- closure, need to know how to capture mutable
- ways to manipulate string
- **continue on rust-lang book**

## prim
Priority heap to track distance from nodes outside of mst.  

### python
- `heapq` implementation of priority queue
- `O(nlog(n))` for heappop, `O(mn)` for loop through all arcs for `n` times, and `O(n^2)` for heapify

## cpp
- `make_heap` is enough for implement a priority queue
    - Write a `bool operater()` for `make_heap`
- No parenthesis around type name of `new type[num]`
