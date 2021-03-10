/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/
use super::ticket::{Ticket, Location, Parsable};
use diesel::pg::PgConnection;
use std::any::Any;
use super::profiler;
use super::db;

pub struct LedgerManager<'a> {
    database: Box<&'a PgConnection>,
}

impl<'a> LedgerManager<'a> {
    pub fn new(db: Box<&PgConnection>) -> LedgerManager {
        return LedgerManager {database: db}
    }

    pub fn manage_incoming_ticket<T: Send + Sync + 'static + Any + Sized>(&mut self, ticket: Box<dyn Parsable<'static>>) {
        let ticket_cast = ticket.as_any();

        let test = profiler::Profiler::invoke("Location Cast");
        match ticket_cast.downcast_ref::<Ticket<Location>>() {
            Some(yes) => { println!("{:?}", yes); },
            None => {}
        }
        drop(test);

        //db::addTicket(*self.database, ticket_structure);
    }
}