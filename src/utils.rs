use clap::parser::ValuesRef;
use std::collections::HashMap;

pub fn parse_metadata(meta: Option<ValuesRef<'_, String>>) -> HashMap<String, String> {
    meta.unwrap_or_default()
        .filter_map(|v| {
            v.split_once(':')
                .map(|(key, value)| (key.to_string(), value.to_string()))
        })
        .collect()
}

pub fn parse_metadata_json(meta: Option<ValuesRef<'_, String>>) -> HashMap<String, String> {
    let maps: Vec<HashMap<String, String>> = meta
        .unwrap_or_default()
        .filter_map(|v| {
            let t = serde_json::from_str::<HashMap<String, String>>(v).ok();
            t
        })
        .collect();

    let mut result = HashMap::new();
    for map in maps {
        for (key, value) in map {
            result.insert(key, value);
        }
    }

    result
}
