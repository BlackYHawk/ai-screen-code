#!/bin/bash
# SSL 证书生成脚本

# 创建证书目录
mkdir -p ssl

# 使用 mkcert 生成证书
# 如果服务器需要远程访问，将 localhost 替换为服务器 IP 或域名
mkcert -key-file ssl/key.pem -cert-file ssl/cert.pem localhost 127.0.0.1

echo "证书已生成到 ssl/ 目录"
echo "  - ssl/cert.pem: 证书"
echo "  - ssl/key.pem: 私钥"
