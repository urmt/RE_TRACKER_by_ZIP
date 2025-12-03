/// Core module containing the main business logic for the RE Tracker
/// 
/// This module exposes all the core functionality needed to track and analyze
/// housing market data.

pub mod models;
pub mod storage;
pub mod data_processor;

// Re-export commonly used types for convenience
pub use models::{HousingData, DataSource, AppConfig, ScrapedData, SmaConfig, MarketSummary};
pub use storage::Storage;
pub use data_processor::{calculate_sma, calculate_listings_sma, generate_market_summary, interpolate_missing_data, remove_outliers};
