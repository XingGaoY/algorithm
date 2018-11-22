extern crate priority_queue;

use std::env;
use std::fs;
use std::u32;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;

struct Arc {
    src: usize,
    dest: usize,
    weight: u32,
}

fn prim(graph: &Vec<Arc>, mst: &mut Vec<Arc>, node_num: usize) -> u32 {
    let mut total = 0;
    let mut dist_mst = PriorityQueue::new();
    let mut pre = vec![0; node_num];

    for x in 0..node_num {
        dist_mst.push(x, Reverse(u32::max_value()));
    }
    dist_mst.change_priority(&0, Reverse(0));
    
    while !dist_mst.is_empty() {
        let node = dist_mst.pop().unwrap();
        let node_idx = node.0;
        let priority = node.1;
        total += priority.0;
        let mst_node = Arc {src : pre[node_idx], dest : node_idx, weight : priority.0};
        mst.push(mst_node);

        for arc in graph.iter() {
            if arc.src == node_idx || arc.dest == node_idx {
                let update_node = arc.src + arc.dest - node_idx;
                let p = dist_mst.get_priority(&update_node).cloned();
                match p {
                    Some(priority) => {
                        if arc.weight < priority.0 {
                            pre[update_node] = node_idx;
                            dist_mst.change_priority(&update_node, Reverse(arc.weight));
                        }
                    }
                    None => {
                    }
                }
            }
        }
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

    let total = prim(&graph,&mut mst, node_num);

    println!("MST with total weight: {}", total);
    for arc in mst.iter() {
        println!("({}, {}, {})", arc.src, arc.dest, arc.weight);
    }
}
