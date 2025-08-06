use petgraph::graph::{NodeIndex, EdgeIndex};
use std::collections::HashMap;

/// A graph morphism mapping nodes and edges from one graph into another.
///
/// The mapping is represented by two hashmaps:
/// - `node_map`: source node index → target node index
/// - `edge_map`: source edge index → target edge index
#[derive(Debug, Clone)]
pub struct Morphism {
    pub node_map: HashMap<NodeIndex, NodeIndex>,
    pub edge_map: HashMap<EdgeIndex, EdgeIndex>,
}

impl Morphism {
    /// Create an empty morphism with no mappings.
    pub fn new() -> Self {
        Morphism {
            node_map: HashMap::new(),
            edge_map: HashMap::new(),
        }
    }

    /// Insert a node mapping (src -> dst).
    pub fn insert_node(&mut self, src: NodeIndex, dst: NodeIndex) {
        self.node_map.insert(src, dst);
    }

    /// Insert an edge mapping (src -> dst).
    pub fn insert_edge(&mut self, src: EdgeIndex, dst: EdgeIndex) {
        self.edge_map.insert(src, dst);
    }

    /// Lookup the image of a source node, if mapped.
    pub fn map_node(&self, src: &NodeIndex) -> Option<&NodeIndex> {
        self.node_map.get(src)
    }

    /// Lookup the image of a source edge, if mapped.
    pub fn map_edge(&self, src: &EdgeIndex) -> Option<&EdgeIndex> {
        self.edge_map.get(src)
    }

    /// Compose this morphism with another: (self ∘ other).
    ///
    /// The result maps elements from `other`'s source through `other` then `self`.
    pub fn compose(&self, other: &Morphism) -> Morphism {
        let mut composed = Morphism::new();
        // Compose node mappings
        for (&s, &m) in &other.node_map {
            if let Some(&t) = self.node_map.get(&m) {
                composed.node_map.insert(s, t);
            }
        }
        // Compose edge mappings
        for (&s_e, &m_e) in &other.edge_map {
            if let Some(&t_e) = self.edge_map.get(&m_e) {
                composed.edge_map.insert(s_e, t_e);
            }
        }
        composed
    }

    /// Invert this morphism: swap source and target mappings.
    pub fn invert(&self) -> Morphism {
        let mut inv = Morphism::new();
        for (&s, &t) in &self.node_map {
            inv.node_map.insert(t, s);
        }
        for (&s_e, &t_e) in &self.edge_map {
            inv.edge_map.insert(t_e, s_e);
        }
        inv
    }
}
