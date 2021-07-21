use std::collections::HashMap;

use with_attrs::Attrs;
use with_attrs_derive::Attrs;

#[derive(Debug, Clone, PartialEq, Attrs)]
pub struct Edge {
    pub edge1: String,
    pub edge2: String,
    pub attrs: HashMap<String, String>,
}
impl Edge {
    pub fn new(two: &str, one: &str) -> Self {
        Self {
            edge1: one.to_string(),
            edge2: two.to_string(),
            attrs: HashMap::new(),
        }
    }
}
