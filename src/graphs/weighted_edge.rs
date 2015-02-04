use Edge;

/// A default implementation of a weighted edge that can be used in graph implementations.
#[derive(Clone, Hash)]
pub struct WeightedEdge<N> {
    source: N,
    target: N,
    weight: i32,
    directed: bool
}

impl<N> Edge<N> for WeightedEdge<N>
    where N: Clone
{
    fn new(source: N, target: N, weight: i32, directed: bool) -> Self {
        WeightedEdge {
            source: source,
            target: target,
            weight: weight,
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

impl<N> PartialEq for WeightedEdge<N>
    where N: Eq
{
    fn eq(&self, other: &WeightedEdge<N>) -> bool {
        self.source == other.source && self.target == other.target && self.weight == other.weight && self.directed == other.directed
    }
}

impl<N> Eq for WeightedEdge<N> where N: Eq { }