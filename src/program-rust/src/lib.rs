//! A program for simple token pool

pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use solana_program::pubkey::Pubkey;

solana_program::declare_id!("9do3HnZWufwTcjnP6MPETTqs1m4tJpJiyY89NgYeDh46");

/// Generates seed bump for stake pool authorities
pub fn find_authority_bump_seed(program_id: &Pubkey, pool: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[&pool.to_bytes()[..32]], program_id)
}
