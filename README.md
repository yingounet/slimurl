# URLSlim

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.93+-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue.svg)](https://www.docker.com/)

A high-performance, self-hosted URL shortening service with QR code generation and analytics.

[中文文档](README_CN.md)

## Features

- **URL Shortening** - Create temporary (1h/24h/7d/30d) or permanent short links
- **Fast Redirects** - 302 redirects with memory cache, 10,000+ QPS
- **QR Codes** - Generate PNG/SVG QR codes on-the-fly
- **Analytics** - Track PV/UV, devices, browsers, OS, and referrers
- **Rate Limiting** - IP-based request throttling
- **Auto Cleanup** - Expired links are automatically removed
- **Single Container** - Easy deployment with Docker
- **SQLite Storage** - Simple backup and maintenance

## Quick Start

### Docker

```bash
docker run -d \
  --name urlslim \
  --restart unless-stopped \
  -p 3000:3000 \
  -v ./data:/app/data \
  -e BASE_URL=https://s.example.com \
  ghcr.io/yourorg/urlslim:latest
```

### Docker Compose

```bash
curl -O https://raw.githubusercontent.com/yourorg/urlslim/main/docker-compose.yml
docker compose up -d
```

### Test

```bash
curl http://localhost:3000/

# Create a short link
curl -X POST http://localhost:3000/api/v1/links \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/very/long/url"}'
```

## API Reference

### Create Short Link

```http
POST /api/v1/links
Content-Type: application/json

{
  "url": "https://example.com/long/url",
  "link_type": "temporary",
  "expires_in": "24h"
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| url | string | Yes | Target URL (max 2048 chars, http/https only) |
| link_type | string | No | `temporary` (default) or `permanent` |
| expires_in | string | No | `1h`, `24h`, `7d`, `30d` (default: 24h) |

**Response:**

```json
{
  "code": "aB3xYz",
  "short_url": "https://s.example.com/aB3xYz",
  "qr_url": "https://s.example.com/aB3xYz/qrcode",
  "expires_at": "2024-01-16T10:00:00Z"
}
```

### Redirect

```http
GET /{code}

Response:
HTTP/1.1 302 Found
Location: https://example.com/long/url
```

### Get QR Code

```http
GET /{code}/qrcode?size=300&format=png
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| size | int | 200 | Image size in pixels (100-1000) |
| format | string | png | `png` or `svg` |

**Response:** `image/png` or `image/svg+xml`

### Get Statistics

```http
GET /api/v1/links/{code}/stats?period=7d
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| period | string | 7d | `1d`, `7d`, `30d`, or `all` |

**Response:**

```json
{
  "pv": 12345,
  "uv": 8900,
  "devices": {
    "mobile": 6000,
    "desktop": 4000,
    "tablet": 500
  },
  "browsers": {
    "Chrome": 7000,
    "Safari": 3000,
    "Firefox": 1500
  },
  "referers": {
    "google.com": 3000,
    "twitter.com": 2000,
    "direct": 5000
  }
}
```

### Health Check

```http
GET /

Response: OK
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| HOST | 0.0.0.0 | Listen address |
| PORT | 3000 | Listen port |
| DATABASE_URL | data/links.db | SQLite database path |
| BASE_URL | http://localhost:3000 | Base URL for short links |
| CACHE_CAPACITY | 10000 | Max cached links |
| RATE_LIMIT_REQUESTS | 100 | Max requests per window |
| RATE_LIMIT_WINDOW_SECS | 60 | Rate limit window in seconds |
| RUST_LOG | info | Log level (trace/debug/info/warn/error) |

## Development

### Prerequisites

- Rust 1.75+
- SQLite3

### Run Locally

```bash
# Clone repository
git clone https://github.com/yourorg/urlslim.git
cd urlslim

# Run
cargo run

# Run tests
cargo test
```

### Build Release

```bash
cargo build --release
```

### Docker Build

```bash
docker build -t urlslim .
```

## Performance

| Metric | Target | Notes |
|--------|--------|-------|
| QPS (redirect) | > 10,000 | Cache hit |
| QPS (create) | > 500 | With DB write |
| P99 Latency | < 20ms | Redirect |
| Memory | < 200MB | Idle + runtime |
| Startup | < 3s | Cold start |

### Benchmark

```bash
# Quick benchmark
make quick-bench

# Full benchmark
make benchmark

# Stress test
make stress
```

See [scripts/README.md](scripts/README.md) for details.

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) 0.7
- **Runtime**: [Tokio](https://tokio.rs/) 1.x
- **Database**: [SQLx](https://github.com/launchbadge/sqlx) with SQLite (WAL mode)
- **Cache**: [Moka](https://github.com/moka-rs/moka) - High-performance in-memory cache
- **QR Code**: [qrcode](https://github.com/qrcode-rust/qrcode) + [image](https://github.com/image-rs/image)
- **Serialization**: [Serde](https://serde.rs/) JSON

## Project Structure

```
urlslim/
├── src/
│   ├── main.rs           # Entry point
│   ├── config.rs         # Configuration
│   ├── error.rs          # Error types
│   ├── db/               # Database layer
│   ├── handlers/         # HTTP handlers
│   ├── middleware/       # Rate limit, stats, cleanup
│   ├── models/           # Data models
│   ├── services/         # Business logic
│   └── utils/            # Utilities (ID gen, UA parser)
├── tests/                # Integration tests
├── scripts/              # Benchmark scripts
├── Dockerfile
├── docker-compose.yml
└── Makefile
```

## Backup & Restore

```bash
# Backup
cp ./data/links.db ./backup/links_$(date +%Y%m%d).db

# Restore
cp ./backup/links_20240115.db ./data/links.db
docker restart urlslim
```

## License

[MIT](LICENSE)
