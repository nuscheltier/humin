use database::properties::Properties;
use database::property::Property;

pub struct Node {
    id: u64,
    //TODO: Do we really need the title?
    //The name for a node would be a property, wouldn't it?
    title: String,
    properties: Properties
}

impl Node {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn new(id: u64, title: String) -> Node {
        let prop = Properties::new();
        Node {
            id: id,
            title: title,
            properties: prop
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn add_property(&mut self, prop: Property) {
        self.properties.append_property(prop);
    }

    pub fn change_property(&mut self, id: usize, prop: Property) {
        self.properties.change_property(id, prop);
    }
}
