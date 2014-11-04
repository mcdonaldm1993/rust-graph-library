#![allow(dead_code)]
#![allow(unused_variable)]
extern crate collections;
extern crate core;

use std::vec::Vec;
use std::int;
use std::collections::HashMap;
use collections::hash::Hash;
use core::cmp::Eq;

#[deriving(Clone)]
pub struct AdjListGraph<I, D> {
    vertices: HashMap<I, Vertex<I, D>>,
}

#[deriving(Clone)]
struct Vertex<I, D> {
    id: I,
    data: D,
    neighbours: Vec<AdjListNode<I>>,
}

#[deriving(Clone)]
struct AdjListNode<I> {
    vertex_id: I,
    weight: int
}

#[deriving(Clone)]
pub struct KCoreDecomposition<I> {
    k: int,
    vertices: Vec<I>
}

struct MetadataDijsktra<I> {
    predecessor: Option<I>,
    visited: bool,
    distance: int
}

pub trait Graph<I: Eq + Hash + Clone, D> {
    fn add_vertex(&mut self, vertex_id: I, data: D) -> ();
    
    fn add_edge(&mut self, start_vertex: I, end_vertex: I) -> ();
    
    fn add_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> ();
    
    fn add_undirected_edge(&mut self, start_vertex: I, end_vertex: I) -> ();
    
    fn add_undirected_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> ();
    
    fn get_vertex_ids(&self) -> Vec<I>;
    
    fn get_vertex_data(& self, vertex_id: &I) -> Option<& D>;
    
    fn get_vertex_neighbours(& self, vertex_id: &I) -> Vec<I>;
    
    fn get_edge_weight(& self, start_vertex: &I, end_vertex: &I) -> int;
    
    fn is_adjacent(& self, start_vertex: &I, end_vertex: &I) -> bool;
    
    fn is_id_in_graph(& self, vertex_id: &I) -> bool;
    
    // fn breadth_first_search(& self) -> () {
    //     // add code here
    // }

    // fn depth_first_search(& self) -> () {
    //     // add code here
    // }

    fn breadth_first_search_from_index(& self, start_vertex: I) -> () {
        // add code here
    }

    fn depth_first_search_from_index(& self, start_vertex: I) -> () {
        // add code here
    }

    fn dijkstras_shortest_path(& self, start_vertex: &I, target_vertex: &I) -> Vec<I> {
        if !self.is_id_in_graph(start_vertex) || !self.is_id_in_graph(target_vertex) {
            return Vec::new();
        }
        
        let mut metadata: HashMap<I, MetadataDijsktra<I>> = HashMap::new();
        let mut vertices: Vec<I> = self.get_vertex_ids();
        
        for id in vertices.iter() {
            let id_val = (*id).clone(); // CHECK THIS SHOULD AUTO DEREFERENCE!!!
            metadata.insert(id_val, MetadataDijsktra {
                predecessor: None,
                visited: false,
                distance: int::MAX
            });
        }
        match metadata.find_mut(start_vertex) {
            Some(ref mut x) => x.distance = 0,
            None => ()
        }
        
        let mut min_id: I = start_vertex.clone();
        while !vertices.is_empty() {
            if metadata.find(target_vertex).unwrap().visited {
                break;
            }
            
            let mut min = int::MAX;
            for id in vertices.iter() {
                if metadata.find(id).unwrap().distance <= min {
                    min = metadata.find(id).unwrap().distance;
                    min_id = id.clone();
                }
            }
            
            metadata.find_mut(&min_id).unwrap().visited = true;
            for i in range(0, vertices.len()) {
                if vertices[i] == min_id {
                    vertices.remove(i);
                    break;
                }
            }
            
            for id in self.get_vertex_neighbours(&min_id).iter() {
                if !metadata.find(id).unwrap().visited {
                    if metadata.find(id).unwrap().distance > (metadata.find(&min_id).unwrap().distance+self.get_edge_weight(&min_id, id)) {
                        metadata.find_mut(id).unwrap().distance = metadata.find(&min_id).unwrap().distance+self.get_edge_weight(&min_id, id);
                        metadata.find_mut(id).unwrap().predecessor = Some(min_id.clone());
                    }
                }
            }
        }
        
        let mut result: Vec<I> = Vec::new();
        let mut last: &I = target_vertex;
        while *last != *start_vertex {
            result.insert(0, last.clone());
            last = metadata.find(last).unwrap().predecessor.as_ref().unwrap();
        }
        
        result.insert(0, start_vertex.clone());
        result
    }

    fn dijkstras_shortest_paths(& self, start_vertex: I) -> HashMap<I, Vec<I>> {
        // add code here
        HashMap::new()
    }

    fn graph_diameter_path(& self) -> Vec<I> {
        // add code here
        Vec::new()
    }

    fn graph_diameter_length(& self) -> int {
        // add code here
        0
    }

    fn k_core_decomposition(& self) -> Vec<KCoreDecomposition<I>> {
        // add code here
        Vec::new()
    }

    fn kruskal_min_spanning_tree(& self) -> AdjListGraph<I, D> {
        // add code here
        AdjListGraph {
            vertices: HashMap::new()
        }
    }

    fn prims_min_spanning_tree(& self) -> AdjListGraph<I, D> {
        // add code here
        AdjListGraph {
            vertices: HashMap::new()
        }
    }

    fn a_star_search(& self, start_vertex: I, target_vertex: I) -> Vec<I> {
        // add code here
        Vec::new()
    }
}

impl<I> AdjListNode<I> {
    pub fn new(vertex_id: I) -> AdjListNode<I> {
        AdjListNode {
            vertex_id: vertex_id,
            weight: 0
        }
    }
    
    pub fn new_with_weight(vertex_id: I, weight: int) -> AdjListNode<I> {
        AdjListNode {
            vertex_id: vertex_id,
            weight: weight
        }
    }
}

impl<I, D> Vertex<I, D> {
    pub fn new(id: I, data: D) -> Vertex<I, D> {
        Vertex {
            id: id,
            data: data,
            neighbours: Vec::new(),
        }
    }
}

impl<I: Eq + Hash + Clone, D> AdjListGraph<I, D> {
    pub fn new() -> AdjListGraph<I, D> {
        AdjListGraph {
            vertices: HashMap::new()
        }
    }
    
    pub fn new_with_capacity(capactiy: uint) -> AdjListGraph<I, D> {
        AdjListGraph {
            vertices: HashMap::with_capacity(capactiy)
        }
    }
    
}

impl<I: Eq + Hash + Clone, D> Graph<I, D> for AdjListGraph<I, D> {
    fn add_vertex(&mut self, vertex_id: I, data: D) -> () {
        let vertex = Vertex::new(vertex_id.clone(), data);
        self.vertices.insert(vertex_id, vertex);
    }
    
    fn add_edge(&mut self, start_vertex: I, end_vertex: I) -> () {
        let vertex = self.vertices.find_mut(&start_vertex);
        let adj_list_node = AdjListNode::new(end_vertex);
        
        match vertex {
            Some(v) => v.neighbours.push(adj_list_node),
            None => ()
        };
    }
    
    fn add_undirected_edge(&mut self, start_vertex: I, end_vertex: I) -> () {
        self.add_edge(start_vertex.clone(), end_vertex.clone());
        self.add_edge(end_vertex, start_vertex);
    }
    
    fn add_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> () {
        let vertex = self.vertices.find_mut(&start_vertex);
        let adj_list_node = AdjListNode::new_with_weight(end_vertex, weight);
        
        match vertex {
            Some(v) => v.neighbours.push(adj_list_node),
            None => ()
        };
    }
    
    fn add_undirected_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> () {
        self.add_edge_with_weight(start_vertex.clone(), end_vertex.clone(), weight);
        self.add_edge_with_weight(end_vertex.clone(), start_vertex.clone(), weight);
    }
    
    fn get_vertex_ids(& self) -> Vec<I>{
        let mut vector: Vec<I> = Vec::new();
        for key in self.vertices.keys() {
            vector.push(key.clone());
        }
        vector
    }
    
    fn get_vertex_data(& self, vertex_id: &I) -> Option<& D>{
        let vertex = self.vertices.find(vertex_id);
        
        match vertex {
            Some(v) => Some(& v.data),
            None => None
        }
    }

    fn get_vertex_neighbours(& self, vertex_id: &I) -> Vec<I> {
        let vertex = self.vertices.find(vertex_id);
        let empty_list = Vec::new();
        
        let neighbour_nodes = match vertex {
            Some(v) => & v.neighbours,
            None => & empty_list
        };
        
        let mut neighbour_ids: Vec<I> = Vec::new();
        for node in neighbour_nodes.iter() {
            neighbour_ids.push(node.vertex_id.clone())
        }
        
        neighbour_ids
    }
    
    fn get_edge_weight(& self, start_vertex: &I, end_vertex: &I) -> int {
        let vertex = self.vertices.find(start_vertex);
        let empty_list = Vec::new();
        let mut result = -1;
        
        let adj_list_nodes = match vertex {
            Some(v) => & v.neighbours,
            None => & empty_list
        };
        
        for node in adj_list_nodes.iter() {
            if node.vertex_id == *end_vertex {
                result = node.weight;
                break;
            }
        }
        
        result
    }
    
    fn is_adjacent(& self, start_vertex: &I, end_vertex: &I) -> bool {
        let vertex = self.vertices.find(start_vertex);
        let empty_list = Vec::new();
        let mut result = false;
        
        let adj_list_nodes = match vertex {
            Some(v) => & v.neighbours,
            None => & empty_list
        };
        
        for node in adj_list_nodes.iter() {
            if node.vertex_id == *end_vertex {
                result = true;
                break;
            }
        }
        
        result
    }
    
    fn is_id_in_graph(& self, vertex_id: &I) -> bool {
        match self.vertices.find(vertex_id) {
            Some(_) => true,
            None => false
        }
    }
    
}