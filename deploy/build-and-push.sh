#!/bin/bash
set -e

# Configuration
REGISTRY=${DOCKER_REGISTRY:-docker.io}
IMAGE_PREFIX=${IMAGE_PREFIX:-your-dockerhub-username/ai-screen-code}

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Building and pushing Docker images...${NC}"

# Get version from git
VERSION=$(git rev-parse --short HEAD)
DATE=$(date +%Y%m%d)
LATEST_TAG="latest"

# Build and push backend
echo -e "${YELLOW}Building backend...${NC}"
docker build -t ${IMAGE_PREFIX}-backend:${VERSION} -t ${IMAGE_PREFIX}-backend:${DATE} -t ${IMAGE_PREFIX}-backend:${LATEST_TAG} ./backend

echo -e "${YELLOW}Pushing backend images...${NC}"
docker push ${IMAGE_PREFIX}-backend:${VERSION}
docker push ${IMAGE_PREFIX}-backend:${DATE}
docker push ${IMAGE_PREFIX}-backend:${LATEST_TAG}

# Build and push frontend
echo -e "${YELLOW}Building frontend...${NC}"
docker build -t ${IMAGE_PREFIX}-frontend:${VERSION} -t ${IMAGE_PREFIX}-frontend:${DATE} -t ${IMAGE_PREFIX}-frontend:${LATEST_TAG} ./frontend

echo -e "${YELLOW}Pushing frontend images...${NC}"
docker push ${IMAGE_PREFIX}-frontend:${VERSION}
docker push ${IMAGE_PREFIX}-frontend:${DATE}
docker push ${IMAGE_PREFIX}-frontend:${LATEST_TAG}

echo -e "${GREEN}Done!${NC}"
echo ""
echo "Images pushed:"
echo "  - ${IMAGE_PREFIX}-backend:${VERSION}"
echo "  - ${IMAGE_PREFIX}-backend:${DATE}"
echo "  - ${IMAGE_PREFIX}-backend:${LATEST_TAG}"
echo "  - ${IMAGE_PREFIX}-frontend:${VERSION}"
echo "  - ${IMAGE_PREFIX}-frontend:${DATE}"
echo "  - ${IMAGE_PREFIX}-frontend:${LATEST_TAG}"
echo ""
echo "To use with docker-compose, set environment variables:"
echo "  export DOCKER_REGISTRY=${REGISTRY}"
echo "  export IMAGE_PREFIX=${IMAGE_PREFIX}"
