use std::collections::HashMap;
use with_attrs_derive::Attrs;
use with_attrs::Attrs;



#[derive(Clone, PartialEq, Attrs, Debug)]
pub struct Node {
    pub value: String,
    attrs: HashMap<String, String>,
}
impl Node {
    pub fn new(a: &str) -> Self {
        Self {
            value: a.to_string(),
            attrs: HashMap::new(),
        }
    }
    pub fn get_attr(&self, attribute_to_find: &str) -> Option<&str> {
        match self.attrs.get(attribute_to_find) {
            Some(k) => Some(&k[..]),
            None => None
        }
        
    }
}
