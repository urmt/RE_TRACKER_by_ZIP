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
