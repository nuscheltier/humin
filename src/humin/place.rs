use super::super::database::db::DB as DB;
use std::collections::HashMap;
//A place can have connections to a person (if we're honest, a place has to be connected to a
//character, since without a character a place isn't worth noting - but it would be nicer for
//planning and worldbuilding if that connection is optional) and it should have connections to one
//or more Events.
pub struct Places {
    places: HashMap<u64, Area>
}

pub struct Area {
    node_id: u64,
    name: String
}

impl Places {
    pub fn new() -> Places {
        Places {
            places: HashMap::new()
        }
    }

    pub fn new_area(&mut self, db: &mut DB, name: String) -> u64 {
        let area = Area::new(db, name);
        let id = area.get_id();
        self.places.insert(id, area);
        id
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<u64, Area> {
        &mut self.places
    }

    pub fn get_all(&self) -> &HashMap<u64, Area> {
        &self.places
    }

    pub fn get_mut(&mut self, id: u64) -> Result<&mut Area, &'static str> {
        if self.places.get(&id).is_some() {
            return Ok(self.places.get_mut(&id).unwrap());
        } else {
            Err("Place not found.")
        }
    }

    pub fn get(&self, id: u64) -> Result<&Area, &'static str> {
        if self.places.get(&id).is_some() {
            return Ok(self.places.get(&id).unwrap());
        } else {
            Err("Place not found.")
        }
    }

}

impl Area {
    fn new(db: &mut DB, name: String) -> Area {
        Area {
            node_id: db.new_node(),
            name: name
        }
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }

    pub fn add_events(&mut self, db: &mut DB, place_id: u64, event_id: u64) -> Result<u64, &'static str> {
        //We want to add an event to a place
        /*if self.places.contains(&place_id) {
            //if everything runs smoothly the id of the edge will be returned
            let mut edge: u64 = 0;
            {
                edge = db.new_edge(event_id, place_id);
            }
            let node = db.find_node_by_id(place_id).unwrap();
            node.add_target(edge);
            Ok(edge)
        } else {
            Err("ID not found")
        }*/
        Ok(0)
    }

    pub fn add_character(&mut self, db: &mut DB, place_id: u64, character_id: u64) -> Result<u64, &'static str> {
        //We want to add a character to a place
        /*if self.places.contains(&place_id) {
            //if everything runs smoothly the id of the edge will be returned
            let mut edge: u64 = 0;
            {
                edge = db.new_edge(character_id, place_id);
            }
            let node = db.find_node_by_id(place_id).unwrap();
            node.add_target(edge);
            Ok(edge)
        } else {
            Err("ID not found")
        }*/
        Ok(0)
    }
}
