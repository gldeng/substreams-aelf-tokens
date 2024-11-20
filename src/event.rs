use substreams::errors::Error;
use substreams_aelf_core::pb::aelf::Block;
use log::log;
use crate::pb::sf::substreams::aelf::token::v1::{Event, Events, StateUpdate, StateUpdates};
use crate::pb::sf::substreams::v1::Clock;
use crate::utils::TransactionTractStateIterator;
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