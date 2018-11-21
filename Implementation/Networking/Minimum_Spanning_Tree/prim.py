"""Prim algorithm for MST problem
"""
import sys
import heapq
import math

def prim(graph, node_num):
    """
        Priority heap implementation of prim algorithm
    """
    total = 0
    mst = []
    dist = []
    pre = [-1]*node_num
    nodes_list = [[math.inf, x] for x in range(node_num)]
    for node in nodes_list:
        dist.append(node)
    nodes_list[0][0] = 0
    heapq.heapify(dist)
    while dist:
        node = heapq.heappop(dist)
        total += node[0]
        node_idx = node[1]
        mst.append((pre[node_idx], node_idx))
        for arc in graph:
            if node_idx in (arc[1], arc[2]):
                update_node = arc[1]+arc[2]-node_idx
                if arc[0] < nodes_list[update_node][0]:
                    pre[update_node] = node_idx
                    nodes_list[update_node][0] = arc[0]
        heapq.heapify(dist)

    return mst, total

def main():
    """Test for kruskal algo
        Read in a file for graph
        compute its mst with kruskal
        and print out mst with total length
    """
    # Read in the test cases in form:
    #   V E _
    #   arc_s arc_d arc_weight
    #   ...
    #   ...
    file_name = sys.argv[1]

    with open(file_name) as file:
        lines = file.readlines()
        # As we need to sort arcs with its weight
        # The simpliest way is to save graph in the form
        # (arc_w, arc_s, arc_d), and we could sort it natually
        node_num, _, _ = map(int, lines[0].split())
        graph = []

        for line in lines[1:]:
            source, end, weight = map(int, line.split())
            graph.append((weight, source, end))

        mst, total = prim(graph, node_num)
        print("MST with total weight {}".format(total))
        print(mst)


if __name__ == "__main__":
    main()
