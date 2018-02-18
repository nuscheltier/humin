use super::super::database::db::DB as DB;
use std::collections::HashMap;

//Things and an Item. Along the lines of organisations and so on.
//An Item should have a name, a history, an owner, a history of owners and a creator and perhaps
//some description

pub struct Things {
    things: HashMap<u64, Item>
}

pub struct Item {
    node_id: u64,
    name: String,
    history: String, //this could be transfered into another struct
    owners: Vec<u64>, //the first element is the current owner, the last element would be the creator
    description: String
}

impl Things {
    pub fn new() -> Things {
        Things {
            things: HashMap::new()
        }
    }

    ///Params:
    ///db ... database
    ///name ... Name of Item
    ///creator ... ID of Character that created that particular item
    ///time ... ID of the Date-Node - needs to be created beforehand
    pub fn new_item(&mut self, db: &mut DB, name: String, creator: u64, time: u64) -> u64 {
        let it = Item::new(db, name, creator, time);
        let id = it.get_id();
        self.things.insert(id, it);
        id
    }

    //get a HashMap of all the Things
    pub fn get_all(&self) -> &HashMap<u64, Item> {
        &self.things
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<u64, Item> {
        &mut self.things
    }

    //get one specific item
    pub fn get(&self, id: u64) -> Result<&Item, &'static str> {
        if self.things.get(&id).is_some() {
            return Ok(self.things.get(&id).unwrap());
        } else {
            Err("Item not found.")
        }
    }

    pub fn get_mut(&mut self, id: u64) -> Result<&mut Item, &'static str> {
        if self.things.get(&id).is_some() {
            return Ok(self.things.get_mut(&id).unwrap());
        } else {
            Err("Item not found.")
        }
    }
}

impl Item {
    fn new(db: &mut DB, name: String, creator: u64, time: u64) -> Item {
        let node_id = db.new_node();
        let node = db.find_node_by_id(node_id).unwrap();
        node.add_property("Name".to_string(), (&name).to_string());
        Item {
            node_id: node_id,
            name: name,
            history: String::new(),
            owners: vec![creator],
            description: String::new()
        }
    }

    pub fn get_id(&self) -> u64 {
        self.node_id
    }

    //the original creator is the first element of the vector
    //since there is no item without creator, we don't need a result or option
    pub fn get_creator(&self) -> u64 {
        *self.owners.first().unwrap()
    }

    //since owners is never empty there is no need for result or option
    pub fn get_current_owner(&self) -> u64 {
        *self.owners.last().unwrap()
    }

    //If the item changes owners the new owner is just added.
    //It is an edge case that more than usize will ever be owners, but just for the sake of it
    pub fn change_owner(&mut self, new_owner: u64) -> Result<(), &'static str> {
        if self.owners.len() == <usize>::max_value() {
            return Err("There are to many owners. Please delete one.");
        } else {
            self.owners.push(new_owner);
            Ok(())
        }
    }

    //If we want to delete an owner, we get a usize from the vector
    pub fn delete_owner(&mut self, vec_id: usize) -> Result<(), &'static str> {
        if vec_id >= self.owners.len() {
            return Err("Wrong ID. You cannot delete this ID.");
        }
        self.owners.remove(vec_id);
        Ok(())
    }

    //The owners should not be able to be changed via this method
    //and it should only live while the item is living.
    pub fn get_all_owners(&self) -> &Vec<u64> {
        &self.owners
    }

    pub fn get_history(&self) -> &str {
        &self.history
    }

    pub fn set_history(&mut self, new_history: String) {
        self.history = new_history;
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}
