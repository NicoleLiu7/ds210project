type Vertex = usize;
type Edge = (Vertex, Vertex);
type AdjacencyLists = Vec<Vec<Vertex>>;
use crate::graph::Graph;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn added_directed_edges(&mut self, edges: &[Edge]) {
        for &(u, v) in edges.iter().filter(|(u, _)| *u < self.n) {
            self.outedges[u].push(v);
        }
    }
    pub fn sort_graph_lists(&mut self) {
        for list in self.outedges.iter_mut() {
            list.sort_unstable();
        }
    }
    pub fn create_directed(n: usize, edges: &[Edge]) -> Graph {
        let mut g = Graph { n, outedges: vec![vec![]; n]};
        g.added_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}