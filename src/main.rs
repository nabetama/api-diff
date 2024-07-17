mod config;
mod diff;
mod utils;

use std::collections::HashMap;

use colored::Colorize;
use reqwest::Client;
use serde_json::Value;
use tokio::main;

use clap::{command, Parser};

use config::load_config;
use utils::hashmap_to_headers;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Compare two API endpoints")]
pub struct Args {
    /// First API endpoint
    #[arg(short = 'a', long)]
    pub endpoint_a: String,

    /// Second API endpoint
    #[arg(short = 'b', long)]
    pub endpoint_b: String,

    /// Show headers diff
    #[arg(long)]
    pub show_headers: bool,

    /// Request headers file (JSON or YAML)
    #[arg(short = 'H', long)]
    pub headers_file: Option<String>,

    /// Query parameters file (JSON or YAML)
    #[arg(short = 'q', long)]
    pub query_file: Option<String>,
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    let headers = if let Some(headers_file) = args.headers_file {
        load_config(&headers_file)?.headers.unwrap_or_default()
    } else {
        HashMap::new()
    };
    let query = if let Some(query_file) = args.query_file {
        load_config(&query_file)?.query.unwrap_or_default()
    } else {
        HashMap::new()
    };

    let request_a = client
        .get(&args.endpoint_a)
        .headers(hashmap_to_headers(headers.clone())?)
        .query(&query)
        .build()?;

    println!("{}", "----------------------".to_string().green());
    println!("Request for endpoint_a:");
    println!("URL: {}", request_a.url());
    println!("Headers: {:?}", request_a.headers());

    let response_a = client.execute(request_a).await?;

    let request_b = client
        .get(&args.endpoint_b)
        .headers(hashmap_to_headers(headers.clone())?)
        .query(&query)
        .build()?;

    println!("{}", "----------------------".to_string().red());
    println!("Request for endpoint_b:");
    println!("URL: {}", request_b.url());
    println!("Headers: {:?}", request_b.headers());

    let response_b = client.execute(request_b).await?;

    let headers_a = response_a.headers().clone();
    let headers_b = response_b.headers().clone();
    if args.show_headers {
        println!("Headers diff :");
        diff::diff_headers(&headers_a, &headers_b);
    }

    let body_a: Value = response_a.json().await?;
    let body_b: Value = response_b.json().await?;
    diff::diff_bodies(&body_a, &body_b);

    Ok(())
}
