/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/
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

    pub fn manage_incoming_ticket<T: Send + Sync + 'static + Any + Sized>(&mut self) {
        let test = profiler::Profiler::invoke("Location Cast");

        drop(test);
    }
}