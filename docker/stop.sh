#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}Stopping Mordor Testnet Infrastructure${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""

# Stop services
echo -e "${YELLOW}Stopping all services...${NC}"
docker-compose down

echo ""
echo -e "${GREEN}All services stopped successfully!${NC}"
echo ""
echo "To remove all data volumes, run: docker-compose down -v"
echo "To start services again, run: ./start.sh"
echo ""
