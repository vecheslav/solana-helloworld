use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Debug, Default, PartialEq)]
pub struct ContractData {
    pub authority: Pubkey,
    pub liquidity_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub liquidity_token_account: Pubkey,
    pub authority_bump_seed: u8,
}
