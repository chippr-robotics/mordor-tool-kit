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

```
┌─────────────────┐
│  Core-Geth      │
│  (Mordor Node)  │
│  :8545 RPC      │
└────────┬────────┘
         │
                  ├──────────┐
                           │          │
                               ┌────▼─────┐  ┌▼──────────┐
                                   │  Fork    │  │   Gas     │
                                       │ Monitor  │  │ Estimator │
                                           │ (Rust)   │  │  (Rust)   │
                                               └────┬─────┘  └┬──────────┘
                                                        │         │
                                                                 │ :9090   │ :9091
                                                                          │/metrics │/metrics
                                                                                   │         │
                                                                                       ┌────▼─────────▼──┐
                                                                                           │   Prometheus    │
                                                                                               │     :9090       │
                                                                                                   └────────┬────────┘
                                                                                                                │
                                                                                                                        ┌────▼─────┐
                                                                                                                                │ Grafana  │
                                                                                                                                        │  :3000   │
                                                                                                                                                └──────────┘
                                                                                                                                                ```

                                                                                                                                                ## Quick Start

                                                                                                                                                ### Prerequisites

                                                                                                                                                - Docker & Docker Compose
                                                                                                                                                - Rust 1.75+ (for building CLI tools)
                                                                                                                                                - 10GB+ free disk space
                                                                                                                                                - 4GB+ RAM

                                                                                                                                                ### Installation

                                                                                                                                                1. **Clone and setup:**
                                                                                                                                                ```bash
                                                                                                                                                git clone <repository>
                                                                                                                                                cd mordor-monitoring
                                                                                                                                                make setup
                                                                                                                                                ```

                                                                                                                                                This will:
                                                                                                                                                - Build all Docker images
                                                                                                                                                - Start all services
                                                                                                                                                - Wait for initialization
                                                                                                                                                - Run health checks

                                                                                                                                                2. **Access the dashboard:**
                                                                                                                                                - Grafana: http://localhost:3000 (admin/admin)
                                                                                                                                                - Prometheus: http://localhost:9092
                                                                                                                                                - Fork Monitor Metrics: http://localhost:9090/metrics
                                                                                                                                                - Gas Estimator Metrics: http://localhost:9091/metrics

                                                                                                                                                ### Manual Installation

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

                                                                                                                                                | Service | Port | Description |
                                                                                                                                                |---------|------|-------------|
                                                                                                                                                | Mordor Node RPC | 8545 | JSON-RPC endpoint |
                                                                                                                                                | Mordor Node WS | 8546 | WebSocket endpoint |
                                                                                                                                                | Fork Monitor | 9090 | Metrics endpoint |
                                                                                                                                                | Gas Estimator | 9091 | Metrics endpoint |
                                                                                                                                                | Prometheus | 9092 | Prometheus UI |
                                                                                                                                                | Grafana | 3000 | Grafana dashboard |

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
                                                                                                                                                │ Metric              │ Value                            │
                                                                                                                                                ├─────────────────────┼──────────────────────────────────┤
                                                                                                                                                │ Chain ID            │ 63                               │
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
                                                                                                                                                grafana/provisioning/dashboards/mordor-dashboard.json
                                                                                                                                                ```

                                                                                                                                                To update the dashboard:
                                                                                                                                                1. Make changes in Grafana UI
                                                                                                                                                2. Export JSON
                                                                                                                                                3. Replace `mordor-dashboard.json`
                                                                                                                                                4. Restart Grafana: `docker-compose restart grafana`

                                                                                                                                                Or use:
                                                                                                                                                ```bash
                                                                                                                                                make update-dashboard
                                                                                                                                                ```

                                                                                                                                                ## Troubleshooting

                                                                                                                                                ### Services not starting

                                                                                                                                                ```bash
                                                                                                                                                # Check Docker logs
                                                                                                                                                make logs

                                                                                                                                                # Check individual services
                                                                                                                                                make logs-fork
                                                                                                                                                make logs-gas
                                                                                                                                                make logs-node

                                                                                                                                                # Restart services
                                                                                                                                                make restart
                                                                                                                                                ```

                                                                                                                                                ### Metrics not updating

                                                                                                                                                ```bash
                                                                                                                                                # Check if services are healthy
                                                                                                                                                make health

                                                                                                                                                # Verify RPC connectivity
                                                                                                                                                curl -X POST -H "Content-Type: application/json" \
                                                                                                                                                  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
                                                                                                                                                    http://localhost:8545

                                                                                                                                                    # Check metrics endpoints
                                                                                                                                                    curl http://localhost:9090/metrics
                                                                                                                                                    curl http://localhost:9091/metrics
                                                                                                                                                    ```

                                                                                                                                                    ### Prometheus not scraping

                                                                                                                                                    ```bash
                                                                                                                                                    # Check Prometheus targets
                                                                                                                                                    curl http://localhost:9092/api/v1/targets

                                                                                                                                                    # View Prometheus logs
                                                                                                                                                    docker-compose logs prometheus

                                                                                                                                                    # Verify network connectivity
                                                                                                                                                    docker-compose exec prometheus ping fork-monitor
                                                                                                                                                    docker-compose exec prometheus ping gas-estimator
                                                                                                                                                    ```

                                                                                                                                                    ### Grafana dashboard not loading

                                                                                                                                                    ```bash
                                                                                                                                                    # Check Grafana logs
                                                                                                                                                    docker-compose logs grafana

                                                                                                                                                    # Verify datasource
                                                                                                                                                    curl -u admin:admin http://localhost:3000/api/datasources

                                                                                                                                                    # Re-provision dashboard
                                                                                                                                                    make update-dashboard
                                                                                                                                                    ```

                                                                                                                                                    ## Performance Tuning

                                                                                                                                                    ### For High-Traffic Networks

                                                                                                                                                    Increase polling intervals in `docker-compose.yml`:

                                                                                                                                                    ```yaml
                                                                                                                                                    fork-monitor:
                                                                                                                                                      environment:
                                                                                                                                                          - POLL_INTERVAL_SECS=2  # Default: 5
                                                                                                                                                              
                                                                                                                                                              gas-estimator:
                                                                                                                                                                environment:
                                                                                                                                                                    - POLL_INTERVAL_SECS=6  # Default: 12
                                                                                                                                                                    ```

                                                                                                                                                                    ### For Limited Resources

                                                                                                                                                                    Reduce Prometheus retention:

                                                                                                                                                                    ```yaml
                                                                                                                                                                    prometheus:
                                                                                                                                                                      command:
                                                                                                                                                                          - '--storage.tsdb.retention.time=7d'  # Default: 30d
                                                                                                                                                                          ```

                                                                                                                                                                          Reduce scrape intervals in `prometheus/prometheus.yml`:

                                                                                                                                                                          ```yaml
                                                                                                                                                                          global:
                                                                                                                                                                            scrape_interval: 30s  # Default: 15s
                                                                                                                                                                            ```

                                                                                                                                                                            ## Development

                                                                                                                                                                            ### Project Structure

                                                                                                                                                                            ```
                                                                                                                                                                            mordor-monitoring/
                                                                                                                                                                            ├── fork-monitor/          # Rust service for fork detection
                                                                                                                                                                            │   ├── src/
                                                                                                                                                                            │   │   ├── main.rs
                                                                                                                                                                            │   │   ├── blockchain.rs
                                                                                                                                                                            │   │   ├── fork_detector.rs
                                                                                                                                                                            │   │   └── metrics.rs
                                                                                                                                                                            │   ├── Cargo.toml
                                                                                                                                                                            │   └── Dockerfile
                                                                                                                                                                            ├── gas-estimator/         # Rust service for gas analysis
                                                                                                                                                                            │   ├── src/
                                                                                                                                                                            │   │   ├── main.rs
                                                                                                                                                                            │   │   ├── gas_oracle.rs
                                                                                                                                                                            │   │   └── metrics.rs
                                                                                                                                                                            │   ├── Cargo.toml
                                                                                                                                                                            │   └── Dockerfile
                                                                                                                                                                            ├── cli/                   # CLI tool
                                                                                                                                                                            │   ├── src/
                                                                                                                                                                            │   │   └── main.rs
                                                                                                                                                                            │   ├── Cargo.toml
                                                                                                                                                                            │   └── Dockerfile
                                                                                                                                                                            ├── prometheus/            # Prometheus configuration
                                                                                                                                                                            │   └── prometheus.yml
                                                                                                                                                                            ├── grafana/              # Grafana provisioning
                                                                                                                                                                            │   └── provisioning/
                                                                                                                                                                            │       ├── datasources/
                                                                                                                                                                            │       └── dashboards/
                                                                                                                                                                            ├── scripts/              # Utility scripts
                                                                                                                                                                            │   └── test.sh
                                                                                                                                                                            ├── docker-compose.yml
                                                                                                                                                                            ├── Makefile
                                                                                                                                                                            └── README.md
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
                                                                                                                                                                            