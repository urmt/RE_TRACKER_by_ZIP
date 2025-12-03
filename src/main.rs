/// RE_TRACKER_by_ZIP - Rossmoor Housing Inventory Tracker
/// 
/// Main entry point for the application.
/// Phase 1: Desktop MVP with data fetching, scraping, and visualization.

mod core;
mod utils;

use anyhow::Result;
use chrono::Utc;
use clap::{Parser, Subcommand};
use core::{AppConfig, DataSource, HousingData, Storage};
use core::data_fetcher::{ZillowConfig, fetch_zillow_data};
use log::{info, error, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use tiny_http::{Server, Response};
use utils::scraper::{ScraperConfig, scrape_zillow};

/// CLI arguments structure
#[derive(Parser)]
#[command(name = "re_tracker")]
#[command(about = "Rossmoor Housing Market Tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch historical data from Zillow Research
    Fetch,
    
    /// Scrape real-time listing data
    Scrape,
    
    /// Start web server to view charts
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    
    /// Show current database statistics
    Stats,
}

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();
    
    info!("========================================");
    info!("RE_TRACKER_by_ZIP - Version 0.1.0");
    info!("Rossmoor Housing Inventory Tracker");
    info!("========================================");
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Fetch => fetch_data()?,
        Commands::Scrape => scrape_data()?,
        Commands::Serve { port } => serve_frontend(port)?,
        Commands::Stats => show_stats()?,
    }
    
    Ok(())
}

/// Fetch historical data from Zillow Research
fn fetch_data() -> Result<()> {
    info!("Fetching historical data...");
    
    let db_path = get_database_path()?;
    let mut storage = Storage::new(&db_path)?;
    
    let config = ZillowConfig::default();
    let data = fetch_zillow_data(&config)?;
    
    info!("Fetched {} data points", data.len());
    storage.bulk_insert(&data)?;
    
    info!("Successfully stored historical data");
    Ok(())
}

/// Scrape real-time listing data
fn scrape_data() -> Result<()> {
    info!("Scraping real-time data...");
    
    let db_path = get_database_path()?;
    let mut storage = Storage::new(&db_path)?;
    
    let config = ScraperConfig::default();
    let scraped = scrape_zillow(&config)?;
    
    // Convert ScrapedData to HousingData
    let housing_data = HousingData {
        date: scraped.timestamp,
        active_listings: scraped.listings_count,
        avg_price_per_sqft: scraped.avg_price_per_sqft,
        data_source: DataSource::Scraped,
        last_updated: Utc::now(),
    };
    
    storage.upsert_housing_data(&housing_data)?;
    
    info!("Successfully stored scraped data");
    Ok(())
}

/// Start HTTP server to serve the frontend
fn serve_frontend(port: u16) -> Result<()> {
    info!("Starting web server on http://localhost:{}", port);
    
    let server = Server::http(format!("0.0.0.0:{}", port))
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;
    
    info!("Server running! Open http://localhost:{} in your browser", port);
    info!("Press Ctrl+C to stop");
    
    for request in server.incoming_requests() {
        let url = request.url().to_string();
        info!("Request: {}", url);
        
        let response = match url.as_str() {
            "/" | "/index.html" => {
                serve_file("frontend/index.html", "text/html")
            },
            "/styles.css" => {
                serve_file("frontend/styles.css", "text/css")
            },
            "/chart.js" => {
                serve_file("frontend/chart.js", "application/javascript")
            },
            "/api/data" => {
                serve_data()
            },
            _ => {
                Response::from_string("404 Not Found")
                    .with_status_code(404)
            }
        };
        
        if let Err(e) = request.respond(response) {
            error!("Failed to send response: {}", e);
        }
    }
    
    Ok(())
}

/// Serve a static file
fn serve_file(path: &str, content_type: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    match fs::read_to_string(path) {
        Ok(content) => {
            Response::from_string(content)
                .with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap()
                )
        },
        Err(e) => {
            error!("Failed to read file {}: {}", path, e);
            Response::from_string(format!("Error: {}", e))
                .with_status_code(500)
        }
    }
}

/// Serve housing data as JSON
fn serve_data() -> Response<std::io::Cursor<Vec<u8>>> {
    match get_all_data() {
        Ok(data) => {
            match serde_json::to_string(&data) {
                Ok(json) => {
                    Response::from_string(json)
                        .with_header(
                            tiny_http::Header::from_bytes(&b"Content-Type"[..], b"application/json").unwrap()
                        )
                },
                Err(e) => {
                    error!("Failed to serialize data: {}", e);
                    Response::from_string(format!("Error: {}", e))
                        .with_status_code(500)
                }
            }
        },
        Err(e) => {
            error!("Failed to get data: {}", e);
            Response::from_string(format!("Error: {}", e))
                .with_status_code(500)
        }
    }
}

/// Get all housing data from database
fn get_all_data() -> Result<Vec<serde_json::Value>> {
    let db_path = get_database_path()?;
    let storage = Storage::new(&db_path)?;
    
    // Get all data (last 365 days)
    let end_date = Utc::now();
    let start_date = end_date - chrono::Duration::days(365);
    let data = storage.get_data_range(start_date, end_date)?;
    
    // Convert to JSON-friendly format
    let json_data: Vec<serde_json::Value> = data.iter().map(|d| {
        serde_json::json!({
            "date": d.date.format("%Y-%m-%d").to_string(),
            "active_listings": d.active_listings,
            "avg_price_per_sqft": d.avg_price_per_sqft,
            "data_source": format!("{:?}", d.data_source).to_lowercase()
        })
    }).collect();
    
    Ok(json_data)
}

/// Show database statistics
fn show_stats() -> Result<()> {
    let db_path = get_database_path()?;
    let storage = Storage::new(&db_path)?;
    
    let count = storage.count_data_points()?;
    info!("Total data points: {}", count);
    
    if count > 0 {
        if let Some(latest) = storage.get_latest_data()? {
            info!("Latest data point:");
            info!("  Date: {}", latest.date.format("%Y-%m-%d"));
            info!("  Active Listings: {}", latest.active_listings);
            if let Some(price) = latest.avg_price_per_sqft {
                info!("  Avg Price/SqFt: ${:.2}", price);
            }
            info!("  Source: {:?}", latest.data_source);
        }
    } else {
        warn!("No data in database. Run 're_tracker fetch' to populate.");
    }
    
    Ok(())
}

/// Get the database file path
/// Uses $HOME/.local/share/re_tracker/ on Linux
fn get_database_path() -> Result<std::path::PathBuf> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
    
    let data_dir = std::path::Path::new(&home)
        .join(".local")
        .join("share")
        .join("re_tracker");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&data_dir)?;
    
    Ok(data_dir.join("housing_data.db"))
}

