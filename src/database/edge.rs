use std::collections::HashMap;

pub struct Edge {
    id: u64,
    properties: HashMap<String, String>,
    origin: u64,
    target: u64
}

impl Edge {
    //getter
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    fn get_origin(&self) -> u64 {
        self.origin
    }

    fn get_target(&self) -> u64 {
        self.target
    }

    //setter
    //fn set_id(&self) -> u64 This should be done with initialization
    //a new Edge should have an ID, a Title, an origin node and a target node. Properties aren't needed
    pub fn new(id: u64, title: String, origin: u64, target: u64) -> Edge {
        let prop = HashMap::new();
        Edge {
            id: id,
            origin: origin,
            target: target,
            properties: prop
        }
    }

    //no possible change of ID, Origin and Target.
}
