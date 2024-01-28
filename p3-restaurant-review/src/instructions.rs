use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum ReviewInstructions {
    AddReview {
        title: String,
        description: String,
        rating: u8,
        location: String,
    },
    UpdateReview {
        title: String,
        description: String,
        rating: u8,
        location: String,
    },
}

#[derive(BorshDeserialize)]
struct ReviewPayload {
    title: String,
    description: String,
    rating: u8,
    location: String,
}

impl ReviewInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ReviewPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddReview {
                title: payload.title,
                description: payload.description,
                rating: payload.rating,
                location: payload.location,
            },
            1 => Self::UpdateReview {
                title: payload.title,
                description: payload.description,
                rating: payload.rating,
                location: payload.location,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
