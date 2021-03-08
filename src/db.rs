use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn established() -> PgConnection {
    PgConnection::establish("postgres://postgres:PASSWORD@10.0.0.73/tangent_core").expect(&format!("Error connecting to {}", "10.0.0.73:5432"))
}

use super::ticket::{Ticket, Location};
pub fn addTicket(connection: &PgConnection, ticket: Ticket<Location>) {

} 