mod helpers;
use crate::helpers::{create_accounts, get_account, program_test, PoolAccounts};
use borsh::BorshDeserialize;
use helloworld::find_authority_bump_seed;
use helloworld::instruction::initialize;
use helloworld::state::ContractData;
use solana_program_test::*;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_initialize() {
    let pool_accounts = PoolAccounts::new();
    let (mut banks_client, payer, recent_blockhash) =
        program_test(pool_accounts.program_id.pubkey())
            .start()
            .await;

    create_accounts(&mut banks_client, &payer, &recent_blockhash, &pool_accounts).await;

    let (authority, _) = find_authority_bump_seed(
        &pool_accounts.program_id.pubkey(),
        &pool_accounts.helloworld.pubkey(),
    );

    let instruction = initialize(
        pool_accounts.program_id.pubkey(),
        pool_accounts.helloworld.pubkey(),
        pool_accounts.liquidity_mint.pubkey(),
        pool_accounts.liquidity_token_account.pubkey(),
        pool_accounts.collateral_mint.pubkey(),
        authority,
    );

    let mut tx =
        Transaction::new_with_payer(&[instruction], Some(&pool_accounts.helloworld.pubkey()));

    tx.sign(&[&pool_accounts.helloworld], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    let helloworld_account =
        get_account(&mut banks_client, &pool_accounts.helloworld.pubkey()).await;

    let helloworld_data = ContractData::try_from_slice(&helloworld_account.data).unwrap();

    assert_eq!(helloworld_account.owner, pool_accounts.program_id.pubkey());
    // assert_eq!(
    //     helloworld_data.liquidity_token_account,
    //     pool_accounts.liquidity_token_account.pubkey()
    // );
}
