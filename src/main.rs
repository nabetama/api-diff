use std::collections::HashMap;

use clap::Parser;
use colored::Colorize;
use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use similar::{ChangeTag, TextDiff};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Compare two API endpoints")]
struct Args {
    /// First API endpoint
    #[arg(short = 'a', long)]
    endpoint1: String,

    /// Second API endpoint
    #[arg(short = 'b', long)]
    endpoint2: String,

    /// Show header diffs
    #[arg(short = 'H', long)]
    show_headers: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    let response1 = client.get(&args.endpoint1).send().await?;
    let response2 = client.get(&args.endpoint2).send().await?;

    let header1 = response1.headers().clone();
    let header2 = response2.headers().clone();

    let body1: Value = response1.json().await?;
    let body2: Value = response2.json().await?;

    if args.show_headers {
        println!("Headers diff :");
        diff_headers(&header1, &header2);
    }

    println!("\nBody Diff:");
    diff_bodies(&body1, &body2);

    Ok(())
}

fn diff_headers(headers1: &HeaderMap, headers2: &HeaderMap) {
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

fn diff_bodies(body1: &Value, body2: &Value) {
    let binding1 = body1.to_string();
    let binding2 = body2.to_string();

    let diff = TextDiff::from_lines(&binding1, &binding2);
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("{}", format!("-{}", change).red()),
            ChangeTag::Insert => print!("{}", format!("+{}", change).green()),
            ChangeTag::Equal => print!("{}", format!(" {}", change).normal()),
        }
    }
}

fn format_diff(value1: &str, value2: &str) -> String {
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
