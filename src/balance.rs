use crate::pb::sf::substreams::aelf::token::v1::{BalanceUpdate, BalanceUpdates};
use crate::pb::sf::substreams::aelf::v1::StateUpdates;
use substreams::errors::Error;

use anyhow::Result;

use prost::encoding::decode_varint;
use substreams::store::{StoreNew, StoreSet, StoreSetBigInt};
use std::str::FromStr;
use substreams::prelude::BigInt;

#[substreams::handlers::map]
fn all_balance_updates(state_updates: StateUpdates) -> Result<BalanceUpdates, Error> {
    let balance_updates: Vec<BalanceUpdate> = state_updates
        .updates
        .iter()
        .filter_map(|u| {
            let splits: Vec<String> = u.key.clone().split("/").map(|s| s.to_string()).collect();
            match splits.as_slice() {
                [contract, _, owner, symbol] => decode_signed_varint64(&mut u.value.as_slice())
                    .map(|bal| BalanceUpdate {
                        contract: contract.to_string(),
                        symbol: symbol.to_string(),
                        owner: owner.trim_matches('"').to_string(),
                        new_balance: bal.to_string(),
                        transaction: u.tx_id.clone(),
                        call_path: u.call_path.to_string(),
                    })
                    .ok(),
                _ => None,
            }
        })
        .collect();
    Ok(BalanceUpdates {
        balance_updates,
        clock: state_updates.clock,
    })
}

#[substreams::handlers::store]
fn store_balances(balance_updates: BalanceUpdates, store: StoreSetBigInt) {
    let mut ordinal = 0;
    for balance_update in balance_updates.balance_updates {
        let key = get_balance_key(&balance_update);
        store.set(ordinal, key, &BigInt::from_str(&balance_update.new_balance).unwrap_or_default());
        ordinal += 1;
    }
}

pub(crate) fn get_balance_key(balance_update: &BalanceUpdate) -> String {
    format!("{}:{}:{}", balance_update.contract, balance_update.symbol, balance_update.owner)
}

fn decode_signed_varint64(bytes: &mut impl prost::bytes::Buf) -> Result<i64> {
    // First, decode it as an unsigned varint.
    let value = decode_varint(bytes)?;

    // Perform zigzag decoding to get the signed integer.
    let signed_val = ((value >> 1) as i64) ^ (-((value & 1) as i64));
    Ok(signed_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_varint64() {
        let data = vec![0xf2u8, 0xc0u8, 0x01u8];
        let val = decode_signed_varint64(&mut data.as_slice()).unwrap();
        assert_eq!(val, 12345);
    }
}
