use anchor_lang::prelude::*;

#[event]
pub struct Deposited {
    pub id: String,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct Withdrawn {
    pub id: String,
    pub amount: u64,
    pub timestamp: i64,
}
