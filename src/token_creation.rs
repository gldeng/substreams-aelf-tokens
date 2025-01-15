use substreams::errors::Error;
use prost::Message;
use crate::pb::sf::substreams::aelf::token::v1::{TokenCreation, TokenCreations};
use crate::pb::sf::substreams::aelf::v1::Events;
use crate::pb::token::TokenCreated;

#[substreams::handlers::map]
fn all_token_creations(events: Events) -> Result<TokenCreations, Error> {
    let token_creations: Vec<TokenCreation> = events.events.iter().flat_map(|evt| {
        evt.log.iter().map(|log| {
            let mut evt_msg: TokenCreated = TokenCreated::default();
            let _ = evt_msg.merge(log.non_indexed.as_slice());
            log.indexed.iter().for_each(|data| { let _ = evt_msg.merge(data.as_slice()); });
            let mut out_evt = TokenCreation {
                contract: log.address.clone(),
                tx_id: evt.tx_id.clone(),
                call_path: evt.call_path.clone(),
                owner: evt_msg.owner.clone().map_or("".to_string(), |addr| addr.to_b58()),
                symbol: evt_msg.symbol,
                token_name: evt_msg.token_name.to_string(),
                total_supply: evt_msg.total_supply.to_string(),
                decimals: evt_msg.decimals.to_string(),
                issuer: evt_msg.issuer.clone().map_or("".to_string(), |addr| addr.to_b58()),
                is_burnable: evt_msg.is_burnable,
                issue_chain_id: evt_msg.issue_chain_id,
                external_info: Default::default(),
            };
            match evt_msg.external_info {
                Some(info) => {
                    info.value.iter().for_each(|(k, v)| {
                        out_evt.external_info.insert(k.clone(), v.clone());
                    })
                }
                _ => {}
            };
            out_evt
        })
    }).collect();
    Ok(TokenCreations {
        token_creations,
        clock: events.clock,
    })
}