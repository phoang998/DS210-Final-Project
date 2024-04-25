use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

#[derive(Debug)]
struct Node {
    id: u32,
    edges: Vec<u32>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    // Read the graph from the file
    fn from_file(file_path: &Path) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut nodes = Vec::new();
        let mut rng = rand::thread_rng();

        // Grab random sample of 30,000 lines
        let lines: Vec<_> = reader.lines().choose_multiple(&mut rng, 30000);

        for line in lines {
            let line = line?:

            // Ignore comments and header
            if line.starts_with('#') || line.contains("FromNodeId") {
                continue;
            }
        }

        for line in reader.lines() {
            let line = line?;
            if !line.starts_with('#') {
                let mut parts = line.split_whitespace();
                let from_node: u32 = parts.next().unwrap().parse().unwrap();
                let to_node: u32 = parts.next().unwrap().parse().unwrap();

                // Check if from_node is already in the list of nodes
                let from_index = match nodes.iter().position(|n| n.id == from_node) {
                    Some(index) => index,
                    None => {
                        nodes.push(Node { id: from_node, edges: Vec::new() });
                        nodes.len() - 1
                    }
                };

                // Add the edge to the from_node
                nodes[from_index].edges.push(to_node);

                // Check if to_node is already in the list of nodes
                if !nodes.iter().any(|n| n.id == to_node) {
                    nodes.push(Node { id: to_node, edges: Vec::new() });
                }
            }
        }

        Ok(Graph { nodes })
    }

    // Calculate distances for all nodes
    fn calculate_distances(&self) -> Vec<f32> {
        // code
    }

    // Calculate the average distance 
    fn average_distance(&self) -> f32 {
        // code
    }

    // Calculate the summary statistics
    fn summary_statistics(&self) -> (f32, f32, f32, Vec<f32>) {
        // code
    }

    // Plot the heatmap
    fn plot_heatmap(&self, heatmap_data: &[f32]) {
        // code
    }
}
