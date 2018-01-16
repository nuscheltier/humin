use super::super::database::db::DB as DB;
use std::collections::HashMap;
//Groups is the struct to collect all Organisations. It's named to make it differentiate more
//from Organisations.
pub struct Groups {
    //groups: Vec<Organisation>
    groups: HashMap<u64, Organisation>
}
//Organisations should have a starting Event (their founding) and some people (the founders).
//But for worldbuilding purposes it should be optional.
pub struct Organisation {
    node_id: u64
}

impl Groups {
    pub fn new() -> Groups {
        Groups {
            groups: HashMap::new()
        }
    }

    pub fn new_organisation(&mut self, db: &mut DB) -> u64 {
        let org = Organisation::new(db);
        let id = org.get_id();
        self.groups.insert(id, org);
        id
    }

    pub fn get_all(&mut self) -> &HashMap<u64, Organisation> {
        &self.groups
    }
}

impl Organisation {
    pub fn new(db: &mut DB) -> Organisation {
        Organisation {
            node_id: db.new_node()
        }
    }

    //free form role for members (like freelancer) will be added at a later date
    pub fn add_member(&mut self, db: &mut DB, id: u64) {
        let relation_id = db.new_edge(self.node_id, id);
        db.find_edge_by_id(relation_id).unwrap().add_property("Rolle".to_string(), "Mitglied".to_string());
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }
}
