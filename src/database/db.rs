extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use serde_json::Value;
//use std::iter::{IntoIter, Map};
use database::edge::Edge;
use database::node::Node;
use database::error::{TitleError, IDError, DataValidError} ;

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
    //TODO: it should be easy doing this using serde_json
    //TODO: A to_string of the Database to use it in another format.
    //TODO: In the same vein as above: a from_string
    pub fn save_database(&self, name: String) {
        //let mut json_string = String::new();
        let mut nodes_string = String::new();
        let mut edges_string = String::new();
        for node in self.nodes.values() {
            let mut vec_origin_string = String::new();
            let mut vec_targets_string = String::new();
            let mut vec_properties = String::new();
            for i in node.get_origins() {
                vec_origin_string.push_str(&format!("{},", i));
            }
            vec_origin_string.pop();
            for i in node.get_targets() {
                vec_targets_string.push_str(&format!("{},", i));
            }
            vec_targets_string.pop();
            for prop in node.get_properties().keys() {
                let value = node.get_properties().get(prop).unwrap();
                vec_properties.push_str(&format!("{{\"{}\":\"{}\"}},", prop, value));
            }
            vec_properties.pop();

            nodes_string.push_str(&format!("{{\"id\":{},\"node\":{{\"id\":{},\"origins\":[{}],\"targets\":[{}],\"properties\":[{}]}}}},", node.get_id(), node.get_id(), vec_origin_string, vec_targets_string, vec_properties));
        }
        println!("{}", nodes_string);
        nodes_string.pop();
        for edge in self.edges.values() {
            let mut vec_properties = String::new();
            for prop in edge.get_properties().keys() {
                let value = edge.get_properties().get(prop).unwrap();
                vec_properties.push_str(&format!("{{\"{}\":\"{}\"}},", prop, value));
            }
            vec_properties.pop();
            edges_string.push_str(&format!("{{\"id\":{},\"edge\":{{\"id\":{},\"origin\":{},\"target\":{},\"properties\":[{}]}}}},", edge.get_id(), edge.get_id(), edge.get_origin(), edge.get_target(), vec_properties));
        }
        edges_string.pop();
        let json_string = format!("{{\"nodes\":[{}],\"edges\":[{}]}}", nodes_string, edges_string);
        println!("{}", json_string);
    }

    ////loads
    //TODO: we need a validator for the database
    //and error handling here
    pub fn load_database(&mut self, name: String) -> Result<(), DataValidError>{
        let mut n_hash: HashMap<u64, Node> = HashMap::new();
        let mut e_hash: HashMap<u64, Edge> = HashMap::new();

        let mut file_data = String::new();
        let mut f = File::open(name).unwrap();

        f.read_to_string(&mut file_data);
        let v: Value = serde_json::from_str(&file_data).unwrap();
        //nodes
        if !v["nodes"].is_array() {
            return Err(DataValidError);
        }
        for node_object in v["nodes"].as_array().unwrap() {
            if !node_object.is_object() {
                return Err(DataValidError);
            }
            let n = node_object.as_object().unwrap();
            if !n["id"].is_u64() {
                return Err(DataValidError);
            }
            let mut node = Node::new(n["id"].as_u64().unwrap());
            if !n["node"].is_object() {
                return Err(DataValidError);
            }
            let n_inner = n["node"].as_object().unwrap();
            //we need the origins and targets
            for o in n_inner["origins"].as_array().unwrap() {
                if !o.is_u64() {
                    return Err(DataValidError);
                }
                node.add_origin(o.as_u64().unwrap());
            }
            for t in n_inner["targets"].as_array().unwrap() {
                if !t.is_u64() {
                    return Err(DataValidError);
                }
                node.add_target(t.as_u64().unwrap());
            }
            //properties
            for p in n_inner["properties"].as_array().unwrap() {
                if !p.is_object() {
                    return Err(DataValidError);
                }
                let p_object = p.as_object().unwrap();
                node.add_property(
                    p_object.keys().next().unwrap().as_str().to_string(),//.unwrap().to_string(),
                    p_object.values().next().unwrap().as_str().unwrap().to_string()
                );
            }
            n_hash.insert(node.get_id(), node);
        }
        if !v["edges"].is_array() {
            return Err(DataValidError);
        }
        //edges
        for edge_object in v["edges"].as_array().unwrap() {
            if !edge_object.is_object() {
                return Err(DataValidError);
            }
            let e = edge_object.as_object().unwrap();
            if !e["edge"].is_object() {
                return Err(DataValidError);
            }
            let e_inner = e["edge"].as_object().unwrap();
            if !(e["id"].is_u64() && e_inner["origin"].is_u64() && e_inner["target"].is_u64()) {
                return Err(DataValidError);
            }
            let mut edge = Edge::new(e["id"].as_u64().unwrap(), e_inner["origin"].as_u64().unwrap(), e_inner["target"].as_u64().unwrap());
            for p in e_inner["properties"].as_array().unwrap() {
                if !p.is_object() {
                    return Err(DataValidError);
                }
                let p_object = p.as_object().unwrap();
                edge.add_property(
                    p_object.keys().next().unwrap().as_str().to_string(),//.unwrap().to_string(),
                    p_object.values().next().unwrap().as_str().unwrap().to_string()
                )
            }
            e_hash.insert(edge.get_id(), edge);
        }
        self.edges.clear();
        self.nodes.clear();
        self.edges = e_hash;
        self.nodes = n_hash;
        Ok(())
        /*for node in v["nodes"].iter() {
            let mut n = Node::new(node["id"]);
            for o in node["node"]["origin"] {
                n.add_origin(o);
            }
        }*/
    }

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

    ////dels

    pub fn del_node(&mut self, id: u64) {
        //every edge that is found inside the origins and targets needs to be deleted too
        let mut edge_origin_ids: Vec<u64> = Vec::new();
        let mut edge_target_ids: Vec<u64> = Vec::new();
        {
            let mut node = self.nodes.get_mut(&id).unwrap();
            edge_origin_ids = node.get_origins().clone();
            edge_target_ids = node.get_targets().clone();
        }

        for edge in edge_origin_ids {
            self.del_edge(edge);
        }
        for edge in edge_target_ids {
            self.del_edge(edge);
        }

        self.nodes.remove(&id);
    }

    pub fn del_edge(&mut self, id: u64) {
        let mut origin: u64 = 0;
        let mut target: u64 = 0;
        {
            let mut edge = self.edges.get_mut(&id).unwrap();
            origin = edge.get_origin();
            target = edge.get_target();
        }

        self.nodes.get_mut(&origin).unwrap().del_origin(id as usize);
        self.nodes.get_mut(&target).unwrap().del_target(id as usize);

        self.edges.remove(&id);
    }
}
