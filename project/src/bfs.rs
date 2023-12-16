use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::graph::Graph;

pub fn read_txt(file_name: &str) -> HashMap<i32, Vec<i32>> {
    let file_x = File::open(file_name).unwrap();
    let reader = BufReader::new(file_x);

    //empty HashMap
    let mut graph = HashMap::new();
    for line in reader.lines() {
        let edges = line.unwrap();
        let nodes: Vec<i32> = edges.split(',').map(|n| n.parse().unwrap()).collect();
        if nodes.len() != 2 {
            panic!("Invalid input file format: each line must contain exactly two nodes");
        }
        graph.entry(nodes[0]).or_insert(Vec::new()).push(nodes[1]);
        graph.entry(nodes[1]).or_insert(Vec::new()).push(nodes[0]);
    }
    graph
}

pub fn average_distance(start: usize, graph: &Graph) -> i32 {
    let mut total_distance = vec![None; graph.n];
    total_distance[start] = Some(0);
    let mut node_queue = VecDeque::new();
    node_queue.push_back(start);

    let mut added_distance = 0u32;
    let mut total_nodes=0u32;

    while let Some(current_node) = node_queue.pop_front() {
        if let Some(current_distance) = total_distance[current_node] {
            for &adjacent_node in &graph.outedges[current_node] {
                if total_distance[adjacent_node].is_none() {
                    total_distance[adjacent_node] = Some(current_distance + 1);
                    node_queue.push_back(adjacent_node);
                    added_distance += current_distance + 1;
                    total_nodes += 1;
                }
            }
        }
    }
    if total_nodes == 0 {
        -1
    } else {
        (added_distance as i32) / (total_nodes as i32)
    }
}
