use substreams::errors::Error;
use substreams_aelf_core::pb::aelf::{Block, LogEvent};
use log::log;
use substreams::matches_keys_in_parsed_expr;
use crate::pb::sf::substreams::aelf;
use crate::pb::sf::substreams::aelf::token::v1::{Event, Events, StateUpdate, StateUpdates};
use crate::pb::sf::substreams::v1::Clock;
use crate::utils::TransactionTractStateIterator;
use substreams_aelf_core::pb_ext;
use crate::state::state_matches;

#[substreams::handlers::map]
fn all_events(blk: Block) -> Result<Events, Error> {
    let events: Vec<Event> = blk.firehose_body.iter()
        .flat_map(|body| {
            body.transaction_traces.iter().flat_map(
                |trace| TransactionTractStateIterator::new(trace).into_iter().flat_map(
                    |tr| {
                        let tx_id = tr.transaction_id.clone();
                        tr.logs.iter().map(
                            {
                                let tx_id = tx_id.clone().map(|x| {
                                    x.to_hex()
                                }).unwrap_or_else(|| "0000000000000000000000000000000000000000000000000000000000000000".to_string());
                                move |log| Event {
                                    log: Some(log.clone()),
                                    tx_id: tx_id.clone(),
                                }
                            })
                    }))
        }).collect();
    Ok(Events {
        events,
        clock: None, // TODO: Add clock
    })
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, Error> {
    let filtered = events.events.into_iter()
        .filter(|e| {
            if let Some(log) = &e.log {
                evt_matches(log, &query).expect("matching calls from query")
            } else {
                false
            }
        }).collect();
    Ok(Events {
        events: filtered,
        clock: events.clock,
    })
}


pub fn evt_matches(log: &LogEvent, query: &str) -> anyhow::Result<bool, Error> {
    matches_keys_in_parsed_expr(&evt_keys(log), query)
}


pub fn evt_keys(log: &LogEvent) -> Vec<String> {
    let mut keys = Vec::new();

    // TODO: Add topics
    // if evt.log.len() > 0 {
    //     let k_log_sign = format!("evt_sig:0x{}", Hex::encode(log.topics.get(0).unwrap()));
    //     keys.push(k_log_sign);
    // }

    let k_log_address = format!("evt_addr:{}", log.address.clone().map_or("".to_string(), |addr| addr.to_b58()));
    keys.push(k_log_address);
    let k_log_name = format!("evt_name:{}", log.name);
    keys.push(k_log_name);
    keys
}

