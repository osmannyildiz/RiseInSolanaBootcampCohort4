use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

pub enum TokenTransferInstructions {
    Transfer(TransferArgs),
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TransferArgs {
    pub amount: u64,
}

impl TokenTransferInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Transfer(TransferArgs::try_from_slice(rest).unwrap()),
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
