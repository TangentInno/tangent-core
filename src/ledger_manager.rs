/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/
use super::ticket::{Ticket, Location, Parsable};
use postgres::{Client};
use std::any::Any;
use super::profiler;
use super::db;

pub struct LedgerManager<'a> {
    database: Box<&'a mut Client>,
}

impl<'a> LedgerManager<'a> {
    pub fn new(db: Box<&mut Client>) -> LedgerManager {
        return LedgerManager {database: db}
    }

    pub fn manage_incoming_ticket<T: Send + Sync + 'static + Any + Sized>(&mut self, ticket: Box<dyn Parsable<'static>>) {
        let ticket_cast = ticket.as_any();

        let test = profiler::Profiler::invoke("Location Cast");
        match ticket_cast.downcast_ref::<Ticket<Location>>() {
            Some(tic) => { 
                db::add_ticket(*self.database, tic);
            },
            None => {}
        }

        drop(test);
    }
}