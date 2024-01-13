use borsh_derive::{BorshDeserialize, BorshSerialize};

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}
