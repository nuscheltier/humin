use super::super::database::db::DB as DB;
//Groups is the struct to collect all Organisations. It's named to make it differentiate more
//from Organisations.
pub struct Groups {
    groups: Vec<Organisation>
}
//Organisations should have a starting Event (their founding) and some people (the founders).
//But for worldbuilding purposes it should be optional.
pub struct Organisation {
    node_id: u64
}

impl Groups {
}

impl Organisation {
    pub fn new(db: &mut DB) -> Organisation {
        Organisation {
            node_id: db.new_node()
        }
    }
}
