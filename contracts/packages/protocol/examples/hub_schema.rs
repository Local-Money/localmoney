use cosmwasm_schema::{export_schema_with_title, remove_schemas};
use localmoney_protocol::hub::{ExecuteMsg, HubConfig, InstantiateMsg, QueryMsg};
use schemars::schema_for;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/hub");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg),
        &out_dir,
        "hub_instantiate_msg",
    );
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "hub_execute_msg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "hub_query_msg");
    export_schema_with_title(&schema_for!(HubConfig), &out_dir, "hub_config");
}
