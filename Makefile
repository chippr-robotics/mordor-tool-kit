.PHONY: help build up down restart logs status health metrics cli-build cli clean test

# Default target
help:
	@echo "Mordor Monitoring System - Available Commands"
	@echo "=============================================="
	@echo "  make build          - Build all Docker images"
	@echo "  make up             - Start all services"
	@echo "  make down           - Stop all services"
	@echo "  make restart        - Restart all services"
	@echo "  make logs           - View logs from all services"
	@echo "  make logs-fork      - View fork monitor logs"
	@echo "  make logs-gas       - View gas estimator logs"
	@echo "  make logs-node      - View Mordor node logs"
	@echo "  make status         - Check blockchain status"
	@echo "  make health         - Health check all containers"
	@echo "  make metrics-fork   - View fork monitor metrics"
	@echo "  make metrics-gas    - View gas estimator metrics"
	@echo "  make cli-build      - Build CLI tool"
	@echo "  make cli            - Run CLI tool"
	@echo "  make monitor        - Monitor blockchain in real-time"
	@echo "  make gas            - Get gas price recommendations"
	@echo "  make clean          - Remove all containers and volumes"
	@echo "  make test           - Run all tests"
	@echo ""
	@echo "Ports:"
	@echo "  - Mordor Node RPC:    http://localhost:8545"
	@echo "  - Fork Monitor:       http://localhost:9090/metrics"
	@echo "  - Gas Estimator:      http://localhost:9091/metrics"
	@echo "  - Prometheus:         http://localhost:9092"
	@echo "  - Grafana:            http://localhost:3000 (admin/admin)"

# Build all images
build:
	@echo "Building Docker images..."
	docker-compose build

# Start all services
up:
	@echo "Starting all services..."
	docker-compose up -d
	@echo ""
	@echo "Services starting..."
	@echo "  - Mordor Node:     http://localhost:8545"
	@echo "  - Grafana:         http://localhost:3000 (admin/admin)"
	@echo "  - Prometheus:      http://localhost:9092"
	@echo ""
	@echo "Run 'make status' to check blockchain status"
	@echo "Run 'make health' to verify all containers are healthy"

# Stop all services
down:
	@echo "Stopping all services..."
	docker-compose down

# Restart all services
restart:
	@echo "Restarting all services..."
	docker-compose restart

# View all logs
logs:
	docker-compose logs -f --tail=100

# View fork monitor logs
logs-fork:
	docker-compose logs -f --tail=100 fork-monitor

# View gas estimator logs
logs-gas:
	docker-compose logs -f --tail=100 gas-estimator

# View Mordor node logs
logs-node:
	docker-compose logs -f --tail=100 mordor-node

# Build CLI tool
cli-build:
	@echo "Building CLI tool..."
	cd cli && cargo build --release
	@echo "CLI built at: cli/target/release/mordor-cli"

# CLI helper
cli:
	@cd cli && cargo run --release -- $(ARGS)

# Status command
status:
	@cd cli && cargo run --release -- status

# Health check
health:
	@cd cli && cargo run --release -- health

# Monitor blockchain
monitor:
	@cd cli && cargo run --release -- monitor

# Gas prices
gas:
	@cd cli && cargo run --release -- gas

# Fork monitor metrics
metrics-fork:
	@cd cli && cargo run --release -- metrics --service fork-monitor

# Gas estimator metrics
metrics-gas:
	@cd cli && cargo run --release -- metrics --service gas-estimator

# Get block info
block:
	@cd cli && cargo run --release -- block $(NUMBER)

# Clean everything
clean:
	@echo "Removing all containers and volumes..."
	docker-compose down -v
	@echo "Cleaning build artifacts..."
	cd fork-monitor && cargo clean || true
	cd gas-estimator && cargo clean || true
	cd cli && cargo clean || true

# Run tests
test:
	@echo "Running tests..."
	@bash scripts/test.sh

# Install CLI tool system-wide
install-cli: cli-build
	@echo "Installing CLI tool to /usr/local/bin..."
	sudo cp cli/target/release/mordor-cli /usr/local/bin/
	@echo "CLI installed! Run 'mordor-cli --help'"

# Quick setup for first time
setup: build up
	@echo "Waiting for services to start (30s)..."
	@sleep 30
	@make health
	@echo ""
	@echo "Setup complete! Open Grafana at http://localhost:3000"
	@echo "Default credentials: admin/admin"

# Update dashboard
update-dashboard:
	@echo "Updating Grafana dashboard..."
	curl -X POST \
		-H "Content-Type: application/json" \
		-d @grafana/provisioning/dashboards/mordor-dashboard.json \
		http://admin:admin@localhost:3000/api/dashboards/db

# Backup data
backup:
	@echo "Creating backup..."
	@mkdir -p backups
	@timestamp=$$(date +%Y%m%d-%H%M%S); \
	docker run --rm --volumes-from mordor-node \
		-v $(PWD)/backups:/backup \
		ubuntu tar czf /backup/mordor-data-$$timestamp.tar.gz /root/.ethereum

# Restore data
restore:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make restore FILE=backups/mordor-data-TIMESTAMP.tar.gz"; \
		exit 1; \
	fi
	@echo "Restoring from $(FILE)..."
	docker run --rm --volumes-from mordor-node \
		-v $(PWD)/backups:/backup \
		ubuntu bash -c "cd / && tar xzf /backup/$(notdir $(FILE))"

# Show metrics URLs
urls:
	@echo "Service URLs:"
	@echo "  Grafana Dashboard:    http://localhost:3000/d/mordor-monitoring"
	@echo "  Prometheus:           http://localhost:9092"
	@echo "  Fork Monitor Metrics: http://localhost:9090/metrics"
	@echo "  Gas Estimator Metrics: http://localhost:9091/metrics"
	@echo "  Mordor RPC:           http://localhost:8545"

# Development - rebuild and restart specific service
dev-fork:
	docker-compose up -d --build fork-monitor
	docker-compose logs -f fork-monitor

dev-gas:
	docker-compose up -d --build gas-estimator
	docker-compose logs -f gas-estimator
