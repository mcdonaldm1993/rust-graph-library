use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;

use Edge;
use Graph;

/// An undirected graph represented by an adjacency list.
#[derive(Clone)]
pub struct UndirectedAdjacencyListGraph<N, E> {
    vertices: HashMap<N, Vec<Rc<E>>>,
    edges: Vec<Rc<E>>,
}

impl<N, E> Graph<N, E> for UndirectedAdjacencyListGraph<N, E> 
    where N: Eq + Clone + Hash,
          E: Eq + Clone + Hash + Edge<N>
{
    fn new() -> UndirectedAdjacencyListGraph<N, E> {
        UndirectedAdjacencyListGraph {
            vertices: HashMap::new(),
            edges: Vec::new()
        }
    }
    
    fn add_node(&mut self, vertex_id: N) -> () {
        self.vertices.insert(vertex_id, Vec::new());
    }
    
    fn add_edge(&mut self, source: N, destination: N, weight: i32) -> () {
        let edge: Rc<E> = Rc::new(Edge::new(source.clone(), destination.clone(), weight, false));
    
        {
            match self.vertices.get_mut(&source) {
                Some(v) => { v.push(edge.clone()); },
                None => ()
            };
        }
        
        let reverse_edge: Rc<E> = Rc::new(Edge::new(destination.clone(), source.clone(), weight, false));

        {
            match self.vertices.get_mut(&destination) {
                Some(v) => { v.push(reverse_edge.clone()); },
                None => ()
            };
        }
        
        self.edges.push(edge);
    }
    
    fn get_nodes(&self) -> Vec<N> {
        self.vertices.keys().map(|x| x.clone()).collect()
    }

    fn get_node_neighbours(& self, node_id: &N) -> Vec<N> {
        let node = self.vertices.get(node_id);
        let mut neighbours = Vec::new();
        
        match node {
            Some(n) => {
                for e in n.iter() {
                    neighbours.push(e.clone().get_target());
                }
                
                neighbours
            },
            None => neighbours
        }
    }
    
    fn get_edge(& self, source: &N, destination: &N) -> Result<E, String> {
        let node = self.vertices.get(source);
        
        match node {
            Some(n) => {
                let mut edge = None;
                for e in n.iter() {
                    if e.clone().get_target() == *destination {
                        edge = Some(e.clone());
                    }
                }
                
                match edge {
                    Some(e) => { return Ok( (*e).clone()); },
                    None => { return Err("No edge exists between the provided vertices.".to_string()); }
                }
            },
            None => { return Err("Node does not exists in graph.".to_string()); }
        }
    }
    
    fn get_edges(&self) -> Vec<E> {
        let mut result = Vec::new();
        
        for edge in self.edges.iter() {
            result.push( (*(*edge)).clone() );
        }
        
        result
    }
    
    fn is_adjacent(& self, source: &N, destination: &N) -> bool {
        let node = self.vertices.get(source);
        
        match node {
            Some(n) => {
                for e in n.iter() {
                    if e.clone().get_target() == *destination {
                        return true;
                    }
                }
            },
            None => ()
        }
        
        return false;
    }
    
    fn is_node_in_graph(& self, node: &N) -> bool {
        match self.vertices.get(node) {
            Some(_) => true,
            None => false
        }
    }
    
    fn degree(& self, node: &N) -> Result<u32, String> {
        let vertex = self.vertices.get(node);
        
        match vertex {
            Some(v) => Ok(v.len() as u32),
            None => Err("An error occured while getting the vertex degree.".to_string())
        }
    }
}