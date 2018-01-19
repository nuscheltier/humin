use super::super::database::db::DB as DB;
use std::collections::HashMap;
//Groups is the struct to collect all Organisations. It's named to make it differentiate more
//from Organisations.
//Also it is the only way to actually build an Organisation and have it cataloged via the HashMap
pub struct Groups {
    //groups: Vec<Organisation>
    groups: HashMap<u64, Organisation>
}
//Organisations should have a starting Event (their founding) and some people (the founders).
//But for worldbuilding purposes it should be optional for building purposes.
//The Name of the Organisation is not optional anymore.
pub struct Organisation {
    node_id: u64,
    name: String
}

impl Groups {
    pub fn new() -> Groups {
        Groups {
            groups: HashMap::new()
        }
    }

    pub fn new_organisation(&mut self, db: &mut DB, name: String) -> u64 {
        let org = Organisation::new(db, name);
        let id = org.get_id();
        self.groups.insert(id, org);
        id
    }

    //get a HashMap of all Organisations
    pub fn get_all(&self) -> &HashMap<u64, Organisation> {
        &self.groups
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<u64, Organisation> {
        &mut self.groups
    }

    //get one specific Organisation
    pub fn get_mut(&mut self, id: u64) -> Result<&mut Organisation, &'static str> {
        if self.groups.get(&id).is_some() {
            return Ok(self.groups.get_mut(&id).unwrap());
        } else {
            Err("Organisation not found.")
        }
    }

    pub fn get(&self, id: u64) -> Result<&Organisation, &'static str> {
        if self.groups.get(&id).is_some() {
            return Ok(self.groups.get(&id).unwrap());
        } else {
            Err("Organisation not found.")
        }
    }
}

impl Organisation {
    fn new(db: &mut DB, name: String) -> Organisation {
        let node = db.new_node();
        db.find_node_by_id(node).unwrap().add_property("Name".to_string(), (&name).to_string());
        Organisation {
            node_id: node,
            name: name
        }
    }

    //free form role for members (like freelancer) will be added at a later date
    pub fn add_member(&mut self, db: &mut DB, id: u64) {
        let relation_id = db.new_edge(self.node_id, id);
        db.find_edge_by_id(relation_id).unwrap().add_property("Rolle".to_string(), "Mitglied".to_string());
    }

    pub fn change_name(&mut self, db: &mut DB, name: String) {
        db.find_node_by_id(self.node_id).unwrap().change_property("Name".to_string(), (&name).to_string());
        self.name = name;
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }
}
