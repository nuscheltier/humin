pub struct Property {
    //Would we need an ID for the property?
    key: String,
    //Would the value be better as an enum?
    value: String
}

impl Property {
    //getter
    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    //setter
    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}
