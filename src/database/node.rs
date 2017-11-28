use std::collections::HashMap;

//TODO
//enum Value {
//  s(String),
//  u(u64),
//  i(i64)
//  }

pub struct Node {
    id: u64,
    origin: Vec<u64>,
    target: Vec<u64>,
    properties: HashMap<String, String>,
}

impl Node {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    pub fn get_origins(&self) -> &Vec<u64> {
        &self.origin
    }

    pub fn get_targets(&self) -> &Vec<u64> {
        &self.target
    }

    pub fn new(id: u64) -> Node {
        let prop = HashMap::new();
        let orig: Vec<u64> = Vec::new();
        let targ: Vec<u64> = Vec::new();

        Node {
            id: id,
            origin: orig,
            target: targ,
            properties: prop
        }
    }

    //it is origin of which edge?
    pub fn add_origin(&mut self, edge_id: u64) {
        self.origin.push(edge_id);
    }

    //it is target of which edge?
    pub fn add_target(&mut self, edge_id: u64) {
        self.target.push(edge_id);
    }

    pub fn del_origin(&mut self, id: usize) {
        self.origin.remove(id);
    }

    pub fn del_target(&mut self, id: usize) {
        self.target.remove(id);
    }

    pub fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn change_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn remove_property(&mut self, key: String) {
        self.properties.remove(&key);
    }
}
