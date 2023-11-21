use crate::models::ip_service_extra_service;
use scanner::types::{ElemUnion, ScriptTable, ScriptUnion, TableTableUnion};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn process_scripts(
    ip_main_id: i64,
    ip_service_id: i64,
    script_union: &ScriptUnion,
) -> anyhow::Result<()> {
    match script_union {
        ScriptUnion::PurpleScript(purple_script) => {
            ip_service_extra_service::Mutation::upsert_ip_service_extra(
                ip_main_id,
                ip_service_id,
                &purple_script.id,
                json!(purple_script
                    .elem
                    .clone()
                    .into_iter()
                    .map(|elem| (elem.key, elem.value))
                    .collect::<HashMap<_, _>>()),
            )
            .await?;
            Ok(())
        }
        ScriptUnion::ScriptElementArray(script_elements) => {
            for script in script_elements {
                let value = if script.table.is_some() && script.elem.is_some() {
                    json!({ &script.id: parse_script_elem(script.elem.as_ref().unwrap()), "table": parse_script_table(script.table.as_ref().unwrap()) })
                } else if script.table.is_some() {
                    parse_script_table(script.table.as_ref().unwrap())
                } else if script.elem.is_some() {
                    parse_script_elem(script.elem.as_ref().unwrap())
                } else {
                    continue;
                };
                ip_service_extra_service::Mutation::upsert_ip_service_extra(
                    ip_main_id,
                    ip_service_id,
                    &script.id,
                    value,
                )
                .await?;
            }
            Ok(())
        }
    }
}

pub fn parse_script_table(script_table: &ScriptTable) -> Value {
    match script_table {
        ScriptTable::IndigoTable(elem) => json!({ elem.key.as_str(): elem.elem }),
        ScriptTable::PurpleTableArray(elem_array) => json!(elem_array
            .into_iter()
            .map(|elem| {
                let key = elem.key.to_owned();
                let value = if let Some(elems) = &elem.elem {
                    elems
                        .into_iter()
                        .map(|e| (e.key.to_owned(), e.value.to_owned()))
                        .collect::<HashMap<_, _>>()
                } else if let Some(table) = &elem.table {
                    match table {
                        TableTableUnion::FluffyTableArray(fluffy_tables) => fluffy_tables
                            .into_iter()
                            .flat_map(|fluffy_table| {
                                fluffy_table
                                    .elem
                                    .to_owned()
                                    .into_iter()
                                    .map(|e| (e.key, e.value))
                            })
                            .collect::<HashMap<_, _>>(),
                        TableTableUnion::TentacledTable(tentacled_table) => tentacled_table
                            .table
                            .elem
                            .to_owned()
                            .into_iter()
                            .map(|e| (e.key, e.value))
                            .collect::<HashMap<_, _>>(),
                    }
                } else {
                    HashMap::new()
                };
                (key, value)
            })
            .collect::<HashMap<_, _>>()),
    }
}

pub fn parse_script_elem(elem: &ElemUnion) -> Value {
    match elem {
        ElemUnion::ElemElem(e) => json!({ e.key.as_str(): e.value }),
        ElemUnion::ElemElemArray(elem_array) => json!(elem_array
            .into_iter()
            .map(|elem| (elem.key.to_owned(), elem.value.to_owned()))
            .collect::<HashMap<_, _>>()),
        ElemUnion::String(string) => json!({ string: string }),
    }
}
