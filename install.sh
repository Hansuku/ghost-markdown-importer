#!/bin/bash
# Ghost Markdown Importer - 一键安装脚本
# https://github.com/Hansuku/ghost-markdown-importer

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检测操作系统
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*|MINGW*|MSYS*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# 检测CPU架构
detect_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x86_64";;
        arm64|aarch64) echo "arm64";;
        *)          echo "unknown";;
    esac
}

# 主安装函数
main() {
    echo -e "${GREEN}Ghost Markdown Importer 安装脚本${NC}"
    echo "======================================="
    
    OS=$(detect_os)
    ARCH=$(detect_arch)
    
    if [[ "$OS" == "unknown" ]] || [[ "$ARCH" == "unknown" ]]; then
        echo -e "${RED}错误: 不支持的操作系统或架构${NC}"
        echo "请手动下载: https://github.com/Hansuku/ghost-markdown-importer/releases"
        exit 1
    fi
    
    if [[ "$OS" == "windows" ]]; then
        echo -e "${YELLOW}Windows用户请手动下载: https://github.com/Hansuku/ghost-markdown-importer/releases${NC}"
        exit 1
    fi
    
    # 确定下载文件名
    if [[ "$OS" == "macos" ]]; then
        FILENAME="gmi-macos-${ARCH}.tar.gz"
    else
        FILENAME="gmi-linux-${ARCH}.tar.gz"
    fi
    
    DOWNLOAD_URL="https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/${FILENAME}"
    
    echo -e "${GREEN}检测到的系统: ${OS} ${ARCH}${NC}"
    echo -e "${GREEN}下载地址: ${DOWNLOAD_URL}${NC}"
    
    # 创建临时目录
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    # 下载文件
    echo -e "${YELLOW}正在下载...${NC}"
    curl -L -o "$FILENAME" "$DOWNLOAD_URL"
    
    # 解压
    echo -e "${YELLOW}正在解压...${NC}"
    tar -xzf "$FILENAME"
    
    # 安装
    echo -e "${YELLOW}正在安装到 /usr/local/bin...${NC}"
    sudo mv gmi /usr/local/bin/
    sudo chmod +x /usr/local/bin/gmi
    
    # 清理
    cd /
    rm -rf "$TEMP_DIR"
    
    echo -e "${GREEN}✅ 安装完成!${NC}"
    echo "运行 'gmi --help' 开始使用"
}

# 检查依赖
command -v curl >/dev/null 2>&1 || {
    echo -e "${RED}错误: 需要 curl${NC}"
    exit 1
}

command -v tar >/dev/null 2>&1 || {
    echo -e "${RED}错误: 需要 tar${NC}"
    exit 1
}

# 运行主函数
main "$@"