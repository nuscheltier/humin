use std::collections::HashMap;

pub struct Edge {
    id: u64,
    properties: HashMap<String, String>,
    origin: u64,
    target: u64
}

impl Edge {
    //getter
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    pub fn get_origin(&self) -> u64 {
        self.origin
    }

    pub fn get_target(&self) -> u64 {
        self.target
    }

    //setter
    //fn set_id(&self) -> u64 This should be done with initialization
    //a new Edge should have an ID, a Title, an origin node and a target node. Properties aren't needed
    pub fn new(id: u64, origin: u64, target: u64) -> Edge {
        let prop = HashMap::new();
        Edge {
            id: id,
            origin: origin,
            target: target,
            properties: prop
        }
    }

    //no possible change of ID, Origin and Target.
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
fn edge_test_new() {
    let edge = Edge::new(1, 1, 1);
    assert_eq!(edge.get_origin(), 1);
    assert_eq!(edge.get_target(), 1);
    assert_eq!(edge.get_id(), 1);
}

#[test]
fn edge_properties() {
    let mut edge = Edge::new(1, 1, 1);
    edge.add_property("Test".to_string(), "test".to_string());
    {
        let prop = edge.get_properties();
        assert_eq!(prop.get("Test"), Some(&("test".to_string())));
    }
    edge.change_property("Test".to_string(), "test2".to_string());
    {
        let prop = edge.get_properties();
        assert_eq!(prop.get("Test"), Some(&("test2".to_string())));
    }
    edge.remove_property("Test".to_string());
    {
        let prop = edge.get_properties();
        assert_eq!(prop.contains_key(&("Test".to_string())), false);
        assert!(prop.is_empty());
    }
}
