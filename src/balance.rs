use crate::pb::sf::substreams::aelf::token::v1::{BalanceUpdate, BalanceUpdates};
use crate::pb::sf::substreams::aelf::v1::StateUpdates;
use substreams::errors::Error;

use anyhow::Result;

use prost::encoding::decode_varint;

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
