# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Common Development Commands

### Building
```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized for size and speed)
cargo build --release

# Run directly without explicit build step
RUST_LOG=info cargo run

# Run with debug-level logging
RUST_LOG=debug cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output visible (useful for debugging)
cargo test -- --nocapture

# Run a specific test function
cargo test test_storage_initialization

# Run tests in a specific module
cargo test core::storage

# Run tests with debug logging
RUST_LOG=debug cargo test -- --nocapture
```

### Code Quality
```bash
# Check code without building (fast)
cargo check

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run linter (clippy) - install with: rustup component add clippy
cargo clippy -- -D warnings

# Update dependencies
cargo update
```

### Database Management
```bash
# Database location varies by platform:
# Linux: ~/.local/share/re_tracker/housing_data.db
# Windows: %USERPROFILE%\.local\share\re_tracker\housing_data.db
# macOS: ~/.local/share/re_tracker/housing_data.db

# View database with sqlite3
sqlite3 ~/.local/share/re_tracker/housing_data.db

# Example queries:
# SELECT COUNT(*) FROM housing_data;
# SELECT * FROM housing_data ORDER BY date DESC LIMIT 10;
# .schema housing_data
```

## Project Overview

**Current Phase**: Phase 0 (Proof of Concept) - Core data storage and processing are implemented.

A cross-platform housing inventory tracking application for ZIP code 90720 (Rossmoor, CA). The application will visualize real-time and historical housing market data through interactive charts, with IPFS-based decentralized P2P synchronization.

**Key Innovation**: Decentralized data distribution via IPFS with peer-to-peer synchronization.

**Target Platforms**: Linux (Fedora), Windows, macOS, Android (Flutter in Phase 3)

### Core Value Proposition
- Zero-cost operation (no paid APIs, hosting, or services)
- Privacy-respecting architecture (no telemetry)
- Offline-first with local SQLite storage
- Transparent, open-source housing market intelligence

## Architecture

### Module Organization

The codebase follows a layered architecture with clear separation of concerns:

```
src/
├── core/                      # Core business logic layer
│   ├── models.rs             # Data structures and types
│   ├── storage.rs            # SQLite database wrapper
│   ├── data_processor.rs     # SMA calculations, statistics
│   └── mod.rs                # Module exports
├── utils/                     # Utility functions (to be implemented)
└── main.rs                    # Application entry point
```

### Core Module Responsibilities

**models.rs** - Defines all data structures:
- `HousingData`: Primary data point (date, listings, price/sqft, source)
- `DataSource`: Enum tracking origin (Historical, Scraped, P2P)
- `AppConfig`: Application configuration with defaults
- `ScrapedData`: Result of scraping operations
- `SmaConfig`: Configuration for Simple Moving Averages
- `MarketSummary`: Statistical summary of market data

**storage.rs** - Database abstraction layer:
- `Storage::new()`: Initialize DB connection and schema
- `upsert_housing_data()`: Insert/update single data point
- `bulk_insert()`: Transaction-based batch inserts
- `get_data_range()`: Query by date range
- `get_latest_data()`: Retrieve most recent point
- `count_data_points()`: Count total records
- Schema: Indexed on `date` and `last_updated`

**data_processor.rs** - Data transformation and analytics:
- `calculate_sma()`: Simple Moving Average for price/sqft
- `calculate_listings_sma()`: SMA for listing counts
- `generate_market_summary()`: Statistical analysis (avg, min, max, % change)
- `interpolate_missing_data()`: Linear interpolation for gaps
- `remove_outliers()`: 3-sigma outlier detection and removal

### Data Flow (Current Implementation)

1. Application starts → initializes logger
2. `main.rs` determines database path (platform-specific)
3. `Storage::new()` creates/opens SQLite DB and schema
4. Sample data inserted if DB is empty (demonstration)
5. Application displays current data statistics

### Data Flow (Future Phases)

Phase 1 will add:
- **Data Fetcher**: Download Zillow Research CSVs → parse → bulk insert
- **Web Scraper**: Scrape Zillow/Redfin → extract metrics → upsert
- **Scheduler**: Cron-like task runner (12-hour intervals)
- **Chart Renderer**: Chart.js frontend → query data → render
- **System Tray**: Desktop integration

Phase 2 will add:
- **P2P Sync**: rust-libp2p → pub/sub on `/rossmoor-housing/data-updates`
- **Conflict Resolution**: Last-write-wins based on timestamp

## Technology Stack

### Core Dependencies
- **rusqlite** (0.30): SQLite with bundled engine
- **chrono** (0.4): Date/time handling with serde
- **serde/serde_json** (1.0): Serialization for IPC and P2P
- **anyhow/thiserror** (1.0): Error handling
- **log/env_logger** (0.11): Logging infrastructure
- **tokio** (1.35): Async runtime (for future IPFS integration)

### Future Dependencies
- **reqwest** (0.11): HTTP client for scraping and CSV downloads
- **scraper** (0.18): HTML parsing with CSS selectors
- **csv** (1.3): Zillow Research data parsing
- **libp2p** (future): P2P networking for IPFS

### Development Dependencies
- **mockito** (1.2): HTTP mocking for tests
- **tempfile** (3.8): Temporary files for DB tests

## Key Design Patterns

### Error Handling
- Use `anyhow::Result<T>` for application-level errors
- Use `thiserror` for custom error types (when needed)
- All database operations return `Result` with context
- Use `.context()` to add human-readable error messages

### Logging
- Initialize with `env_logger::init()` in main
- Control verbosity via `RUST_LOG` environment variable
- Use `info!()` for user-visible events
- Use `debug!()` for development/troubleshooting
- Never log sensitive data

### Database Patterns
- Use `INSERT OR REPLACE` for upserts (idempotent)
- Batch inserts wrapped in transactions for performance
- All queries use parameterized statements (SQL injection safe)
- Indexes on frequently queried columns (date, last_updated)
- Store dates as ISO 8601 strings (YYYY-MM-DD)

### Testing Patterns
- Unit tests in same file as implementation (`#[cfg(test)] mod tests`)
- Use `tempfile::NamedTempFile` for isolated DB tests
- Helper functions (e.g., `create_test_data()`) for test fixtures
- Test edge cases: empty data, insufficient data, outliers

## Development Workflow

### Adding New Data Processing Functions

1. Define function signature in `src/core/data_processor.rs`
2. Implement with proper logging (debug for details, info for summaries)
3. Add unit tests in same file under `#[cfg(test)] mod tests`
4. Export via `src/core/mod.rs` if needed by other modules
5. Run `cargo test` and `cargo clippy`

### Adding New Database Operations

1. Add method to `Storage` struct in `src/core/storage.rs`
2. Use parameterized queries with `rusqlite::params!`
3. Add context to all `Result` returns
4. Write test with temp database
5. Consider if an index is needed for performance

### Modifying Data Models

1. Update struct in `src/core/models.rs`
2. Update database schema in `Storage::initialize_schema()` if persisted
3. Add migration logic if schema changes (future: use `refinery` crate)
4. Update serialization tests
5. Update all dependent code

## Project Roadmap Context

**Phase 0 (Complete)**: Core data models, storage, processing
**Phase 1 (Next)**: Data fetching, scraping, Chart.js UI, system tray
**Phase 2**: IPFS/libp2p integration, P2P sync
**Phase 3**: Flutter mobile app
**Phase 4**: Launch and distribution

When implementing Phase 1 features, place:
- Data fetcher: `src/core/data_fetcher.rs`
- Web scraper: `src/utils/scraper.rs`
- Scheduler: `src/utils/scheduler.rs`
- Frontend: `frontend/` directory (HTML/CSS/JS)

## Data Sources

### Historical Data
- **Source**: Zillow Research (https://www.zillow.com/research/data/)
- **Format**: CSV files with median home values and listing prices
- **Update Frequency**: Monthly (around 16th-20th)
- **Access**: Direct HTTPS download

### Real-Time Data (Phase 1)
- **Sources**: Zillow and Redfin public listing pages
- **Metrics**: Active listings count, average price per square foot
- **Update Frequency**: Every 12 hours (12 AM and 12 PM local time)
- **Method**: Ethical web scraping with rate limiting (max 1 req/min)
- **Important**: Respect robots.txt, use appropriate User-Agent

## Security and Privacy

- **HTTPS only** for all network requests
- **No telemetry** or user tracking
- **Local storage** only (no cloud dependencies)
- **Rate limiting** for ethical scraping
- **Input validation** for all external data
- **P2P message verification** (Phase 2: signature verification)

## Performance Targets

- **Desktop**: <3s cold start, <500ms chart render, <100MB RAM
- **Database**: Bulk inserts >1000 records/sec
- **SMA Calculation**: <50ms for 365 days of data
- **Network**: <30s scraping timeout, 12-hour update intervals

## GitHub Repository
https://github.com/urmt/RE_TRACKER_by_ZIP

## License
MIT or Apache 2.0 (dual-licensed)

# RE_TRACKER_by_ZIP - Rossmoor Housing Inventory Tracker

## Project Overview
A cross-platform housing inventory tracking application focused on ZIP code 90720 (Rossmoor, CA). The application visualizes real-time and historical housing market data through interactive charts, accessible via system tray icons (desktop) or notification widgets (mobile).

**Key Innovation**: IPFS-based decentralized distribution with peer-to-peer data synchronization ensures all running instances receive updates without centralized infrastructure.

**Target Platforms**: Linux (Fedora), Windows, macOS, Android

**Core Value Proposition**: Free, transparent, always-available housing market intelligence for Rossmoor residents and investors.

## Project Goals
- Zero-cost operation (no paid APIs, hosting, or services)
- Decentralized P2P data synchronization via IPFS
- Cross-platform native performance
- Real-time and historical market data visualization
- Privacy-respecting architecture

## Technology Stack

### Desktop Application (Tauri + Rust)
- **Frontend**: HTML/CSS/JavaScript with Chart.js
- **Backend**: Rust for data processing, SQLite for storage
- **System Tray**: tray-icon crate
- **HTTP Client**: reqwest for scraping
- **HTML Parser**: scraper crate (based on selectors)
- **P2P**: rust-libp2p for IPFS pub/sub

### Mobile Application (Flutter)
- **Framework**: Flutter/Dart
- **Charts**: fl_chart package
- **Storage**: sqflite for SQLite
- **HTTP**: http package
- **HTML Parsing**: html package
- **P2P**: dart-libp2p or WebRTC fallback

### Shared Components
- **IPFS**: Public gateways for distribution + libp2p pub/sub
- **Pinning**: Pinata free tier (1GB storage)
- **Data Format**: JSON for IPC and P2P messages

## Architecture

### Core Modules
1. **Data Fetcher Module**: Acquire data from external sources
   - Historical Data Loader: Zillow Research CSV files
   - Real-time Scraper: Ethical web scraping with rate limiting
   - Scheduler: 12-hour interval updates

2. **Data Processor Module**: Transform raw data into chart-ready format
   - Calculate Simple Moving Averages (SMA)
   - Merge historical and real-time datasets
   - Handle missing data interpolation
   - Validate data integrity

3. **Storage Module**: Persist data locally for offline access
   - Desktop: SQLite
   - Mobile: SQLite (sqflite)
   - Schema: date, active_listings, avg_price_per_sqft, data_source, last_updated

4. **Chart Renderer Module**: Visualize data with interactive controls
   - Dual-axis line chart (listings + price/sqft)
   - Dynamic SMA overlays
   - Zoom/pan controls
   - Responsive canvas scaling

5. **P2P Sync Module**: Broadcast/receive data updates across IPFS
   - Topic: /rossmoor-housing/data-updates
   - Conflict resolution: Last-write-wins based on timestamp

6. **UI Layer**: Platform-specific interfaces
   - Desktop: System tray icon with popup chart
   - Mobile: Home screen widget with full app view

## Data Sources

### Historical Data
- **Source**: Zillow Research (https://www.zillow.com/research/data/)
- **Files**: CSV files with median home values and listing prices
- **Update Frequency**: Monthly (around 16th-20th)
- **Access**: Direct HTTPS download

### Real-Time Data
- **Sources**: Zillow and Redfin public listing pages
- **Metrics**: Active listings count, average price per square foot
- **Update Frequency**: Every 12 hours (12 AM and 12 PM local time)
- **Method**: Ethical web scraping with rate limiting (max 1 req/min)

## Key Features
- Interactive dual-axis line chart
- Adjustable Simple Moving Averages (7, 30, 90-day periods)
- Adjustable timeframes (7 days, 30 days, 1 year, all-time)
- Zoom, tooltips, legend toggles
- Offline capability with cached data
- P2P data synchronization across all instances
- Export to CSV for external analysis
- Dark mode with WCAG AA contrast compliance

## Development Standards
- **Comments**: Descriptive comments for novice programmers
- **Error Handling**: Comprehensive error checking for all complex operations
- **Code Quality**: Follow Rust and Dart best practices
- **Testing**: Unit tests for core modules, integration tests for data flow
- **Logging**: Detailed logging for debugging (no external telemetry)
- **Security**: HTTPS only, signature verification for P2P messages

## Implementation Roadmap

### Phase 0: Proof of Concept (Current Phase)
- Python/Rust script to scrape Zillow
- Simple HTML page with Chart.js rendering
- Validate data accuracy

### Phase 1: Desktop MVP
1. Set up Tauri project structure
2. Implement data fetcher (CSV download + scraping)
3. Create SQLite schema and storage module
4. Build chart UI with Chart.js
5. Implement SMA calculations
6. Add system tray integration
7. Testing and optimization

### Phase 2: IPFS Integration
1. Integrate rust-libp2p
2. Implement pub/sub topic subscription
3. Create message protocol and serialization
4. Set up IPFS distribution and pinning
5. Multi-node testing

### Phase 3: Mobile Port
1. Create Flutter project with Rust FFI bindings
2. Port core data modules
3. Design responsive mobile UI
4. Build home screen widget
5. Implement background services (WorkManager)
6. Device testing and APK build

### Phase 4: Launch
1. Create documentation
2. Publish to GitHub (MIT/Apache 2.0 license)
3. Set up IPFS distribution
4. Community announcement

## Project Structure
```
RE_TRACKER_by_ZIP/
├── src/
│   ├── core/
│   │   ├── data_fetcher.rs      # HTTP requests, scraping logic
│   │   ├── data_processor.rs    # SMA calculations, data merging
│   │   ├── storage.rs           # SQLite wrapper
│   │   ├── p2p_sync.rs          # libp2p integration
│   │   └── models.rs            # Data structures
│   ├── utils/
│   │   ├── scraper.rs           # HTML parsing helpers
│   │   └── scheduler.rs         # Cron-like task execution
│   └── main.rs                  # Application entry point
├── src-tauri/                   # Tauri-specific code
│   ├── main.rs                  # Desktop app initialization
│   ├── commands.rs              # Tauri commands for frontend IPC
│   └── tray.rs                  # System tray management
├── frontend/                    # Desktop UI
│   ├── index.html               # Chart UI
│   ├── chart.js                 # Chart.js configuration
│   └── styles.css               # Responsive CSS
├── mobile/                      # Flutter mobile app
│   └── lib/
│       ├── main.dart            # Mobile app entry point
│       ├── screens/
│       ├── widgets/
│       └── services/
├── tests/                       # Unit and integration tests
├── docs/                        # Additional documentation
├── .gitignore
├── README.md
├── Cargo.toml                   # Rust dependencies
├── package.json                 # Node/Tauri dependencies
└── WARP.md                      # This file

```

## Security Considerations
- **Data Integrity**: Signature verification for P2P messages
- **Network Security**: HTTPS only, rate limiting
- **IP Privacy**: Optional Tor integration, VPN recommended
- **Local Security**: Optional database encryption
- **Scraping Defense**: User-agent rotation, request delays

## Performance Targets
- **Desktop**: <3s cold start, <500ms chart render, <100MB RAM
- **Mobile**: <5s cold start, <1% battery per hour, <10MB APK
- **Network**: <30s scraping, 30s timeouts

## Testing Strategy
- Unit tests for data processor, scraper, storage
- Integration tests for end-to-end data flow
- UI tests for desktop (WebDriver) and mobile (Flutter)
- Performance tests for chart rendering and memory usage

## Maintenance
- **Monthly**: Verify data source URLs, update scraping selectors
- **Quarterly**: Security audit, performance profiling
- **Yearly**: Major version release, code refactoring

## GitHub Repository
https://github.com/urmt/RE_TRACKER_by_ZIP

## License
MIT or Apache 2.0 (to be determined)

## Success Metrics (12 months post-launch)
- 100+ active users across all platforms
- 99% uptime for data updates
- <5 critical bugs reported
- 10+ community contributions
