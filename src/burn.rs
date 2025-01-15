use substreams::errors::Error;
use prost::Message;
use crate::pb::sf::substreams::aelf::token::v1::{Burn, Burns};
use crate::pb::sf::substreams::aelf::v1::Events;
use crate::pb::token::Burned;

#[substreams::handlers::map]
fn all_burns(events: Events) -> Result<Burns, Error> {
    let burns: Vec<Burn> = events.events.iter().flat_map(|evt| {
        evt.log.iter().map(|log| {
            let mut evt_msg: Burned = Burned::default();
            let _ = evt_msg.merge(log.non_indexed.as_slice());
            log.indexed.iter().for_each(|data| { let _ = evt_msg.merge(data.as_slice()); });
            Burn {
                contract: log.address.clone(),
                burner: evt_msg.burner.clone().map_or("".to_string(), |addr| addr.to_b58()),
                symbol: evt_msg.symbol,
                amount: evt_msg.amount.to_string(),
                tx_id: evt.tx_id.clone(),
                call_path: evt.call_path.clone(),
            }
        })
    }).collect();
    Ok(Burns {
        burns,
        clock: events.clock,
    })
}