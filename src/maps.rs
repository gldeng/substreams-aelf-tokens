use std::io::Cursor;
use std::ptr::hash;
use substreams::errors::Error;
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

mod trace_utils {
    use substreams_aelf_core::pb::aelf::*;

    enum TraceGroup {
        Pre,
        Self_,
        Inline,
        Post,
    }

    pub struct TransactionTractStateIterator<'a> {
        current_trace_group: TraceGroup,
        current_index: usize,
        is_successful: bool,
        this_trace: &'a TransactionTrace,
        pre_traces_iters: Vec<TransactionTractStateIterator<'a>>,
        inline_traces_iters: Vec<TransactionTractStateIterator<'a>>,
        post_traces_iters: Vec<TransactionTractStateIterator<'a>>,
    }

    fn is_successful<'a>(trace: &'a TransactionTrace) -> bool {
        if trace.execution_status != substreams_aelf_core::pb::aelf::ExecutionStatus::Executed.into() { return false; }
        if trace.pre_traces.iter().any(|trace| !is_successful(trace)) { return false; }
        if trace.inline_traces.iter().any(|trace| !is_successful(trace)) { return false; }
        if trace.post_traces.iter().any(|trace| !is_successful(trace)) { return false; }
        return true;
    }

    impl<'a> TransactionTractStateIterator<'a> {
        pub fn new(trace: &'a TransactionTrace) -> Self {
            let successful = is_successful(trace);
            let pre_traces_iters = trace.pre_traces.iter().filter(|trace| is_successful(*trace)).map(|t| Self::new(t)).collect();
            let inline_traces_iters = if successful {
                trace.inline_traces.iter().map(|t| Self::new(t)).collect()
            } else {
                vec![]
            };
            let post_traces_iters = trace.post_traces.iter().filter(|trace| is_successful(*trace)).map(|t| Self::new(t)).collect();
            Self {
                current_trace_group: TraceGroup::Pre,
                current_index: 0,
                is_successful: successful,
                this_trace: trace,
                pre_traces_iters,
                inline_traces_iters,
                post_traces_iters,
            }
        }


        fn next_pre(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
            if self.current_index < self.pre_traces_iters.len() {
                self.pre_traces_iters[self.current_index].next()
            } else {
                None
            }
        }
        fn next_inline(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
            if self.current_index < self.inline_traces_iters.len() {
                self.inline_traces_iters[self.current_index].next()
            } else {
                None
            }
        }
        fn next_post(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
            if self.current_index < self.post_traces_iters.len() {
                self.post_traces_iters[self.current_index].next()
            } else {
                None
            }
        }
    }

    impl<'a> Iterator for TransactionTractStateIterator<'a> {
        type Item = &'a TransactionTrace;


        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.current_trace_group {
                    TraceGroup::Pre => {
                        if let Some(trace) = self.next_pre() {
                            return Some(trace);
                        } else {
                            self.current_trace_group = TraceGroup::Self_;
                            self.current_index = 0;
                        }
                    }
                    TraceGroup::Self_ => {
                        self.current_trace_group = TraceGroup::Inline;
                        self.current_index = 0;
                        return Some(self.this_trace);
                    }
                    TraceGroup::Inline => {
                        if let Some(trace) = self.next_inline() {
                            self.current_index += 1;
                            return Some(trace);
                        }
                        self.current_trace_group = TraceGroup::Post;
                        self.current_index = 0;
                    }
                    TraceGroup::Post => {
                        return self.next_post();
                    }
                }
            }
        }
    }
}


#[substreams::handlers::map]
fn all_state_updates(blk: Block) -> Result<StateUpdates, Error> {
    let updates: Vec<StateUpdate> = blk.firehose_body.iter()
        .flat_map(|body| {
            body.transaction_traces.iter().flat_map(
                |trace| trace_utils::TransactionTractStateIterator::new(trace).into_iter().flat_map(
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
    let token_address = address!("JRmBduh4nXWi1aXgdUsj5gJrzeZb2LxmrAbf7W99faZSvoAaE");
    if token_address.is_none() {
        return Err(anyhow!("invalid contract address"));
    }
    let balance_changes: Vec<BalanceChange> = state_updates.updates.iter().filter(
        |u| u.key.starts_with("JRmBduh4nXWi1aXgdUsj5gJrzeZb2LxmrAbf7W99faZSvoAaE/Balances")
    ).filter_map(
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
        let mut data = vec![0xf2u8, 0xc0u8, 0x01u8];
        let val = decode_signed_varint64(&mut data.as_slice()).unwrap();
        assert_eq!(val, 12345);
    }
}