extern crate graph;

use graph::Graph;
use graph::Edge;
use graph::graphs::UndirectedAdjacencyListGraph;
use graph::graphs::WeightedEdge;
use graph::graphs::UnweightedEdge;
use graph::GraphPath;
use std::collections::HashMap;

#[test]
fn test_add_node () {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    
    assert!(graph.is_node_in_graph(&1)); 
}

#[test]
fn test_add_edge () {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    
    graph.add_edge(1, 2, 0);
    
    assert!(graph.is_adjacent(&1, &2));
}

#[test]
fn test_add_edge_with_weight () {
    let mut graph: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    
    graph.add_edge(1, 2, 10);
    
    assert!(graph.is_adjacent(&1, &2));
    assert_eq!(graph.get_edge(&1, &2).unwrap().get_weight(), 10);
}

#[test]
fn test_get_degree () {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    
    
    assert_eq!(graph.degree(&1).unwrap(), 2);
    assert_eq!(graph.degree(&2).unwrap(), 1)
}

#[test]
fn test_graph_diameter_unweighted_graph() {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(3, 4, 0);
    
    let diameter: GraphPath<i32> = graph.diameter_path().unwrap();
    
    assert_eq!(diameter.get_distance(), 3);
    assert!(diameter.get_path().contains(&2));
    assert!(diameter.get_path().contains(&1));
    assert!(diameter.get_path().contains(&3));
    assert!(diameter.get_path().contains(&4));
    assert_eq!(diameter.get_path().len(), 4);
}

#[test]
fn test_graph_diameter_weighted_graph() {
    let mut graph: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 5);
    graph.add_edge(1, 3, 10);
    graph.add_edge(3, 4, 3);
    graph.add_edge(2, 3, 6);
    
    let diameter: GraphPath<i32> = graph.diameter_path().unwrap();
    
    assert_eq!(diameter.get_distance(), 13);
    assert!(diameter.get_path().contains(&1));
    assert!(diameter.get_path().contains(&3));
    assert!(diameter.get_path().contains(&4));
    assert_eq!(diameter.get_path().len(), 3);
}

#[test]
fn test_dijsktras_shortest_path_unweighted_graph() {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(3, 4, 0);
    
    let shortest_path = graph.dijkstras_shortest_path(&2, &3).unwrap();
    
    assert_eq!(shortest_path.get_distance(), 2);
    assert!(shortest_path.get_path().contains(&2));
    assert!(shortest_path.get_path().contains(&1));
    assert!(shortest_path.get_path().contains(&3));
    assert_eq!(shortest_path.get_path().len(), 3);
}

#[test]
fn test_dijsktras_shortest_path_weighted_graph() {
    let mut graph: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 5);
    graph.add_edge(1, 3, 10);
    graph.add_edge(3, 4, 3);
    graph.add_edge(2, 3, 6);
    
    let shortest_path = graph.dijkstras_shortest_path(&2, &3).unwrap();
    
    assert_eq!(shortest_path.get_distance(), 6);
    assert!(shortest_path.get_path().contains(&2));
    assert!(shortest_path.get_path().contains(&3));
    assert_eq!(shortest_path.get_path().len(), 2);
}

#[test]
fn test_dijsktras_shortest_paths_unweighted_graph() {
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(3, 4, 0);
    
    let shortest_paths = graph.dijkstras_shortest_paths(&1).unwrap();
    
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
    let mut graph: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    
    graph.add_edge(1, 2, 5);
    graph.add_edge(1, 3, 10);
    graph.add_edge(3, 4, 3);
    graph.add_edge(2, 3, 6);
    
    let shortest_paths = graph.dijkstras_shortest_paths(&1).unwrap();
    
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
    let mut graph: UndirectedAdjacencyListGraph<i32, UnweightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    graph.add_node(5);
    
    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(2, 3, 0);
    graph.add_edge(2, 5, 0);
    
    let k_core_decomposition: HashMap<u32, Vec<i32>> = graph.k_core_decomposition();
    
    assert_eq!(k_core_decomposition.get(&0).unwrap().len(), 1);
    assert!(k_core_decomposition.get(&0).unwrap().contains(&4));
    
    assert_eq!(k_core_decomposition.get(&1).unwrap().len(), 1);
    assert!(k_core_decomposition.get(&1).unwrap().contains(&5));
    
    assert_eq!(k_core_decomposition.get(&2).unwrap().len(), 3);
    assert!(k_core_decomposition.get(&2).unwrap().contains(&1));
    assert!(k_core_decomposition.get(&2).unwrap().contains(&2));
    assert!(k_core_decomposition.get(&2).unwrap().contains(&3));
}

#[test]
fn test_kruskal_min_spanning_tree() {
    let mut graph: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = Graph::new();
    
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);
    graph.add_node(5);
    
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 5, 1);
    graph.add_edge(2, 3, 5);
    graph.add_edge(2, 5, 4);
    graph.add_edge(3, 4, 2);
    graph.add_edge(3, 5, 6);
    graph.add_edge(4, 5, 7);
    
    let mut mst: UndirectedAdjacencyListGraph<i32, WeightedEdge<i32>> = graph.kruskal_min_spanning_tree();
    
    assert!(mst.is_node_in_graph(&1));
    assert!(mst.is_node_in_graph(&2));
    assert!(mst.is_node_in_graph(&3));
    assert!(mst.is_node_in_graph(&4));
    assert!(mst.is_node_in_graph(&5));
    
    assert!(mst.get_edge(&1, &2).is_ok());
    assert!(mst.get_edge(&1, &5).is_ok());
    assert!(mst.get_edge(&2, &3).is_ok());
    assert!(mst.get_edge(&3, &4).is_ok());
    
    assert!(mst.get_edge(&2, &5).is_err());
    assert!(mst.get_edge(&3, &5).is_err());
    assert!(mst.get_edge(&4, &5).is_err());
}