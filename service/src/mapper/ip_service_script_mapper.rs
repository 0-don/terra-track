use crate::models::ip_service_script_service::ip_service_script_m;
use scanner::types::{ElemUnion, Script, ScriptUnion, Table, TableUnion};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn process_scripts(
    ip_main_id: i64,
    ip_service_id: i64,
    script_union: &ScriptUnion,
) -> anyhow::Result<()> {
    match script_union {
        ScriptUnion::Script(script) => {
            process_single_script(ip_main_id, ip_service_id, script).await
        }
        ScriptUnion::ScriptArray(scripts) => {
            for script in scripts {
                process_single_script(ip_main_id, ip_service_id, script).await?;
            }
            Ok(())
        }
    }
}

async fn process_single_script(
    ip_main_id: i64,
    ip_service_id: i64,
    script: &Script,
) -> anyhow::Result<()> {
    let mut json_map = serde_json::Map::new();

    json_map.insert("output".to_string(), json!({ &script.id: script.output }));

    if let Some(value) = &script.value {
        json_map.insert("value".to_string(), json!(value));
    }

    if let Some(elem_union) = &script.elem {
        json_map.insert("elem".to_string(), parse_script_elem(elem_union));
    }

    if let Some(table_union) = &script.table {
        json_map.insert("table".to_string(), parse_script_table(table_union));
    }

    ip_service_script_m::Mutation::upsert_ip_service_script(
        ip_main_id,
        ip_service_id,
        &script.id,
        json!(Value::Object(json_map)),
    )
    .await?;

    Ok(())
}

fn parse_script_elem(elem_union: &ElemUnion) -> Value {
    match elem_union {
        ElemUnion::String(s) => json!({ "value": s }),
        ElemUnion::StringArray(arr) => json!(arr),
        ElemUnion::Elem(e) => {
            let mut map = HashMap::new();
            map.insert(&e.key, &e.value);
            json!(map)
        }
        ElemUnion::ElemArray(arr) => {
            let map: HashMap<_, _> = arr
                .iter()
                .map(|e| (&e.key, &e.value))
                .collect();
            json!(map)
        }
        ElemUnion::ElemUnion(arr) => {
            let values: Vec<_> = arr.iter().map(parse_script_elem).collect();
            json!(values)
        }
    }
}

fn parse_script_table(table_union: &TableUnion) -> Value {
    match table_union {
        TableUnion::Table(table) => parse_table(table),
        TableUnion::TableArray(tables) => {
            let values: Vec<_> = tables.iter().map(parse_table).collect();
            json!(values)
        }
    }
}

fn parse_table(table: &Table) -> Value {
    let mut map = HashMap::new();

    let elem_value = match &table.elem {
        Some(elem_union) => parse_script_elem(elem_union),
        None => Value::Null,
    };
    map.insert(table.key.clone(), elem_value);

    let table_value = match &table.table {
        Some(table_union) => parse_script_table(table_union),
        None => Value::Null,
    };
    map.insert("table".to_string(), table_value);

    json!(map)
}
