/// Core data models for the RE Tracker application
/// 
/// This module defines the primary data structures used throughout the application
/// for representing housing market data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single data point for housing market information
/// This structure is used to store both historical and real-time data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HousingData {
    /// The date this data point represents (format: YYYY-MM-DD)
    pub date: DateTime<Utc>,
    
    /// Number of active property listings on this date
    pub active_listings: i32,
    
    /// Average price per square foot for listings on this date
    /// None if data is unavailable
    pub avg_price_per_sqft: Option<f64>,
    
    /// Source of this data point
    pub data_source: DataSource,
    
    /// Timestamp when this data was last updated in our system
    pub last_updated: DateTime<Utc>,
}

/// Enum to track where data originated from
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataSource {
    /// Data from Zillow Research CSV files (historical)
    Historical,
    
    /// Data obtained through web scraping
    Scraped,
    
    /// Data received from P2P network
    P2P,
}

/// Configuration for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// ZIP code to track (default: 90720 for Rossmoor, CA)
    pub zip_code: String,
    
    /// Update interval in hours (default: 12)
    pub update_interval_hours: u32,
    
    /// Maximum age of cached data in days before refresh (default: 30)
    pub cache_max_age_days: u32,
    
    /// Enable P2P data synchronization
    pub enable_p2p: bool,
    
    /// Enable detailed logging for debugging
    pub enable_debug_logging: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            zip_code: "90720".to_string(),
            update_interval_hours: 12,
            cache_max_age_days: 30,
            enable_p2p: false, // Disabled by default until Phase 2
            enable_debug_logging: false,
        }
    }
}

/// Represents the result of a scraping operation
#[derive(Debug, Clone)]
pub struct ScrapedData {
    /// Number of active listings found
    pub listings_count: i32,
    
    /// Average price per square foot calculated from listings
    pub avg_price_per_sqft: Option<f64>,
    
    /// Timestamp when scraping was performed
    pub timestamp: DateTime<Utc>,
    
    /// URL that was scraped
    pub source_url: String,
}

/// Simple Moving Average configuration
#[derive(Debug, Clone, Copy)]
pub struct SmaConfig {
    /// Period in days for the moving average (e.g., 7, 30, 90)
    pub period_days: u32,
    
    /// Whether this SMA should be displayed
    pub enabled: bool,
}

impl SmaConfig {
    pub fn new(period_days: u32) -> Self {
        Self {
            period_days,
            enabled: true,
        }
    }
}

/// Statistical summary of housing data over a time period
#[derive(Debug, Clone)]
pub struct MarketSummary {
    /// Average number of listings in the period
    pub avg_listings: f64,
    
    /// Average price per square foot in the period
    pub avg_price_per_sqft: f64,
    
    /// Minimum price per square foot observed
    pub min_price_per_sqft: f64,
    
    /// Maximum price per square foot observed
    pub max_price_per_sqft: f64,
    
    /// Percentage change from start to end of period
    pub price_change_percent: f64,
    
    /// Number of data points in this summary
    pub data_points: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.zip_code, "90720");
        assert_eq!(config.update_interval_hours, 12);
    }

    #[test]
    fn test_data_source_serialization() {
        let source = DataSource::Historical;
        let json = serde_json::to_string(&source).unwrap();
        let deserialized: DataSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, deserialized);
    }
}
