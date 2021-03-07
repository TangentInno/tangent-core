/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

#[derive(Debug)]
pub enum LedgerType {
    Location
}

pub trait Ledger {
    fn get_ledger_type(&self) -> LedgerType;
    fn populate_ledger_tickets(&self) -> &Self;
    fn populate_ledger_identifier(&self) -> &Self;
}

#[derive(Debug)]
pub struct Ticket<T> {
    pub ledger_identifier: LedgerType,
    pub ticket_identifer: String,

    pub owner: String,
    pub previous_hash: String,
    pub next_hash: String,

    pub data: T,
}
