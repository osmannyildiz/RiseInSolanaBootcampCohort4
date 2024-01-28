use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token::{
        instruction::transfer_checked,
        state::{Account, Mint},
    },
};

solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let source_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?; // information about the token we're transferring
    let destination_account_info = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?; // the program of the token we're transferring

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);
    if expected_authority != *authority_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // let source_account = Account::unpack(&source_account_info.try_borrow_data()?)?;
    // let transfer_amount = source_account.amount; // we are transferring the whole balance of the source account
    let transfer_amount = 7000;

    let mint = Mint::unpack(&mint_info.try_borrow_data()?)?;
    let decimals = mint.decimals;

    // invoke the transfer
    msg!("Attempting to transfer {} tokens", transfer_amount);
    invoke_signed(
        &transfer_checked(
            token_program_info.key,
            source_account_info.key,
            mint_info.key,
            destination_account_info.key,
            authority_info.key,
            &[], // in this case, signer is this account, so we leave it blank. if it was a multisig account, we could give other accounts' public keys here (?)
            transfer_amount,
            decimals,
        )
        .unwrap(),
        &[
            source_account_info.clone(),
            mint_info.clone(),
            destination_account_info.clone(),
            authority_info.clone(),
            token_program_info.clone(), // not required, but better for clarity
        ],
        &[&[b"authority", &[bump_seed]]],
    )
}
