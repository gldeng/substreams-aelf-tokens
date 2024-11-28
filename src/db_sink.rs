use crate::pb::sf::substreams::aelf::token::v1::{BalanceUpdates, Transfers};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetBigInt};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

fn db_write_balances(
    tables: &mut Tables,
    clock: &Clock,
    balance_updates: BalanceUpdates,
    balances_store: &StoreGetBigInt,
) {
    let Clock {
        id: block_hash,
        number: block_num,
        timestamp,
    } = clock.clone();
    let timestamp = timestamp.unwrap_or_default();

    let mut ordinal = 0;
    for balance_update in balance_updates.balance_updates {
        let key = crate::balance::get_balance_key(&balance_update);
        let table_key = [
            ("contract", (&balance_update).contract.to_string()),
            ("symbol", (&balance_update).symbol.to_string()),
            ("owner", (&balance_update).owner.to_string()),
        ];
        let balance_row = match balances_store.get_at(ordinal - 1, key) {
            Some(_) => tables.update_row("balances", table_key),
            _ => tables.create_row("balances", table_key),
        };

        balance_row
            .set("contract", balance_update.contract.clone())
            .set("symbol", balance_update.symbol.clone())
            .set("owner", balance_update.owner.clone())
            .set("balance", balance_update.new_balance.clone())
            .set("transaction", balance_update.transaction.clone())
            .set("call_path", balance_update.call_path.clone())
            .set("block_num", block_num)
            .set("block_hash", block_hash.clone())
            .set("timestamp", &timestamp);

        tables
            .create_row(
                "balance_updates",
                [
                    ("block_hash", block_hash.to_string()),
                    ("ordinal", ordinal.to_string()),
                ],
            )
            .set("contract", balance_update.contract)
            .set("symbol", balance_update.symbol)
            .set("owner", balance_update.owner)
            .set("balance", balance_update.new_balance)
            .set("transaction", balance_update.transaction)
            .set("call_path", balance_update.call_path)
            .set("block_num", block_num)
            .set("block_hash", block_hash.to_string())
            .set("ordinal", ordinal.to_string())
            .set("timestamp", &timestamp);
        ordinal += 1;
    }
}

fn db_write_transfers(tables: &mut Tables, clock: &Clock, transfers: Transfers) {
    let Clock {
        id: block_hash,
        number: block_num,
        timestamp,
    } = clock.clone();
    let timestamp = timestamp.unwrap_or_default();

    let mut ordinal = 0;
    for transfer in transfers.transfers {
        tables
            .create_row(
                "transfers",
                [
                    ("block_hash", block_hash.to_string()),
                    ("ordinal", ordinal.to_string()),
                ],
            )
            .set("contract", transfer.contract)
            .set("symbol", transfer.symbol)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("amount", transfer.amount)
            .set("memo", transfer.memo)
            .set("block_num", block_num)
            .set("block_hash", block_hash.to_string())
            .set("ordinal", ordinal.to_string())
            .set("transaction", transfer.tx_id)
            .set("call_path", transfer.call_path)
            .set("timestamp", timestamp.to_string());
        ordinal += 1;
    }
}

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    transfers: Transfers,
    balance_updates: BalanceUpdates,
    balances_store: StoreGetBigInt,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    db_write_transfers(&mut tables, &clock, transfers);
    db_write_balances(&mut tables, &clock, balance_updates, &balances_store);
    Ok(tables.to_database_changes())
}
