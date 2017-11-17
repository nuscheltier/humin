use std::collections::HashMap;

pub struct Node {
    id: u64,
    properties: HashMap<String, String>,
}

impl Node {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    pub fn new(id: u64) -> Node {
        let prop = HashMap::new();
        Node {
            id: id,
            properties: prop
        }
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
