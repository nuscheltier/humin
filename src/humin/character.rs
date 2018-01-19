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
        Character {
            node_id: db.new_node(),
            name: name
        }
    }
    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }
}
