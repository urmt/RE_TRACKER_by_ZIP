# RE_TRACKER_by_ZIP

**Rossmoor Housing Inventory Tracker** - A decentralized, cross-platform housing market data visualization tool for ZIP code 90720 (Rossmoor, CA).

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)

## 🏠 Overview

RE_TRACKER_by_ZIP is a free, transparent, always-available housing market intelligence application designed for Rossmoor residents and real estate investors. The application provides real-time and historical housing market data through interactive charts, accessible via system tray icons (desktop) or notification widgets (mobile).

### Key Features

- 📊 **Interactive dual-axis line charts** showing active listings and price per square foot
- 📈 **Adjustable Simple Moving Averages** (7, 30, 90-day periods)
- 📅 **Multiple timeframes** (7 days, 30 days, 1 year, all-time)
- 💾 **Offline capability** with local data caching
- 🌐 **IPFS-based P2P synchronization** (coming in Phase 2)
- 🎨 **Dark mode** with WCAG AA contrast compliance
- 💰 **Zero-cost operation** - no paid APIs or services

## 🚀 Current Status: Phase 0 (Proof of Concept)

This is currently a proof of concept implementation demonstrating the core data storage and processing functionality. The application can:

- ✅ Initialize and manage a SQLite database
- ✅ Store and retrieve housing market data
- ✅ Calculate Simple Moving Averages
- ✅ Generate market statistics and summaries
- ✅ Handle data validation and outlier detection

### Coming in Phase 1 (Desktop MVP)
- ⏳ Historical data fetching from Zillow Research
- ⏳ Real-time web scraping
- ⏳ Chart UI with Chart.js
- ⏳ System tray integration

## 📋 Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://rustup.rs/))
- **SQLite** (bundled with rusqlite crate)
- **Git** for version control

## 🔧 Installation

### Clone the Repository

```bash
git clone https://github.com/urmt/RE_TRACKER_by_ZIP.git
cd RE_TRACKER_by_ZIP
```

### Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Run the Application

```bash
# Run with default logging
RUST_LOG=info cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Run the release build
./target/release/re_tracker_by_zip
```

## 📁 Project Structure

```
RE_TRACKER_by_ZIP/
├── src/
│   ├── core/                    # Core business logic
│   │   ├── models.rs           # Data structures
│   │   ├── storage.rs          # SQLite database wrapper
│   │   ├── data_processor.rs  # SMA calculations & analysis
│   │   └── mod.rs              # Module exports
│   ├── utils/                   # Utility modules
│   │   └── mod.rs
│   └── main.rs                  # Application entry point
├── tests/                       # Test files
├── docs/                        # Documentation
├── frontend/                    # Web UI (Phase 1)
├── Cargo.toml                   # Rust dependencies
├── WARP.md                      # Detailed project specification
├── README.md                    # This file
└── .gitignore
```

## 💻 Usage

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_storage_initialization
```

### Database Location

The application stores data in:
- **Linux**: `~/.local/share/re_tracker/housing_data.db`
- **Windows**: `%USERPROFILE%\.local\share\re_tracker\housing_data.db`
- **macOS**: `~/.local/share/re_tracker/housing_data.db`

## 🔍 Example Output

```
========================================
RE_TRACKER_by_ZIP - Version 0.1.0
Rossmoor Housing Inventory Tracker
========================================
Tracking ZIP code: 90720
Update interval: 12 hours
Database location: "/home/user/.local/share/re_tracker/housing_data.db"
Current data points in database: 30
Latest data point:
  Date: 2024-12-03
  Active Listings: 44
  Avg Price/SqFt: $432.50
  Source: Historical
========================================
```

## 🛠️ Development

### Code Style

This project follows Rust best practices and includes:
- **Comprehensive comments** for novice programmers
- **Error handling** with `anyhow` and `thiserror`
- **Logging** with `log` and `env_logger`
- **Unit tests** for all core modules

### Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📊 Data Sources

### Historical Data
- **Source**: [Zillow Research Data](https://www.zillow.com/research/data/)
- **Format**: CSV files with median home values and listing prices
- **Update Frequency**: Monthly (around 16th-20th of each month)

### Real-Time Data (Phase 1)
- **Sources**: Zillow and Redfin public listing pages
- **Method**: Ethical web scraping with rate limiting
- **Update Frequency**: Every 12 hours

## 🔒 Security & Privacy

- **HTTPS only** for all network requests
- **Local data storage** - no cloud dependencies
- **No telemetry** or user tracking
- **Optional database encryption** (coming in Phase 2)
- **Ethical scraping** with respect for robots.txt and rate limits

## 📝 License

This project is dual-licensed under:
- MIT License
- Apache License 2.0

Choose whichever license works best for your use case.

## 🙏 Acknowledgments

- [Zillow Research](https://www.zillow.com/research/) for providing historical housing data
- The Rust community for excellent tooling and libraries
- Rossmoor, CA community

## 📧 Contact

- **GitHub**: [@urmt](https://github.com/urmt)
- **Project Repository**: [RE_TRACKER_by_ZIP](https://github.com/urmt/RE_TRACKER_by_ZIP)
- **Issues**: [GitHub Issues](https://github.com/urmt/RE_TRACKER_by_ZIP/issues)

## 🗺️ Roadmap

### Phase 0: Proof of Concept ✅ (Current)
- [x] Project setup and structure
- [x] Core data models
- [x] SQLite storage implementation
- [x] Data processing and SMA calculations

### Phase 1: Desktop MVP (Q1 2025)
- [ ] Historical data fetcher
- [ ] Web scraping implementation
- [ ] Chart UI with Chart.js
- [ ] System tray integration
- [ ] Comprehensive testing

### Phase 2: IPFS Integration (Q2 2025)
- [ ] P2P data synchronization
- [ ] Decentralized app distribution
- [ ] Multi-node testing

### Phase 3: Mobile Port (Q3 2025)
- [ ] Flutter application
- [ ] Home screen widget
- [ ] Background data updates

### Phase 4: Launch (Q4 2025)
- [ ] Documentation completion
- [ ] Community outreach
- [ ] IPFS distribution setup

---

**Made with ❤️ for the Rossmoor community**
