#![no_std]

use escrow_io::*;
use gmeta::metawasm;
use gstd::prelude::*;
use primitive_types::U256;

#[metawasm]
pub trait Metawasm {
    type State = BTreeMap<WalletId, Wallet>;

    fn info(wallet_id: U256, state: Self::State) -> Wallet {
        *state
            .get(&wallet_id)
            .unwrap_or_else(|| panic!("Wallet with the {wallet_id} ID doesn't exist"))
    }

    fn created_wallets(state: Self::State) -> Vec<(WalletId, Wallet)> {
        state
            .iter()
            .map(|(wallet_id, wallet)| (*wallet_id, *wallet))
            .collect()
    }
}
