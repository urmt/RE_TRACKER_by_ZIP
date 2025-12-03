/// Data processor module for calculating Simple Moving Averages and processing housing data
/// 
/// This module transforms raw housing data into chart-ready format with
/// calculated metrics like SMAs and statistical summaries.

use crate::core::models::{HousingData, SmaConfig, MarketSummary};
use anyhow::Result;
use log::{info, debug};

/// Calculate Simple Moving Average (SMA) for a given dataset
/// 
/// # Arguments
/// * `data` - Vector of housing data points, should be sorted by date
/// * `config` - SMA configuration specifying the period
/// 
/// # Returns
/// Vector of (date_index, sma_value) tuples for data points where SMA can be calculated
/// 
/// # Example
/// ```
/// let sma_values = calculate_sma(&housing_data, &SmaConfig::new(7));
/// ```
pub fn calculate_sma(data: &[HousingData], config: &SmaConfig) -> Vec<(usize, f64)> {
    let period = config.period_days as usize;
    
    // Need at least 'period' data points to calculate SMA
    if data.len() < period {
        debug!("Not enough data points ({}) for SMA period {}", data.len(), period);
        return Vec::new();
    }
    
    let mut sma_values = Vec::new();
    
    // Calculate SMA for each point where we have enough historical data
    for i in (period - 1)..data.len() {
        // Sum the price/sqft values for the last 'period' days
        let mut sum = 0.0;
        let mut count = 0;
        
        for j in (i - period + 1)..=i {
            if let Some(price) = data[j].avg_price_per_sqft {
                sum += price;
                count += 1;
            }
        }
        
        // Only calculate SMA if we have at least some valid data points
        if count > 0 {
            let sma = sum / count as f64;
            sma_values.push((i, sma));
            debug!("SMA at index {}: ${:.2}", i, sma);
        }
    }
    
    info!("Calculated {} SMA values for period {}", sma_values.len(), period);
    sma_values
}

/// Calculate Simple Moving Average for listing counts
/// 
/// # Arguments
/// * `data` - Vector of housing data points, should be sorted by date
/// * `config` - SMA configuration specifying the period
/// 
/// # Returns
/// Vector of (date_index, sma_value) tuples
pub fn calculate_listings_sma(data: &[HousingData], config: &SmaConfig) -> Vec<(usize, f64)> {
    let period = config.period_days as usize;
    
    if data.len() < period {
        return Vec::new();
    }
    
    let mut sma_values = Vec::new();
    
    for i in (period - 1)..data.len() {
        let sum: i32 = data[(i - period + 1)..=i]
            .iter()
            .map(|d| d.active_listings)
            .sum();
        
        let sma = sum as f64 / period as f64;
        sma_values.push((i, sma));
    }
    
    info!("Calculated {} listing SMA values for period {}", sma_values.len(), period);
    sma_values
}

/// Generate a statistical summary of housing data over a time period
/// 
/// # Arguments
/// * `data` - Vector of housing data points to summarize
/// 
/// # Returns
/// MarketSummary with calculated statistics, or an error if insufficient data
pub fn generate_market_summary(data: &[HousingData]) -> Result<MarketSummary> {
    if data.is_empty() {
        anyhow::bail!("Cannot generate summary from empty dataset");
    }
    
    // Calculate average listings
    let total_listings: i32 = data.iter().map(|d| d.active_listings).sum();
    let avg_listings = total_listings as f64 / data.len() as f64;
    
    // Filter data with valid price/sqft values
    let prices: Vec<f64> = data.iter()
        .filter_map(|d| d.avg_price_per_sqft)
        .collect();
    
    if prices.is_empty() {
        anyhow::bail!("No valid price data available for summary");
    }
    
    // Calculate price statistics
    let sum_prices: f64 = prices.iter().sum();
    let avg_price_per_sqft = sum_prices / prices.len() as f64;
    
    let min_price_per_sqft = prices.iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    
    let max_price_per_sqft = prices.iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    
    // Calculate percentage change from start to end
    let first_price = data.iter()
        .find_map(|d| d.avg_price_per_sqft)
        .unwrap_or(0.0);
    
    let last_price = data.iter()
        .rev()
        .find_map(|d| d.avg_price_per_sqft)
        .unwrap_or(0.0);
    
    let price_change_percent = if first_price > 0.0 {
        ((last_price - first_price) / first_price) * 100.0
    } else {
        0.0
    };
    
    let summary = MarketSummary {
        avg_listings,
        avg_price_per_sqft,
        min_price_per_sqft,
        max_price_per_sqft,
        price_change_percent,
        data_points: data.len(),
    };
    
    info!("Generated market summary: avg_listings={:.1}, avg_price=${:.2}, change={:.2}%", 
          summary.avg_listings, summary.avg_price_per_sqft, summary.price_change_percent);
    
    Ok(summary)
}

/// Interpolate missing data points in a time series
/// This uses linear interpolation between known values
/// 
/// # Arguments
/// * `data` - Mutable reference to housing data (will be modified in place)
/// 
/// # Returns
/// Number of data points that were interpolated
pub fn interpolate_missing_data(data: &mut [HousingData]) -> usize {
    let mut interpolated_count = 0;
    
    // Find gaps where price/sqft is None
    let mut i = 0;
    while i < data.len() {
        if data[i].avg_price_per_sqft.is_none() {
            // Find the previous valid price
            let prev_price = if i > 0 {
                data[..i].iter()
                    .rev()
                    .find_map(|d| d.avg_price_per_sqft)
            } else {
                None
            };
            
            // Find the next valid price
            let next_price = data[(i + 1)..]
                .iter()
                .find_map(|d| d.avg_price_per_sqft);
            
            // Interpolate if we have both boundaries
            if let (Some(prev), Some(next)) = (prev_price, next_price) {
                let interpolated = (prev + next) / 2.0;
                data[i].avg_price_per_sqft = Some(interpolated);
                interpolated_count += 1;
                debug!("Interpolated price at index {}: ${:.2}", i, interpolated);
            }
        }
        i += 1;
    }
    
    if interpolated_count > 0 {
        info!("Interpolated {} missing data points", interpolated_count);
    }
    
    interpolated_count
}

/// Validate data integrity and remove outliers
/// Outliers are defined as values more than 3 standard deviations from the mean
/// 
/// # Arguments
/// * `data` - Mutable reference to housing data (outliers will be marked as None)
/// 
/// # Returns
/// Number of outliers detected and removed
pub fn remove_outliers(data: &mut [HousingData]) -> usize {
    // Calculate mean and standard deviation
    let prices: Vec<f64> = data.iter()
        .filter_map(|d| d.avg_price_per_sqft)
        .collect();
    
    if prices.len() < 3 {
        // Not enough data to calculate meaningful statistics
        return 0;
    }
    
    let sum: f64 = prices.iter().sum();
    let mean = sum / prices.len() as f64;
    
    let variance: f64 = prices.iter()
        .map(|&p| (p - mean).powi(2))
        .sum::<f64>() / prices.len() as f64;
    
    let std_dev = variance.sqrt();
    
    // Mark outliers (more than 3 standard deviations from mean)
    let mut outlier_count = 0;
    let threshold = 3.0 * std_dev;
    
    for point in data.iter_mut() {
        if let Some(price) = point.avg_price_per_sqft {
            if (price - mean).abs() > threshold {
                debug!("Removing outlier: ${:.2} (mean: ${:.2}, std_dev: ${:.2})", 
                       price, mean, std_dev);
                point.avg_price_per_sqft = None;
                outlier_count += 1;
            }
        }
    }
    
    if outlier_count > 0 {
        info!("Removed {} outliers (mean=${:.2}, std_dev=${:.2})", 
              outlier_count, mean, std_dev);
    }
    
    outlier_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::core::models::DataSource;

    fn create_test_data(count: usize) -> Vec<HousingData> {
        (0..count).map(|i| HousingData {
            date: Utc::now(),
            active_listings: 40 + i as i32,
            avg_price_per_sqft: Some(400.0 + i as f64 * 10.0),
            data_source: DataSource::Historical,
            last_updated: Utc::now(),
        }).collect()
    }

    #[test]
    fn test_calculate_sma() {
        let data = create_test_data(30);
        let config = SmaConfig::new(7);
        let sma_values = calculate_sma(&data, &config);
        
        // Should have SMA values for all but the first 6 data points
        assert_eq!(sma_values.len(), 24);
    }

    #[test]
    fn test_calculate_sma_insufficient_data() {
        let data = create_test_data(5);
        let config = SmaConfig::new(7);
        let sma_values = calculate_sma(&data, &config);
        
        // Should return empty vector
        assert_eq!(sma_values.len(), 0);
    }

    #[test]
    fn test_generate_market_summary() {
        let data = create_test_data(10);
        let summary = generate_market_summary(&data).unwrap();
        
        assert!(summary.avg_listings > 0.0);
        assert!(summary.avg_price_per_sqft > 0.0);
        assert_eq!(summary.data_points, 10);
    }
}
