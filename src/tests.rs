mod node;
mod graph;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_distance() {
        let nodes = vec![
            Node { id: 1, edges: vec![2, 3, 4] },
            Node { id: 2, edges: vec![1, 3] },
            Node { id: 3, edges: vec![1, 2, 4] },
            Node { id: 4, edges: vec![1, 3] },
        ];
        let graph = Graph { nodes };
        let average_distance = graph.average_distance();
        assert_eq!(average_distance, 1.6);
    }
}