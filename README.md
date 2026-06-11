# SimpleHTTP

A minimal **HTTP/1.1 web server** written in Rust from scratch (no heavy frameworks).

This project demonstrates low-level networking, multi-threading, and basic HTTP protocol handling.

## Features

- **HTTP/1.1** compliant response handling
- **ThreadPool** for concurrent connection handling
- Serves static HTML files
- Simple routing (basic GET support)
- Configurable address and port
- Non-blocking listener with graceful handling

## Quick Start

### Prerequisites
- Rust (stable)

### Build & Run

```bash
git clone https://github.com/ironwill01/SimpleHTTP.git
cd SimpleHTTP
cargo run
