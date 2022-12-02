use escrow::io::EscrowAction;
use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};

#[cfg(debug_assertions)]
const PATH: &str = "./target/wasm32-unknown-unknown/debug/escrow.opt.wasm";

#[cfg(not(debug_assertions))]
const PATH: &str = "./target/wasm32-unknown-unknown/release/staking.opt.wasm";

pub const FT_PROGRAM_ID: u64 = 2;
pub const ESCROW_PROGRAM_ID: u64 = 13370;
pub const FOREIGN_USER: u64 = 1337;
pub const BUYER: [u64; 2] = [12, 34];
pub const SELLER: [u64; 2] = [56, 78];
pub const AMOUNT: [u128; 2] = [12345, 54321];
pub const WALLET: [u128; 2] = [0, 1];
pub const AMOUNT_REMAINDER: u128 = 20000;
pub const NONEXISTENT_WALLET: u128 = 999999;

#[tokio::test]
async fn init() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::bytes_now(),
            escrow_create_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
