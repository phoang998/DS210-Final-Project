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

        Ok(Graph { nodes })
    }

    // Calculate distances for all nodes
    fn calculate_distances(&self) -> Vec<f32> {
        let mut distances = Vec::new();

        for node in &self.nodes {
            for edge_id in &node.edges {
                if let Some(edge) = self.nodes.iter().find(|&n| n.id == *edge_id) {
                    let distance = (node.id as f32 - edge.id as f32).abs();
                    distances.push(distance);
                }
            }
        }

        distances
    }

    // Calculate the average distance 
    fn average_distance(&self) -> f32 {
        let distances = self.calculate_distances();
        let sum: f32 = distances.iter().sum();
        let count = distances.len() as f32;
        sum / count
    }

    // Calculate the summary statistics
    fn summary_statistics(&self) -> (f32, Vec<f32>) {
        let distances = self.calculate_distances();
    let max = *distances.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min = *distances.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let mut sorted_distances = distances.clone();
    sorted_distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted_distances.len() / 2;
    let q1 = if sorted_distances.len() % 2 == 0 {
        (sorted_distances[mid / 2 - 1] + sorted_distances[mid / 2]) as f32 / 2.0
    } else {
        sorted_distances[mid / 2] as f32
    };
    let q3 = if sorted_distances.len() % 2 == 0 {
        (sorted_distances[mid + mid / 2 - 1] + sorted_distances[mid + mid / 2]) as f32 / 2.0
    } else {
        sorted_distances[mid + mid / 2] as f32
    };
    let median = if sorted_distances.len() % 2 == 0 {
        (sorted_distances[mid - 1] + sorted_distances[mid]) as f32 / 2.0
    } else {
        sorted_distances[mid] as f32
    };
    let mean = distances.iter().sum::<f32>() / distances.len() as f32;
    let variance = distances.iter().map(|x| (*x - mean).powi(2)).sum::<f32>() / distances.len() as f32;
    let std_dev = variance.sqrt();
    (min, q1, median, q3, max, std_dev, distances)
    }
}

// Print results 
fn main () {
    // code
}