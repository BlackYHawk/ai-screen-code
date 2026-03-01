# Docker 部署指南

## 方式一：本地构建部署（传统方式）

```bash
# 构建并启动
docker-compose up -d --build

# 仅启动
docker-compose up -d
```

## 方式二：使用预构建镜像（推荐）

### 1. 配置镜像地址

编辑 `.env` 文件：

```bash
# Docker Hub 示例
DOCKER_REGISTRY=docker.io
IMAGE_PREFIX=your-username/ai-screen-code
IMAGE_TAG=latest
```

### 2. 首次构建并推送到远程

```bash
# 构建并推送镜像到远程仓库
./deploy/build-and-push.sh
```

### 3. 在服务器上拉取并运行

```bash
# 拉取最新镜像
docker-compose pull

# 启动服务
docker-compose up -d
```

### 4. 使用特定版本

```bash
# 使用特定版本标签
export IMAGE_TAG=20260301
docker-compose pull
docker-compose up -d
```

## 环境变量说明

| 变量 | 说明 | 示例 |
|------|------|------|
| DOCKER_REGISTRY | Docker 仓库地址 | `docker.io`, `ghcr.io` |
| IMAGE_PREFIX | 镜像前缀 | `your-username/ai-screen-code` |
| IMAGE_TAG | 镜像标签 | `latest`, `v1.0.0`, `20260301` |

## 部署流程

1. **开发机/本地**:
   - 修改代码
   - 运行 `./deploy/build-and-push.sh` 推送到远程仓库

2. **生产服务器**:
   - 运行 `docker-compose pull` 拉取新镜像
   - 运行 `docker-compose up -d` 重启服务

## 常用命令

```bash
# 拉取最新镜像
docker-compose pull

# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down

# 重启服务
docker-compose restart
```
