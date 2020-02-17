//! Various utilities for the CLI

use anyhow::format_err;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

lazy_static! {
    /// The regular expression for a key-value pair
    pub(crate) static ref KV_REGEX: Regex = Regex::new(r"(?ms)^(?P<key>[a-zA-Z][a-zA-Z0-9_-]*)=(?P<value>.*)")
        .expect("Could not compile regex");
}

pub(crate) fn parse_kv_pairs<'a, T>(
    raw_kv_pairs: T,
) -> anyhow::Result<HashMap<String, Option<String>>>
where
    T: IntoIterator<Item = &'a str>,
{
    let mut data = HashMap::new();

    for raw_kv_pair in raw_kv_pairs {
        if let Some(captures) = KV_REGEX.captures(raw_kv_pair) {
            let key = captures.name("key").expect("Expected key").as_str();

            let value = captures.name("value").expect("Expected value").as_str();

            data.insert(
                key.to_string(),
                if value == "" {
                    None
                } else {
                    Some(value.into())
                },
            );
        } else {
            return Err(format_err!(
                "Could not parse key-value pair: {}",
                raw_kv_pair
            ));
        }
    }

    Ok(data)
}
