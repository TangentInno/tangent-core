use diesel::prelude::*;
use diesel::pg::PgConnection;

static PASSWORD_POSTGRES: &str= "";

pub fn established() -> PgConnection {
    PgConnection::establish(&("postgres://postgres:".to_string() + PASSWORD_POSTGRES + "@10.0.0.73/tangent_core")).expect(&format!("Error connecting to {}", "10.0.0.73:5432"))
}