# Quick Reference Guide

## Common Commands

### Starting and Stopping

```bash
# Start all services
./start.sh

# Stop all services
./stop.sh

# Restart all services
docker-compose restart

# Start specific service
docker-compose up -d [service-name]

# Stop specific service
docker-compose stop [service-name]
```

### Viewing Logs

```bash
# All services
docker-compose logs -f

# Specific service (last 100 lines, follow)
docker-compose logs -f --tail=100 [service-name]

# Examples
docker-compose logs -f mordor-node
docker-compose logs -f safe-transaction-service
docker-compose logs -f ipfs
```

### Service Status

```bash
# Check status of all services
docker-compose ps

# Check health
docker inspect --format='{{.State.Health.Status}}' [container-name]
```

### Resource Usage

```bash
# Monitor resource usage
docker stats

# Check disk usage
docker system df

# Check volume sizes
docker system df -v
```

## Service URLs

| Service | URL | Credentials |
|---------|-----|-------------|
| Grafana | http://localhost:3000 | admin/admin |
| Prometheus | http://localhost:9092 | - |
| Safe API | http://localhost:8000 | - |
| Safe API Docs | http://localhost:8000/api/ | - |
| RabbitMQ UI | http://localhost:15672 | safe/safe |
| IPFS API | http://localhost:5001 | - |
| IPFS Gateway | http://localhost:8080 | - |
| Mordor RPC | http://localhost:8545 | - |
| Fork Monitor | http://localhost:9090/metrics | - |
| Gas Estimator | http://localhost:9091/metrics | - |

## Quick Checks

### Is Mordor node syncing?

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}'
```

### Get current block number

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Check IPFS node

```bash
curl http://localhost:5001/api/v0/version
```

### Test Safe Transaction Service

```bash
curl http://localhost:8000/api/v1/about/
```

### View Fork Monitor metrics

```bash
curl http://localhost:9090/metrics | grep etc_mordor
```

### View Gas Estimator metrics

```bash
curl http://localhost:9091/metrics | grep etc_mordor
```

## Troubleshooting

### Service won't start

```bash
# Check logs
docker-compose logs [service-name]

# Check if port is in use
sudo lsof -i :[port-number]

# Check Docker status
docker info
```

### Out of disk space

```bash
# Remove unused containers and images
docker system prune -a

# Remove specific volumes (WARNING: data loss)
docker volume rm [volume-name]
```

### Reset everything

```bash
# Stop and remove all containers, networks, and volumes
docker-compose down -v

# Start fresh
./start.sh
```

### Database connection issues

```bash
# Check PostgreSQL is healthy
docker-compose ps postgres

# Restart PostgreSQL
docker-compose restart postgres

# View PostgreSQL logs
docker-compose logs postgres
```

### Safe service migrations

```bash
# Run migrations manually
docker-compose exec safe-transaction-service python manage.py migrate

# Create superuser
docker-compose exec safe-transaction-service python manage.py createsuperuser
```

## Configuration Changes

### Update environment variables

```bash
# Edit .env file
nano .env

# Restart affected services
docker-compose up -d
```

### Update Prometheus config

```bash
# Edit configuration
nano prometheus/prometheus.yml

# Restart Prometheus
docker-compose restart prometheus
```

### Update Grafana dashboard

```bash
# Edit dashboard JSON
nano grafana/provisioning/dashboards/mordor-dashboard.json

# Restart Grafana
docker-compose restart grafana
```

## Data Management

### Backup

```bash
# Backup all volumes
mkdir -p backups
docker run --rm \
  --volumes-from mordor-node \
  -v $(pwd)/backups:/backup \
  ubuntu tar czf /backup/mordor-$(date +%Y%m%d).tar.gz /root/.ethereum
```

### Restore

```bash
# Stop services
docker-compose down

# Restore data
docker run --rm \
  --volumes-from mordor-node \
  -v $(pwd)/backups:/backup \
  ubuntu tar xzf /backup/mordor-YYYYMMDD.tar.gz -C /

# Start services
./start.sh
```

### Clean up old data

```bash
# Remove old Prometheus data (keeps 30 days by default)
# Configured in docker-compose.yml

# Vacuum PostgreSQL
docker-compose exec postgres vacuumdb -U safe -d safe -v -z

# Clean IPFS repo
docker-compose exec ipfs ipfs repo gc
```

## Performance Monitoring

### Check system resources

```bash
# Real-time stats
docker stats

# Disk I/O
docker stats --format "table {{.Name}}\t{{.BlockIO}}"

# Network usage
docker stats --format "table {{.Name}}\t{{.NetIO}}"
```

### Prometheus queries

Access Prometheus at http://localhost:9092 and try these queries:

```promql
# Current block height
etc_mordor_block_height

# Gas price median
etc_mordor_gas_price_median_wei

# Active forks
etc_mordor_active_forks

# Block time rate
rate(etc_mordor_block_height[5m])
```

## Maintenance Tasks

### Regular tasks

```bash
# Update Docker images (weekly)
docker-compose pull
docker-compose up -d

# Clean up Docker (monthly)
docker system prune -a

# Backup data (daily/weekly)
./backup.sh  # Create this script as needed

# Check logs for errors (daily)
docker-compose logs --since 24h | grep -i error

# Monitor disk space (daily)
df -h
docker system df
```

### Security updates

```bash
# Pull latest images
docker-compose pull

# Rebuild custom images
docker-compose build --pull

# Restart services
docker-compose up -d
```

## Development Tips

### Rebuild a service after code changes

```bash
# For fork-monitor
docker-compose build fork-monitor
docker-compose up -d fork-monitor

# For gas-estimator
docker-compose build gas-estimator
docker-compose up -d gas-estimator
```

### Access service shell

```bash
# PostgreSQL
docker-compose exec postgres psql -U safe -d safe

# Redis
docker-compose exec redis redis-cli

# IPFS
docker-compose exec ipfs sh

# Safe Transaction Service
docker-compose exec safe-transaction-service bash
```

### Run commands in services

```bash
# Django management commands
docker-compose exec safe-transaction-service python manage.py [command]

# Examples
docker-compose exec safe-transaction-service python manage.py shell
docker-compose exec safe-transaction-service python manage.py dbshell
```

## Environment Variables Reference

Key variables in `.env`:

| Variable | Purpose | Default |
|----------|---------|---------|
| `ETHEREUM_CHAIN_ID` | Network chain ID | 63 (Mordor) |
| `POSTGRES_USER` | PostgreSQL user | safe |
| `POSTGRES_PASSWORD` | PostgreSQL password | safe |
| `DJANGO_SECRET_KEY` | Django secret | changeme... |
| `ETHEREUM_NODE_URL` | Mordor node URL | http://mordor-node:8545 |

**Important**: Change all passwords in production!

## Getting Help

1. Check service logs: `docker-compose logs [service-name]`
2. Review documentation: `README.md` and `SERVICES.md`
3. Check service health: `docker-compose ps`
4. Review GitHub issues: https://github.com/chippr-robotics/mordor-tool-kit/issues
5. Consult upstream documentation:
   - IPFS: https://docs.ipfs.tech/
   - Safe: https://docs.safe.global/
   - Prometheus: https://prometheus.io/docs/
   - Grafana: https://grafana.com/docs/

## Common Scenarios

### Just want to monitor the blockchain?

Start minimal services:
```bash
# Edit docker-compose.yml to comment out Safe-related services
docker-compose up -d mordor-node fork-monitor gas-estimator prometheus grafana
```

### Just want IPFS?

```bash
docker-compose up -d ipfs
```

### Just want Safe Transaction Service?

```bash
docker-compose up -d mordor-node postgres redis rabbitmq safe-transaction-service safe-worker safe-scheduler
```

### Want everything?

```bash
./start.sh
```
