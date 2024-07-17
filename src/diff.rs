use std::collections::HashMap;

use colored::Colorize;
use reqwest::header::HeaderMap;
use serde_json::Value;
use similar::{ChangeTag, TextDiff};

pub fn diff_headers(headers1: &HeaderMap, headers2: &HeaderMap) {
    let headers1_map: HashMap<_, _> = headers1
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap()))
        .collect();
    let headers2_map: HashMap<_, _> = headers2
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap()))
        .collect();
    for key in headers1_map
        .keys()
        .chain(headers2_map.keys())
        .collect::<Vec<_>>()
    {
        let value1 = headers1_map.get(key).unwrap_or(&"");
        let value2 = headers2_map.get(key).unwrap_or(&"");
        if value1 != value2 {
            println!("{}:\n{}", key, format_diff(value1, value2));
        }
    }
}

pub fn diff_bodies(body1: &Value, body2: &Value) {
    let formatted_body1 = serde_json::to_string_pretty(body1).unwrap();
    let formatted_body2 = serde_json::to_string_pretty(body2).unwrap();
    let diff = TextDiff::from_lines(&formatted_body1, &formatted_body2);
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("{}", format!("-{}", change).red()),
            ChangeTag::Insert => print!("{}", format!("+{}", change).green()),
            ChangeTag::Equal => print!(" {}", change),
        }
    }
}

pub fn format_diff(value1: &str, value2: &str) -> String {
    let diff = TextDiff::from_lines(value1, value2);
    let mut result = String::new();
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => result.push_str(&format!("-{}", change).red().to_string()),
            ChangeTag::Insert => result.push_str(&format!("+{}", change).green().to_string()),
            ChangeTag::Equal => result.push_str(&format!(" {}", change).normal().to_string()),
        }
    }
    result
}
