use database::error::PropertyError;
use database::property::Property;

pub struct Properties {
    properties: Vec<Property>
}

impl Properties {
    pub fn new() -> Properties {
        let x: Vec<Property> = Vec::new();
        Properties {
            properties: x
        }
    }
    pub fn append_property(&mut self, prop: Property) {
        self.properties.push(prop);
    }

    /*fn delete_property(&self, id: usize) {
        self.properties.remove(id)?;
    }*/
    pub fn delete_property(&mut self, id: usize) -> Result<String, PropertyError> {
        if id > self.properties.len() {
            return Err(PropertyError);
        } else {
            self.properties.remove(id);
            return Ok("ok".to_string());
        }
    }

    pub fn change_property(&mut self, id: usize, prop: Property) {
        //concurrent use of the database is not implemented and not planned
        self.delete_property(id);
        self.properties.insert(id, prop);
    }
}
