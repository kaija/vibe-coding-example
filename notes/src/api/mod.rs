pub mod auth;
pub mod notes;

use rocket::{Route, routes};

pub fn get_routes() -> Vec<Route> {
    routes![
        auth::register,
        auth::login,
        auth::me,
        notes::create_note,
        notes::get_notes,
        notes::get_note,
        notes::update_note,
        notes::delete_note,
    ]
}