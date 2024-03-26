use clap::Parser;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::Value;
use std::process;

/// Makes a GET request to a specified URL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The URL to request
    url: String,

    /// Data to send with the request
    #[clap(short, long)]
    data: Option<String>,

    /// Verbose output including the response headers
    #[clap(
        short,
        long,
        default_value = "false",
        help = "Verbose output including the response headers"
    )]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = reqwest::Client::new();
    let request_builder = if let Some(body_data) = &args.data {
        client
            .post(&args.url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .body(body_data.to_string())
    } else {
        client.get(&args.url).header(ACCEPT, "application/json")
    };

    if args.verbose {
        println!("Making request to: {:?}", request_builder);
    }
    let response = request_builder.send().await;

    match response {
        Ok(resp) => {
            if args.verbose {
                println!("response headers: {{");
                for (name, value) in resp.headers() {
                    println!("  {}: {:?}", name, value);
                }
                println!("}}");
            }
            if resp.status().is_success() {
                let json: Value = resp.json().await.expect("Error parsing JSON");
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            } else {
                eprintln!("Request failed with status: {}", resp.status());
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error making request: {}", err);
            process::exit(1);
        }
    }
}
