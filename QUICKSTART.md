# Quick Start Guide

**For resuming development on RE_TRACKER_by_ZIP**

## Current Status (as of Dec 3, 2025)
✅ **Phase 0** - Complete  
✅ **Phase 1** - Complete  
⏳ **Phase 2** - Not started (IPFS)  
⏳ **Phase 3** - Not started (Mobile)

---

## First Steps When Resuming

1. **Read STATUS.md** - Full project status and roadmap
2. **Read WARP.md** - Architecture and development patterns
3. **Check database**: `cargo run --release -- stats`
4. **Test current state**: `cargo run --release -- serve --port 8080`

---

## What Works Right Now

### Running the Application
```bash
# Populate database with synthetic data
cargo run --release -- fetch

# Add scraped data point
cargo run --release -- scrape

# Start web dashboard
cargo run --release -- serve --port 8080

# View current statistics
cargo run --release -- stats
```

### View the Dashboard
Open http://localhost:8080 in your browser to see:
- Interactive dual-axis chart (listings + price/sqft)
- Adjustable timeframes and SMA overlays
- Statistics cards with market trends
- Dark mode responsive design

---

## Project Structure

```
RE_TRACKER_by_ZIP/
├── src/
│   ├── main.rs                 # CLI + HTTP server
│   ├── core/
│   │   ├── models.rs          # Data structures
│   │   ├── storage.rs         # SQLite database
│   │   ├── data_processor.rs  # SMA calculations
│   │   └── data_fetcher.rs    # Historical data (NEW in Phase 1)
│   └── utils/
│       └── scraper.rs         # Web scraping (NEW in Phase 1)
├── frontend/                   # Web dashboard (NEW in Phase 1)
│   ├── index.html
│   ├── styles.css
│   └── chart.js
├── STATUS.md                   # Project tracking
├── WARP.md                     # Development guide
├── README.md                   # User documentation
└── QUICKSTART.md              # This file
```

---

## What to Work On Next

### Option A: Enhance Phase 1 (Production-Ready)
**Goal**: Make it work with real data

1. **Real Zillow CSV Parser**
   - Download CSV from https://www.zillow.com/research/data/
   - Parse date columns (format: "2024-01-31", "2024-02-29", etc.)
   - Filter for ZIP 90720
   - Extract median prices and inventory counts
   
2. **Real Web Scraping**
   - Add `headless_chrome` crate to Cargo.toml
   - Launch headless browser
   - Navigate to Zillow search page for 90720
   - Extract listing count from rendered page
   - Handle anti-scraping (delays, User-Agent rotation)
   
3. **System Tray Icon**
   - Add `tray-icon` crate
   - Create tray menu: "Open Dashboard", "Refresh Data", "Exit"
   - Launch browser on "Open Dashboard" click

### Option B: Start Phase 2 (IPFS)
**Goal**: Decentralized P2P data sharing

1. **Setup libp2p**
   ```toml
   libp2p = { version = "0.53", features = ["tcp", "mdns", "gossipsub"] }
   ```

2. **Create P2P Module**
   - `src/core/p2p_sync.rs`
   - Initialize libp2p swarm
   - Subscribe to `/rossmoor-housing/data-updates` topic
   - Publish data updates on new scrapes

3. **Test Locally**
   - Run two instances on different ports
   - Verify P2P discovery and message passing

---

## Common Commands

```bash
# Development
cargo check                    # Fast syntax check
cargo build                    # Debug build
cargo build --release          # Optimized build
cargo test                     # Run tests
cargo clippy                   # Lint

# Database
sqlite3 ~/.local/share/re_tracker/housing_data.db "SELECT * FROM housing_data"

# Git
git status                     # Check changes
git add -A                     # Stage all
git commit -m "message"        # Commit
git push                       # Push to GitHub
```

---

## Important Notes

### Current Limitations
- **Synthetic Data**: Both fetch and scrape use generated data, not real sources
- **No Scheduling**: Manual runs only (no cron/background service)
- **No System Tray**: Desktop-only via web browser

### Database Location
- Linux: `~/.local/share/re_tracker/housing_data.db`
- Windows: `%USERPROFILE%\.local\share\re_tracker\housing_data.db`
- macOS: `~/.local/share/re_tracker/housing_data.db`

### Logging
Always use `RUST_LOG=info` or `RUST_LOG=debug` for visibility:
```bash
RUST_LOG=debug cargo run --release -- serve
```

---

## Resources

- **GitHub**: https://github.com/urmt/RE_TRACKER_by_ZIP
- **Zillow Research**: https://www.zillow.com/research/data/
- **Chart.js Docs**: https://www.chartjs.org/docs/latest/
- **Rust Book**: https://doc.rust-lang.org/book/

---

## Questions to Answer Before Starting

1. Do you want to finish Phase 1 with real data?
2. Or jump to Phase 2 for P2P functionality?
3. Is the current UI acceptable or needs work?
4. Should we add automated scheduling (cron)?

Check **STATUS.md** for detailed task breakdown!

---

**Last Sync**: December 3, 2025  
**Git Branch**: master  
**Git Remote**: origin (https://github.com/urmt/RE_TRACKER_by_ZIP.git)
