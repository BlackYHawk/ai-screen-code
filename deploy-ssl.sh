#!/bin/bash
# 阿里云 Linux 服务器 SSL 证书生成脚本

set -e

echo "=== 阿里云 Linux SSL 证书生成 ==="

# 检查是否为 root 用户
if [ "$EUID" -ne 0 ]; then
  echo "请使用 sudo 运行此脚本"
  exit 1
fi

# 安装 mkcert
echo "1. 安装 mkcert..."
if command -v mkcert &> /dev/null; then
  echo "mkcert 已安装"
else
  # Ubuntu/Debian
  if [ -f /etc/debian_version ]; then
    apt update && apt install -y libnss3-tools
    wget -O /usr/local/bin/mkcert https://github.com/FiloSottile/mkcert/releases/download/v1.4.4/mkcert-v1.4.4-linux-amd64
    chmod +x /usr/local/bin/mkcert
  # CentOS/RHEL
  elif [ -f /etc/redhat-release ]; then
    yum install -y nss-tools
    wget -O /usr/local/bin/mkcert https://github.com/FiloSottile/mkcert/releases/download/v1.4.4/mkcert-v1.4.4-linux-amd64
    chmod +x /usr/local/bin/mkcert
  fi
fi

# 安装本地 CA
echo "2. 安装本地 CA..."
mkcert -install

# 创建证书目录
mkdir -p /opt/ai-screen-code/ssl

# 生成证书（替换为你的服务器 IP 或域名）
SERVER_HOST="${1:-你的服务器IP}"
echo "3. 为 $SERVER_HOST 生成证书..."
mkcert -key-file /opt/ai-screen-code/ssl/key.pem -cert-file /opt/ai-screen-code/ssl/cert.pem "$SERVER_HOST" 127.0.0.1 localhost

echo ""
echo "=== 证书生成完成 ==="
echo "证书位置: /opt/ai-screen-code/ssl/"
echo "  - cert.pem"
echo "  - key.pem"
echo ""
echo "下一步: 修改 docker-compose.yml 中的 SSL 路径，然后运行 docker-compose up -d"
