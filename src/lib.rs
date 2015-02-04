extern crate disjoint_set;

use disjoint_set::DisjointSet;

use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Hasher;
use std::hash::Hash;
use std::i32;
use std::cmp::Eq;


pub mod graphs;

/// A struct used to store data used during the execution of Dijkstra's algorithm
#[derive(Clone)]
struct MetadataDijsktra<N> {
    predecessor: Option<N>,
    visited: bool,
    distance: i32
}

/// A struct used to store data used during the execution of the K core decomposition algorithm
#[derive(Clone)]
struct MetadataKCore<N> {
    id: N,
    degree: u32,
    core: u32
}



/// The `Graph` trait is used to implement common operations on a graph and provide implementations of graph algorithms
/// that use these operations so that concrete types of graphs can be implemented and the algorithms used on them.
pub trait Graph<N, E>
    where N: Eq + Clone + Hash<Hasher>,
          E: Eq + Clone + Hash<Hasher> + Edge<N>
{
    /// Creates a new instance of the graph.
    fn new() -> Self;
    
    /// The method to add a node to the graph.
    fn add_node(&mut self, node_id: N) -> ();
    
    /// The method to add an edge to the graph between two nodes, specifying a weight.
    fn add_edge(&mut self, source: N, destination: N, weight: i32) -> ();
    
    /// The method to return a vector of IDs of all nodes in the graph.
    fn get_nodes(&self) -> Vec<N>;
    
    /// The method to get the list of nodes that are adjacent to a node.
    fn get_node_neighbours(& self, node_id: &N) -> Vec<N>;
    
    /// The method to get edge between two nodes.
    fn get_edge(& self, source: &N, destination: &N) -> Result<E, String>;
    
    /// The method to get the edge set in the graph.
    fn get_edges(&self) -> Vec<E>;
    
    /// The method to check if two nodes are adjacent.
    fn is_adjacent(& self, source: &N, destination: &N) -> bool;
    
    /// The method to check if a node is in the graph.
    fn is_node_in_graph(& self, node: &N) -> bool;
    
    /// The method to return the degree of a node.
    fn degree(& self, node: &N) -> Result<u32, String>;

    /// Performs Dijkstra's shortest path algorithm on the graph.
    ///
    /// Returns the `GraphPath` between the two vertices and will end prematurely once the path has been found.
    /// Returns an error string if an error occured during the algorithm execution.
    /// 
    /// This algorithm runs in worst case O(V<sup>2</sup>) time.
    fn dijkstras_shortest_path(& self, source: &N, destination: &N) -> Result<GraphPath<N>, String> where Self: Sized {
        if !self.is_node_in_graph(source) || !self.is_node_in_graph(destination) {
            return Err(String::from_str("The start or target node does not exist in the graph."));
        }
        
        let mut metadata: HashMap<N, MetadataDijsktra<N>>;
        let mut nodes: Vec<N> = self.get_nodes();
        
        metadata = try!(create_dijkstra_metadata(&nodes, source));
        
        let mut min_id: N;
        while !nodes.is_empty() {
            match metadata.get(destination) {
                Some(ref x) => {
                    if x.visited {
                        break;
                    }
                },
                None => return Err(String::from_str("There was an error retrieving metadata about the target node."))
            }
            
            min_id = try!(get_min_distance(&nodes, &metadata));
            
            match metadata.get_mut(&min_id) {
                Some(ref mut x) => x.visited = true,
                None => return Err(String::from_str("There was an error retrieving metadata about a node."))
            }

            remove_from_list(&mut nodes, &min_id);
            
            try!(perform_edge_relaxation(self, &mut metadata, &min_id));
        }
        
        backtrack_vertex_predecessor(&metadata, source, destination)
    }
    
    /// Performs Dijkstra's shortest path algorithm on the graph.
    ///
    /// Returns a `HashMap` of target vertices to the `GraphPath` between the vertex and the target vertex.
    /// The `HashMap` will be empty if an error occured.
    /// 
    /// This algorithm runs in worst case O(V<sup>2</sup>) time.
    fn dijkstras_shortest_paths(& self, source: &N) -> Result<HashMap<N, GraphPath<N>>, String> where Self: Sized {
        if !self.is_node_in_graph(source) {
            return Err(String::from_str("The start node does not exist in the graph."));
        }
        
        let mut metadata: HashMap<N, MetadataDijsktra<N>>;
        let mut nodes: Vec<N> = self.get_nodes();
        let nodes_copy: Vec<N> = nodes.clone();
        
        metadata = try!(create_dijkstra_metadata(&nodes, source));
        
        let mut min_id: N;
        while !nodes.is_empty() {
            min_id = try!(get_min_distance(&nodes, &metadata));
            
            match metadata.get_mut(&min_id) {
                Some(ref mut x) => x.visited = true,
                None => return Err(String::from_str("There was an error retrieving metadata about a node."))
            }

            remove_from_list(&mut nodes, &min_id);
            
            try!(perform_edge_relaxation(self, &mut metadata, &min_id));
        }
        
        let mut result: HashMap<N, GraphPath<N>> = HashMap::new();
        for id in nodes_copy.iter() {
            result.insert(id.clone(), 
                match backtrack_vertex_predecessor(&metadata, source, id) {
                    Ok(x) => x,
                    Err(_) => GraphPath::new()
            });
        }
        
        Ok(result)
    }

    /// Finds the diameter of the graph.
    ///
    /// Returns a `GraphPath` of the path that determined the diameter of the graph.
    ///
    /// This uses the dijkstras_shortest_paths function to get all shortest paths pairs and find the longest.
    /// This algorithm runs in O(V<sup>3</sup>) time.
    fn diameter_path(& self) -> Result<GraphPath<N>, String> where Self: Sized {
        let nodes: Vec<N> = self.get_nodes();
        let mut longest_path: GraphPath<N> = GraphPath::new();
        let mut longest_distance = i32::MIN;

        for id in nodes.iter() {
            let longest_paths = try!(self.dijkstras_shortest_paths(id));
            for path in longest_paths.values() {
                if path.get_distance() > longest_distance {
                    longest_path = path.clone();
                    longest_distance = path.get_distance();
                }
            }
        }
        
        Ok(longest_path)
    }

    /// Finds the k core of each vertex in the graph.
    ///
    /// Returns a `HashMap` with the core as a key and a vector of all vertex IDs in that core as a value.
    ///
    /// This algorithm runs in O(E) time.
    fn k_core_decomposition(& self) -> HashMap<u32, Vec<N>> where Self: Sized {
        let mut buckets: HashMap<i32, HashSet<N>> = HashMap::new();
        let mut metadata: HashMap<N, MetadataKCore<N>> = HashMap::new();
        let mut result: HashMap<u32, Vec<N>> = HashMap::new();
        let mut max_degree: i32 = 0;
        let mut current_core: i32 = 0;
        
        for v in self.get_nodes().iter() {
            let degree = match self.degree(v) {
                Ok(d) => d,
                Err(_) => 0
            } as i32;
            
            if !buckets.contains_key(&degree) {
                buckets.insert(degree, HashSet::new());
            }
            
            buckets.get_mut(&degree).unwrap().insert(v.clone());
            
            metadata.insert(v.clone(), MetadataKCore {
                id: v.clone(),
                degree: degree as u32,
                core: 0
            });
            
            if degree > max_degree { max_degree = degree };            
        }
        
        loop {
            let mut v: N;
            
            match get_next_vertex(&mut buckets, &mut current_core) {
                Ok(vertex) => v = vertex,
                Err(_) => if current_core == max_degree { break; } else { continue; }
            }
            
            let mut v_vertex_meta = metadata.get(&v).cloned().unwrap();
            v_vertex_meta.core = v_vertex_meta.degree;
            
            if !result.contains_key(&v_vertex_meta.core) {
                result.insert( v_vertex_meta.core, Vec::new());
            }
            
            match result.get_mut(&v_vertex_meta.core) {
                None => (),
                Some(x) => x.push(v.clone())
            }
            
            let v_degree = v_vertex_meta.degree;

            for u in self.get_node_neighbours(&v).iter() {
                let mut u_vertex_meta = metadata.get(u).cloned().unwrap();
                
                if u_vertex_meta.degree > v_degree {
                    {
                        match buckets.get_mut(&(u_vertex_meta.degree as i32)) {
                            Some(bucket) => { bucket.remove(u); },
                            None => { continue; }
                        }
                    }
                    
                    u_vertex_meta.degree = u_vertex_meta.degree-1;
                    
                    {
                        if buckets.contains_key(&(u_vertex_meta.degree as i32)) {
                            buckets.get_mut(&(u_vertex_meta.degree as i32)).unwrap().insert(u.clone());
                        } else {
                            let mut hashset = HashSet::new();
                            hashset.insert(u.clone());
                            buckets.insert(u_vertex_meta.degree as i32, hashset);
                        }
                    }
                    
                    metadata.insert(u.clone(), u_vertex_meta);
                }
            }
        }
        
        result
    }

    /// Creates a minimum spanning tree of the graph using Kruskal's algorithm.
    ///
    /// Returns the minimum spanning tree in the same graph implementation that is used.
    ///
    /// This algorithm runs in O(Eα(V)) time, where α is the extremely slowly growing inverse of the single-valued Ackermann function.
    fn kruskal_min_spanning_tree(& self) -> Self where Self: Sized {
        let mut node_set: DisjointSet<N> = DisjointSet::new();
        let mut mst_edges = Vec::new();
        let mut buckets: HashMap<i32, HashSet<E>> = HashMap::new();
        
        let mut current_bucket: i32 = 0;
        let mut max_bucket: i32 = i32::MIN;
        let n = self.get_nodes().len()-1;
        
        for e in self.get_edges().iter() {
            let weight = e.get_weight();
            
            if !buckets.contains_key(&weight) {
                buckets.insert(weight, HashSet::new());
            }
            
            buckets.get_mut(&weight).unwrap().insert(e.clone());
            
            if weight > max_bucket { max_bucket = weight };            
        }
        
        for n in self.get_nodes().iter() {
            node_set.make_set(n.clone());
        }
        
        loop {
            if mst_edges.len() == n {
                break;
            }
            
            let mut e: E;
            
            match get_next_vertex(&mut buckets, &mut current_bucket) {
                Ok(edge) => e = edge,
                Err(_) => if current_bucket == max_bucket { break; } else { continue; }
            }
            
            if node_set.find(e.get_target()) != node_set.find(e.get_source()) {
                mst_edges.push(e.clone());
                node_set.union(e.get_target(), e.get_source());
            }
        }
        
        let mut mst: Self = Graph::new();
        
        for n in self.get_nodes().iter() {
            mst.add_node(n.clone());
        }
        
        for e in mst_edges.iter() {
            mst.add_edge(e.get_source(), e.get_target(), e.get_weight());
        }
        
        return mst;
    }
}



pub trait Edge<N> {
    fn new(source: N, target: N, weight: i32, directed: bool) -> Self;
    
    fn get_weight(&self) -> i32;
    
    fn get_source(&self) -> N;
    
    fn get_target(&self) -> N;
    
    fn is_directed(&self) -> bool;
}



/// A struct used to represent a path in a graph.
///
/// The struct contains the path of vertex IDs and the distance of the path.
#[derive(Show, Clone)]
pub struct GraphPath<N> {
    distance: i32,
    path: Vec<N>
}

impl<N> GraphPath<N> {
    fn new() -> GraphPath<N> {
        GraphPath {
            distance: 0,
            path: Vec::new()
        }
    }
    
    fn set_distance(&mut self, distance: i32) -> () {
        self.distance = distance;
    }
    
    fn set_path(&mut self, path: Vec<N>) -> () {
        self.path = path;
    }
    
    /// Retrieves the distance of the `GraphPath`
    pub fn get_distance(& self) -> i32 {
        self.distance
    }
    
    /// Retrieves the path.
    ///
    /// This is a vector of vertex IDs that are in order of visitation.
    pub fn get_path(& self) -> &Vec<N> {
        &self.path
    }
}



////////////////////////////////////////////////////////////////////////////////
// Private functions used in the graph trait provided functions
////////////////////////////////////////////////////////////////////////////////

fn create_dijkstra_metadata<N>(vertices: &Vec<N>, start_vertex: &N) -> Result<HashMap<N, MetadataDijsktra<N>>, String> 
    where N: Eq + Clone + Hash<Hasher>
{
        let mut metadata: HashMap<N, MetadataDijsktra<N>> = HashMap::new();
                
        for id in vertices.iter() {
            let id_val = id.clone();
            metadata.insert(id_val, MetadataDijsktra {
                predecessor: None,
                visited: false,
                distance: i32::MAX
            });
        }
        
        match metadata.get_mut(start_vertex) {
            Some(ref mut x) => x.distance = 0,
            None => return Err(String::from_str("An error occured while initialising the metadata."))
        }
        
        Ok(metadata)
}

fn get_min_distance<N>(vertices: &Vec<N>, metadata: &HashMap<N, MetadataDijsktra<N>>) -> Result<N, String> 
 where N: Eq + Clone + Hash<Hasher>
{
    let mut min = i32::MAX;
    let mut min_id = vertices[0].clone();
    
    for id in vertices.iter() {
        let id_meta;
        match metadata.get(id) {
            Some(x) => id_meta = x,
            None => return Err(String::from_str("An error occured while attempting to find a minimum distance."))
        }
        
        if id_meta.distance <= min {
            min = id_meta.distance;
            min_id = id.clone();
        }
    }
    
    Ok(min_id)
}

fn remove_from_list<N>(vertices: &mut Vec<N>, id: &N) -> () 
    where N: Eq
{
    for i in 0..vertices.len() {
        if vertices[i] == *id {
            vertices.remove(i);
            break;
        }
    }
}

fn perform_edge_relaxation<N, E, G>(graph: &G, metadata: &mut HashMap<N, MetadataDijsktra<N>>, min_id: &N) -> Result<(), String> 
    where N: Eq + Clone + Hash<Hasher>,
          E: Eq + Clone + Hash<Hasher> + Edge<N>,
          G: Graph<N, E>
{
    for id in graph.get_node_neighbours(min_id).iter() {
        let min_id_meta;
        match metadata.get(min_id).cloned() {
            Some(x) => min_id_meta = x,
            None => return Err(String::from_str("An error occured while performing edge relaxation"))
        }
        
        let mut id_meta;
        match metadata.get_mut(id) {
            Some(x) => id_meta = x,
            None => return Err(String::from_str("An error occured while performing edge relaxation"))
        }
        
        if !id_meta.visited {
            let length = min_id_meta.distance + try!(graph.get_edge(min_id, id)).get_weight();
            if id_meta.distance > length {
                id_meta.distance = length;
                id_meta.predecessor = Some(min_id.clone());
            }
        }
    }
    
    Ok(())
}

fn backtrack_vertex_predecessor<N>(metadata: &HashMap<N, MetadataDijsktra<N>>, start_vertex: &N, target_vertex: &N) -> Result<GraphPath<N>, String>
    where N: Eq + Clone + Hash<Hasher>
{
    let mut result: GraphPath<N> = GraphPath::new();
    
    match metadata.get(target_vertex) {
        Some(ref x) => result.set_distance(x.distance),
        None => return Err(String::from_str("An error occured while backtracking from a vertex"))
    }
    
    let mut path: Vec<N> = Vec::new();
    let mut last: &N = target_vertex;
    
    while *last != *start_vertex {
        path.insert(0, last.clone());
        match metadata.get(last) {
            Some(ref x) =>  
                match x.predecessor {
                    Some(ref y) => last = y,
                    None => return Err(String::from_str("An error occured while backtracking from a vertex"))
                },
            None => return Err(String::from_str("An error occured while backtracking from a vertex"))
        }
    }
    
    path.insert(0, start_vertex.clone());   
    
    result.set_path(path);
    
    Ok(result)
}

fn get_next_vertex<N>(buckets: &mut HashMap<i32, HashSet<N>>, current_core: &mut i32) -> Result<N, ()> 
    where N: Eq + Clone + Hash<Hasher>
{
    let current_core_bucket;
    
    match buckets.get_mut(current_core) {
        Some(bucket) => { current_core_bucket = bucket; },
        None => { *current_core += 1; return Err(()); }
    }
    
    if !current_core_bucket.is_empty() { 
        let mut v = None;
        
        for x in current_core_bucket.iter() {
            v = Some(x.clone());
            break;
        }
        
        match v {
            Some (vector) => { current_core_bucket.remove(&vector); return Ok(vector); },
            None =>{ return Err(()); }
        }
    } else {
        *current_core += 1;
        Err(())
    }
}