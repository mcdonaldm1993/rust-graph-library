use Edge;

/// A default implementation of a weighted edge that can be used in graph implementations.
///
/// The weight parameter is ignored when constructing an edge and all edges have weight = 1.
#[derive(Clone)]
pub struct UnweightedEdge<N> {
    source: N,
    target: N,
    weight: i32,
    directed: bool
}

impl<N> Edge<N> for UnweightedEdge<N>
    where N: Clone
{
    fn new(source: N, target: N, weight: i32, directed: bool) -> Self {
        UnweightedEdge {
            source: source,
            target: target,
            weight: 1,
            directed: directed
        }
    }
    
    fn get_weight(&self) -> i32 {
        self.weight
    }
    
    fn get_source(&self) -> N {
        self.source.clone()
    }
    
    fn get_target(&self) -> N {
        self.target.clone()
    }
    
    fn is_directed(&self) -> bool {
        self.directed
    }
}