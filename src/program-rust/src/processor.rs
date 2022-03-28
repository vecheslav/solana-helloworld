use crate::find_authority_bump_seed;
use crate::instruction::ContractInstruction;
use crate::state::ContractData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::{initialize_account, initialize_mint};

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    input: &[u8],        // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    let instruction = ContractInstruction::try_from_slice(input)?;

    match instruction {
        ContractInstruction::Init => {
            let accounts_iter = &mut accounts.iter();

            let helloworld_account_info = next_account_info(accounts_iter)?; // storage for this contract
            let liquidity_mint_info = next_account_info(accounts_iter)?; // Token to be used as provided liquidity
            let liquidity_token_account_info = next_account_info(accounts_iter)?; // Account which will receive this liquidity
            let collateral_mint_info = next_account_info(accounts_iter)?; // Collateral mint account to store new Token (mint)
            let authority_info = next_account_info(accounts_iter)?;
            let rent_info = next_account_info(accounts_iter)?; // Rent info is a system account that is used to calculate rent checks
            let _spl_token_info = next_account_info(accounts_iter)?;

            // The account must be owned by the program in order to modify its data (it's init offchain with proper ID)
            if helloworld_account_info.owner != program_id {
                msg!("Contract account does not have the correct program id");
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut contract_data: ContractData =
                ContractData::try_from_slice(&helloworld_account_info.data.borrow())?;

            if contract_data != ContractData::default() {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            // Check that provided authority account is correct
            let (authority, authority_bump_seed) =
                find_authority_bump_seed(program_id, &helloworld_account_info.key);

            if *authority_info.key != authority {
                return Err(ProgramError::InvalidArgument);
            }

            // Initialize account for spl token
            spl_initialize_account(
                liquidity_token_account_info.clone(),
                liquidity_mint_info.clone(),
                authority_info.clone(),
                rent_info.clone(),
            )?;

            // Unpack data from account
            let liquidity_mint =
                spl_token::state::Mint::unpack_from_slice(&liquidity_mint_info.data.borrow())?;

            // Initialize new mint (token) for collateral asset
            spl_initialize_mint(
                collateral_mint_info.clone(),
                authority_info.clone(),
                rent_info.clone(),
                liquidity_mint.decimals, // new mint will be init with same decimals
            )?;

            let rent = &Rent::from_account_info(rent_info)?;

            // Check rent for contract info
            if !rent.is_exempt(
                helloworld_account_info.lamports(),
                helloworld_account_info.data_len(),
            ) {
                return Err(ProgramError::AccountNotRentExempt);
            }

            contract_data.authority = *authority_info.key;
            contract_data.liquidity_mint = *liquidity_mint_info.key;
            contract_data.collateral_mint = *collateral_mint_info.key;
            contract_data.liquidity_token_account = *liquidity_token_account_info.key;
            contract_data.authority_bump_seed = authority_bump_seed;

            contract_data.serialize(&mut *helloworld_account_info.data.borrow_mut())?;
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

/// Create an account instruction.
pub fn spl_initialize_account<'a>(
    account: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    autority: AccountInfo<'a>,
    rent: AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let ix = initialize_account(&spl_token::id(), account.key, mint.key, autority.key)?;

    invoke(&ix, &[account, mint, autority, rent])
}

/// Create a mint instruction.
pub fn spl_initialize_mint<'a>(
    mint: AccountInfo<'a>,
    mint_authority: AccountInfo<'a>,
    rent: AccountInfo<'a>,
    decimals: u8,
) -> Result<(), ProgramError> {
    let ix = initialize_mint(
        &spl_token::id(),
        mint.key,
        mint_authority.key,
        None,
        decimals,
    )?;

    invoke(&ix, &[mint, rent])
}
