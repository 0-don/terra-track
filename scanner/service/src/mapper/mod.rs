pub mod ip_host_script_mapper;
pub mod ip_os_mapper;
pub mod ip_service_mapper;
pub mod ip_service_script_mapper;
pub mod ip_location_mapper;
use parser::{
    types::{ElemUnion, Script, Table, TableUnion},
    ELEM, TABLE, VALUE,
};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

pub fn process_single_script(script: &Script) -> Value {
    let mut json_map = serde_json::Map::new();

    json_map.insert(script.id.clone(), json!(script.output));

    if let Some(value) = &script.value {
        json_map.insert(VALUE.to_string(), json!(value));
    }

    if let Some(elem_union) = &script.elem {
        json_map.insert(ELEM.to_string(), parse_script_elem(elem_union));
    }

    if let Some(table_union) = &script.table {
        json_map.insert(TABLE.to_string(), parse_script_table(table_union));
    }

    json_map = flatten_script_elem(json_map);

    json!(json_map)
}

fn flatten_script_elem(mut json_map: Map<String, Value>) -> Map<String, Value> {
    let mut root_map = Map::new();
    flatten_map(&mut json_map, &mut root_map);
    root_map
}

fn flatten_map(current_map: &mut Map<String, Value>, root_map: &mut Map<String, Value>) {
    let keys_to_process: Vec<String> = current_map.keys().cloned().collect();

    for key in keys_to_process {
        if let Some(value) = current_map.remove(&key) {
            match &value {
                Value::Object(obj) if key == ELEM || key == TABLE => {
                    // Directly flatten the contents of 'elem' or 'table' into the root map
                    let mut nested_obj = obj.clone();
                    flatten_map(&mut nested_obj, root_map);
                }
                Value::Object(obj) => {
                    // Check if the object contains a 'value' key
                    if let Some(inner_value) = obj.get(VALUE) {
                        merge_into_root_map(root_map, key, inner_value.clone());
                    } else {
                        let mut nested_obj = obj.clone();
                        let mut nested_root_map = Map::new();
                        flatten_map(&mut nested_obj, &mut nested_root_map);
                        merge_into_root_map(root_map, key, Value::Object(nested_root_map));
                    }
                }
                Value::Array(arr) => {
                    // Flatten each element in the array
                    for elem in arr {
                        if let Value::Object(obj) = elem {
                            let mut nested_obj = obj.clone();
                            flatten_map(&mut nested_obj, root_map);
                        }
                        // Additional handling for non-object elements within the array if needed
                    }
                }
                _ => {
                    merge_into_root_map(root_map, key, value.clone());
                }
            }
        }
    }
}

fn merge_into_root_map(root_map: &mut Map<String, Value>, key: String, value: Value) {
    root_map.insert(key, value);
}

pub fn parse_script_elem(elem_union: &ElemUnion) -> Value {
    match elem_union {
        ElemUnion::String(s) => json!({ VALUE: s }),
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
    let mut map = Map::new();

    if let Some(elem_union) = &table.elem {
        map.insert(ELEM.to_string(), parse_script_elem(elem_union));
    }

    if let Some(table_union) = &table.table {
        match table_union {
            TableUnion::Table(inner_table) => {
                // Nest the inner table within the current table's key if it exists
                if let Some(key) = &table.key {
                    let inner_table_value = parse_table(inner_table);
                    map.insert(key.clone(), inner_table_value);
                } else {
                    map.insert(TABLE.to_string(), parse_table(inner_table));
                }
            }
            TableUnion::TableArray(inner_tables) => {
                // Nest each inner table within the current table's key if it exists
                let values: Vec<_> = inner_tables.iter().map(parse_table).collect();
                if let Some(key) = &table.key {
                    map.insert(key.clone(), json!(values));
                } else {
                    map.insert(TABLE.to_string(), json!(values));
                }
            }
        }
    }

    json!(map)
}
