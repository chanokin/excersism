pub mod graph {


    pub mod graph_items{

        pub mod node{

            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }


            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter()
                                      .map(|(name, value)| (name.to_string(), value.to_string()))
                                      .collect();

                    self
                }
                
                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|val| val.as_str())
                }


                
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub source: String,
                pub target: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(source: &str, target: &str) -> Self {
                    Self {
                        source: String::from(source),
                        target: String::from(target),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter()
                                      .map(|(name, value)| (name.to_string(), value.to_string()))
                                      .collect();

                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|val| val.as_str())
                }
            }
        }
    }


    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
                Default::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.clone();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter()
                              .map(|(name, value)| (name.to_string(), value.to_string()))
                              .collect();

            self
        }
        
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        pub fn edge(&self, source: &str, target: &str) -> Option<&Edge> {
            self.edges.iter()
                      .find(|edge| 
                            edge.source == source && edge.target == target)
        }

    
    }
}
