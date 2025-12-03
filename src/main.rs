/// RE_TRACKER_by_ZIP - Rossmoor Housing Inventory Tracker
/// 
/// Main entry point for the application.
/// This is a Phase 0 proof of concept that will be expanded in future phases.

mod core;
mod utils;

use anyhow::Result;
use chrono::Utc;
use core::{AppConfig, DataSource, HousingData, Storage};
use log::{info, error};
use std::env;

fn main() -> Result<()> {
    // Initialize the logger
    // Set RUST_LOG=debug for verbose output, or RUST_LOG=info for normal output
    env_logger::init();
    
    info!("========================================");
    info!("RE_TRACKER_by_ZIP - Version 0.1.0");
    info!("Rossmoor Housing Inventory Tracker");
    info!("========================================");
    
    // Load configuration (using defaults for now)
    let config = AppConfig::default();
    info!("Tracking ZIP code: {}", config.zip_code);
    info!("Update interval: {} hours", config.update_interval_hours);
    
    // Initialize database
    let db_path = get_database_path()?;
    info!("Database location: {:?}", db_path);
    
    let mut storage = Storage::new(&db_path)
        .map_err(|e| {
            error!("Failed to initialize database: {}", e);
            e
        })?;
    
    // Check if we have any existing data
    let data_count = storage.count_data_points()?;
    info!("Current data points in database: {}", data_count);
    
    if data_count == 0 {
        info!("No data found. Run data fetcher to populate database.");
        info!("This is a proof of concept - data fetching will be implemented in Phase 1.");
    } else {
        // Show the most recent data
        if let Some(latest) = storage.get_latest_data()? {
            info!("Latest data point:");
            info!("  Date: {}", latest.date.format("%Y-%m-%d"));
            info!("  Active Listings: {}", latest.active_listings);
            if let Some(price) = latest.avg_price_per_sqft {
                info!("  Avg Price/SqFt: ${:.2}", price);
            }
            info!("  Source: {:?}", latest.data_source);
        }
    }
    
    // Example: Insert some test data for demonstration
    if data_count == 0 {
        info!("Inserting sample data for demonstration...");
        insert_sample_data(&mut storage)?;
        info!("Sample data inserted successfully!");
    }
    
    info!("========================================");
    info!("Application initialized successfully!");
    info!("Next steps:");
    info!("  1. Implement data fetcher (Phase 1)");
    info!("  2. Implement web scraper (Phase 1)");
    info!("  3. Build chart UI (Phase 1)");
    info!("========================================");
    
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

/// Insert sample data for demonstration purposes
/// This will be replaced by real data fetching in Phase 1
fn insert_sample_data(storage: &mut Storage) -> Result<()> {
    use chrono::Duration;
    
    let now = Utc::now();
    let mut sample_data = Vec::new();
    
    // Generate 30 days of sample data
    for i in 0..30 {
        let date = now - Duration::days(30 - i);
        
        // Simulate realistic housing data with some variation
        let base_listings = 45;
        let listings_variation = (i % 7) as i32 - 3;
        
        let base_price = 425.0;
        let price_variation = ((i as f64) * 2.5) - 30.0;
        
        sample_data.push(HousingData {
            date,
            active_listings: base_listings + listings_variation,
            avg_price_per_sqft: Some(base_price + price_variation),
            data_source: DataSource::Historical,
            last_updated: now,
        });
    }
    
    storage.bulk_insert(&sample_data)?;
    
    Ok(())
}
