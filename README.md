# Mordor Testnet Monitoring System

A comprehensive, production-ready monitoring solution for Ethereum Classic's Mordor testnet, built with Rust, Prometheus, and Grafana.

![Architecture](https://img.shields.io/badge/Architecture-Microservices-blue)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Monitoring](https://img.shields.io/badge/Monitoring-Prometheus%2FGrafana-red)

## Features

### 🔍 Fork Detection
- Real-time blockchain fork monitoring
- Automatic reorganization detection
- Fork depth analysis
- Competing block tracking
- Historical fork analytics

### ⛽ Gas Price Analysis
- Multi-percentile gas price tracking (min, p25, median, p75, max)
- Network utilization metrics
- Transaction throughput analysis
- Gas price recommendations (slow/standard/fast/instant)
- 20-block rolling window analysis

### 📊 Comprehensive Metrics
- Block height and timestamps
- Block production time
- Mining difficulty tracking
- Gas usage and limits
- Transaction counts
- Network health indicators

### 🛠️ CLI Tools
- Real-time blockchain monitoring
- Detailed block inspection
- Health checks for all services
- Metrics querying
- Gas price recommendations

## Architecture

The Mordor toolkit includes a comprehensive Docker infrastructure for monitoring and interacting with the Mordor testnet:

### Core Services
- **Mordor Node** (Core-Geth) - Ethereum Classic Mordor testnet node
- **Fork Monitor** - Real-time blockchain fork detection
- **Gas Estimator** - Gas price analysis and recommendations

### Monitoring Stack
- **Prometheus** - Metrics collection and storage
- **Grafana** - Metrics visualization and dashboards

### Additional Infrastructure
- **IPFS (Kubo)** - Decentralized storage
- **Safe Transaction Service** - Multisig transaction management with PostgreSQL, Redis, and RabbitMQ

```
┌─────────────────┐
│  Core-Geth      │
│  (Mordor Node)  │
│  :8545 RPC      │
└────────┬────────┘
         │
         ├──────────┬──────────┬─────────────┐
         │          │          │             │
    ┌────▼─────┐  ┌▼──────────┐  ┌─────▼────┐  ┌──▼──────┐
    │  Fork    │  │   Gas     │  │  IPFS    │  │  Safe   │
    │ Monitor  │  │ Estimator │  │  (Kubo)  │  │  Tx Svc │
    │ (Rust)   │  │  (Rust)   │  │  :5001   │  │  :8000  │
    └────┬─────┘  └┬──────────┘  └──────────┘  └─────────┘
         │         │
         │ :9090   │ :9091
         │/metrics │/metrics
         │         │
    ┌────▼─────────▼──┐
    │   Prometheus    │
    │     :9092       │
    └────────┬────────┘
             │
        ┌────▼─────┐
        │ Grafana  │
        │  :3000   │
        └──────────┘
```
## Quick Start

### Prerequisites
- Docker 20.10+
- Docker Compose 1.29+
- Rust 1.75+ (for building CLI tools)
- 20GB+ free disk space (includes Safe Transaction Service)
- 8GB+ RAM recommended

### Installation

#### Using Docker Compose (Recommended)

The complete infrastructure including IPFS and Safe Transaction Service is available in the `docker/` directory:

```bash
# Clone the repository
git clone https://github.com/chippr-robotics/mordor-tool-kit
cd mordor-tool-kit/docker

# Copy environment template and configure
cp .env.example .env
# Edit .env with your settings

# Start all services
./start.sh
# Or use: make docker-up (from root directory)
```

**What you get:**
- Mordor testnet node (Core-Geth)
- Fork monitor and gas estimator
- Prometheus & Grafana monitoring
- IPFS node (Kubo) for decentralized storage
- Safe Transaction Service with full stack (PostgreSQL, Redis, RabbitMQ)

**Access the services:**
- Grafana: http://localhost:3000 (admin/admin)
- Prometheus: http://localhost:9092
- Safe Transaction Service: http://localhost:8000
- IPFS API: http://localhost:5001
- IPFS Gateway: http://localhost:8080
- RabbitMQ Management: http://localhost:15672 (safe/safe)
- Fork Monitor Metrics: http://localhost:9090/metrics
- Gas Estimator Metrics: http://localhost:9091/metrics
- Mordor RPC: http://localhost:8545

For detailed Docker setup instructions, see [docker/README.md](docker/README.md).

### Manual Installation (Development)

```bash
# Build images
make build

# Start services
make up

 # Check status
make health
make status
```

## Service Ports

### Core Services
| Service | Port | Description |
|---------|------|-------------|
| Mordor Node RPC | 8545 | JSON-RPC endpoint |
| Mordor Node WS | 8546 | WebSocket endpoint |
| Fork Monitor | 9090 | Metrics endpoint |
| Gas Estimator | 9091 | Metrics endpoint |

### Monitoring
| Service | Port | Description |
|---------|------|-------------|
| Prometheus | 9092 | Prometheus UI |
| Grafana | 3000 | Grafana dashboard |

### Additional Services (Docker)
| Service | Port | Description |
|---------|------|-------------|
| IPFS API | 5001 | IPFS API endpoint |
| IPFS Gateway | 8080 | IPFS HTTP gateway |
| Safe Transaction Service | 8000 | Safe API endpoint |
| PostgreSQL | 5432 | Database |
| Redis | 6379 | Cache |
| RabbitMQ | 5672 | Message broker (AMQP) |
| RabbitMQ Management | 15672 | RabbitMQ web UI |

 ## CLI Usage
### Building the CLI

```bash
make cli-build
```

 Or install system-wide:
 ```bash
make install-cli
  ```

 ### Commands

**Get blockchain status:**
```bash
make status
 # or
mordor-cli status
```

**Monitor in real-time:**
```bash
make monitor
# or
mordor-cli monitor --interval 5
```

**Get block details:**
```bash
mordor-cli block latest
mordor-cli block 1234567
```

**Check service health:**
```bash
make health
# or
mordor-cli health
```

**Get gas price recommendations:**
```bash
make gas
# or
mordor-cli gas
```
**View metrics:**
```bash
make metrics-fork
make metrics-gas
# or
mordor-cli metrics --service fork-monitor
mordor-cli metrics --service gas-estimator
```

### Example Output

```bash
$ make status
Mordor Testnet Status
==================================================
┌─────────────────────┬──────────────────────────────────┐
│ Metric                  │       Value                            │
├─────────────────────┼──────────────────────────────────┤
│ Chain ID                │ 63                                 │
│ Current Block       │ 12345678                         │
│ Syncing             │ No                               │
│ Gas Price           │ 1000000000 wei (1.00 Gwei)      │
│ Latest Block Time   │ 2025-10-23 14:30:45 UTC         │
│ Transactions        │ 5                                │
│ Gas Used            │ 150000 / 8000000 (1.88%)        │
└─────────────────────┴──────────────────────────────────┘
 ```

```bash
 $ make monitor
Monitoring Mordor Testnet (Ctrl+C to stop)
======================================================================
14:30:45 Block 12345678 (+13.2s) | Txs: 5 | Gas: 150000/8000000 (1.9%) | Difficulty: 131072
14:31:01 Block 12345679 (+16.0s) | Txs: 3 | Gas: 105000/8000000 (1.3%) | Difficulty: 131072
14:31:14 Block 12345680 (+13.0s) | Txs: 8 | Gas: 240000/8000000 (3.0%) | Difficulty: 131072
                                                                                                                                                ```

## Metrics

### Fork Monitor Metrics

| Metric | Type | Description |
|--------|------|-------------|
  | `etc_mordor_block_height` | Gauge | Current block height |
 | `etc_mordor_block_timestamp` | Gauge | Block timestamp (Unix) |
| `etc_mordor_block_gas_used` | Gauge | Gas used in current block |
 | `etc_mordor_block_gas_limit` | Gauge | Block gas limit |
| `etc_mordor_block_time_seconds` | Histogram | Time between blocks |
 | `etc_mordor_block_difficulty` | Histogram | Block difficulty |
 | `etc_mordor_transaction_count` | Gauge | Transactions in current block |
 | `etc_mordor_fork_total` | Counter | Total forks detected |
 | `etc_mordor_fork_depth` | Histogram | Fork reorganization depth |
| `etc_mordor_active_forks` | Gauge | Currently active forks |
  | `etc_mordor_missed_blocks_total` | Counter | Total missed blocks |

### Gas Estimator Metrics

| Metric | Type | Description |
 |--------|------|-------------|
| `etc_mordor_gas_price_min_wei` | Gauge | Minimum gas price |
 | `etc_mordor_gas_price_max_wei` | Gauge | Maximum gas price |
  | `etc_mordor_gas_price_median_wei` | Gauge | Median gas price |
 | `etc_mordor_gas_price_p25_wei` | Gauge | 25th percentile gas price |
 | `etc_mordor_gas_price_p75_wei` | Gauge | 75th percentile gas price |
 | `etc_mordor_gas_price_mean_wei` | Gauge | Mean gas price |
 | `etc_mordor_gas_utilization_percent` | Gauge | Gas utilization percentage |
| `etc_mordor_avg_tx_per_block` | Gauge | Average transactions per block |

## Makefile Commands

### Basic Operations
```bash
make build          # Build all Docker images
make up             # Start all services
make down           # Stop all services
make restart        # Restart all services
make clean          # Remove all containers and volumes
 ```

### Monitoring
```bash
make status         # Check blockchain status
make health         # Health check all containers
make monitor        # Monitor blockchain in real-time
make gas            # Get gas price recommendations
```
### Logs
```bash
make logs           # View all logs
make logs-fork      # View fork monitor logs
make logs-gas       # View gas estimator logs
make logs-node      # View Mordor node logs
```

### Metrics
```bash
make metrics-fork   # View fork monitor metrics
make metrics-gas    # View gas estimator metrics
make urls           # Show all service URLs
```

### Development
```bash
make dev-fork       # Rebuild and restart fork monitor
make dev-gas        # Rebuild and restart gas estimator
make cli-build      # Build CLI tool
make install-cli    # Install CLI system-wide
```

### Backup & Restore
```bash
make backup         # Backup blockchain data
make restore FILE=backups/mordor-data-TIMESTAMP.tar.gz  # Restore from backup
```

## Testing

Run the comprehensive test suite:

```bash
 make test
```

This will verify:
- ✓ Docker is running
- ✓ All containers are healthy
- ✓ RPC endpoints are responding
- ✓ Metrics are being collected
- ✓ Prometheus is scraping
- ✓ Grafana is accessible
- ✓ Dashboards are provisioned
- ✓ Metrics are updating

## Configuration

### Environment Variables

**Fork Monitor:**
```bash
RPC_URL=http://mordor-node:8545    # RPC endpoint
POLL_INTERVAL_SECS=5                # Polling interval
RUST_LOG=info                       # Log level
```

**Gas Estimator:**
```bash
RPC_URL=http://mordor-node:8545    # RPC endpoint
POLL_INTERVAL_SECS=12               # Polling interval
RUST_LOG=info                       # Log level
```

### Prometheus Configuration

Edit `prometheus/prometheus.yml` to adjust:
- Scrape intervals
- Retention periods
- Alert rules
- Additional targets

### Grafana Dashboard

The dashboard is automatically provisioned from:
```
docker/grafana/provisioning/dashboards/mordor-dashboard.json
```

To update the dashboard:
1. Make changes in Grafana UI
2. Export JSON
3. Replace `mordor-dashboard.json`
4. Restart Grafana: `cd docker && docker-compose restart grafana`

## Project Structure

```
mordor-tool-kit/
├── docker/                      # Docker Compose infrastructure
│   ├── docker-compose.yml      # All services configuration
│   ├── README.md               # Docker setup documentation
│   ├── .env.example            # Environment variables template
│   ├── start.sh                # Quick start script
│   ├── stop.sh                 # Stop script
│   ├── prometheus/             # Prometheus configuration
│   │   └── prometheus.yml
│   └── grafana/                # Grafana provisioning
│       └── provisioning/
│           ├── datasources/
│           └── dashboards/
├── fork-monitor/               # Fork detection service
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile
├── gas-estimator/              # Gas price analysis service
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile
├── cli/                        # Command-line interface
│   ├── src/
│   └── Cargo.toml
├── Makefile                    # Build and management commands
└── README.md                   # This file
```

                                                                                                                                                                            ### Building Components

                                                                                                                                                                            **Fork Monitor:**
                                                                                                                                                                            ```bash
                                                                                                                                                                            cd fork-monitor
                                                                                                                                                                            cargo build --release
                                                                                                                                                                            ```

                                                                                                                                                                            **Gas Estimator:**
                                                                                                                                                                            ```bash
                                                                                                                                                                            cd gas-estimator
                                                                                                                                                                            cargo build --release
                                                                                                                                                                            ```

                                                                                                                                                                            **CLI Tool:**
                                                                                                                                                                            ```bash
                                                                                                                                                                            cd cli
                                                                                                                                                                            cargo build --release
                                                                                                                                                                            ```

                                                                                                                                                                            ### Running Tests

                                                                                                                                                                            **Unit tests:**
                                                                                                                                                                            ```bash
                                                                                                                                                                            cd fork-monitor && cargo test
                                                                                                                                                                            cd gas-estimator && cargo test
                                                                                                                                                                            cd cli && cargo test
                                                                                                                                                                            ```

                                                                                                                                                                            **Integration tests:**
                                                                                                                                                                            ```bash
                                                                                                                                                                            make test
                                                                                                                                                                            ```

                                                                                                                                                                            ## Contributing

                                                                                                                                                                            1. Fork the repository
                                                                                                                                                                            2. Create a feature branch
                                                                                                                                                                            3. Make your changes
                                                                                                                                                                            4. Add tests
                                                                                                                                                                            5. Submit a pull request

                                                                                                                                                                            ## License

                                                                                                                                                                            MIT License - see LICENSE file for details

                                                                                                                                                                            ## Support

                                                                                                                                                                            For issues and questions:
                                                                                                                                                                            - GitHub Issues: <repository-url>/issues
                                                                                                                                                                            - Documentation: <docs-url>

                                                                                                                                                                            ## Acknowledgments

                                                                                                                                                                            - Ethereum Classic core-geth team
                                                                                                                                                                            - Prometheus & Grafana communities
                                                                                                                                                                            - Rust ethers-rs library maintainers

                                                                                                                                                                            ---

                                                                                                                                                                            **Built with ❤️ for the Ethereum Classic ecosystem**
                                                                                                                                                                            
