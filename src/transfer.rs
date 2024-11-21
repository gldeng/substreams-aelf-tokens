use substreams::errors::Error;
use crate::pb::sf::substreams::aelf::token::v1::{Events, Transfer, Transfers};
use prost::Message;
use crate::pb::token::Transferred;

#[substreams::handlers::map]
fn all_transfers(events: Events) -> Result<Transfers, Error> {
    let transfers: Vec<Transfer> = events.events.iter().flat_map(|evt| {
        evt.log.iter().map(|log| {
            let mut transferred: Transferred = Transferred::default();
            let _ = transferred.merge(&*log.non_indexed.as_slice());
            log.indexed.iter().for_each(|data| { let _ = transferred.merge(&*data.as_slice()); });
            Transfer {
                contract: log.address.clone(),
                from: transferred.from.clone().map_or("".to_string(), |addr| addr.to_b58()),
                to: transferred.to.clone().map_or("".to_string(), |addr| addr.to_b58()),
                symbol: transferred.symbol,
                amount: transferred.amount.to_string(),
                memo: transferred.memo,
            }
        })
    }).collect();
    Ok(Transfers {
        transfers
    })
}