use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use crate::pb::sf::substreams::aelf;

use std::io::Cursor;
use std::ptr::hash;
use crate::pb::token;
use anyhow::{anyhow, Result};

use substreams_aelf_core::pb::aelf::{Address, LogEvent, TransactionExecutingStateSet, TransactionTrace};
use substreams_aelf_core::pb::aelf::Block;
use substreams_aelf_core::pb_ext::*;
use substreams_aelf::address;
use crate::pb::sf::substreams::aelf::token::v1::{BalanceChange, BalanceChanges, StateUpdate, StateUpdates};
use crate::pb::token::Transferred;
use prost::Message;
use prost::encoding::{decode_varint, varint};
use std::borrow::BorrowMut;
use crate::utils::TransactionTractStateIterator;

#[substreams::handlers::map]
fn all_state_updates(blk: Block) -> Result<StateUpdates, Error> {
    let updates: Vec<StateUpdate> = blk.firehose_body.iter()
        .flat_map(|body| {
            body.transaction_traces.iter().flat_map(
                |trace| TransactionTractStateIterator::new(trace).into_iter().flat_map(
                    |tr| {
                        let tx_id = tr.transaction_id.clone();
                        tr.state_set.iter().flat_map(move |s| s.writes.iter().map({
                            let tx_id = tx_id.clone().map(|x| {
                                x.to_hex()
                            }).unwrap_or_else(|| "0000000000000000000000000000000000000000000000000000000000000000".to_string());
                            move |(k, v)| StateUpdate {
                                tx_id: tx_id.clone(),
                                key: k.to_string(),
                                value: v.clone(),
                            }
                        }))
                    }))
        }).collect();
    Ok(StateUpdates {
        updates
    })
}

#[substreams::handlers::map]
fn all_balance_changes(state_updates: StateUpdates) -> Result<BalanceChanges, Error> {
    let balance_changes: Vec<BalanceChange> = state_updates.updates.iter()
        .filter_map(
            |u| {
                let splits: Vec<String> = u.key.clone().split("/").map(|s| s.to_string()).collect();
                match splits.as_slice() {
                    [contract, _, owner, symbol] => {
                        decode_signed_varint64(&mut u.value.as_slice()).map(
                            |bal| BalanceChange {
                                contract: "JRmBduh4nXWi1aXgdUsj5gJrzeZb2LxmrAbf7W99faZSvoAaE".to_string(),
                                symbol: symbol.to_string(),
                                owner: owner.trim_matches('"').to_string(),
                                new_balance: bal.to_string(),
                                old_balance: "0".to_string(),
                                transfer_value: "0".to_string(),
                                transaction: u.tx_id.clone(),
                            }
                        ).ok()
                    }
                    _ => {
                        None
                    }
                }
            }
        ).collect();
    Ok(BalanceChanges {
        balance_changes
    })
}

#[substreams::handlers::map]
fn filtered_state_updates(query: String, state_updates: StateUpdates) -> Result<StateUpdates, Error> {
    let filtered = state_updates.updates.into_iter().filter(|s| state_matches(s, &query).unwrap_or(false)).collect();
    Ok(StateUpdates {
        updates: filtered
    })
}


fn decode_signed_varint64(bytes: &mut impl prost::bytes::Buf) -> Result<i64> {
    // First, decode it as an unsigned varint.
    let value = decode_varint(bytes)?;

    // Perform zigzag decoding to get the signed integer.
    let signed_val = ((value >> 1) as i64) ^ (-((value & 1) as i64));
    Ok(signed_val)
}

pub fn state_matches(state_update: &aelf::token::v1::StateUpdate, query: &str) -> Result<bool, Error> {
    matches_keys_in_parsed_expr(&state_keys(state_update), query)
}


pub fn state_keys(state_update: &aelf::token::v1::StateUpdate) -> Vec<String> {
    state_update.key.split("/").enumerate()
        .map(|(i, part)| format!("st_{}:{}", i, part)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_varint64() {
        let mut data = vec![0xf2u8, 0xc0u8, 0x01u8];
        let val = decode_signed_varint64(&mut data.as_slice()).unwrap();
        assert_eq!(val, 12345);
    }
}