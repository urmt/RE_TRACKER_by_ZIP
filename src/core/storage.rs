/// Storage module for persisting housing data using SQLite
/// 
/// This module provides a safe interface for storing and retrieving housing market data.
/// It handles database initialization, data insertion, and querying operations.

use crate::core::models::{HousingData, DataSource};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::info;
use rusqlite::{Connection, params};
use std::path::Path;

/// Main database interface for the application
pub struct Storage {
    /// SQLite database connection
    conn: Connection,
}

impl Storage {
    /// Create a new Storage instance and initialize the database schema
    /// 
    /// # Arguments
    /// * `db_path` - Path to the SQLite database file
    /// 
    /// # Returns
    /// A Result containing the Storage instance or an error
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        info!("Initializing database at {:?}", db_path.as_ref());
        
        // Open or create the database file
        let conn = Connection::open(db_path)
            .context("Failed to open database connection")?;
        
        let mut storage = Storage { conn };
        
        // Initialize the schema if it doesn't exist
        storage.initialize_schema()
            .context("Failed to initialize database schema")?;
        
        Ok(storage)
    }
    
    /// Create the database schema (tables and indexes)
    /// This is idempotent - safe to call multiple times
    fn initialize_schema(&mut self) -> Result<()> {
        info!("Creating database schema");
        
        // Create the main housing_data table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS housing_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL UNIQUE,
                active_listings INTEGER NOT NULL,
                avg_price_per_sqft REAL,
                data_source TEXT NOT NULL,
                last_updated TEXT NOT NULL
            )",
            [],
        ).context("Failed to create housing_data table")?;
        
        // Create index on date for faster queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_housing_data_date ON housing_data(date)",
            [],
        ).context("Failed to create date index")?;
        
        // Create index on last_updated for finding stale data
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_housing_data_updated ON housing_data(last_updated)",
            [],
        ).context("Failed to create last_updated index")?;
        
        info!("Database schema initialized successfully");
        Ok(())
    }
    
    /// Insert or update a housing data point
    /// If data for the same date exists, it will be updated
    /// 
    /// # Arguments
    /// * `data` - The housing data to store
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn upsert_housing_data(&mut self, data: &HousingData) -> Result<()> {
        let date_str = data.date.format("%Y-%m-%d").to_string();
        let last_updated_str = data.last_updated.to_rfc3339();
        let source_str = match data.data_source {
            DataSource::Historical => "historical",
            DataSource::Scraped => "scraped",
            DataSource::P2P => "p2p",
        };
        
        // Use INSERT OR REPLACE to handle both insert and update
        self.conn.execute(
            "INSERT OR REPLACE INTO housing_data 
             (date, active_listings, avg_price_per_sqft, data_source, last_updated)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                date_str,
                data.active_listings,
                data.avg_price_per_sqft,
                source_str,
                last_updated_str,
            ],
        ).context("Failed to upsert housing data")?;
        
        info!("Stored data for date: {}", date_str);
        Ok(())
    }
    
    /// Insert multiple housing data points in a single transaction
    /// This is much faster than inserting one at a time
    /// 
    /// # Arguments
    /// * `data_points` - Vector of housing data to store
    /// 
    /// # Returns
    /// Result indicating success or failure
    pub fn bulk_insert(&mut self, data_points: &[HousingData]) -> Result<()> {
        info!("Bulk inserting {} data points", data_points.len());
        
        let tx = self.conn.transaction()
            .context("Failed to start transaction")?;
        
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO housing_data 
                 (date, active_listings, avg_price_per_sqft, data_source, last_updated)
                 VALUES (?1, ?2, ?3, ?4, ?5)"
            ).context("Failed to prepare bulk insert statement")?;
            
            for data in data_points {
                let date_str = data.date.format("%Y-%m-%d").to_string();
                let last_updated_str = data.last_updated.to_rfc3339();
                let source_str = match data.data_source {
                    DataSource::Historical => "historical",
                    DataSource::Scraped => "scraped",
                    DataSource::P2P => "p2p",
                };
                
                stmt.execute(params![
                    date_str,
                    data.active_listings,
                    data.avg_price_per_sqft,
                    source_str,
                    last_updated_str,
                ]).context("Failed to execute bulk insert")?;
            }
        }
        
        tx.commit().context("Failed to commit transaction")?;
        info!("Successfully inserted {} data points", data_points.len());
        Ok(())
    }
    
    /// Retrieve all housing data within a date range
    /// 
    /// # Arguments
    /// * `start_date` - Beginning of date range (inclusive)
    /// * `end_date` - End of date range (inclusive)
    /// 
    /// # Returns
    /// Vector of HousingData sorted by date (oldest first)
    pub fn get_data_range(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<HousingData>> {
        let start_str = start_date.format("%Y-%m-%d").to_string();
        let end_str = end_date.format("%Y-%m-%d").to_string();
        
        let mut stmt = self.conn.prepare(
            "SELECT date, active_listings, avg_price_per_sqft, data_source, last_updated
             FROM housing_data
             WHERE date >= ?1 AND date <= ?2
             ORDER BY date ASC"
        ).context("Failed to prepare query")?;
        
        let rows = stmt.query_map(params![start_str, end_str], |row| {
            let date_str: String = row.get(0)?;
            let data_source_str: String = row.get(3)?;
            let last_updated_str: String = row.get(4)?;
            
            Ok(HousingData {
                date: DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str))
                    .unwrap()
                    .with_timezone(&Utc),
                active_listings: row.get(1)?,
                avg_price_per_sqft: row.get(2)?,
                data_source: match data_source_str.as_str() {
                    "historical" => DataSource::Historical,
                    "scraped" => DataSource::Scraped,
                    "p2p" => DataSource::P2P,
                    _ => DataSource::Historical,
                },
                last_updated: DateTime::parse_from_rfc3339(&last_updated_str)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        }).context("Failed to query data range")?;
        
        let data: Result<Vec<HousingData>, _> = rows.collect();
        data.context("Failed to collect query results")
    }
    
    /// Get the most recent data point in the database
    /// 
    /// # Returns
    /// Option containing the most recent HousingData, or None if database is empty
    pub fn get_latest_data(&self) -> Result<Option<HousingData>> {
        let mut stmt = self.conn.prepare(
            "SELECT date, active_listings, avg_price_per_sqft, data_source, last_updated
             FROM housing_data
             ORDER BY date DESC
             LIMIT 1"
        ).context("Failed to prepare query for latest data")?;
        
        let mut rows = stmt.query([])
            .context("Failed to execute query for latest data")?;
        
        if let Some(row) = rows.next()? {
            let date_str: String = row.get(0)?;
            let data_source_str: String = row.get(3)?;
            let last_updated_str: String = row.get(4)?;
            
            Ok(Some(HousingData {
                date: DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str))
                    .unwrap()
                    .with_timezone(&Utc),
                active_listings: row.get(1)?,
                avg_price_per_sqft: row.get(2)?,
                data_source: match data_source_str.as_str() {
                    "historical" => DataSource::Historical,
                    "scraped" => DataSource::Scraped,
                    "p2p" => DataSource::P2P,
                    _ => DataSource::Historical,
                },
                last_updated: DateTime::parse_from_rfc3339(&last_updated_str)
                    .unwrap()
                    .with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Count the total number of data points in the database
    /// 
    /// # Returns
    /// The number of data points stored
    pub fn count_data_points(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM housing_data",
            [],
            |row| row.get(0)
        ).context("Failed to count data points")?;
        
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_storage_initialization() {
        let temp_db = NamedTempFile::new().unwrap();
        let storage = Storage::new(temp_db.path());
        assert!(storage.is_ok());
    }

    #[test]
    fn test_insert_and_retrieve() {
        let temp_db = NamedTempFile::new().unwrap();
        let mut storage = Storage::new(temp_db.path()).unwrap();
        
        let data = HousingData {
            date: Utc::now(),
            active_listings: 42,
            avg_price_per_sqft: Some(450.50),
            data_source: DataSource::Scraped,
            last_updated: Utc::now(),
        };
        
        storage.upsert_housing_data(&data).unwrap();
        let count = storage.count_data_points().unwrap();
        assert_eq!(count, 1);
    }
}
