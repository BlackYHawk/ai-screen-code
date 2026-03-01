#!/bin/bash

# AI Screen Code 部署脚本
# 使用方法: ./deploy/deploy.sh [env]

set -e

ENV=${1:-production}
REGISTRY=${DOCKER_REGISTRY:-docker.io}
NAMESPACE=${NAMESPACE:-blackyawk}

echo "========================================="
echo "AI Screen Code 部署脚本"
echo "环境: $ENV"
echo "镜像仓库: $REGISTRY"
echo "========================================="

# 检查必要的工具
command -v docker >/dev/null 2>&1 || { echo "Docker 未安装"; exit 1; }
command -v kubectl >/dev/null 2>&1 || { echo "kubectl 未安装"; exit 1; }

# 加载环境变量
if [ -f ".env" ]; then
    echo "加载环境变量..."
    export $(cat .env | grep -v '^#' | xargs)
fi

# 构建后端镜像
echo "构建后端镜像..."
cd backend
docker build -t ${REGISTRY}/${NAMESPACE}/ai-screen-backend:${ENV} .
docker tag ${REGISTRY}/${NAMESPACE}/ai-screen-backend:${ENV} ${REGISTRY}/${NAMESPACE}/ai-screen-backend:latest

# 构建前端镜像
echo "构建前端镜像..."
cd ../frontend
docker build -t ${REGISTRY}/${NAMESPACE}/ai-screen-frontend:${ENV} .
docker tag ${REGISTRY}/${NAMESPACE}/ai-screen-frontend:${ENV} ${REGISTRY}/${NAMESPACE}/ai-screen-frontend:latest

# 推送到镜像仓库
echo "推送镜像到仓库..."
docker push ${REGISTRY}/${NAMESPACE}/ai-screen-backend:${ENV}
docker push ${REGISTRY}/${NAMESPACE}/ai-screen-backend:latest
docker push ${REGISTRY}/${NAMESPACE}/ai-screen-frontend:${ENV}
docker push ${REGISTRY}/${NAMESPACE}/ai-screen-frontend:latest

# 部署到 Kubernetes
echo "部署到 Kubernetes..."
cd ../deploy
kubectl apply -f kubernetes.yml

# 显示部署状态
echo "========================================="
echo "部署完成！查看服务状态:"
echo "kubectl get pods -n ai-screen-code"
echo "kubectl get svc -n ai-screen-code"
echo "========================================="
