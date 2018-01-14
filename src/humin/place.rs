use super::super::database::db::DB as DB;
//A place can have connections to a person (if we're honest, a place has to be connected to a
//character, since without a character a place isn't worth noting - but it would be nicer for
//planning and worldbuilding if that connection is optional) and it should have connections to one
//or more Events.
pub struct Place {
    places: Vec<u64>
}

impl Place {
    pub fn new() -> Place {
        Place {
            places: Vec::new()
        }
    }

    pub fn add_events(&mut self, db: &mut DB, place_id: u64, event_id: u64) -> Result<u64, &'static str> {
        //We want to add an event to a place
        if self.places.contains(&place_id) {
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
        }
    }

    pub fn add_character(&mut self, db: &mut DB, place_id: u64, character_id: u64) -> Result<u64, &'static str> {
        //We want to add a character to a place
        if self.places.contains(&place_id) {
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
        }
    }
}
