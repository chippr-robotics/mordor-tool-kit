# Mordor Infrastructure Services Overview

This document provides detailed information about each service in the Mordor Docker Compose stack.

## Service Details

### 1. Mordor Node (Core-Geth)

**Purpose**: Ethereum Classic Mordor testnet node  
**Image**: `etclabscore/core-geth:latest`  
**Ports**:
- 8545: HTTP JSON-RPC
- 8546: WebSocket
- 30303: P2P networking

**Configuration**:
- Network: Mordor testnet (Chain ID: 63)
- Sync mode: Fast
- CORS enabled for development
- Full API access: eth, net, web3, personal, admin

**Data Persistence**: `mordor-data` volume

**Use Cases**:
- Blockchain state queries
- Transaction submission
- Smart contract interaction
- Block and transaction data retrieval

---

### 2. Fork Monitor

**Purpose**: Real-time blockchain fork detection and monitoring  
**Build**: Custom Rust application from `../fork-monitor`  
**Port**: 9090 (Prometheus metrics)

**Features**:
- Detects blockchain reorganizations
- Tracks fork depth
- Monitors competing blocks
- Exports metrics to Prometheus

**Environment Variables**:
- `RPC_URL`: Mordor node endpoint
- `POLL_INTERVAL_SECS`: 5 seconds
- `RUST_LOG`: info

**Metrics Exported**:
- Block height
- Fork count
- Fork depth
- Active forks

---

### 3. Gas Estimator

**Purpose**: Gas price analysis and recommendations  
**Build**: Custom Rust application from `../gas-estimator`  
**Port**: 9091 (Prometheus metrics)

**Features**:
- Multi-percentile gas price tracking
- Network utilization analysis
- Transaction throughput metrics
- Gas price recommendations

**Environment Variables**:
- `RPC_URL`: Mordor node endpoint
- `POLL_INTERVAL_SECS`: 12 seconds
- `RUST_LOG`: info

**Metrics Exported**:
- Gas prices (min, p25, median, p75, max)
- Gas utilization
- Average transactions per block

---

### 4. Prometheus

**Purpose**: Metrics collection and time-series storage  
**Image**: `prom/prometheus:latest`  
**Port**: 9092 (web UI on 9090 internally)

**Features**:
- Scrapes metrics from fork-monitor and gas-estimator
- 30-day data retention
- Stores time-series data
- Provides query API for Grafana

**Configuration**: `prometheus/prometheus.yml`

**Targets**:
- Fork Monitor (9090)
- Gas Estimator (9091)
- IPFS (5001)
- Self-monitoring

---

### 5. Grafana

**Purpose**: Metrics visualization and dashboards  
**Image**: `grafana/grafana:latest`  
**Port**: 3000

**Features**:
- Pre-configured Prometheus datasource
- Auto-provisioned dashboards
- Real-time metrics visualization
- Custom dashboard support

**Default Credentials**:
- Username: admin
- Password: admin

**Dashboards**:
- Mordor Testnet Monitoring (auto-provisioned)

---

### 6. IPFS (Kubo)

**Purpose**: Decentralized file storage and distribution  
**Image**: `ipfs/kubo:latest`  
**Ports**:
- 5001: API
- 8080: Gateway
- 4001: Swarm (P2P)

**Features**:
- Content-addressed storage
- Peer-to-peer file sharing
- Decentralized storage network
- HTTP gateway for content access

**Profile**: Server (optimized for server environments)

**Data Persistence**: `ipfs-data` volume

**Use Cases**:
- Store transaction metadata
- Decentralized file hosting
- Content distribution
- Off-chain data storage

---

### 7. PostgreSQL

**Purpose**: Database for Safe Transaction Service  
**Image**: `postgres:14-alpine`  
**Port**: 5432

**Configuration**:
- Database: safe
- User: safe
- Password: safe (change in production)

**Data Persistence**: `postgres-data` volume

**Health Check**: `pg_isready` with 10s interval

---

### 8. Redis

**Purpose**: Caching and session storage for Safe Transaction Service  
**Image**: `redis:7-alpine`  
**Port**: 6379

**Features**:
- In-memory data store
- Fast caching layer
- Session management
- Queue support

**Data Persistence**: `redis-data` volume

**Health Check**: `redis-cli ping` with 10s interval

---

### 9. RabbitMQ

**Purpose**: Message broker for Safe Transaction Service  
**Image**: `rabbitmq:3-management-alpine`  
**Ports**:
- 5672: AMQP protocol
- 15672: Management web UI

**Features**:
- Message queuing
- Task distribution
- Reliable message delivery
- Management web interface

**Default Credentials**:
- Username: safe
- Password: safe (change in production)

**Data Persistence**: `rabbitmq-data` volume

**Health Check**: `rabbitmq-diagnostics ping` with 10s interval

---

### 10. Safe Transaction Service

**Purpose**: Multisig transaction management API  
**Image**: `safeglobal/safe-transaction-service:latest`  
**Port**: 8000

**Features**:
- Safe multisig wallet support
- Transaction proposal and execution
- Transaction history
- Off-chain signatures
- Ethereum Classic Mordor support

**Configuration**:
- Chain ID: 63 (Mordor testnet)
- Database: PostgreSQL
- Cache: Redis
- Message Queue: RabbitMQ (via Celery)

**Dependencies**:
- Requires healthy PostgreSQL
- Requires healthy Redis
- Requires healthy RabbitMQ
- Requires running Mordor node

**API Documentation**: Available at http://localhost:8000/api/

---

### 11. Safe Worker (Celery)

**Purpose**: Background task processing for Safe Transaction Service  
**Image**: `safeglobal/safe-transaction-service:latest`

**Features**:
- Processes transaction indexing
- Updates contract data
- Handles blockchain events
- Background job processing

**Configuration**: Same as Safe Transaction Service

---

### 12. Safe Scheduler (Celery Beat)

**Purpose**: Periodic task scheduling for Safe Transaction Service  
**Image**: `safeglobal/safe-transaction-service:latest`

**Features**:
- Schedules periodic tasks
- Triggers blockchain updates
- Maintains data freshness
- Automated maintenance tasks

**Configuration**: Same as Safe Transaction Service

---

## Service Dependencies

```
Mordor Node
    ├── Fork Monitor
    ├── Gas Estimator
    ├── Safe Transaction Service
    │       ├── PostgreSQL
    │       ├── Redis
    │       └── RabbitMQ
    └── (no dependencies)

Fork Monitor & Gas Estimator
    └── Prometheus
            └── Grafana

IPFS
    └── (independent service)
```

## Resource Requirements

### Minimum Requirements
- **CPU**: 4 cores
- **RAM**: 8 GB
- **Disk**: 20 GB free space
- **Network**: Stable internet connection

### Recommended Requirements
- **CPU**: 8+ cores
- **RAM**: 16 GB
- **Disk**: 50+ GB SSD
- **Network**: High-bandwidth connection for faster sync

## Storage Volumes

| Volume Name | Purpose | Typical Size |
|-------------|---------|--------------|
| `mordor-data` | Blockchain data | 10-15 GB |
| `prometheus-data` | Metrics storage | 1-2 GB |
| `grafana-data` | Dashboards and config | < 100 MB |
| `ipfs-data` | IPFS repository | Varies |
| `postgres-data` | Database | 1-5 GB |
| `redis-data` | Cache data | < 500 MB |
| `rabbitmq-data` | Message queue | < 500 MB |

## Security Considerations

### Production Deployment Checklist

- [ ] Change all default passwords
- [ ] Generate strong `DJANGO_SECRET_KEY` for Safe service
- [ ] Configure proper firewall rules
- [ ] Use HTTPS with valid certificates
- [ ] Restrict `DJANGO_ALLOWED_HOSTS`
- [ ] Set up proper CORS policies
- [ ] Enable authentication for Grafana
- [ ] Secure RabbitMQ management interface
- [ ] Regular security updates
- [ ] Monitor for vulnerabilities
- [ ] Implement backup strategy
- [ ] Set up monitoring alerts

### Network Security

Services are isolated on the `mordor-network` bridge network. Only necessary ports are exposed to the host. Internal service communication uses container names as hostnames.

## Monitoring and Observability

### Available Metrics

**Fork Monitor**:
- `etc_mordor_block_height`
- `etc_mordor_fork_total`
- `etc_mordor_fork_depth`
- `etc_mordor_active_forks`
- `etc_mordor_block_time_seconds`

**Gas Estimator**:
- `etc_mordor_gas_price_*_wei`
- `etc_mordor_gas_utilization_percent`
- `etc_mordor_avg_tx_per_block`

**IPFS**:
- Available at `/debug/metrics/prometheus` if metrics enabled

### Health Monitoring

Use `docker-compose ps` to check service health status. Services with health checks:
- PostgreSQL
- Redis
- RabbitMQ

## Troubleshooting

### Common Issues

1. **Services won't start**
   - Check Docker daemon is running
   - Verify port availability
   - Check disk space
   - Review service logs

2. **Mordor node not syncing**
   - Check network connectivity
   - Verify P2P port (30303) is accessible
   - Ensure sufficient disk space

3. **Safe service database errors**
   - Verify PostgreSQL is healthy
   - Check database migrations
   - Review connection settings

4. **IPFS connection issues**
   - Check swarm port (4001)
   - Verify peer connectivity
   - Review IPFS logs

### Log Access

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f [service-name]

# Example
docker-compose logs -f safe-transaction-service
```

## Backup and Recovery

### Backup Important Data

```bash
# Backup all volumes
docker run --rm \
  --volumes-from mordor-node \
  --volumes-from safe-postgres \
  -v $(pwd)/backups:/backup \
  ubuntu tar czf /backup/mordor-backup-$(date +%Y%m%d).tar.gz \
  /root/.ethereum /var/lib/postgresql/data

# Backup configuration
tar czf config-backup-$(date +%Y%m%d).tar.gz docker/
```

### Restore

Restore by extracting backup archives to appropriate volume locations and restarting services.

## Performance Tuning

### Mordor Node
- Adjust cache sizes in command parameters
- Use SSD for blockchain data
- Increase connection limits for high-traffic scenarios

### PostgreSQL
- Tune `shared_buffers` and `work_mem`
- Configure appropriate connection pooling
- Regular VACUUM operations

### Prometheus
- Adjust retention time based on needs
- Configure scrape intervals appropriately
- Consider remote storage for long-term retention

## Support and Documentation

- Main Repository: https://github.com/chippr-robotics/mordor-tool-kit
- Docker README: [README.md](README.md)
- IPFS Documentation: https://docs.ipfs.tech/
- Safe Documentation: https://docs.safe.global/
- Prometheus: https://prometheus.io/docs/
- Grafana: https://grafana.com/docs/
