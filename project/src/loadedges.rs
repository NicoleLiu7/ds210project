use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

// reads edge, returns vector of tuple pair
pub fn load_edges<P: AsRef<Path>>(filepath: P) -> io::Result<Vec<(usize, usize)>> {
    let file = File::open(filepath)?;
    let bufread = BufReader::new(file);
    let mut edge = Vec::new();

    for line in bufread.lines() {
        let x_line = line?;
        let nodes: Vec<&str> = x_line.split_whitespace().collect();

        if nodes.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Line does not contain only 2 vertices"));
        }

        let node_a=nodes[0].parse::<usize>().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse the first vertex"))?;
        let node_b=nodes[1].parse::<usize>().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse the second vertex"))?;
        edge.push((node_a, node_b));
    }
    Ok(edge)
}