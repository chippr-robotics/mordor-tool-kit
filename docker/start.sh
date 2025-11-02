#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Mordor Testnet Infrastructure Startup${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Error: Docker is not running${NC}"
    exit 1
fi

# Check if .env exists, if not copy from example
if [ ! -f .env ]; then
    echo -e "${YELLOW}No .env file found. Copying from .env.example...${NC}"
    cp .env.example .env
    echo -e "${YELLOW}Please review and update .env with your settings${NC}"
    echo ""
fi

# Start services
echo -e "${GREEN}Starting all services...${NC}"
docker-compose up -d

echo ""
echo -e "${GREEN}Waiting for services to be ready...${NC}"
sleep 10

# Check service status
echo ""
echo -e "${GREEN}Service Status:${NC}"
docker-compose ps

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Services are starting!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Access the following services:"
echo ""
echo "  Grafana:                  http://localhost:3000 (admin/admin)"
echo "  Prometheus:               http://localhost:9092"
echo "  Safe Transaction Service: http://localhost:8000"
echo "  RabbitMQ Management:      http://localhost:15672 (safe/safe)"
echo "  IPFS API:                 http://localhost:5001"
echo "  IPFS Gateway:             http://localhost:8080"
echo "  Mordor RPC:               http://localhost:8545"
echo "  Fork Monitor Metrics:     http://localhost:9090/metrics"
echo "  Gas Estimator Metrics:    http://localhost:9091/metrics"
echo ""
echo "View logs with: docker-compose logs -f"
echo "Stop services with: docker-compose down"
echo ""
