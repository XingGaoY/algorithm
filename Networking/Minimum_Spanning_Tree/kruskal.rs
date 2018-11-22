use std::env;
use std::fs;
use std::cmp;

struct Arc {
    src: usize,
    dest: usize,
    weight: u32,
}

fn kruskal(graph: &Vec<Arc>, mst: &mut Vec<Arc>, node_num: usize) -> u32 {
    let mut total = 0;
    let mut parent = vec![0; node_num];
    let mut cluster = vec![0; node_num];
    
    //let mut find_root = |node: usize| -> u32 {cluster[parent[node]]};

    let mut cluster_no = 1;

    for arc in graph.iter(){
        if cluster[parent[arc.src]]*cluster[parent[arc.dest]] != 0
            && cluster[parent[arc.src]] == cluster[parent[arc.dest]] {
            continue;
        }

        if cluster[parent[arc.src]]+cluster[parent[arc.dest]] == 0 {
            parent[arc.src] = cluster_no;
            parent[arc.dest] = cluster_no;
            cluster[cluster_no] = cluster_no;
            cluster_no += 1;
        } else if cluster[parent[arc.src]] * cluster[parent[arc.dest]] == 0 {
            let min_p= cmp::max(parent[arc.src], parent[arc.dest]);
            parent[arc.src] = min_p;
            parent[arc.dest] = min_p;
        } else {
            let min_no = cmp::min(cluster[parent[arc.src]], cluster[parent[arc.dest]]);
            cluster[parent[arc.src]] = min_no;
            cluster[parent[arc.dest]] = min_no;
        }

        total += arc.weight;
        let mst_arc = Arc {src: arc.src, dest: arc.dest, weight: arc.weight};
        mst.push(mst_arc);
    }

    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut graph: Vec<Arc> = Vec::new();
    let mut mst: Vec<Arc> = Vec::new();

    let f = fs::read_to_string(filename).expect("file not found");
    let mut file: Vec<_> = f.lines().collect();
    let info: Vec<_> = file[0].split_whitespace().collect();
    let node_num = info[0].parse::<usize>().unwrap();
    file.remove(0);

    for line in file {
        let info: Vec<_> = line.split_whitespace().collect();
        let new_arc = Arc {
            src: info[0].parse::<usize>().unwrap(),
            dest: info[1].parse::<usize>().unwrap(),
            weight: info[2].parse::<u32>().unwrap(),
        };
        graph.push(new_arc);
    }

    graph.sort_by(|a, b| a.weight.cmp(&b.weight));

    let total = kruskal(&graph,&mut mst, node_num);

    println!("MST with total weight: {}", total);
    for arc in mst.iter() {
        println!("({}, {}, {})", arc.src, arc.dest, arc.weight);
    }
}
