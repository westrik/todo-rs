#![deny(
clippy::all,
clippy::pedantic,
// TODO: turn on `clippy::cargo` before publishing
)]
pub mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod test_routing_logic {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}
