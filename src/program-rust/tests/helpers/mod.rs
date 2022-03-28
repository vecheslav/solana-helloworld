#![allow(dead_code)]

use helloworld::state::ContractData;
use solana_program::{
    borsh::get_packed_len, hash::Hash, program_pack::Pack, pubkey::Pubkey, system_instruction,
};
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::{
    signature::Keypair, signer::Signer, transaction::Transaction, transport::TransportError,
};
use spl_token as token;

#[derive(Debug)]
pub struct PoolAccounts {
    pub helloworld: Keypair,
    pub liquidity_owner: Keypair,
    pub liquidity_mint: Keypair,
    pub liquidity_token_account: Keypair,
    pub collateral_mint: Keypair,
}

impl PoolAccounts {
    pub fn new() -> Self {
        Self {
            helloworld: Keypair::new(),
            liquidity_owner: Keypair::new(),
            liquidity_mint: Keypair::new(),
            liquidity_token_account: Keypair::new(),
            collateral_mint: Keypair::new(),
        }
    }
}

pub async fn create_accounts(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    pool_accounts: &PoolAccounts,
) {
    // Create token (incoming)
    create_mint(
        banks_client,
        &payer,
        &recent_blockhash,
        &pool_accounts.liquidity_mint,
        &pool_accounts.liquidity_owner.pubkey(),
    )
    .await
    .unwrap();

    // Pool accounts
    create_contract_accounts(
        banks_client,
        payer,
        recent_blockhash,
        &pool_accounts.helloworld,
        &pool_accounts.collateral_mint,
        &pool_accounts.liquidity_token_account,
    )
    .await
    .unwrap();
}

/// Create pool accounts
pub async fn create_contract_accounts(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    helloworld: &Keypair,
    collateral_mint: &Keypair,
    liquidity_token_account: &Keypair,
) -> Result<(), TransportError> {
    let rent = banks_client.get_rent().await.unwrap();

    let helloworld_rent = rent.minimum_balance(get_packed_len::<ContractData>());
    let collateral_mint_rent = rent.minimum_balance(token::state::Mint::LEN);
    let liquidity_token_account_rent = rent.minimum_balance(token::state::Account::LEN);

    let mut tx = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &helloworld.pubkey(),
                helloworld_rent,
                get_packed_len::<ContractData>() as u64,
                &helloworld::id(),
            ),
            // Pool mint account
            system_instruction::create_account(
                &payer.pubkey(),
                &collateral_mint.pubkey(),
                collateral_mint_rent,
                token::state::Mint::LEN as u64,
                &token::id(),
            ),
            // Liquidity token account
            system_instruction::create_account(
                &payer.pubkey(),
                &liquidity_token_account.pubkey(),
                liquidity_token_account_rent,
                token::state::Account::LEN as u64,
                &token::id(),
            ),
        ],
        Some(&payer.pubkey()),
    );

    tx.sign(
        &[payer, helloworld, collateral_mint, liquidity_token_account],
        *recent_blockhash,
    );
    banks_client.process_transaction(tx).await?;

    Ok(())
}

pub async fn create_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    recent_blockhash: &Hash,
    mint: &Keypair,
    manager: &Pubkey,
) -> Result<(), TransportError> {
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(token::state::Mint::LEN);

    let mut tx = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                mint_rent,
                token::state::Mint::LEN as u64,
                &token::id(),
            ),
            token::instruction::initialize_mint(&token::id(), &mint.pubkey(), &manager, None, 0)
                .unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    tx.sign(&[payer, mint], *recent_blockhash);
    banks_client.process_transaction(tx).await?;

    Ok(())
}

pub async fn get_account(banks_client: &mut BanksClient, pubkey: &Pubkey) -> Account {
    banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "helloworld",
        helloworld::id(),
        processor!(helloworld::processor::process_instruction),
    )
}
