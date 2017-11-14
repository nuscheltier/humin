use database::properties::Properties;

pub struct Edge {
    id: u64,
    title: String,
    properties: Properties,
    origin: u64,
    target: u64
}

impl Edge {
    //getter
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_properties(&self) -> &Properties {
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
        let prop = Properties::new();
        Edge {
            id: id,
            title: title,
            origin: origin,
            target: target,
            properties: prop
        }
    }

    //The title can be changed
    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    //no possible change of ID, Origin and Target.
}
