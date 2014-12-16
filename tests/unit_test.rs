extern crate graph;

use graph::Graph;
use graph::UndirectedAdjListGraph;
use graph::GraphPath;
use std::collections::HashMap;

#[test]
fn test_add_vertex () {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    
    assert!(graph.is_id_in_graph(&1)); 
}

#[test]
fn test_add_edge () {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    
    graph.add_edge(1, 2);
    
    assert!(graph.is_adjacent(&1, &2));
}

#[test]
fn test_add_edge_with_weight () {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    
    graph.add_edge_with_weight(1, 2, 10);
    
    assert!(graph.is_adjacent(&1, &2));
    assert_eq!(graph.get_edge_weight(&1, &2), 10);
}

#[test]
fn test_get_vertex_degree () {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    
    
    assert_eq!(graph.vertex_degree(&1).unwrap(), 2);
    assert_eq!(graph.vertex_degree(&2).unwrap(), 1)
}

#[test]
fn test_graph_diameter_unweighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(3, 4);
    
    let diameter: GraphPath<int> = graph.diameter_path();
    
    assert_eq!(diameter.get_distance(), 3);
    assert!(diameter.get_path().contains(&2));
    assert!(diameter.get_path().contains(&1));
    assert!(diameter.get_path().contains(&3));
    assert!(diameter.get_path().contains(&4));
    assert_eq!(diameter.get_path().len(), 4);
}

#[test]
fn test_graph_diameter_weighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge_with_weight(1, 2, 5);
    graph.add_edge_with_weight(1, 3, 10);
    graph.add_edge_with_weight(3, 4, 3);
    graph.add_edge_with_weight(2, 3, 6);
    
    let diameter: GraphPath<int> = graph.diameter_path();
    
    assert_eq!(diameter.get_distance(), 13);
    assert!(diameter.get_path().contains(&1));
    assert!(diameter.get_path().contains(&3));
    assert!(diameter.get_path().contains(&4));
    assert_eq!(diameter.get_path().len(), 3);
}

#[test]
fn test_dijsktras_shortest_path_unweighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(3, 4);
    
    let shortest_path = graph.dijkstras_shortest_path(&2, &3);
    
    assert_eq!(shortest_path.get_distance(), 2);
    assert!(shortest_path.get_path().contains(&2));
    assert!(shortest_path.get_path().contains(&1));
    assert!(shortest_path.get_path().contains(&3));
    assert_eq!(shortest_path.get_path().len(), 3);
}

#[test]
fn test_dijsktras_shortest_path_weighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge_with_weight(1, 2, 5);
    graph.add_edge_with_weight(1, 3, 10);
    graph.add_edge_with_weight(3, 4, 3);
    graph.add_edge_with_weight(2, 3, 6);
    
    let shortest_path = graph.dijkstras_shortest_path(&2, &3);
    
    assert_eq!(shortest_path.get_distance(), 6);
    assert!(shortest_path.get_path().contains(&2));
    assert!(shortest_path.get_path().contains(&3));
    assert_eq!(shortest_path.get_path().len(), 2);
}

#[test]
fn test_dijsktras_shortest_paths_unweighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(3, 4);
    
    let shortest_paths = graph.dijkstras_shortest_paths(&1);
    
    assert_eq!(shortest_paths.get(&1).unwrap().get_distance(), 0);
    assert!(shortest_paths.get(&1).unwrap().get_path().contains(&1));
    assert_eq!(shortest_paths.get(&1).unwrap().get_path().len(), 1);
    
    assert_eq!(shortest_paths.get(&2).unwrap().get_distance(), 1);
    assert!(shortest_paths.get(&2).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&2).unwrap().get_path().contains(&2));
    assert_eq!(shortest_paths.get(&2).unwrap().get_path().len(), 2);
    
    assert_eq!(shortest_paths.get(&3).unwrap().get_distance(), 1);
    assert!(shortest_paths.get(&3).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&3).unwrap().get_path().contains(&3));
    assert_eq!(shortest_paths.get(&3).unwrap().get_path().len(), 2);
    
    assert_eq!(shortest_paths.get(&4).unwrap().get_distance(), 2);
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&3));
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&4));
    assert_eq!(shortest_paths.get(&4).unwrap().get_path().len(), 3);
}

#[test]
fn test_dijsktras_shortest_paths_weighted_graph() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    
    graph.add_edge_with_weight(1, 2, 5);
    graph.add_edge_with_weight(1, 3, 10);
    graph.add_edge_with_weight(3, 4, 3);
    graph.add_edge_with_weight(2, 3, 6);
    
    let shortest_paths = graph.dijkstras_shortest_paths(&1);
    
    assert_eq!(shortest_paths.get(&1).unwrap().get_distance(), 0);
    assert!(shortest_paths.get(&1).unwrap().get_path().contains(&1));
    assert_eq!(shortest_paths.get(&1).unwrap().get_path().len(), 1);
    
    assert_eq!(shortest_paths.get(&2).unwrap().get_distance(), 5);
    assert!(shortest_paths.get(&2).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&2).unwrap().get_path().contains(&2));
    assert_eq!(shortest_paths.get(&2).unwrap().get_path().len(), 2);
    
    assert_eq!(shortest_paths.get(&3).unwrap().get_distance(), 10);
    assert!(shortest_paths.get(&3).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&3).unwrap().get_path().contains(&3));
    assert_eq!(shortest_paths.get(&3).unwrap().get_path().len(), 2);
    
    assert_eq!(shortest_paths.get(&4).unwrap().get_distance(), 13);
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&1));
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&3));
    assert!(shortest_paths.get(&4).unwrap().get_path().contains(&4));
    assert_eq!(shortest_paths.get(&4).unwrap().get_path().len(), 3);
}

#[test]
fn test_k_core_decomposition() {
    let mut graph: UndirectedAdjListGraph<int, int> = UndirectedAdjListGraph::new();
    
    graph.add_vertex(1, 1);
    graph.add_vertex(2, 2);
    graph.add_vertex(3, 3);
    graph.add_vertex(4, 4);
    graph.add_vertex(5, 5);
    
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(2, 5);
    
    let k_core_decomposition: HashMap<uint, Vec<int>> = graph.k_core_decomposition();
    
    assert_eq!(k_core_decomposition.get(&0).unwrap().len(), 1);
    assert!(k_core_decomposition.get(&0).unwrap().contains(&4));
    
    assert_eq!(k_core_decomposition.get(&1).unwrap().len(), 1);
    assert!(k_core_decomposition.get(&1).unwrap().contains(&5));
    
    assert_eq!(k_core_decomposition.get(&2).unwrap().len(), 3);
    assert!(k_core_decomposition.get(&2).unwrap().contains(&1));
    assert!(k_core_decomposition.get(&2).unwrap().contains(&2));
    assert!(k_core_decomposition.get(&2).unwrap().contains(&3));
}