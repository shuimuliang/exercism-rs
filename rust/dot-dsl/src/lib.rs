pub mod graph {

    use self::graph_items::node::Node;
    use self::graph_items::edge::Edge;
    use std::collections::HashMap;
    use std::fmt::Error;

    pub mod graph_items {
        pub mod edge {
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: Vec<(String, String)>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Self {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: Vec::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|attr|
                        (attr.0.to_string(), attr.1.to_string())).collect();
                    self
                }
            }
        }

        pub mod node{
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub label: String,
                pub attrs: Vec<(String, String)>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        label: name.to_string(),
                        attrs: Vec::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|attr| (attr.0.to_string(), attr.1.to_string()) )
                        .collect();
                    self
                }

                pub fn get_attr(&self, label: &str) -> Option<&str> {
                    let value = self.attrs.iter().find(|&p| p.0.eq(label));
                    Some(&value.unwrap().1)
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().fold(HashMap::new(), |mut acc, attr| {
                acc.insert(attr.0.to_string(), attr.1.to_string());
                acc
            });
            self
        }

        pub fn get_node(&self, label: &str) -> Result<Node, Error> {
            let ref_node = self.nodes.iter().find(|&p| p.label.eq(label));
            let res = ref_node.cloned();
            Ok(res.unwrap())
            // for node in self.nodes.iter() {
            //     if node.label.eq(label) {
            //         return node
            //     }
            // }
        }
    }
}
