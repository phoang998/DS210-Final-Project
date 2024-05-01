mod node;
mod graph;

use std::path::Path;
use crate::graph::Graph;

fn main () {
    println!("Starting the program");

    // Read the graph from the file
    let graph = Graph::from_file(Path::new("roadNet-PA.txt")).expect("Failed to read the file");

    // Confirm the graph is being read 
    println!("Read the graph from the file");

    // Calculate the average distance
    let average_distance = graph.average_distance();

    // Print the average distance
    println!("Average Distance: {}", average_distance);

    // Calculate the degree distribution
    let distribution = graph.degree_distribution();

    // Print the degree distribution
    println!("Degree Distribution: {:?}", distribution);

    // Calculate the summary statistics
    let (min, q1, median, q3, max, std_dev, _distribution) = graph.summary_statistics();

    // Print the summary statistics
    println!("Min: {}, Q1: {}, Median: {}, Q3: {}, Max: {}, Standard Deviation: {}", min, q1, median, q3, max, std_dev);
}
