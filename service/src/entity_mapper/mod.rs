use crate::models::ip_service_extra_service;

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
                    .iter()
                    .map(|elem| (&elem.key, &elem.value))
                    .collect::<HashMap<_, _>>()),
            )
            .await?;
            Ok(())
        }
        ScriptUnion::ScriptElementArray(script_elements) => {
            for script in script_elements {
                let value = match (&script.elem, &script.table) {
                    (Some(elem), Some(table)) => {
                        json!({ &script.id: parse_script_elem(elem), "table": parse_script_table(table) })
                    }
                    (Some(elem), None) => parse_script_elem(elem),
                    (None, Some(table)) => parse_script_table(table),
                    (None, None) => continue,
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
        ScriptTable::IndigoTable(elem) => json!({ elem.key.as_str(): &elem.elem }),
        ScriptTable::PurpleTableArray(elem_array) => json!(elem_array
            .iter()
            .map(|elem| {
                let key = &elem.key;
                let value = if let Some(elems) = &elem.elem {
                    elems
                        .iter()
                        .map(|e| (e.key.as_str(), &e.value))
                        .collect::<HashMap<_, _>>()
                } else if let Some(table) = &elem.table {
                    match table {
                        TableTableUnion::FluffyTableArray(fluffy_tables) => fluffy_tables
                            .iter()
                            .flat_map(|fluffy_table| {
                                fluffy_table.elem.iter().map(|e| (e.key.as_str(), &e.value))
                            })
                            .collect::<HashMap<_, _>>(),
                        TableTableUnion::TentacledTable(tentacled_table) => tentacled_table
                            .table
                            .elem
                            .iter()
                            .map(|e| (e.key.as_str(), &e.value))
                            .collect::<HashMap<_, _>>(),
                    }
                } else {
                    HashMap::new()
                };
                (key.as_str(), value)
            })
            .collect::<HashMap<_, _>>()),
    }
}

pub fn parse_script_elem(elem: &ElemUnion) -> Value {
    match elem {
        ElemUnion::ElemElem(e) => json!({ e.key.as_str(): &e.value }),
        ElemUnion::ElemElemArray(elem_array) => json!(elem_array
            .iter()
            .map(|elem| (elem.key.as_str(), &elem.value))
            .collect::<HashMap<_, _>>()),
        ElemUnion::String(string) => json!({ string: string }),
    }
}
