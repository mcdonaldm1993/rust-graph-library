extern crate core;

use std::vec::Vec;
use std::int;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use core::cmp::Eq;

/// A struct used to store data used during the execution of Dijkstra's algorithm
#[derive(Clone)]
struct MetadataDijsktra<I> {
    predecessor: Option<I>,
    visited: bool,
    distance: int
}

/// A struct used to store data used during the execution of the K core decomposition algorithm
#[derive(Clone)]
struct MetadataKCore<I> {
    id: I,
    degree: uint,
    core: uint
}

/// The `Graph` trait is used to implement common operations on a graph and provide implementations of graph algorithms
/// that use these operations so that concrete types of graphs can be implemented and the algorithms used on them.
pub trait Graph<I: Eq + Hash + Clone, D> {
    /// The method to add a vertex to the graph.
    fn add_vertex(&mut self, vertex_id: I, data: D) -> ();
    
    /// The method to add an edge to the graph between two vertices.
    fn add_edge(&mut self, start_vertex: I, end_vertex: I) -> ();
    
    /// The method to add an edge to the graph between two vertices, specifying a weight.
    fn add_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> ();
    
    /// The method to return a vector of IDs of all vertices in the graph.
    fn get_vertex_ids(&self) -> Vec<I>;
    
    /// The method to retrieve the data associated with a vertex.
    fn get_vertex_data(& self, vertex_id: &I) -> Option<& D>;
    
    /// The method to get the list of vertices that are adjacent to a vertex.
    fn get_vertex_neighbours(& self, vertex_id: &I) -> Vec<I>;
    
    /// The method to get the weight of an edge between two vertices.
    fn get_edge_weight(& self, start_vertex: &I, end_vertex: &I) -> int;
    
    /// The method to check if two vertices are adjacent.
    fn is_adjacent(& self, start_vertex: &I, end_vertex: &I) -> bool;
    
    /// The method to check if a vertex is in the graph.
    fn is_id_in_graph(& self, vertex_id: &I) -> bool;
    
    /// The method to return the degree of a vertex.
    fn vertex_degree(& self, vertex_id: &I) -> Result<uint, String>;

    /// Performs Dijkstra's shortest path algorithm on the graph.
    ///
    /// Returns the `GraphPath` between the two vertices and will end prematurely once the path has been found.
    /// The `GraphPath` will be empty if an error occured during the algorithm.
    /// 
    /// This algorithm runs in worst case O(V<sup>2</sup>) time.
    fn dijkstras_shortest_path(& self, start_vertex: &I, target_vertex: &I) -> GraphPath<I> {
        if !self.is_id_in_graph(start_vertex) || !self.is_id_in_graph(target_vertex) {
            return GraphPath::new();
        }
        
        let mut metadata: HashMap<I, MetadataDijsktra<I>>;
        let mut vertices: Vec<I> = self.get_vertex_ids();
        
        match create_dijkstra_metadata(&vertices, start_vertex) {
            Ok(x) => metadata = x,
            Err(e) => return GraphPath::new()
        }
        
        let mut min_id: I;
        while !vertices.is_empty() {
            match metadata.get(target_vertex) {
                Some(ref x) => {
                    if x.visited {
                        break;
                    }
                },
                None => return GraphPath::new()
            }
            
            match get_min_distance(&vertices, &metadata) {
                Ok(x) => min_id = x,
                Err(e) => return GraphPath::new()
            }
            
            match metadata.get_mut(&min_id) {
                Some(ref mut x) => x.visited = true,
                None => return GraphPath::new()
            }

            remove_from_list(&mut vertices, &min_id);
            
            match perform_edge_relaxation(self, &mut metadata, &min_id) {
                Ok(x) => (),
                Err(e) => return GraphPath::new() 
            }
        }
        
        match backtrack_vertex_predecessor(&metadata, start_vertex, target_vertex) {
            Ok(x) => x,
            Err(e) => GraphPath::new()
        }
    }
    
    /// Performs Dijkstra's shortest path algorithm on the graph.
    ///
    /// Returns a `HashMap` of target vertices to the `GraphPath` between the vertex and the target vertex.
    /// The `HashMap` will be empty if an error occured.
    /// 
    /// This algorithm runs in worst case O(V<sup>2</sup>) time.
    fn dijkstras_shortest_paths(& self, start_vertex: &I) -> HashMap<I, GraphPath<I>> {
        if !self.is_id_in_graph(start_vertex) {
            return HashMap::new();
        }
        
        let mut metadata: HashMap<I, MetadataDijsktra<I>>;
        let mut vertices: Vec<I> = self.get_vertex_ids();
        let vertices_copy: Vec<I> = vertices.clone();
        
        match create_dijkstra_metadata(&vertices, start_vertex) {
            Ok(x) => metadata = x,
            Err(e) => return HashMap::new()
        }
        
        let mut min_id: I;
        while !vertices.is_empty() {
            match get_min_distance(&vertices, &metadata) {
                Ok(x) => min_id = x,
                Err(e) => return HashMap::new()
            }
            
            match metadata.get_mut(&min_id) {
                Some(ref mut x) => x.visited = true,
                None => return HashMap::new()
            }

            remove_from_list(&mut vertices, &min_id);
            
            match perform_edge_relaxation(self, &mut metadata, &min_id) {
                Ok(x) => (),
                Err(e) => return HashMap::new() 
            }
        }
        
        let mut result: HashMap<I, GraphPath<I>> = HashMap::new();
        for id in vertices_copy.iter() {
            result.insert(id.clone(), 
                match backtrack_vertex_predecessor(&metadata, start_vertex, id) {
                    Ok(x) => x,
                    Err(e) => GraphPath::new()
            });
        }
        
        result
    }

    /// Finds the diameter of the graph.
    ///
    /// Returns a `GraphPath` of the path that determined the diameter of the graph.
    ///
    /// This uses the dijkstras_shortest_paths function to get all shortest paths pairs and find the longest.
    /// This algorithm runs in O(V<sup>3</sup>) time.
    fn diameter_path(& self) -> GraphPath<I>{
        let vertices: Vec<I> = self.get_vertex_ids();
        let mut longest_path: GraphPath<I> = GraphPath::new();
        let mut longest_distance = int::MIN;
        
        for id in vertices.iter() {
            let longest_paths = self.dijkstras_shortest_paths(id);
            for path in longest_paths.values() {
                if path.get_distance() > longest_distance {
                    longest_path = path.clone();
                    longest_distance = path.get_distance();
                }
            }
            
        }
        
        longest_path
    }

    /// Finds the k core of each vertex in the graph.
    ///
    /// Returns a `HashMap` with the core as a key and a vector of all vertex IDs in that core as a value.
    ///
    /// This algorithm runs in O(E) time.
    fn k_core_decomposition(& self) -> HashMap<uint, Vec<I>> {
        let mut buckets: HashMap<uint, HashSet<I>> = HashMap::new();
        let mut metadata: HashMap<I, MetadataKCore<I>> = HashMap::new();
        let mut result: HashMap<uint, Vec<I>> = HashMap::new();
        let mut max_degree: uint = 0;
        let mut current_core: uint = 0;
        
        for v in self.get_vertex_ids().iter() {
            let degree = match self.vertex_degree(v) {
                Ok(d) => d,
                Err(_) => 0
            };
            
            if !buckets.contains_key(&degree) {
                buckets.insert(degree, HashSet::new());
            }
            
            buckets.get_mut(&degree).unwrap().insert(v.clone());
            
            metadata.insert(v.clone(), MetadataKCore {
                id: v.clone(),
                degree: degree,
                core: 0
            });
            
            if degree > max_degree { max_degree = degree };            
        }
        
        loop {
            let mut v: I;
            
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

            for u in self.get_vertex_neighbours(&v).iter() {
                let mut u_vertex_meta = metadata.get(u).cloned().unwrap();
                
                if u_vertex_meta.degree > v_degree {
                    {
                        match buckets.get_mut(&u_vertex_meta.degree) {
                            Some(bucket) => { bucket.remove(u); },
                            None => { continue; }
                        }
                    }
                    
                    u_vertex_meta.degree = u_vertex_meta.degree-1;
                    
                    {
                        if buckets.contains_key(& (u_vertex_meta.degree)) {
                            buckets.get_mut(& (u_vertex_meta.degree-1)).unwrap().insert(u.clone());
                        } else {
                            let mut hashset = HashSet::new();
                            hashset.insert(u.clone());
                            buckets.insert(u_vertex_meta.degree, hashset);
                        }
                    }
                    
                    metadata.insert(u.clone(), u_vertex_meta);
                }
            }
        }
        
        result
    }

    // fn kruskal_min_spanning_tree(& self) -> UndirectedAdjListGraph<I, D> {
    //     // add code here
    //     UndirectedAdjListGraph {
    //         vertices: HashMap::new()
    //     }
    // }

    // fn prims_min_spanning_tree(& self) -> UndirectedAdjListGraph<I, D> {
    //     // add code here
    //     UndirectedAdjListGraph {
    //         vertices: HashMap::new()
    //     }
    // }

    // fn a_star_search(& self, start_vertex: I, target_vertex: I) -> Vec<I> {
    //     // add code here
    //     Vec::new()
    // }
}



/// A struct used to represent a path in a graph.
///
/// The struct contains the path of vertex IDs and the distance of the path.
#[derive(Show, Clone)]
pub struct GraphPath<I> {
    distance: int,
    path: Vec<I>
}

impl<I> GraphPath<I> {
    fn new() -> GraphPath<I> {
        GraphPath {
            distance: 0,
            path: Vec::new()
        }
    }
    
    fn set_distance(&mut self, distance: int) -> () {
        self.distance = distance;
    }
    
    fn set_path(&mut self, path: Vec<I>) -> () {
        self.path = path;
    }
    
    /// Retrieves the distance of the `GraphPath`
    pub fn get_distance(& self) -> int {
        self.distance
    }
    
    /// Retrieves the path.
    ///
    /// This is a vector of vertex IDs that are in order of visitation.
    pub fn get_path(& self) -> &Vec<I> {
        &self.path
    }
}



/// An undirected graph represented by an adjacency list.
/// 
/// The graph can be used both as a weighted or unweighted graph depending on which edge insertion operation is used.
#[derive(Clone)]
pub struct UndirectedAdjListGraph<I, D> {
    vertices: HashMap<I, Vertex<I, D>>,
}

impl<I: Eq + Hash + Clone, D> UndirectedAdjListGraph<I, D> {
    /// Creates a new emtpy `UndirectedAdjListGraph<I, D>`.
    pub fn new() -> UndirectedAdjListGraph<I, D> {
        UndirectedAdjListGraph {
            vertices: HashMap::new()
        }
    }
    
    /// Creates a new empty `UndirectedAdjListGraph<I, D>` with the given initial capacity.
    pub fn new_with_capacity(capactiy: uint) -> UndirectedAdjListGraph<I, D> {
        UndirectedAdjListGraph {
            vertices: HashMap::with_capacity(capactiy)
        }
    }
}

impl<I: Eq + Hash + Clone, D> Graph<I, D> for UndirectedAdjListGraph<I, D> {
    fn add_vertex(&mut self, vertex_id: I, data: D) -> () {
        let vertex = Vertex::new(vertex_id.clone(), data);
        self.vertices.insert(vertex_id, vertex);
    }
    
    /// Adds an edge to the graph between the two vertices.
    ///
    /// This is added with a weight of 1.
    /// Use only this if you want to use the graph as unweighted.
    fn add_edge(&mut self, start_vertex: I, end_vertex: I) -> () {
        {
            let start_vertex_ref = self.vertices.get_mut(&start_vertex);
            let end_vertex_adj_list_node = AdjListNode::new(end_vertex.clone());
            
            match start_vertex_ref {
                Some(v) => v.neighbours.push(end_vertex_adj_list_node),
                None => ()
            };
        }
        
        {
            let end_vertex_ref = self.vertices.get_mut(&end_vertex);
            let start_vertex_adj_list_node = AdjListNode::new(start_vertex);
            
            match end_vertex_ref {
                Some(v) => v.neighbours.push(start_vertex_adj_list_node),
                None => ()
            };
        }
    }
    
    /// Adds an edge to the graph between the two vertices.
    ///
    /// This is added with the weight specified.
    /// Use only this if you want to use the graph as weighted.
    fn add_edge_with_weight(&mut self, start_vertex: I, end_vertex: I, weight: int) -> () {
        {
            let start_vertex_ref = self.vertices.get_mut(&start_vertex);
            let end_vertex_adj_list_node = AdjListNode::new_with_weight(end_vertex.clone(), weight);
            
            match start_vertex_ref {
                Some(v) => v.neighbours.push(end_vertex_adj_list_node),
                None => ()
            };
        }
        
        {
            let end_vertex_ref = self.vertices.get_mut(&end_vertex);
            let start_vertex_adj_list_node = AdjListNode::new_with_weight(start_vertex, weight);
            
            match end_vertex_ref {
                Some(v) => v.neighbours.push(start_vertex_adj_list_node),
                None => ()
            };
        }
    }
    
    fn get_vertex_ids(& self) -> Vec<I>{
        let mut vector: Vec<I> = Vec::new();
        for key in self.vertices.keys() {
            vector.push(key.clone());
        }
        vector
    }
    
    fn get_vertex_data(& self, vertex_id: &I) -> Option<& D>{
        let vertex = self.vertices.get(vertex_id);
        
        match vertex {
            Some(v) => Some(& v.data),
            None => None
        }
    }

    fn get_vertex_neighbours(& self, vertex_id: &I) -> Vec<I> {
        let vertex = self.vertices.get(vertex_id);
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
        let vertex = self.vertices.get(start_vertex);
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
        let vertex = self.vertices.get(start_vertex);
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
        match self.vertices.get(vertex_id) {
            Some(_) => true,
            None => false
        }
    }
    
    fn vertex_degree(& self, vertex_id: &I) -> Result<uint, String> {
        let vertex = self.vertices.get(vertex_id);
        
        match vertex {
            Some(ref v) => Ok(v.vertex_degree()),
            None => Err(String::from_str("An error occured while getting the vertex degree"))
        }
    }
}



/// Struct used to store vertex data in the `UndirectedAdjListGraph`
#[derive(Clone)]
struct Vertex<I, D> {
    id: I,
    data: D,
    neighbours: Vec<AdjListNode<I>>,
}

impl<I, D> Vertex<I, D> {
    pub fn new(id: I, data: D) -> Vertex<I, D> {
        Vertex {
            id: id,
            data: data,
            neighbours: Vec::new(),
        }
    }
    
    pub fn vertex_degree(& self) -> uint {
        self.neighbours.len()
    }
}



// Struct used to store a vertex ID and weight associated with it in the adjacency list of the `UndirectedAdjListGraph`
#[derive(Clone)]
struct AdjListNode<I> {
    vertex_id: I,
    weight: int
}

impl<I> AdjListNode<I> {
    pub fn new(vertex_id: I) -> AdjListNode<I> {
        AdjListNode {
            vertex_id: vertex_id,
            weight: 1
        }
    }
    
    pub fn new_with_weight(vertex_id: I, weight: int) -> AdjListNode<I> {
        AdjListNode {
            vertex_id: vertex_id,
            weight: weight
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// Private functions used in the graph trait provided functions
////////////////////////////////////////////////////////////////////////////////

fn create_dijkstra_metadata<I: Eq + Hash + Clone>(vertices: &Vec<I>, start_vertex: &I) -> Result<HashMap<I, MetadataDijsktra<I>>, String> {
        let mut metadata: HashMap<I, MetadataDijsktra<I>> = HashMap::new();
                
        for id in vertices.iter() {
            let id_val = id.clone();
            metadata.insert(id_val, MetadataDijsktra {
                predecessor: None,
                visited: false,
                distance: int::MAX
            });
        }
        
        match metadata.get_mut(start_vertex) {
            Some(ref mut x) => x.distance = 0,
            None => return Err(String::from_str("An error occured while initialising the metadata"))
        }
        
        Ok(metadata)
}

fn get_min_distance<I: Eq + Hash + Clone>(vertices: &Vec<I>, metadata: &HashMap<I, MetadataDijsktra<I>>) -> Result<I, String> {
    let mut min = int::MAX;
    let mut min_id = vertices[0].clone();
    
    for id in vertices.iter() {
        let id_meta;
        match metadata.get(id) {
            Some(x) => id_meta = x,
            None => return Err(String::from_str("An error occured while attempting to find a minimum distance"))
        }
        
        if id_meta.distance <= min {
            min = id_meta.distance;
            min_id = id.clone();
        }
    }
    
    Ok(min_id)
}

fn remove_from_list<I: Eq>(vertices: &mut Vec<I>, id: &I) -> () {
    for i in range(0, vertices.len()) {
        if vertices[i] == *id {
            vertices.remove(i);
            break;
        }
    }
}

fn perform_edge_relaxation<I: Eq + Hash + Clone, D, G: Graph<I, D>>(graph: &G, metadata: &mut HashMap<I, MetadataDijsktra<I>>, min_id: &I) -> Result<(), String> {
    for id in graph.get_vertex_neighbours(min_id).iter() {
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
            if id_meta.distance > (min_id_meta.distance + graph.get_edge_weight(min_id, id)) {
                id_meta.distance = min_id_meta.distance + graph.get_edge_weight(min_id, id);
                id_meta.predecessor = Some(min_id.clone());
            }
        }
    }
    
    Ok(())
}

fn backtrack_vertex_predecessor<I: Eq + Hash + Clone>(metadata: &HashMap<I, MetadataDijsktra<I>>, start_vertex: &I, target_vertex: &I) -> Result<GraphPath<I>, String> {
    let mut result: GraphPath<I> = GraphPath::new();
    
    match metadata.get(target_vertex) {
        Some(ref x) => result.set_distance(x.distance),
        None => return Err(String::from_str("An error occured while backtracking from a vertex"))
    }
    
    let mut path: Vec<I> = Vec::new();
    let mut last: &I = target_vertex;
    
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

fn get_next_vertex<I: Eq + Hash + Clone>(buckets: &mut HashMap<uint, HashSet<I>>, current_core: &mut uint) -> Result<I, ()> {
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