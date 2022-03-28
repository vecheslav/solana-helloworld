//! A program for simple token pool

pub mod instruction;
pub mod processor;
pub mod state;

mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use solana_program::pubkey::Pubkey;

/// Generates seed bump for stake pool authorities
pub fn find_authority_bump_seed(program_id: &Pubkey, pool: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[&pool.to_bytes()[..32]], program_id)
}
