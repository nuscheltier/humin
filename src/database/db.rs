use std::collections::HashMap;
use database::edge::Edge;
use database::node::Node;
use database::error::TitleError;
use database::error::IDError;

struct DB {
    file: String,
    //wouldn't it be better, if we saved the relations by their respective Node-IDs?
    //or should we just also save relations from the Node-IDs?
    //TODO
    edges: HashMap<u64, Edge>,
    edges_by_title: HashMap<String, Edge>,
    nodes: HashMap<u64, Node>,
    nodes_by_title: HashMap<String, Node>
}

impl DB {
    fn find_node_by_title(&self, title: String) -> Result<&Node, TitleError> {
        if self.nodes_by_title.get(&title).is_some() {
            return Ok(self.nodes_by_title.get(&title).unwrap());
        } else {
            return Err(TitleError);
        }
    }

    fn find_node_by_id(&self, id: u64) -> Result<&Node, IDError> {
        if self.nodes.get(&id).is_some() {
            return Ok(self.nodes.get(&id).unwrap());
        } else {
            return Err(IDError);
        }
    }

    fn find_edge_by_title(&self, title: String) -> Result<&Edge, TitleError> {
        if self.edges_by_title.get(&title).is_some() {
            return Ok(self.edges_by_title.get(&title).unwrap());
        } else {
            return Err(TitleError);
        }
    }

    fn find_edge_by_id(&self, id: u64) -> Result<&Edge, IDError> {
        if self.edges.get(&id).is_some() {
            return Ok(self.edges.get(&id).unwrap());
        } else {
            return Err(IDError);
        }
    }

    /*fn find_relation_node(node: Node) -> Vec<Edge> {
    }*/
}
