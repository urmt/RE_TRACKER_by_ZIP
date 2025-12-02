Systems Analysis Document: Rossmoor Housing Inventory Tracker
Executive Summary
This document presents a comprehensive systems analysis for a cross-platform housing inventory tracking application focused on ZIP code 90720 (Rossmoor, CA). The application visualizes real-time and historical housing market data through interactive charts, accessible via system tray icons (desktop) or notification widgets (mobile).
Key Innovation: IPFS-based decentralized distribution with peer-to-peer data synchronization ensures all running instances receive updates without centralized infrastructure.
Target Platforms: Linux (Fedora), Windows, macOS, Android
Core Value Proposition: Free, transparent, always-available housing market intelligence for Rossmoor residents and investors.

Requirements Overview
Functional Requirements

Data Collection

Historical data: Monthly aggregates from Zillow Research CSV files (2012-present)
Real-time data: Web scraping of public listing pages (twice daily)
Metrics: Active listings count, average price per square foot


Data Visualization

Live-updating line chart with dual metrics
Adjustable Simple Moving Averages (7, 30, 90-day periods)
Adjustable timeframes (7 days, 30 days, 1 year, all-time)
Interactive features: zoom, tooltips, legend toggles


Cross-Platform Deployment

Desktop: System tray icon with chart popup
Mobile: Widget/notification-based access
IPFS distribution for decentralized hosting


Data Synchronization

P2P broadcasting of new data via IPFS pub/sub
All instances update simultaneously without central server



Non-Functional Requirements

Cost: Zero-cost operation (no paid APIs, hosting, or services)
Performance: Low resource usage, offline capability
Reliability: Graceful error handling, cached fallbacks
Accessibility: Color-blind friendly palettes, responsive design
Maintainability: Modular architecture, comprehensive logging


System Architecture
High-Level Architecture
┌─────────────────────────────────────────────────────────────┐
│                    IPFS Network Layer                       │
│  (Pinata/Public Gateway for distribution + libp2p pub/sub)  │
└────────────────┬────────────────────────────┬───────────────┘
                 │                            │
         ┌───────▼────────┐          ┌────────▼──────────┐
         │  Desktop App   │          │   Mobile App      │
         │  (Tauri/Rust)  │          │  (Flutter/Dart)   │
         └───────┬────────┘          └────────┬──────────┘
                 │                            │
         ┌───────▼────────────────────────────▼───────────┐
         │        Application Core (Shared Logic)         │
         │  - Data Fetcher Module                         │
         │  - Data Processor Module                       │
         │  - Chart Renderer Module                       │
         │  - Storage Module (IndexedDB/SQLite)           │
         │  - P2P Sync Module (libp2p)                    │
         └───────┬────────────────────────────────────────┘
                 │
         ┌───────▼────────────────────────────────────────┐
         │           External Data Sources               │
         │  - Zillow Research CSV (monthly)              │
         │  - Zillow/Redfin scraping (12hr intervals)   │
         └───────────────────────────────────────────────┘
Component Architecture
1. Data Fetcher Module

Responsibility: Acquire data from external sources
Sub-components:

Historical Data Loader: Downloads/parses Zillow CSV files
Real-time Scraper: Ethical web scraping with rate limiting
Scheduler: Cron-like job execution (12-hour intervals)



2. Data Processor Module

Responsibility: Transform raw data into chart-ready format
Operations:

Calculate SMA for configurable periods
Merge historical and real-time datasets
Handle missing data interpolation
Validate data integrity



3. Storage Module

Responsibility: Persist data locally for offline access
Technology:

Desktop: SQLite or file-based JSON
Mobile/Browser: IndexedDB


Schema:

  housing_data {
    date: DateTime (primary key)
    active_listings: Integer
    avg_price_per_sqft: Float
    data_source: Enum(historical, scraped)
    last_updated: DateTime
  }
4. Chart Renderer Module

Responsibility: Visualize data with interactive controls
Technology: Chart.js (lightweight) or D3.js (advanced customization)
Features:

Dual-axis line chart
Dynamic SMA overlays
Zoom/pan controls
Responsive canvas scaling



5. P2P Sync Module

Responsibility: Broadcast/receive data updates across IPFS
Technology: js-libp2p or rust-libp2p
Protocol:

Topic: /rossmoor-housing/data-updates
Message format: JSON with timestamp, data hash, and delta
Conflict resolution: Last-write-wins based on timestamp



6. UI Layer

Desktop: System tray icon (using tray-icon crate for Tauri)
Mobile: Home screen widget (Flutter widget integration)
Popup Window: 800x600px chart view with controls


Data Flow
Initial Data Load (First Run)
[User launches app]
        ↓
[Check local storage for cached data]
        ↓
[If empty → Download Zillow CSV files]
        ↓
[Parse and populate local database]
        ↓
[Run initial scrape for current data]
        ↓
[Render chart with full dataset]
        ↓
[Join IPFS pub/sub network]
Periodic Data Update (Every 12 hours)
[Scheduler triggers update]
        ↓
[Scrape Zillow/Redfin for current listings]
        ↓
[Calculate new avg price per sqft]
        ↓
[Store in local database]
        ↓
[Broadcast update via IPFS pub/sub]
        ↓
[All subscribed peers receive update]
        ↓
[Peers update local storage and re-render charts]
User Interaction Flow
[User clicks system tray icon]
        ↓
[Load data from local storage]
        ↓
[Render chart with current settings]
        ↓
[User adjusts SMA period (e.g., 30-day)]
        ↓
[Recalculate SMA on-the-fly]
        ↓
[Update chart without re-fetching]
        ↓
[User zooms into specific time range]
        ↓
[Re-render visible portion]

Technology Stack
Desktop Application (Tauri + Rust)
Rationale: Tauri offers lightweight native binaries (~3MB), Rust's memory safety, and native system tray support. Superior to Electron for resource efficiency.
Stack:

Frontend: HTML/CSS/JavaScript with Chart.js
Backend: Rust for data processing, SQLite for storage
System Tray: tray-icon crate
HTTP Client: reqwest for scraping
HTML Parser: scraper crate (based on selectors)
P2P: rust-libp2p for IPFS pub/sub

Deployment: Single executable per platform, distributed via IPFS
Mobile Application (Flutter)
Rationale: Flutter provides true cross-platform code sharing, excellent performance, and native widget support for Android.
Stack:

Framework: Flutter/Dart
Charts: fl_chart package
Storage: sqflite for SQLite
HTTP: http package
HTML Parsing: html package
P2P: dart-libp2p (experimental) or WebRTC fallback
Notifications: flutter_local_notifications for background updates

Deployment: APK distributed via IPFS, sideloaded installation
Shared Components

IPFS Gateway: Public gateways (ipfs.io, cloudflare-ipfs.com) for initial app download
Pinning Service: Pinata free tier (1GB storage) for keeping app and data updates available
Data Format: JSON for inter-process communication and P2P messages


Data Sources and Integration
1. Historical Data: Zillow Research
Source: https://www.zillow.com/research/data/
Files Required:

Zip_zhvi_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv (median home values)
Zip_MedianListingPrice_AllHomes.csv (listing prices)
Custom calculation for price/sqft using multiple files

Access Method:

Direct HTTPS download (no authentication)
Updated monthly around the 16th
CSV parsing with standard libraries

Challenges:

Zillow may not provide granular per-sqft historical data for specific ZIPs
Mitigation: Calculate approximations from median prices and average home sizes (if available), or focus on listing counts + recent scraped data for price/sqft trends

2. Real-Time Data: Web Scraping
Target Sites:

Zillow: https://www.zillow.com/rossmoor-ca-90720/
Redfin: https://www.redfin.com/zipcode/90720

Scraping Strategy:
javascript// Pseudocode
async function scrapeZillow() {
  const html = await fetch(url, {
    headers: { 'User-Agent': 'Mozilla/5.0...' }
  });
  
  const dom = parseHTML(html);
  const listingsCount = dom.select('.search-count').text(); // e.g., "42 Homes"
  const listings = dom.selectAll('.list-card');
  
  let totalPricePerSqft = 0;
  let validListings = 0;
  
  listings.forEach(card => {
    const price = parsePrice(card.select('.list-card-price').text());
    const sqft = parseInt(card.select('.list-card-sqft').text());
    if (price && sqft) {
      totalPricePerSqft += (price / sqft);
      validListings++;
    }
  });
  
  return {
    count: parseInt(listingsCount),
    avgPricePerSqft: totalPricePerSqft / validListings,
    timestamp: Date.now()
  };
}
```

**Ethical Considerations**:
- Respect `robots.txt`
- Rate limiting: Max 1 request per minute
- User-agent rotation to appear as regular browser traffic
- Scraping only public data (no authentication required)
- Fail gracefully if blocked (use cached data)

**Fallback Strategy**:
- If primary scraper fails, try alternate site (Redfin)
- If both fail, log error and continue with cached data
- Display "Last Updated: X hours ago" in UI

### 3. Data Update Frequency

- **Historical CSV**: Check monthly (day 16-20 of each month)
- **Scraped Data**: Every 12 hours (12 AM and 12 PM local time)
- **User-Triggered**: Manual refresh button (rate-limited to once per hour)

---

## User Interface and Experience

### Desktop UI Design

**System Tray Icon**:
- Icon: Simple house graphic with upward/downward trend arrow (changes color based on 7-day trend)
- Tooltip: "Rossmoor Housing - Active: 42 | Avg: $450/sqft"

**Chart Window (Popup)**:
```
┌────────────────────────────────────────────────────────┐
│ Rossmoor Housing Inventory Tracker            [_][□][X]│
├────────────────────────────────────────────────────────┤
│ ┌──────────────────────────────────────────────────┐   │
│ │  Controls:                                       │   │
│ │  Timeframe: [7D][30D][90D][1Y][All] ◄─────────► │   │
│ │  SMA: [7D✓][30D✓][90D ] [Custom: ___]           │   │
│ │  ☐ Show Listings  ☑ Show Price/SqFt             │   │
│ └──────────────────────────────────────────────────┘   │
│                                                        │
│ ┌──────────────────────────────────────────────────┐   │
│ │          [INTERACTIVE CHART AREA]                │   │
│ │  ┌─────────────────────────────────────────┐    │   │
│ │  │  Dual-axis line chart:                  │    │   │
│ │  │  - Left axis: # of listings (blue)      │    │   │
│ │  │  - Right axis: $/sqft (green)           │    │   │
│ │  │  - SMA overlays (dashed lines)          │    │   │
│ │  │  - Hover tooltips with exact values     │    │   │
│ │  │  - Zoom: Scroll wheel / pinch gesture   │    │   │
│ │  └─────────────────────────────────────────┘    │   │
│ └──────────────────────────────────────────────────┘   │
│                                                        │
│ Last Updated: 2 hours ago  [Refresh] [Export CSV]     │
└────────────────────────────────────────────────────────┘
```

**Color Scheme**:
- Background: Dark mode (#1E1E1E) with light mode toggle
- Listings line: Blue (#4A90E2) - high contrast
- Price/sqft line: Green (#7ED321) - distinct hue
- SMA overlays: Semi-transparent versions of primary colors
- Grid lines: Subtle gray (#3A3A3A)
- Tooltips: White text on dark background with drop shadow

**Accessibility**:
- All colors pass WCAG AA contrast requirements
- Keyboard navigation support (Tab, Arrow keys, Enter)
- Screen reader compatible labels

### Mobile UI Design

**Home Screen Widget** (Android):
- 4x2 grid space showing mini-chart thumbnail
- Tap to open full app
- Background updates every 12 hours

**Full App Screen**:
- Single-page interface with chart occupying 70% of screen
- Bottom drawer for controls (swipe up to reveal)
- Pinch-to-zoom on chart
- Floating action button for manual refresh

### Interaction Patterns

1. **First-Time User**:
   - Loading screen: "Downloading historical data... 45%"
   - Brief tutorial overlay: "Adjust timeframe here ↑"
   - Default view: Last 30 days, 7-day SMA enabled

2. **Returning User**:
   - Instant load from cache
   - Background update indicator (small spinner in corner)
   - Notification if major trend change (>20% in listings)

3. **Power User**:
   - Export to CSV for external analysis
   - Custom SMA periods (e.g., 14-day for swing traders)
   - Toggle between linear and logarithmic scales

---

## Cross-Platform Implementation

### Code Sharing Strategy

**Shared Core (Rust)**:
```
src/
  core/
    data_fetcher.rs      (HTTP requests, scraping logic)
    data_processor.rs    (SMA calculations, data merging)
    storage.rs           (SQLite wrapper, unified interface)
    p2p_sync.rs          (libp2p integration)
    models.rs            (Data structures: HousingData, Config)
  utils/
    scraper.rs           (HTML parsing helpers)
    scheduler.rs         (Cron-like task execution)
```

**Platform-Specific**:

**Desktop (Tauri)**:
```
src-tauri/
  main.rs              (App initialization, tray setup)
  commands.rs          (Tauri commands for frontend IPC)
  tray.rs              (System tray icon management)
frontend/
  index.html           (Chart UI)
  chart.js             (Chart.js configuration)
  styles.css           (Responsive CSS)
```

**Mobile (Flutter)**:
```
lib/
  main.dart            (App entry point, widget tree)
  screens/
    chart_screen.dart  (Main chart view)
  widgets/
    control_panel.dart (SMA/timeframe controls)
  services/
    data_service.dart  (Bridge to Rust via FFI)
  native/
    ffi_bindings.dart  (Rust-Flutter interface)
```

**Build Process**:
1. Compile Rust core to static libraries (`.a` for iOS, `.so` for Android, `.dll` for Windows)
2. Link libraries into platform-specific builds
3. Desktop: `tauri build --target x86_64-unknown-linux-gnu` (Fedora), `--target x86_64-pc-windows-msvc`, etc.
4. Mobile: `flutter build apk --release`

### Platform-Specific Considerations

**Linux (Fedora)**:
- System tray support via `libappindicator` (built into Tauri)
- Packaging: AppImage for portability
- Auto-start: Create `.desktop` file in `~/.config/autostart/`

**Windows**:
- System tray: Native Windows API via Tauri
- Installer: NSIS-based installer (Tauri default)
- Auto-start: Registry key in `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run`

**macOS**:
- Menu bar extra (macOS equivalent of system tray)
- DMG distribution
- Notarization required for Gatekeeper (free with Apple Developer account)

**Android**:
- Background service for data updates (WorkManager)
- Widget provider for home screen widget
- APK sideloading (no Google Play required)

---

## Decentralized Features: IPFS Integration

### Architecture Overview
```
┌─────────────────────────────────────────────────────┐
│             IPFS Distributed Network                │
│                                                     │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    │
│  │  Node A  │◄──►│  Node B  │◄──►│  Node C  │    │
│  │(Desktop) │    │ (Mobile) │    │(Desktop) │    │
│  └────┬─────┘    └────┬─────┘    └────┬─────┘    │
│       │               │               │           │
│       └───────────────┼───────────────┘           │
│                       │                           │
│              ┌────────▼────────┐                  │
│              │  IPFS Pub/Sub   │                  │
│              │ Topic: /rossmoor│                  │
│              └─────────────────┘                  │
└─────────────────────────────────────────────────────┘
Implementation Details
1. Application Distribution:

Package app as IPFS Content Addressable Archive (CAR)
Pin to Pinata: QmXxxxxx/rossmoor-tracker-v1.0.0/
Users fetch via: ipfs get QmXxxxxx or through HTTP gateway
Auto-update: Check /ipfs/QmXxxxxx/latest.json for new versions

2. Data Synchronization Protocol:
Message Format:
json{
  "type": "data_update",
  "timestamp": 1703001234567,
  "data": {
    "date": "2024-12-15",
    "active_listings": 42,
    "avg_price_per_sqft": 450.25
  },
  "node_id": "QmNodeABC123...",
  "signature": "0xSignature..." 
}
Pub/Sub Flow:

Node A scrapes new data at 12:00 PM
Node A publishes to /rossmoor-housing/updates
Nodes B and C (subscribed) receive message
Nodes validate timestamp and signature
Nodes update local storage and re-render charts

3. Peer Discovery:

Bootstrap nodes: Use public IPFS bootstrappers
DHT: Kademlia for peer routing
mDNS: Local network discovery (LAN peers)

4. Conflict Resolution:

Each node tracks last-seen timestamp per data point
If duplicate timestamps with different values: Average the values
Outlier detection: Reject data >2 standard deviations from recent mean

5. Bandwidth Optimization:

Delta updates: Only broadcast changed fields
Compression: Gzip JSON before transmission
Batching: Aggregate multiple updates into single message if within 1-hour window

IPFS vs. Traditional Server
AspectIPFS ApproachTraditional ServerCost$0 (free pinning tier)$5-50/month (VPS)CensorshipResistantSingle point of failureScalabilityAutomatic (more peers = more bandwidth)Manual scalingLatencyVariable (depends on peer proximity)ConsistentComplexityHigher (P2P networking)Lower (HTTP REST API)
Recommendation: IPFS is ideal for this use case given the zero-cost constraint and decentralized ethos. However, implement HTTP gateway fallback for users behind restrictive firewalls.

Testing and Debugging
Testing Strategy
1. Unit Tests:

Data processor: SMA calculation accuracy
Scraper: HTML parsing against saved fixtures
Storage: CRUD operations with SQLite
P2P: Message serialization/deserialization

2. Integration Tests:

End-to-end data flow: Fetch → Process → Store → Render
IPFS pub/sub: Multi-node message propagation
Error handling: Network failures, malformed data

3. UI Tests:

Desktop: Tauri WebDriver tests
Mobile: Flutter integration tests
Visual regression: Percy or Chromatic for screenshot diffs

4. Performance Tests:

Chart rendering: 10,000+ data points without lag
Memory usage: <100MB desktop, <50MB mobile
Scraping: Complete within 30 seconds

Debugging Tools
Built-in Logging:
rustuse log::{info, warn, error};

fn fetch_data() -> Result<HousingData> {
    info!("Starting data fetch at {}", Utc::now());
    match scrape_zillow() {
        Ok(data) => {
            info!("Successfully scraped {} listings", data.count);
            Ok(data)
        },
        Err(e) => {
            error!("Scraping failed: {}", e);
            Err(e)
        }
    }
}
```

**Log Locations**:
- Desktop: `~/.local/share/rossmoor-tracker/logs/`
- Mobile: Internal storage `/data/data/com.rossmoor/logs/`

**Debug UI**:
- Hidden panel (Ctrl+Shift+D): Shows raw data, last fetch status, peer count
- Network inspector: Log all HTTP requests with timing

**Crash Reporting**:
- Local crash dumps (no telemetry to respect privacy)
- User can optionally submit logs via GitHub issue template

---

## Security Considerations

### Threat Model

**Potential Threats**:
1. Malicious IPFS peers broadcasting false data
2. Web scraping IP bans
3. Man-in-the-middle attacks on HTTP requests
4. Local data tampering
5. Privacy: Exposing user's IP via IPFS

### Mitigation Strategies

**1. Data Integrity**:
- **Signature Verification**: Each P2P message signed with node's private key
- **Consensus Mechanism**: Require 3+ nodes to agree on data before accepting (if >3 nodes active)
- **Sanity Checks**: Reject listings count <5 or >500, price/sqft <$100 or >$2000

**2. Network Security**:
- **HTTPS Only**: All HTTP requests use TLS
- **Certificate Pinning**: For Zillow/Redfin domains (optional, may break with certificate rotation)
- **Rate Limiting**: Hard-coded max 1 request/min to prevent abuse

**3. IP Privacy**:
- **Tor Integration** (optional): Route scraping through Tor exit nodes
- **VPN Recommendation**: Suggest VPN usage in documentation
- **IPFS NAT Traversal**: Use relay nodes to obscure direct IP connections

**4. Local Security**:
- **Data Encryption**: Encrypt SQLite database with user-derived key (optional feature)
- **Sandboxing**: Tauri apps run in sandboxed environment by default
- **Permissions**: Request only necessary Android permissions (Internet, Storage)

**5. Scraping Defense**:
- **User-Agent Rotation**: Cycle through 10+ common browser UAs
- **Request Delays**: Randomize delay (30-90 seconds) between requests
- **Captcha Handling**: Manual fallback if CAPTCHA detected (prompt user to visit site)

---

## Performance Optimization

### Desktop Performance

**Target Metrics**:
- Cold start: <3 seconds
- Chart render: <500ms for 1 year of data
- Memory: <100MB RAM
- CPU: <5% idle, <30% during scraping

**Optimizations**:
1. **Lazy Loading**: Load historical data on-demand based on selected timeframe
2. **Data Decimation**: Reduce data points for older timeframes (e.g., daily → weekly for >1 year view)
3. **Canvas Rendering**: Use Chart.js canvas mode (faster than SVG)
4. **Worker Threads**: Offload SMA calculations to background threads
5. **Asset Optimization**: Minify JS/CSS, compress images

### Mobile Performance

**Target Metrics**:
- Cold start: <5 seconds
- Battery: <1% per hour in background
- APK size: <10MB

**Optimizations**:
1. **Flutter AOT Compilation**: Ahead-of-time compilation for faster startup
2. **Tree Shaking**: Remove unused Flutter/Dart libraries
3. **Efficient Widgets**: Use `const` constructors, `ListView.builder` for large lists
4. **Background Restrictions**: Limit background work to only essential data fetches
5. **ProGuard**: Obfuscate and shrink APK

### Network Optimization

1. **HTTP Compression**: Accept gzip/brotli encoding
2. **Caching**: Cache CSV files for 30 days, HTML for 12 hours
3. **Parallel Requests**: Fetch historical CSVs concurrently
4. **Timeout Configuration**: 30-second timeout for scraping (fail fast)

---

## Deployment and Distribution

### Build Pipeline
```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   GitHub     │────►│   CI/CD      │────►│   IPFS Pin   │
│  Repository  │     │  (Actions)   │     │   (Pinata)   │
└──────────────┘     └──────────────┘     └──────────────┘
                            │
                            ├──► Linux Binary (.AppImage)
                            ├──► Windows Installer (.exe)
                            ├──► macOS Bundle (.dmg)
                            └──► Android APK (.apk)
GitHub Actions Workflow:
yamlname: Build and Deploy
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Build Tauri App
        run: |
          npm install
          npm run tauri build
      - name: Upload to IPFS
        run: |
          ipfs add -r dist/
          curl -X POST "https://api.pinata.cloud/pinning/pinFileToIPFS" \
            -H "Authorization: Bearer $PINATA_JWT" \
            -F file=@dist/bundle.tar.gz
User Installation Process
Desktop:

Visit ipfs://QmXxxxxx or gateway URL
Download platform-specific installer
Run installer (auto-start enabled by default)
App appears in system tray

Mobile:

Enable "Install from Unknown Sources" in Android settings
Download APK from IPFS gateway
Install APK
Grant necessary permissions
Add home screen widget

Update Mechanism
Semantic Versioning: v1.2.3 (major.minor.patch)
Update Check:

On app start, fetch /ipfs/QmLatestVersion/version.json
Compare with local version
If newer version available, prompt user: "Update to v1.3.0? [Download] [Skip]"
Background download of new binary
Prompt to restart app after download completes

Rollback: Keep previous version in backups/ folder for easy rollback if update fails

Maintenance and Operations
Ongoing Maintenance Tasks
Monthly:

Verify Zillow CSV URLs still valid
Update scraping selectors if site redesign detected
Review error logs for patterns
Re-pin app to IPFS to ensure availability

Quarterly:

Security audit of dependencies (cargo audit, npm audit)
Performance profiling (identify bottlenecks)
User feedback review (GitHub issues/discussions)

Yearly:

Major version release with new features
Code refactoring for maintainability
Documentation updates

Monitoring Strategy
Self-Monitoring (No External Services):

Local log files with rotation (max 100MB)
Daily health check: App attempts data fetch, logs success/failure
User notification if >3 consecutive failures

Community Monitoring:

GitHub Issues for bug reports
Public dashboard (IPFS-hosted) showing:

Number of active nodes
Last successful data update timestamp
Average price/sqft trend (last 7 days)



Backup and Recovery
Data Backup:

SQLite database automatically backed up daily to backups/ folder
Keep last 7 days of backups
User can manually export all data to CSV

Disaster Recovery:

If local database corrupted: Re-download historical CSVs, re-scrape current data
If IPFS pin lost: Community re-pins from local copies
If scraping permanently blocked: Switch to alternative data source (documented in README)


Risk Analysis
Technical Risks
RiskLikelihoodImpactMitigationZillow blocks scrapingHighHighMultiple fallback sources (Redfin, Realtor.com), document manual update processIPFS adoption frictionMediumMediumProvide HTTP gateway mirrors, clear installation docsCross-platform bugsMediumMediumComprehensive testing matrix, phased rolloutData corruptionLowHighDaily backups, validation checksPerformance degradationLowMediumRegular profiling, optimize hotspots
Legal Risks
Web Scraping Legality:

Status: Generally legal in US (hiQ Labs v. LinkedIn)
Best Practices: Scrape only public data, respect robots.txt, rate limit
TOS Concerns: May violate Zillow/Redfin TOS (low enforcement risk for personal use)
Mitigation: Document ethical scraping practices, provide opt-out mechanism

Data Accuracy Liability:

Disclaimer: "Data provided for informational purposes only, not financial advice"
Attribution: Clearly label data sources
Error Handling: Display data quality indicators (e.g., "Last update failed")

Operational Risks
Data Source Changes:

Risk: Zillow discontinues Research CSV files or changes format
Probability: Low (stable since 2012)
Impact:
High (historical data unavailable)

Mitigation: Archive CSV files on IPFS, create community mirror

Community Dependency:

Risk: If few users adopt, P2P sync ineffective
Mitigation: App works fully in standalone mode, P2P is enhancement

Maintenance Burden:

Risk: Single developer overwhelmed
Mitigation: Open-source project, encourage contributions, modular architecture for easy onboarding


Recommendations and Improvements
Phase 1 Enhancements (Post-MVP)

Additional Metrics:

Days on market (DOM) average
Price reductions percentage
New listings vs. delisted homes


Predictive Analytics:

Linear regression for price forecasting
Seasonal decomposition of time series (STL)


Comparative Analysis:

Compare 90720 to neighboring ZIPs (90721, 90808)
National market benchmarking


Alerts System:

Push notifications for significant changes (e.g., "Listings dropped 20%!")
Customizable thresholds



Phase 2 Features (Advanced)

Machine Learning:

Price prediction model (using historical + macroeconomic data)
Anomaly detection for unusual market movements


Social Features:

Anonymous market sentiment sharing via IPFS pub/sub
Community annotations on chart ("Interest rate hike here")


API for Developers:

Expose data via local REST API for third-party integrations
Webhook support for automated trading strategies


Advanced Visualizations:

Heatmaps for ZIP code comparison
Candlestick charts for high/low/close prices
Volume indicators



Alternative Architecture Considerations
If IPFS Proves Too Complex:

Fallback: Static site hosted on GitHub Pages + GitHub Actions for updates
Sync: WebSocket server (free tier on Render.com) for real-time updates
Trade-off: Centralization, but easier debugging

If Scraping Becomes Unreliable:

Alternative 1: Crowdsourced data (users manually submit listings)
Alternative 2: Partner with local realtors for data feed
Alternative 3: Monthly-only updates via Zillow CSV (abandon real-time)


Implementation Roadmap
Phase 0: Proof of Concept (2 weeks)

 Python script to scrape Zillow, output JSON
 Simple HTML page with Chart.js rendering data
 Validate data accuracy against manual checks

Phase 1: Desktop MVP (6 weeks)
Week 1-2: Core Infrastructure

 Set up Tauri project structure
 Implement data fetcher (CSV download + scraping)
 Create SQLite schema and storage module

Week 3-4: UI Development

 Build chart UI with Chart.js
 Implement SMA calculations
 Add timeframe and control panel

Week 5: System Tray Integration

 Create system tray icon with dynamic tooltip
 Handle click events to show/hide chart

Week 6: Testing and Polish

 Write unit tests for core modules
 User acceptance testing
 Fix bugs and optimize performance

Phase 2: IPFS Integration (4 weeks)
Week 1-2: P2P Foundation

 Integrate rust-libp2p
 Implement pub/sub topic subscription
 Create message protocol and serialization

Week 3: Distribution Setup

 Create IPFS CAR of app bundle
 Set up Pinata pinning automation
 Document IPFS installation process

Week 4: Multi-Node Testing

 Test sync between 3+ nodes
 Implement conflict resolution
 Stress test with simulated network failures

Phase 3: Mobile Port (6 weeks)
Week 1-2: Flutter Setup

 Create Flutter project structure
 Implement Rust FFI bindings
 Port core data modules

Week 3-4: Mobile UI

 Design responsive chart layout
 Build control panel bottom sheet
 Create home screen widget

Week 5: Background Services

 Implement WorkManager for periodic updates
 Add notification system
 Handle Android permissions

Week 6: Testing and Deployment

 Device testing (various Android versions)
 Build signed APK
 Create installation guide

Phase 4: Launch (2 weeks)

 Create project website (IPFS-hosted)
 Write comprehensive documentation
 Publish to GitHub with open-source license (MIT/Apache 2.0)
 Announce on relevant forums (r/RealEstate, local Rossmoor groups)
 Set up community support channels (Discord/GitHub Discussions)

Total Timeline: ~18 weeks (4.5 months)
Resource Requirements:

1 full-time developer (Rust + Tauri + Flutter experience)
0.25 FTE designer (UI mockups and iconography)
Community testers (volunteer basis)


Conclusion
This systems analysis presents a comprehensive blueprint for a decentralized, cross-platform housing inventory tracker focused on Rossmoor, CA. The architecture leverages modern technologies (Tauri, Flutter, IPFS) to deliver a zero-cost, privacy-respecting solution for real-time market intelligence.
Key Strengths:

Complete independence from paid services
Resilient P2P architecture
Cross-platform native performance
Extensible design for future enhancements

Key Challenges:

Web scraping reliability
IPFS complexity for non-technical users
Ongoing maintenance commitment

Success Metrics (12 months post-launch):

100+ active users across all platforms
99% uptime for data updates
<5 critical bugs reported
10+ community contributions

This project exemplifies how modern decentralized technologies can democratize access to valuable market data, empowering individuals to make informed housing decisions without dependence on proprietary platforms.
