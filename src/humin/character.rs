use super::super::database::db::DB as DB;
use std::collections::HashMap;
//A list of characters
pub struct People {
    people: HashMap<u64, Character>
}

pub struct Character {
    node_id: u64,
    name: String
}

impl People {
    pub fn new() -> People {
        People {
            people: HashMap::new()
        }
    }

    pub fn new_character(&mut self, db: &mut DB, name: String) -> u64 {
        let peop = Character::new(db, name);
        let id = peop.get_id();
        self.people.insert(id, peop);
        id
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<u64, Character> {
        &mut self.people
    }

    pub fn get_all(&self) -> &HashMap<u64, Character> {
        &self.people
    }

    pub fn get_mut(&mut self, id: u64) -> Result<&mut Character, &'static str> {
        if self.people.get(&id).is_some() {
            return Ok(self.people.get_mut(&id).unwrap());
        } else {
            Err("Character not found.")
        }
    }

    pub fn get(&self, id: u64) -> Result<&Character, &'static str> {
        if self.people.get(&id).is_some() {
            return Ok(self.people.get(&id).unwrap());
        } else {
            Err("Character not found.")
        }
    }
}

impl Character {
    fn new(db: &mut DB, name: String) -> Character {
        let node = db.new_node();
        db.find_node_by_id(node).unwrap().add_property("Name".to_string(), (&name).to_string());
        Character {
            node_id: node,
            name: name
        }
    }
    pub fn change_name(&mut self, db: &mut DB, name: String) {
        db.find_node_by_id(self.node_id).unwrap().change_property("Name".to_string(), (&name).to_string());
        self.name = name;
    }

    pub fn set_birthday(&mut self, db: &mut DB, id: u64, time: String) {
        let relation_id = db.new_edge(self.node_id, id);
        db.find_edge_by_id(relation_id).unwrap().add_property("Geburtstag".to_string(), time);
    }

    pub fn change_birthday(&mut self, db: &mut DB, id: u64, new_id: u64, time: String) {
        let mut edges_orig: Vec<u64> = Vec::new();
        {
            let node = db.find_node_by_id(self.node_id).unwrap();
            edges_orig = node.get_origins().clone();
        }
        //we have to delete the edge... 
        for org in edges_orig {
            let mut edge_id: u64 = 0;
            let mut edge_target: u64 = 0;
            {
                let edge = db.find_edge_by_id(org).unwrap();
                if edge.get_properties().contains_key("Geburtstag") {
                    edge_id = org;
                    edge_target = edge.get_target();
                }
            }
            //we need to delete the edge, but also delete a date-node if it isn't referenced
            //anymore to prevent unneccessary data clutter within the database
            db.clean_node(edge_target);
            db.del_edge(org);
        }
        //... and create a new one to the date node
        self.set_birthday(db, new_id, time);

        /*for edge in edges_orig {
            let mut e = db.find_edge_by_id(edge).unwrap();
            if e.get_properties().get(&("Geburtstag".to_string())).is_some() {
                e.change_property("Geburtstag".to_string(), (&time).to_string());
            }
        }*/
    }
    
    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }
}
