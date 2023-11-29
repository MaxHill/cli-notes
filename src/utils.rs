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
