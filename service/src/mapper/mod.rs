pub mod ip_host_script_mapper;
pub mod ip_service_script_mapper;
use scanner::types::{ElemUnion, Script, Table, TableUnion};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn process_single_script(script: &Script) -> Value {
    let mut json_map = serde_json::Map::new();

    json_map.insert(script.id.clone(), json!(script.output));

    if let Some(value) = &script.value {
        json_map.insert("value".to_string(), json!(value));
    }

    if let Some(elem_union) = &script.elem {
        json_map.insert("elem".to_string(), parse_script_elem(elem_union));
    }

    if let Some(table_union) = &script.table {
        json_map.insert("table".to_string(), parse_script_table(table_union));
    }

    json!(json_map)
}

pub fn parse_script_elem(elem_union: &ElemUnion) -> Value {
    match elem_union {
        ElemUnion::String(s) => json!({ "value": s }),
        ElemUnion::StringArray(arr) => json!(arr),
        ElemUnion::Elem(e) => {
            let mut map = HashMap::new();
            map.insert(&e.key, &e.value);
            json!(map)
        }
        ElemUnion::ElemArray(arr) => {
            let map: HashMap<_, _> = arr.iter().map(|e| (&e.key, &e.value)).collect();
            json!(map)
        }
        ElemUnion::ElemUnion(arr) => {
            let values: Vec<_> = arr.iter().map(parse_script_elem).collect();
            json!(values)
        }
    }
}

pub fn parse_script_table(table_union: &TableUnion) -> Value {
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

    if table.key.is_some() {
        map.insert(table.key.clone().unwrap(), elem_value);
    } else {
        map.insert("elem".to_string(), elem_value);
    }

    let table_value = match &table.table {
        Some(table_union) => Some(parse_script_table(table_union)),
        None => None,
    };

    if let Some(table_value) = table_value {
        map.insert("table".to_string(), table_value);
    }

    json!(map)
}
