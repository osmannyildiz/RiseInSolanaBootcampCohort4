use crate::instructions::ReviewInstructions;
use crate::state::{AccountState, ReviewError};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

pub mod instructions;
pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ReviewInstructions::unpack(instruction_data)?;

    match instruction {
        ReviewInstructions::AddReview {
            title,
            description,
            rating,
        } => add_review(program_id, accounts, title, description, rating),
        ReviewInstructions::UpdateReview {
            title,
            description,
            rating,
        } => update_review(program_id, accounts, title, description, rating),
    }

    Ok(())
}

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    rating: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let initializer_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !initializer_account.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer_account.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if rating < 1 || rating > 10 {
        return Err(ReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer_account.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer_account.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer_account.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;
    msg!("PDA created: {}", pda);

    let mut account_state =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap(); // unpacking account state
    if account_state.is_initialized {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_state.title = title;
    account_state.description = description;
    account_state.rating = rating;
    account_state.is_initialized = true;

    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?; // serializing account state

    Ok(())
}
