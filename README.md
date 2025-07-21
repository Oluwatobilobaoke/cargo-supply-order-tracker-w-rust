🚢 A Rust-based CLI application for managing cargo supply orders with tracking capabilities.

## Features
- 📦 Add and manage cargo orders with detailed information
- 📊 View all orders with status tracking (Pending, Processing, Fulfilled, Cancelled, Refunded)
- 🗑️ Automatically remove fulfilled orders using HashMap retention
- 🔄 Persistent order management with thread-safe global state
- 🎯 Clean CLI interface with input validation

## Tech Stack
- **Language**: Rust
- **Data Structure**: HashMap for efficient order lookup
- **Concurrency**: Arc<Mutex<>> for thread-safe operations
- **Dependencies**: once_cell for lazy static initialization

## Architecture
- Modular design with separate utils module
- Global static OrderTracker with proper error handling
- Clean separation of concerns between lib.rs, main.rs, and utils.rs

Perfect for learning Rust ownership, error handling, and CLI application development!