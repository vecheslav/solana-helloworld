//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::sysvar;

/// Instructions supported by the program
#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum ContractInstruction {
    /// Init contract
    Init,

    /// Swap
    Swap,
}

pub fn initialize(
    program_id: Pubkey,
    helloworld: Pubkey,
    liquidity_mint: Pubkey,
    liquidity_token_account: Pubkey,
    collateral_mint: Pubkey,
    authority: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(helloworld, false),
        AccountMeta::new_readonly(liquidity_mint, false),
        AccountMeta::new(liquidity_token_account, false),
        AccountMeta::new(collateral_mint, false),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    Instruction::new_with_borsh(program_id, &ContractInstruction::Init, accounts)
}
