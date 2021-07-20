pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use self::graph_items::Attrs;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    pub mod graph_items {
        pub trait Attrs {
            fn with_attrs(self, attrs: &[(&str, &str)]) -> Self;
        }
        pub mod edge {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub edge1: String,
                pub edge2: String,
                pub attributes: Option<Vec<(String, String)>>,
            }
            impl Edge {
                pub fn new(two: &str, one: &str) -> Self {
                    Self {
                        edge1: one.to_string(),
                        edge2: two.to_string(),
                        attributes: None,
                    }
                }
            }
            impl super::Attrs for Edge {
                fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attributes: Some(
                            attrs
                                .iter()
                                .map(|(one, two)| (one.to_string(), two.to_string()))
                                .collect::<Vec<(String, String)>>(),
                        ),
                        ..self
                    }
                }
            }
        }
        pub mod node {

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub value: String,
                attributes: Option<Vec<(String, String)>>,
            }
            impl Node {
                pub fn new(a: &str) -> Self {
                    Self {
                        value: a.to_string(),
                        attributes: None,
                    }
                }
                pub fn get_attr(&self, attribute_to_find: &str) -> Option<&str> {
                    Some(
                        &self
                            .attributes
                            .iter()
                            .find(|z| attribute_to_find == z[0].0)
                            .unwrap()[0]
                            .1[..],
                    )
                }
            }
            impl super::Attrs for Node {
                fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attributes: Some(
                            attrs
                                .iter()
                                .map(|(one, two)| (one.to_string(), two.to_string()))
                                .collect::<Vec<(String, String)>>(),
                        ),
                        ..self
                    }
                }
            }
        }
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
        pub fn get_node(&self, node_value: &str) -> Option<Node> {
            for node in &self.nodes {
                if node.value == node_value {
                    return Some(node.to_owned());
                }
            }
            None
        }
    }
    impl Attrs for Graph {
        fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Self {
                attrs: attrs
                    .iter()
                    .map(|(one, two)| (one.to_string(), two.to_string()))
                    .collect::<HashMap<String, String>>(),
                ..self
            }
        }
    }
}
