use crate::instructions::ReviewInstructions;
use crate::state::{AccountState, ReviewError};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked, // for Solana Playground, change this as "borsh0_10"
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
            title: _,
            description,
            rating,
        } => update_review(program_id, accounts, description, rating),
    }
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
        return Err(ReviewError::InvalidPDA.into());
    }

    if rating < 1 || rating > 10 {
        msg!("Invalid rating");
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
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap(); // unpacking state account's state

    if account_state.is_initialized {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_state.title = title;
    account_state.description = description;
    account_state.rating = rating;
    account_state.is_initialized = true;

    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?; // serializing state account's state

    Ok(())
}

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    description: String,
    rating: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let initializer_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;

    if pda_account.owner != program_id {
        msg!("Illegal owner");
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer_account.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_state =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap(); // unpacking state account's state

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[
            initializer_account.key.as_ref(),
            account_state.title.as_bytes().as_ref(),
        ],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    if !account_state.is_initialized {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    if rating < 1 || rating > 10 {
        msg!("Invalid rating");
        return Err(ReviewError::InvalidRating.into());
    }

    msg!("Review before update:");
    msg!("- Title: {}", account_state.title);
    msg!("- Description: {}", account_state.description);
    msg!("- Rating: {}", account_state.rating);

    account_state.description = description;
    account_state.rating = rating;
    // we don't want to change the title since we use it as an identifier

    msg!("Review after update:");
    msg!("- Title: {}", account_state.title);
    msg!("- Description: {}", account_state.description);
    msg!("- Rating: {}", account_state.rating);

    account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?; // serializing state account's state

    Ok(())
}
