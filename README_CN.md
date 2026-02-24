# URLSlim

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue.svg)](https://www.docker.com/)

一个高性能、自托管的短链接服务，支持二维码生成和访问统计。

[English](README.md)

## 功能特性

- **短链接生成** - 支持临时链接 (1h/24h/7d/30d) 或永久链接
- **快速重定向** - 302 跳转，内存缓存，QPS 10,000+
- **二维码生成** - 实时生成 PNG/SVG 格式二维码
- **访问统计** - 追踪 PV/UV、设备类型、浏览器、操作系统、来源
- **请求限流** - 基于 IP 的请求频率限制
- **自动清理** - 过期链接自动清理
- **单容器部署** - Docker 一键部署
- **SQLite 存储** - 简单的备份和维护

## 快速开始

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

### 测试

```bash
curl http://localhost:3000/

# 创建短链接
curl -X POST http://localhost:3000/api/v1/links \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/very/long/url"}'
```

## API 文档

### 创建短链接

```http
POST /api/v1/links
Content-Type: application/json

{
  "url": "https://example.com/long/url",
  "link_type": "temporary",
  "expires_in": "24h"
}
```

**参数说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| url | string | 是 | 目标 URL（最大 2048 字符，仅支持 http/https） |
| link_type | string | 否 | `temporary`（默认）或 `permanent` |
| expires_in | string | 否 | `1h`、`24h`、`7d`、`30d`（默认 24h） |

**响应：**

```json
{
  "code": "aB3xYz",
  "short_url": "https://s.example.com/aB3xYz",
  "qr_url": "https://s.example.com/aB3xYz/qrcode",
  "expires_at": "2024-01-16T10:00:00Z"
}
```

### 重定向

```http
GET /{code}

响应:
HTTP/1.1 302 Found
Location: https://example.com/long/url
```

### 获取二维码

```http
GET /{code}/qrcode?size=300&format=png
```

**参数说明：**

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| size | int | 200 | 图片尺寸（像素），范围 100-1000 |
| format | string | png | `png` 或 `svg` |

**响应：** `image/png` 或 `image/svg+xml`

### 获取统计数据

```http
GET /api/v1/links/{code}/stats?period=7d
```

**参数说明：**

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| period | string | 7d | `1d`、`7d`、`30d` 或 `all` |

**响应：**

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

### 健康检查

```http
GET /

响应: OK
```

## 配置说明

| 环境变量 | 默认值 | 说明 |
|----------|--------|------|
| HOST | 0.0.0.0 | 监听地址 |
| PORT | 3000 | 监听端口 |
| DATABASE_URL | data/links.db | SQLite 数据库路径 |
| BASE_URL | http://localhost:3000 | 短链接基础 URL |
| CACHE_CAPACITY | 10000 | 缓存容量（链接数） |
| RATE_LIMIT_REQUESTS | 100 | 时间窗口内最大请求数 |
| RATE_LIMIT_WINDOW_SECS | 60 | 限流时间窗口（秒） |
| RUST_LOG | info | 日志级别（trace/debug/info/warn/error） |

## 开发指南

### 环境要求

- Rust 1.75+
- SQLite3

### 本地运行

```bash
# 克隆仓库
git clone https://github.com/yourorg/urlslim.git
cd urlslim

# 运行
cargo run

# 测试
cargo test
```

### 构建发布版本

```bash
cargo build --release
```

### Docker 构建

```bash
docker build -t urlslim .
```

## 性能指标

| 指标 | 目标值 | 说明 |
|------|--------|------|
| QPS（重定向） | > 10,000 | 缓存命中时 |
| QPS（创建链接） | > 500 | 含数据库写入 |
| P99 延迟 | < 20ms | 重定向 |
| 内存占用 | < 200MB | 空载 + 运行时 |
| 启动时间 | < 3s | 冷启动 |

### 性能测试

```bash
# 快速测试
make quick-bench

# 完整基准测试
make benchmark

# 压力测试
make stress
```

详见 [scripts/README.md](scripts/README.md)。

## 技术栈

- **Web 框架**: [Axum](https://github.com/tokio-rs/axum) 0.7
- **异步运行时**: [Tokio](https://tokio.rs/) 1.x
- **数据库**: [SQLx](https://github.com/launchbadge/sqlx) + SQLite (WAL 模式)
- **缓存**: [Moka](https://github.com/moka-rs/moka) - 高性能内存缓存
- **二维码**: [qrcode](https://github.com/qrcode-rust/qrcode) + [image](https://github.com/image-rs/image)
- **序列化**: [Serde](https://serde.rs/) JSON

## 项目结构

```
urlslim/
├── src/
│   ├── main.rs           # 入口文件
│   ├── config.rs         # 配置管理
│   ├── error.rs          # 错误类型
│   ├── db/               # 数据库层
│   ├── handlers/         # HTTP 处理器
│   ├── middleware/       # 中间件（限流、统计、清理）
│   ├── models/           # 数据模型
│   ├── services/         # 业务逻辑
│   └── utils/            # 工具函数（ID 生成、UA 解析）
├── tests/                # 集成测试
├── scripts/              # 性能测试脚本
├── Dockerfile
├── docker-compose.yml
└── Makefile
```

## 备份与恢复

```bash
# 备份
cp ./data/links.db ./backup/links_$(date +%Y%m%d).db

# 恢复
cp ./backup/links_20240115.db ./data/links.db
docker restart urlslim
```

## 许可证

[MIT](LICENSE)
