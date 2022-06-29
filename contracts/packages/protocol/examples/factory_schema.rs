use cosmwasm_schema::{export_schema_with_title, remove_schemas};
use localterra_protocol::factory::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use schemars::schema_for;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/factory");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg),
        &out_dir,
        "factory_instantiate_msg",
    );
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "factory_execute_msg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "factory_query_msg");
    export_schema_with_title(&schema_for!(Config), &out_dir, "factory_config");
}
