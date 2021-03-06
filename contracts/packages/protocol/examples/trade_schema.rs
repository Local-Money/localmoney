use cosmwasm_schema::{export_schema_with_title, remove_schemas};
use localterra_protocol::trade::{ExecuteMsg, InstantiateMsg, QueryMsg, Trade, TradeState};
use schemars::schema_for;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/trade");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg),
        &out_dir,
        "trade_instantiate_msg",
    );
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "trade_execute_msg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "trade_query_msg");
    export_schema_with_title(&schema_for!(TradeState), &out_dir, "trade_state");
    export_schema_with_title(&schema_for!(Trade), &out_dir, "trade_data");
}
