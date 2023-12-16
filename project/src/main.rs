//import modules
mod loadedges;
mod graph;
mod bfs;
use bfs::{average_distance, read_txt};
use std::collections::HashMap;

//count dist of degrees
fn count_degrees(result: Vec<i32>) -> Vec<(usize, usize)> {
    let mut count = HashMap::new();

    for degree in result {
        if degree != -1 {
            *count.entry(degree as usize).or_insert(0) += 1;
        }
    }
    let mut count_vec: Vec<_> = count.into_iter().collect();
    count_vec.sort_by_key(|k| k.0);
}

fn main() {
    let facebook_edges = match load_edges::load_edges("facebook_combined.txt") {
        Ok(edges) => edges,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    let facebook_graph = graph::Graph::create_directed(4039, &facebook_edges);

    let result: Vec<_> = (0..4039).map(|i| bfs::average_distance(i, &facebook_graph)).collect();
    println!("There are a total of {} edges, {} nodes", facebook_edges.len(), facebook_graph.node_count());
    println!("The degree distribution is {:?}", count_degrees(result));

    let minimum = facebook_graph.outedges.iter().map(|l| l.len()).min().unwrap();
    let maximum = facebook_graph.outedges.iter().map(|l| l.len()).max().unwrap();

    println!("The minimum value of connections is {}", minimum);
    println!("The maximum value of connections is {}", maximum);
}

//tests
fn load_edges_test() {
    let edges = load_edges("test_edges.txt").unwrap();
    assert_eq!(edges, vec![(0,1),(1,2),(2,3)]);
    let edges = load_edges("empty_file.txt");
    assert!(edges.is_err());

}

fn directed_edges_test() {
    let edges = vec![(0,3),(1,5),(2,3),(4,7)];
    let graph = graph::Graph::create_directed(7, &edges);
    let count_edges: Vec<_> = graph.outedges.iter().map(|j| j.len()).collect();

    assert_eq!(count_edges, vec![1, 1, 1, 0, 1, 0, 0])
}