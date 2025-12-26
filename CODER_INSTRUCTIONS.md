# CODER_INSTRUCTIONS.md

**Instructions for AI Coding Assistants (VS Code, Cursor, GitHub Copilot, etc.)**

This file guides AI coding agents working in this repository. It points to all important documentation, coding standards, and project-specific rules.

---

## 📋 Essential Files to Read First

### 1. **WARP.md** (Primary Development Guide)
**Location**: `/WARP.md` (root directory)

**Purpose**: Comprehensive development guide created for Warp AI agents, but applicable to all AI assistants.

**Contains**:
- Common development commands (build, test, lint)
- Project architecture and module organization
- Core module responsibilities (models, storage, data_processor, data_fetcher, scraper)
- Data flow explanations
- Technology stack and dependencies
- Key design patterns (error handling, logging, database, testing)
- Development workflow for common tasks
- Database management
- Project roadmap context

**When to reference**: Always consult this before making architectural changes or adding new features.

---

### 2. **STATUS.md** (Project Tracking)
**Location**: `/STATUS.md` (root directory)

**Purpose**: Current project phase, completion status, and roadmap.

**Contains**:
- Phase completion checklist (Phase 0 ✅, Phase 1 ✅, Phase 2-4 pending)
- Detailed feature breakdown for each phase
- Current limitations
- Next development tasks (prioritized)
- Dependencies added per phase
- Performance metrics

**When to reference**: Before starting new work to understand what's complete and what's next.

---

### 3. **QUICKSTART.md** (Quick Reference)
**Location**: `/QUICKSTART.md` (root directory)

**Purpose**: Fast onboarding for resuming development.

**Contains**:
- Current status summary
- Commands to run the application
- Project structure overview
- "What to work on next" suggestions
- Common commands reference
- Important notes and limitations

**When to reference**: When quickly resuming work or checking how to run specific commands.

---

### 4. **README.md** (User Documentation)
**Location**: `/README.md` (root directory)

**Purpose**: User-facing documentation and project overview.

**Contains**:
- Project overview and features
- Installation instructions
- Usage examples
- Data sources explanation
- Roadmap timeline

**When to reference**: When writing user-facing features or documentation.

---

## 🎯 Project-Specific Coding Rules

### General Development Standards

**From WARP.md and project conventions:**

1. **Comments**: Write descriptive comments for novice programmers to understand
   - Every module should have a doc comment explaining its purpose
   - Complex functions need implementation comments
   - Public APIs must have rustdoc comments with examples

2. **Error Handling**: Comprehensive error checking for all complex operations
   - Use `anyhow::Result<T>` for application-level errors
   - Use `thiserror` for custom error types when needed
   - Always use `.context()` to add human-readable error messages
   - All database operations must return `Result` with context

3. **Logging**: Detailed logging for debugging (no external telemetry)
   - Initialize with `env_logger::init()` in main
   - Use `info!()` for user-visible events
   - Use `debug!()` for development/troubleshooting
   - Never log sensitive data
   - Control via `RUST_LOG` environment variable

4. **Testing**:
   - Unit tests in same file as implementation (`#[cfg(test)] mod tests`)
   - Use `tempfile::NamedTempFile` for isolated DB tests
   - Helper functions (e.g., `create_test_data()`) for test fixtures
   - Test edge cases: empty data, insufficient data, outliers
   - Run `cargo test` and `cargo clippy` before committing

5. **Database Patterns**:
   - Use `INSERT OR REPLACE` for upserts (idempotent)
   - Batch inserts wrapped in transactions for performance
   - All queries use parameterized statements (SQL injection safe)
   - Indexes on frequently queried columns
   - Store dates as ISO 8601 strings (YYYY-MM-DD)

6. **Code Quality**:
   - Follow Rust best practices
   - Run `cargo fmt` before committing
   - Run `cargo clippy -- -D warnings` before committing
   - Keep functions focused and single-purpose
   - Extract magic numbers into named constants

---

## 🏗️ Architecture Overview

### Module Organization

```
src/
├── main.rs              # CLI interface + HTTP server
├── core/                # Core business logic layer
│   ├── models.rs       # Data structures (HousingData, DataSource, etc.)
│   ├── storage.rs      # SQLite database wrapper
│   ├── data_processor.rs  # SMA calculations, statistics
│   ├── data_fetcher.rs    # Historical data fetching
│   └── mod.rs          # Module exports
└── utils/               # Utility functions
    ├── scraper.rs      # Real-time web scraping
    └── mod.rs          # Module exports
```

### Key Design Principles

1. **Layered Architecture**: Clear separation between core logic, utilities, and presentation
2. **Database Abstraction**: All DB access goes through `Storage` struct
3. **Idempotent Operations**: Data fetching and scraping can be run multiple times safely
4. **Offline-First**: Local SQLite storage, no cloud dependencies
5. **Privacy-Respecting**: No telemetry, HTTPS only, local data only

---

## 🔧 Development Workflow

### Adding New Features

**For Data Processing Functions**:
1. Define function in `src/core/data_processor.rs`
2. Implement with proper logging (debug for details, info for summaries)
3. Add unit tests in same file
4. Export via `src/core/mod.rs` if needed by other modules
5. Run `cargo test` and `cargo clippy`

**For Database Operations**:
1. Add method to `Storage` struct in `src/core/storage.rs`
2. Use parameterized queries with `rusqlite::params!`
3. Add context to all `Result` returns
4. Write test with temp database
5. Consider if an index is needed for performance

**For Data Models**:
1. Update struct in `src/core/models.rs`
2. Update database schema in `Storage::initialize_schema()` if persisted
3. Add migration logic if schema changes
4. Update serialization tests
5. Update all dependent code

**For Frontend Changes**:
1. HTML: `frontend/index.html`
2. Styles: `frontend/styles.css` (dark mode variables in `:root`)
3. JavaScript: `frontend/chart.js` (Chart.js configuration)
4. Test by running server: `cargo run --release -- serve --port 8080`

---

## 📦 Dependencies Reference

### Core Dependencies (Cargo.toml)
- **rusqlite** (0.30): SQLite with bundled engine
- **chrono** (0.4): Date/time handling with serde
- **serde/serde_json** (1.0): Serialization for IPC and P2P
- **anyhow/thiserror** (1.0): Error handling
- **log/env_logger** (0.11): Logging infrastructure
- **tokio** (1.35): Async runtime (for future IPFS integration)
- **reqwest** (0.11): HTTP client for scraping and CSV downloads
- **scraper** (0.18): HTML parsing with CSS selectors
- **csv** (1.3): Zillow Research data parsing
- **tiny_http** (0.12): HTTP server for frontend
- **clap** (4.4): CLI argument parsing

### Development Dependencies
- **mockito** (1.2): HTTP mocking for tests
- **tempfile** (3.8): Temporary files for DB tests

---

## 🚀 Common Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Optimized build
cargo check                    # Fast syntax check

# Run
RUST_LOG=info cargo run --release -- fetch   # Fetch historical data
RUST_LOG=info cargo run --release -- scrape  # Scrape real-time data
RUST_LOG=info cargo run --release -- serve   # Start web server (port 8080)
RUST_LOG=info cargo run --release -- stats   # View database statistics

# Test & Quality
cargo test                     # Run all tests
cargo test -- --nocapture      # Run tests with output
cargo test test_name           # Run specific test
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Lint with warnings as errors

# Database
sqlite3 ~/.local/share/re_tracker/housing_data.db "SELECT * FROM housing_data"
```

---

## 🎨 Frontend Guidelines

### Chart.js Configuration
- **Location**: `frontend/chart.js`
- **Chart Type**: Dual-axis line chart
- **Y-Axes**: Left (listings, blue), Right (price/sqft, green)
- **Features**: Timeframe selection, SMA overlays, responsive tooltips

### CSS Styling
- **Location**: `frontend/styles.css`
- **Theme**: Dark mode with CSS variables
- **Colors**: Defined in `:root` (--bg-primary, --accent-primary, etc.)
- **Responsive**: Mobile breakpoint at 768px

### API Endpoint
- **Endpoint**: `/api/data`
- **Format**: JSON array of objects
- **Schema**: `{date, active_listings, avg_price_per_sqft, data_source}`

---

## ⚠️ Current Limitations & Notes

### Synthetic Data
- **data_fetcher.rs**: Currently generates 6 months of synthetic historical data
- **scraper.rs**: Currently generates synthetic scraped data
- **Reason**: Real Zillow CSV parsing and web scraping require complex handling
- **TODO**: Implement real data sources in future enhancement

### No System Tray Yet
- Planned for Phase 1 enhancements
- Would use `tray-icon` crate
- Not implemented due to platform-specific complexity

### No Automated Scheduling
- Manual invocation only (via CLI)
- Future: Add cron-like scheduler in `src/utils/scheduler.rs`

---

## 📊 Database Schema

```sql
CREATE TABLE housing_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    active_listings INTEGER NOT NULL,
    avg_price_per_sqft REAL,
    data_source TEXT NOT NULL,
    last_updated TEXT NOT NULL
);

CREATE INDEX idx_housing_data_date ON housing_data(date);
CREATE INDEX idx_housing_data_updated ON housing_data(last_updated);
```

**Location**: 
- Linux: `~/.local/share/re_tracker/housing_data.db`
- Windows: `%USERPROFILE%\.local\share\re_tracker\housing_data.db`
- macOS: `~/.local/share/re_tracker/housing_data.db`

---

## 🔄 Git Workflow

```bash
git status                     # Check changes
git add -A                     # Stage all changes
git commit -m "message"        # Commit with message
git push                       # Push to GitHub

# Always include co-author in commits when AI makes changes:
# Co-Authored-By: Warp <agent@warp.dev>
```

**Repository**: https://github.com/urmt/RE_TRACKER_by_ZIP

---

## 🎯 Next Development Tasks

### Priority: Phase 1 Enhancements (Real Data)

1. **Real Zillow CSV Parser**
   - Download from https://www.zillow.com/research/data/
   - Parse date columns and extract ZIP 90720 data
   - Implement in `src/core/data_fetcher.rs`

2. **Real Web Scraping**
   - Add `headless_chrome` or `chromiumoxide` crate
   - Implement JavaScript-aware scraping
   - Handle anti-scraping measures
   - Update `src/utils/scraper.rs`

3. **System Tray Icon**
   - Add `tray-icon` crate
   - Create platform-specific menu
   - Launch browser on click

### Alternative: Phase 2 (IPFS/P2P)

1. Add `libp2p` to Cargo.toml
2. Create `src/core/p2p_sync.rs`
3. Implement pub/sub on `/rossmoor-housing/data-updates` topic
4. Test with multiple local nodes

---

## 📚 Additional Resources

- **Zillow Research**: https://www.zillow.com/research/data/
- **Chart.js Docs**: https://www.chartjs.org/docs/latest/
- **Rust Book**: https://doc.rust-lang.org/book/
- **rusqlite Guide**: https://docs.rs/rusqlite/latest/rusqlite/

---

## 🤝 Contributing Guidelines

1. **Read WARP.md first** - Understand architecture before coding
2. **Follow coding standards** - Listed in this document
3. **Write tests** - Unit tests for all new functionality
4. **Run quality checks** - `cargo fmt`, `cargo clippy`, `cargo test`
5. **Update documentation** - Keep WARP.md and STATUS.md in sync
6. **Clear commit messages** - Describe what and why
7. **Test locally** - Run `cargo run --release -- serve` to verify

---

## ❓ Questions or Clarifications?

If you need clarification on any aspect of this project:

1. Check **WARP.md** for architecture details
2. Check **STATUS.md** for current phase and features
3. Check **QUICKSTART.md** for quick command reference
4. Review inline comments in source files
5. Check git history for context on recent changes

---

**Last Updated**: December 26, 2025  
**Project Phase**: Phase 1 Complete ✅  
**Primary Contact**: urmt (GitHub)  
**License**: MIT or Apache 2.0 (dual-licensed)
