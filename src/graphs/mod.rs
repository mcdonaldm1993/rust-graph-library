//! This module contains default implementations for graphs and edges

pub use self::weighted_edge::WeightedEdge;
pub use self::unweighted_edge::UnweightedEdge;
pub use self::undirected_adjacency_list_graph::UndirectedAdjacencyListGraph;

mod weighted_edge;
mod unweighted_edge;
mod undirected_adjacency_list_graph;