#include <iostream>
#include <fstream>
#include <vector>
#include <climits>
#include <algorithm>

using namespace std;

struct arc {
    int src;
    int dest;
    int weight;

    arc(int src, int dest, int weight): src(src), dest(dest), weight(weight) {
    }
};

struct dist {
    int node;
    int distance;

    dist(int node, int distance) : node(node), distance(distance) {
    }
};

class CompareDist {
public:
    bool operator()(const dist* a, const dist* b) const {
        return a->distance > b->distance;
    }
};

int prim(const vector<arc> graph, vector<arc> &mst, int node_num) {
    int total = 0, node_idx, update_node;
    vector<dist*> dist_mst;
    int *pre = new int[node_num];
    dist **node_list = new dist*[node_num];

    for (int i=0; i < node_num; i++) {
        node_list[i] = new dist(i, INT_MAX);
        dist_mst.push_back(node_list[i]);
    }
    node_list[0]->distance = 0;
    fill_n(pre, node_num, -1);

    while (!dist_mst.empty()) {
        dist* node = dist_mst.front();
        total += node->distance;
        node_idx = node->node;
        mst.emplace_back(pre[node_idx], node_idx, node->distance);
        for (arc a : graph) {
            if (a.src == node_idx || a.dest == node_idx) {
                update_node = a.src + a.dest - node_idx;
                if (a.weight < node_list[update_node]->distance) {
                    pre[update_node] = node_idx;
                    node_list[update_node]->distance = a.weight;
                }
            }
        }
        dist_mst.erase(dist_mst.begin());
        make_heap(dist_mst.begin(), dist_mst.end(), CompareDist());
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

    int total = prim(graph, mst, node_num);

    cout << "Total weight: " << total << endl;
    for (arc a : mst)
        cout << "{" << a.src << ", " << a.dest << ", " << a.weight << "}";

    cout << endl;

    return 0;
}
