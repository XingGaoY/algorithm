"""
Implementation of kruskal and learning PEP8
"""
import sys

def kruskal(graph, node_num):
    """Kruskal:
        Add arcs in increasing order of weight as long as they do not form a cycle
    """
    graph = sorted(graph)
    parent = [0]*node_num
    cluster = [-1]*node_num
    
    def find_root(node):
        """find root of cluster in O(1)"""
        return cluster[parent[node]]

    total = 0
    disjoint_set = 1
    mst = []

    # initiat all nodes to tree 0
    # indicate it has not been visited
    # where find_root() returns 0
    cluster[0] = 0

    for arc in graph:
        weight, source, dest = arc
        # not 0 and same root
        if find_root(source)*find_root(dest) != 0 and find_root(source) == find_root(dest):
            continue

        if find_root(source)+find_root(dest) == 0:
            # not in any cluster either
            # create a new cluster
            parent[source] = disjoint_set
            parent[dest] = disjoint_set
            if cluster[disjoint_set] == -1:
                # cluster hasn't been used
                cluster[disjoint_set] = disjoint_set
                disjoint_set += 1
        elif find_root(source)*find_root(dest) == 0:
            # one of them is in a cluster
            # set its root to the cluster
            parent[dest] = parent[source] = max(parent[dest], parent[source])
        else:   # different cluster
            min_no = min(find_root(dest), find_root(source))
            cluster[parent[source]] = cluster[parent[dest]] = min_no

        total += weight
        mst.append([source, dest, weight])

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

    mst, total = kruskal(graph, node_num)
    print("MST with total weight {}".format(total))
    print(mst)

if __name__ == "__main__":
    main()
