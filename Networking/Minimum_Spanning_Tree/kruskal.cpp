#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

using namespace std;

struct arc {
    int src;
    int dest;
    int weight;

    arc(int src, int dest, int weight): src(src), dest(dest), weight(weight) {
    }
};

int kruskal(const vector<arc> graph, vector<arc> &mst, int node_num) {
    int total = 0, clust_no = 1;
    int *parent = new int[node_num]{};
    int *cluster = new int[node_num];
    fill_n(cluster, node_num, -1);
    cluster[0] = 0;

    auto find_root = [parent, cluster](int node)->int {
        return cluster[parent[node]];
    };

    for (arc a : graph) {
        if (find_root(a.src)*find_root(a.dest) != 0
            && find_root(a.src) == find_root(a.dest))
            continue;

        if (find_root(a.src)+find_root(a.dest) == 0) {
            parent[a.src] = parent[a.dest] = clust_no;
            cluster[clust_no] = clust_no;
            clust_no++;
        } else if (find_root(a.src)*find_root(a.dest) == 0) {
            parent[a.src] = parent[a.dest] = max(parent[a.src], parent[a.dest]);
        } else {
            int min_no = min(find_root(a.src), find_root(a.dest));
            cluster[parent[a.src]] = cluster[parent[a.dest]] = min_no;
        }

        total += a.weight;
        mst.push_back(a);
    }

    return total;
}

int main(int argc, char** argv) {
    ifstream in(argv[1]);
    vector<arc> graph;
    vector<arc> mst;

    int node_num, arc_num, dummy;
    int src, dest, weight;

    in >> node_num >> arc_num >> dummy;
    while (in >> src >> dest >> weight)
        graph.emplace_back(src, dest, weight);

    sort(graph.begin(), graph.end(),
        [](const arc& a1, const arc& a2)->bool {
            return a1.weight < a2.weight;
        });

    int total = kruskal(graph, mst, node_num);

    cout << "Total weight: " << total << endl;
    for (arc a : mst)
        cout << "{" << a.src << ", " << a.dest << ", " << a.weight << "}";
    cout << endl;

    return(0);
}
