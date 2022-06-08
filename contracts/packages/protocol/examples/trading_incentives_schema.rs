use std::env::current_dir;
use std::fs::create_dir_all;
use cosmwasm_schema::{export_schema_with_title, remove_schemas};
use schemars::schema_for;
use localterra_protocol::trading_incentives::{ExecuteMsg, QueryMsg, InstantiateMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/trading_incentives");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(&schema_for!(InstantiateMsg), &out_dir, "trading_incentives_instantiate_msg");
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "trading_incentives_execute_msg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "trading_incentives_query_msg");

}
