use std::collections::HashMap;
use super::super::database::db::DB as DB;

//A Date is connected to at least one event
//every Event has one Date.
//So once an Event is created and the Date isn't created
//yet, it needs to be created.

//I don't think a seperate Date Structure is needed atm
/*pub struct Date {
    node_id: u64
}

impl Date {
    pub fn new(db: &mut DB, end_id: u64) -> Date {
        let node_id = db.new_node();
        db.new_edge(node_id, end_id);
        Date {
            node_id: node_id
        }
    }
}*/

pub struct Calendar {
    //dates: HashMap<u64, Date>
    dates: Vec<u64>
}

impl Calendar {
    pub fn new() -> Calendar {
        let v: Vec<u64> = Vec::new();
        Calendar{
            dates: v
        }
    }

    //perhaps a Struct to structure the whole date would be good? ... perhaps
    pub fn add_date(&mut self, db: &mut DB, date: String, id: u64) -> Result<(), &'static str> {
        if self.dates.contains(&id) {
            return Err("This id already exists");
        } else {
            let node_id = db.new_node();
            let edge_id = db.new_edge(node_id, id);
            let mut node = db.find_node_by_id(node_id).unwrap();
            node.add_property("Zeitpunkt".to_string(), date);
            //it doesn't really matter whether it is an origin or a target of the edges
            node.add_origin(edge_id);
            self.dates.push(node_id);
            Ok(())
        }
    }

    pub fn get_all(&self) -> &Vec<u64> {
        &self.dates
    }

    pub fn remove_date(&mut self, db: &mut DB, id: u64) -> Result<(), &'static str> {
        if !self.dates.contains(&id) {
            return Err("There is no such date to be deleted.");
        } else {
            db.del_node(id);
            Ok(())
        }
    }
}
