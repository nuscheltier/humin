#[macro_use]
extern crate serde_json;

pub mod database;
pub mod humin;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
