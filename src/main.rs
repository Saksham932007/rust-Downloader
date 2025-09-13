use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::{Result, Context};
use futures_util::StreamExt; // <-- Add this line

/// A simple wget clone written in Rust to download files from a URL.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The URL of the file to download
    #[arg(required = true)]
    url: String,

    /// Optional output filename
    #[arg(short, long)]
    output: Option<String>,
}

// The #[tokio::main] macro transforms our async main function into a regular,
// synchronous main function that starts the Tokio runtime.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Use a try block to handle errors gracefully with anyhow
    if let Err(e) = download_file(&cli.url, &cli.output).await {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Downloads a file from the given URL and saves it to an optional output path.
async fn download_file(url: &str, output: &Option<String>) -> Result<()> {
    // Determine the output filename. If an output path is provided, use it.
    // Otherwise, try to extract the filename from the URL.
    let filename = match output {
        Some(name) => name.clone(),
        None => Path::new(url)
            .file_name()
            .and_then(|s| s.to_str())
            .context(format!("Could not determine filename from URL: {}", url))?
            .to_string(),
    };

    println!("📥 Downloading {} to {}", url, &filename);

    let client = Client::new();
    let response = client.get(url)
        .send()
        .await
        .context("Failed to send HTTP request")?;

    // Check if the request was successful
    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    // Get the content length from the headers, if available
    let content_length = response.content_length().unwrap_or(0);

    // Create a progress bar
    let pb = ProgressBar::new(content_length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("=> "));
    pb.set_message(format!("Downloading {}", &filename));

    // Create the output file
    let mut dest = File::create(&filename)
        .await
        .context(format!("Failed to create file: {}", &filename))?;

    // Stream the response body
    let mut stream = response.bytes_stream();

    // Asynchronously read chunks from the stream and write them to the file
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Error while downloading file chunk")?;
        dest.write_all(&chunk)
            .await
            .context("Failed to write chunk to file")?;
        
        // Increment the progress bar by the chunk size
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message(format!("✅ Downloaded {} successfully!", &filename));

    Ok(())
}