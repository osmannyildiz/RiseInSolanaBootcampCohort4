use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // entrypoint::{self, ProgramResult}, // this doesn't work for some reason
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey, // contract identifier, like the contract address. since we didn't need it, we prefixed it with an underscore
    accounts: &[AccountInfo], // accounts involved in this transaction. this must include CounterAccount
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    // here we know that we have only one account to work with
    // if we had more, we would need to loop and call next_account_info multiple times

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => counter_account.counter += args.amount,
        CounterInstructions::Decrement(args) => {
            if args.amount > counter_account.counter {
                counter_account.counter = 0;
            } else {
                counter_account.counter -= args.amount;
            }
        }
        CounterInstructions::Update(args) => counter_account.counter = args.value,
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let account_key = Pubkey::default();
        let mut account_lamports = 0;
        let mut account_data = vec![0; mem::size_of::<u32>()];
        let account_owner = Pubkey::default();

        let counter_account = AccountInfo::new(
            &account_key,
            false, // is this account going to sign transactions?
            true,
            &mut account_lamports,
            &mut account_data,
            &account_owner,
            false,            // is this account a program?
            Epoch::default(), // when the next rent fee is due?
        );
        let accounts = vec![counter_account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut decrement2_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        // test increment
        let increment_amount = 7u32;
        increment_instruction_data.extend_from_slice(&increment_amount.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            7
        );

        // test decrement
        let decrement_amount = 5u32;
        decrement_instruction_data.extend_from_slice(&decrement_amount.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );

        // test decrement below zero
        let decrement2_amount = 99u32;
        decrement2_instruction_data.extend_from_slice(&decrement2_amount.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // test update
        let update_value = 42u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            42
        );

        // test reset
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
