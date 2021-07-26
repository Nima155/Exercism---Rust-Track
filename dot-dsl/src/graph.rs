use std::collections::HashMap;

use with_attrs::Attrs;
use with_attrs_derive::Attrs;

use crate::edge::Edge;
use crate::node::Node;

pub mod graph_items {
    pub use crate::edge; // re-exporting, as in making the module availabe to outsiders
    pub use crate::node;
    pub use with_attrs::Attrs;
    pub use with_attrs_derive::Attrs;
}

#[derive(Attrs)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::new(),
        }
    }
    pub fn with_nodes(self, vec_of_nodes: &Vec<Node>) -> Self {
        Self {
            nodes: vec_of_nodes.clone(),
            ..self
        }
    }
    pub fn with_edges(self, vec_of_edges: &Vec<Edge>) -> Self {
        Self {
            edges: vec_of_edges.clone(),
            ..self
        }
    }
    pub fn get_node(&self, node_value: &str) -> Option<&Node> {
        self.nodes.iter().find(|ele| node_value == ele.value)
    }
}
