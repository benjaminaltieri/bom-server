#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]

#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate thiserror;
extern crate uuid;

pub mod client;
pub mod errors;
pub mod parts_list;
pub mod query;
pub mod response;
pub mod routes;

use std::sync::RwLock;

use crate::parts_list::PartsList;

/// Use Reader Writer Lock to control access to a parts list
pub struct SharedPartsList(RwLock<PartsList>);

impl SharedPartsList {
    pub fn new() -> SharedPartsList {
        SharedPartsList(RwLock::new(PartsList::new()))
    }
}

impl Default for SharedPartsList {
    fn default() -> Self {
        Self::new()
    }
}

/// Create reactor for bom-server taking ownership of a parts list instance
/// and mount all API paths from the routes module
pub fn make_rocket(parts_list: SharedPartsList) -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::list_parts,
                routes::create_part,
                routes::get_part,
                routes::delete_part,
                routes::get_children,
                routes::update_children,
                routes::get_contained,
            ],
        )
        .manage(parts_list)
}
