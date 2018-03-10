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

#[test]
fn node_new_test() {
    let node = Node::new(1);
    assert_eq!(node.get_id(), 1);
    assert!(node.get_origins().is_empty());
    assert!(node.get_targets().is_empty());
}

#[test]
fn node_origin_test() {
    let mut node = Node::new(1);
    node.add_origin(1);
    node.add_origin(2);
    assert!(node.get_targets().is_empty());
    {
        let origin = node.get_origins();
        assert_eq!(origin[0], 1);
        assert_eq!(origin[1], 2);
        assert_eq!(origin.len(), 2);
    }
    node.del_origin(0);
    {
        let origin = node.get_origins();
        assert_eq!(origin[0], 2);
        assert_eq!(origin.len(), 1);
    }
}

#[test]
fn node_target_test() {
    let mut node = Node::new(1);
    node.add_target(1);
    node.add_target(2);
    assert!(node.get_origins().is_empty());
    {
        let target = node.get_targets();
        assert_eq!(target[0], 1);
        assert_eq!(target[1], 2);
        assert_eq!(target.len(), 2);
    }
    node.del_target(0);
    {
        let target = node.get_targets();
        assert_eq!(target[0], 2);
        assert_eq!(target.len(), 1);
    }
}

#[test]
fn node_properties() {
    let mut node = Node::new(1);
    node.add_property("Test".to_string(), "test".to_string());
    {
        let prop = node.get_properties();
        assert_eq!(prop.get("Test"), Some(&("test".to_string())));
    }
    node.change_property("Test".to_string(), "test2".to_string());
    {
        let prop = node.get_properties();
        assert_eq!(prop.get("Test"), Some(&("test2".to_string())));
    }
    node.remove_property("Test".to_string());
    {
        let prop = node.get_properties();
        assert_eq!(prop.contains_key(&("Test".to_string())), false);
        assert!(prop.is_empty());
    }
}
