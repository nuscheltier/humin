use std::collections::HashMap;
//use std::iter::{IntoIter, Map};
use database::edge::Edge;
use database::node::Node;
use database::error::TitleError;
use database::error::IDError;

pub struct DB {
    file: String,
    //wouldn't it be better, if we saved the relations by their respective Node-IDs?
    //or should we just also save relations from the Node-IDs?
    //TODO
    edges: HashMap<u64, Edge>,
    nodes: HashMap<u64, Node>,
}

impl DB {
    ////init

    pub fn new(name: String) -> DB {
        DB {    
            file: "".to_string(),
            edges: HashMap::new(),
            nodes: HashMap::new()
        }
        //if name == "" {
            //return DB {
            //};
        //} else {
        //}
    }
    ////saves

    /*fn find_node_by_title(&self, title: String) -> Result<&Node, TitleError> {
        if self.nodes_by_title.get(&title).is_some() {
            return Ok(self.nodes_by_title.get(&title).unwrap());
        } else {
            return Err(TitleError);
        }
    }*/
    ////finds

    pub fn find_node_by_id(&mut self, id: u64) -> Result<&mut Node, IDError> {
        if self.nodes.get(&id).is_some() {
            return Ok(self.nodes.get_mut(&id).unwrap());
        } else {
            return Err(IDError);
        }
    }

    pub fn find_edge_by_id(&mut self, id: u64) -> Result<&mut Edge, IDError> {
        if self.edges.get(&id).is_some() {
            return Ok(self.edges.get_mut(&id).unwrap());
        } else {
            return Err(IDError);
        }
    }

    //it would be better if we saved the properties seperately - but only if time is of the essence and for really big databases should it be needed
    pub fn find_node_by_property_value(&mut self, prop: String, prop_val: String) -> Vec<u64> {
        //TODO: at the moment you have to know the exact value. 
        let mut vec_ids: Vec<u64> = Vec::new();
        for id in self.nodes.keys() {
            if self.nodes.get(&id).unwrap().get_properties().keys().any(|ref x| x == &&prop) && self.nodes.get(&id).unwrap().get_properties().get(&prop).unwrap() == &prop_val {
                vec_ids.push(*id);
            }
        }
        vec_ids
    }

    //is the node in any relationship with another node by one specific property?
    pub fn find_target_by_property(&mut self, orig: u64, prop: String) -> Vec<u64> {
        self.find_target_by_property_value(orig, prop, "".to_string())
    }

    //is the node in any relationship with another node by one specific property and property value?
    pub fn find_target_by_property_value(&mut self, orig: u64, prop: String, prop_val: String) -> Vec<u64> {
        let mut vec_ids: Vec<u64> = Vec::new();

        if prop_val != "" {
            let mut id_iter: Vec<u64> = Vec::new();
            {
                id_iter = self.find_node_by_id(orig).unwrap().get_origins().clone();
            }
            for id in id_iter {
                let edge = self.find_edge_by_id(id).unwrap();
                //do the properties of that particular edge have that property?
                if edge.get_properties().keys().any(|ref x| x == &&prop) && edge.get_properties().get(&prop).unwrap() == &prop_val {
                    vec_ids.push(edge.get_target());
                }
            }
            return vec_ids;
        } else {
            //every edge_id that is originated in that node
            let mut id_iter: Vec<u64> = Vec::new();
            {
                id_iter = self.find_node_by_id(orig).unwrap().get_origins().clone();
            }
            for id in id_iter {
                let edge = self.find_edge_by_id(id).unwrap();
                //do the properties of that particular edge have that property?
                if edge.get_properties().keys().any(|ref x| x == &&prop) {
                    vec_ids.push(edge.get_target());
                }
            }
            return vec_ids;
        }
    }

    ////news

    pub fn new_node(&mut self) -> u64 {
        let id = self.nodes.len() as u64;
        let node = Node::new(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn new_edge(&mut self, org: u64, tar: u64) -> u64 {
        let id = self.edges.len() as u64;
        let edge = Edge::new(id, org, tar);
        self.edges.insert(id, edge);
        id
    }
}
