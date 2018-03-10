use std::collections::HashMap;
use super::super::database::db::DB as DB;
//TODO: Struct Date
//Date has a tuple (year, month, day)

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
        //since we get the target ID we don't want it to be featured twice or more in our dates
        //and we don't want any date to be target of any date
        let mut dates_contain_bool = false;
        {
            let target_id = id;
            let target_node = db.find_node_by_id(target_id);
            if target_node.is_ok() {
                let target_node_targets_vector = target_node.unwrap().get_targets();
                for edge_id in target_node_targets_vector {
                    //we trust our data to be valid
                    let bl = self.dates.contains(&(db.find_edge_by_id(*edge_id).unwrap().get_origin()));
                    if bl {
                        dates_contain_bool = bl;
                    }
                }
            }
        }
        if self.dates.contains(&id) || dates_contain_bool {
            return Err("This id already exists");
        } else {
            let node_id = db.new_node();
            println!("add_date node_id: {}", node_id);
            let edge_id = db.new_edge(node_id, id);
            let mut node = db.find_node_by_id_as_mut(node_id).unwrap();
            node.add_property("Zeitpunkt".to_string(), date);
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
