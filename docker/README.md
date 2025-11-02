# Mordor Testnet Docker Compose Infrastructure

This directory contains the Docker Compose configuration for the complete Mordor testnet monitoring and infrastructure stack.

## Services

### Core Services

- **mordor-node**: Ethereum Classic Mordor testnet node (core-geth)
- **fork-monitor**: Real-time blockchain fork monitoring service
- **gas-estimator**: Gas price analysis and estimation service

### Monitoring & Visualization

- **prometheus**: Metrics collection and storage
- **grafana**: Metrics visualization and dashboards

### Decentralized Storage

- **ipfs**: IPFS node (Kubo) for decentralized file storage

### Safe Transaction Service Stack

- **safe-transaction-service**: Safe multisig transaction service API
- **safe-worker**: Celery worker for Safe transaction processing
- **safe-scheduler**: Celery beat scheduler for Safe service
- **postgres**: PostgreSQL database for Safe service
- **redis**: Redis cache for Safe service
- **rabbitmq**: RabbitMQ message broker for Safe service

## Quick Start

### Prerequisites

- Docker 20.10+
- Docker Compose 1.29+
- 20GB+ free disk space
- 8GB+ RAM recommended

### Installation

1. **Copy the environment file:**
   ```bash
   cd docker
   cp .env.example .env
   ```

2. **Edit the `.env` file with your settings** (especially change the default passwords in production).

3. **Start all services:**
   ```bash
   docker-compose up -d
   ```

4. **Check service status:**
   ```bash
   docker-compose ps
   ```

5. **View logs:**
   ```bash
   docker-compose logs -f
   ```

## Service URLs

After starting the services, you can access them at:

| Service | URL | Default Credentials |
|---------|-----|---------------------|
| Grafana Dashboard | http://localhost:3000 | admin/admin |
| Prometheus | http://localhost:9092 | - |
| Safe Transaction Service | http://localhost:8000 | - |
| RabbitMQ Management | http://localhost:15672 | safe/safe |
| IPFS API | http://localhost:5001 | - |
| IPFS Gateway | http://localhost:8080 | - |
| Mordor RPC | http://localhost:8545 | - |
| Fork Monitor Metrics | http://localhost:9090/metrics | - |
| Gas Estimator Metrics | http://localhost:9091/metrics | - |

## Configuration

### Prometheus

Edit `prometheus/prometheus.yml` to configure:
- Scrape intervals
- Target endpoints
- Alert rules

### Grafana

Dashboards are automatically provisioned from:
```
grafana/provisioning/dashboards/mordor-dashboard.json
```

To customize:
1. Make changes in the Grafana UI
2. Export the dashboard JSON
3. Replace `mordor-dashboard.json`
4. Restart Grafana: `docker-compose restart grafana`

### Safe Transaction Service

The Safe Transaction Service requires configuration for the Mordor testnet:

1. **Chain ID**: Set to 63 (Mordor)
2. **Ethereum Node**: Points to the local mordor-node service
3. **Database**: PostgreSQL with persistent storage

**Important**: In production, ensure you:
- Change all default passwords
- Set a strong `DJANGO_SECRET_KEY`
- Configure proper Safe contract addresses if needed
- Set up proper backup strategies for PostgreSQL data

### IPFS

The IPFS node runs with the `server` profile by default, optimized for server environments. You can customize this in the docker-compose.yml file.

## Management Commands

### Start services
```bash
docker-compose up -d
```

### Stop services
```bash
docker-compose down
```

### Stop services and remove volumes (WARNING: data loss)
```bash
docker-compose down -v
```

### View logs for all services
```bash
docker-compose logs -f
```

### View logs for a specific service
```bash
docker-compose logs -f [service-name]
# Example: docker-compose logs -f safe-transaction-service
```

### Restart a service
```bash
docker-compose restart [service-name]
```

### Rebuild a service
```bash
docker-compose up -d --build [service-name]
```

## Volumes

The following Docker volumes are created for persistent data:

- `mordor-data`: Blockchain data for the Mordor node
- `prometheus-data`: Prometheus metrics storage
- `grafana-data`: Grafana configuration and dashboards
- `ipfs-data`: IPFS data storage
- `postgres-data`: PostgreSQL database
- `redis-data`: Redis cache
- `rabbitmq-data`: RabbitMQ message queue

## Networking

All services run on a bridge network called `mordor-network`, allowing them to communicate with each other using service names as hostnames.

## Health Checks

Several services have built-in health checks:

- PostgreSQL: `pg_isready`
- Redis: `redis-cli ping`
- RabbitMQ: `rabbitmq-diagnostics ping`

These health checks ensure dependent services wait for their dependencies to be ready.

## Troubleshooting

### Services won't start

1. Check Docker is running: `docker info`
2. Check available disk space: `df -h`
3. Check service logs: `docker-compose logs [service-name]`

### Mordor node won't sync

1. Check if ports are accessible
2. Check logs: `docker-compose logs mordor-node`
3. Ensure you have enough disk space

### Safe Transaction Service errors

1. Ensure PostgreSQL is healthy: `docker-compose ps postgres`
2. Check database migrations ran: `docker-compose logs safe-transaction-service`
3. Verify the Mordor node is synced and accessible

### IPFS connection issues

1. Check IPFS is running: `docker-compose ps ipfs`
2. Test API: `curl http://localhost:5001/api/v0/version`
3. Check firewall settings for port 4001

## Production Considerations

For production deployments:

1. **Security**:
   - Change all default passwords
   - Use strong secrets for DJANGO_SECRET_KEY
   - Configure proper firewall rules
   - Use HTTPS with proper certificates
   - Restrict CORS and allowed hosts

2. **Backups**:
   - Regular backups of PostgreSQL data
   - Backup Grafana dashboards and configurations
   - Consider backing up IPFS data

3. **Monitoring**:
   - Set up alerts in Prometheus/Grafana
   - Monitor disk usage
   - Monitor network connectivity

4. **Resources**:
   - Allocate sufficient RAM (8GB+ recommended)
   - Ensure fast SSD storage
   - Monitor and scale as needed

## Contributing

To add new services:

1. Add service definition to `docker-compose.yml`
2. Add any required configuration files
3. Update this README
4. Test the complete stack

## Support

For issues and questions:
- GitHub Issues: https://github.com/chippr-robotics/mordor-tool-kit/issues
- Documentation: See main repository README.md
