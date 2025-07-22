# Ghost Markdown Importer

[English Version](README.md) | 中文版本

一个强大的 Rust CLI 工具，将 Markdown 文件转换为 Ghost CMS 导入格式（JSON 或 ZIP 包）。

## 功能特性

- ✅ **Ghost CMS 兼容**：生成完美的 Ghost 导入 JSON 格式
- 📝 **Markdown 处理**：完整的 markdown 到 HTML 转换，支持 frontmatter
- 📦 **灵活导出**：JSON 或 ZIP 格式，可选图片包含
- 🔄 **递归处理**：处理整个目录树
- 🎯 **Frontmatter 支持**：YAML frontmatter 用于文章元数据
- 🏷️ **标签管理**：自动标签创建和分配
- 👤 **作者支持**：多作者，默认回退
- 🔍 **文件过滤**：排除模式和详细日志

## 快速开始

### 1. 下载二进制文件（30秒）
```bash
# Linux/macOS - 一键安装
curl -sSL https://raw.githubusercontent.com/Hansuku/ghost-markdown-importer/main/install.sh | bash

# 或者手动下载
# 访问 https://github.com/Hansuku/ghost-markdown-importer/releases/latest
```

### 2. 基本使用
```bash
# 转换当前目录的markdown文件
gmi ./my-blog-posts

# 指定输出文件
gmi ./content --output ghost-import.json

# 创建包含图片的ZIP包
gmi ./content --format zip --output blog-export.zip --include-images
```

## 安装

### 快速安装（推荐）
为您的平台下载最新的预编译二进制文件：

- **Linux x86_64**: [gmi-linux-x86_64.tar.gz](https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/gmi-linux-x86_64.tar.gz)
- **macOS Intel**: [gmi-macos-x86_64.tar.gz](https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/gmi-macos-x86_64.tar.gz)
- **macOS Apple Silicon**: [gmi-macos-arm64.tar.gz](https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/gmi-macos-arm64.tar.gz)
- **Windows x86_64**: [gmi-windows-x86_64.zip](https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/gmi-windows-x86_64.zip)

#### Linux/macOS
```bash
# 下载并安装（替换为您的平台）
curl -L -o gmi.tar.gz https://github.com/Hansuku/ghost-markdown-importer/releases/latest/download/gmi-linux-x86_64.tar.gz
tar -xzf gmi.tar.gz
sudo mv gmi /usr/local/bin/

# 验证安装
gmi --help
```

#### Windows
1. 从发布页下载 Windows zip 文件
2. 将 `gmi.exe` 解压到 PATH 中的文件夹
3. 打开命令提示符并运行：`gmi --help`

### 从源码安装
```bash
git clone https://github.com/Hansuku/ghost-markdown-importer.git
cd ghost-markdown-importer
cargo install --path .
```

### 从 Crates.io 安装（发布后）
```bash
cargo install ghost-markdown-importer
```

## 使用

### 基本使用

```bash
# 将文件夹的 markdown 文件转换为 Ghost JSON
gmi ./my-blog-posts

# 输出到特定文件
gmi ./content --output ghost-import.json

# 创建包含图片的 ZIP 包
gmi ./content --output blog-export.zip --format zip --include-images

# 使用自定义设置递归处理
gmi ./content --recursive --author "hansuku" --default-tags "技术,博客,rust"
```

### 高级使用

```bash
# 带排除的详细处理
gmi ./content \
  --recursive \
  --verbose \
  --author "hansuku" \
  --default-tags "开发,编程" \
  --exclude "草稿/*,旧文件/*" \
  --output ghost-export.json

# 完整的 ZIP 包
gmi ./content \
  --format zip \
  --output complete-blog.zip \
  --include-images \
  --recursive \
  --author "hansuku" \
  --default-tags "精选"
```

## 输入格式

### Markdown 文件

您的 markdown 文件应使用 YAML frontmatter 作为元数据：

```markdown
---
title: "我的精彩文章"
date: 2024-01-15
author: "hansuku"
tags: ["rust", "教程", "编程"]
slug: "my-awesome-post"
description: "这篇文章是关于一些令人惊叹的内容"
featured: true
image: "/images/awesome.jpg"
status: "published"
---

# 你好世界

这是您文章的内容。支持所有 markdown 功能：

- **粗体文本**
- *斜体文本*
- `代码块`
- [链接](https://example.com)
- 图片：![Alt text](/images/example.png)

## 代码块

```rust
fn main() {
    println!("你好, Ghost!");
}
```

## 表格

| 功能 | 支持 |
|------|------|
| Markdown | ✅ |
| 图片 | ✅ |
| Frontmatter | ✅ |
```

### 支持的 Frontmatter 字段

| 字段 | 类型 | 描述 |
|------|------|------|
| `title` | 字符串 | 文章标题（必需） |
| `date` | 字符串/日期 | 发布日期 |
| `author` | 字符串 | 作者姓名 |
| `tags` | 数组 | 标签列表 |
| `slug` | 字符串 | URL 别名（缺失时自动生成） |
| `description` | 字符串 | 文章摘要 |
| `featured` | 布尔值 | 标记为精选文章 |
| `image` | 字符串 | 特色图片 URL |
| `status` | 字符串 | `published`、`draft` 或 `scheduled` |
| `category` | 字符串 | 主要分类 |

## 输出格式

### JSON 导出
创建可直接用于 Ghost 导入的单个 `.json` 文件：

```json
{
  "meta": {
    "exported_on": 1704312169707,
    "version": "5.75.1"
  },
  "data": {
    "posts": [
      {
        "id": 1,
        "title": "我的精彩文章",
        "slug": "my-awesome-post",
        "html": "<h1>你好世界</h1>...",
        "status": "published",
        "published_at": "2024-01-15T00:00:00.000Z",
        ...
      }
    ],
    "tags": [...],
    "users": [...]
  }
}
```

### ZIP 导出
创建包含以下内容的完整包：
- `ghost-import.json` - JSON 导入文件
- `content/images/` - 按目录结构组织的所有图片

## CLI 选项

```
用法:
    gmi [选项] <输入>

参数:
    <输入>    包含 markdown 文件的输入目录

选项:
    -o, --output <输出>          输出文件路径（JSON 或 ZIP）
    -f, --format <格式>         导出格式 [默认: json] [可选值: json, zip]
    -r, --recursive               递归处理目录
    -a, --author <作者>         没有作者的文章的默认作者姓名
        --default-tags <标签>...   添加到所有文章的默认标签
        --exclude <模式>...    排除匹配这些模式的文件
        --include-images          在 ZIP 导出中包含图片
    -v, --verbose                 详细输出
    -h, --help                    打印帮助
```

## 示例

### 示例目录结构
```
我的博客/
├── posts/
│   ├── 2024-01-15-hello-world.md
│   ├── 2024-01-20-rust-tutorial.md
│   └── drafts/
│       └── 2024-01-25-draft-post.md
├── images/
│   ├── hello-world.png
│   └── rust-logo.png
└── assets/
    └── css/
```

### 处理示例
```bash
# 处理所有文章，排除草稿
gmi ./我的博客/posts \
  --recursive \
  --exclude "drafts/*" \
  --author "技术博主" \
  --default-tags "博客,技术" \
  --output ghost-blog.json \
  --verbose

# 创建包含图片的完整 ZIP 包
gmi ./我的博客 \
  --recursive \
  --format zip \
  --output complete-blog.zip \
  --include-images \
  --author "技术博主" \
  --default-tags "精选"
```

## 导入到 Ghost

### 方法 1: Ghost 管理后台
1. 进入您的 Ghost 管理后台
2. 导航到 **设置** → **实验室**
3. 滚动到 **导入内容**
4. 上传生成的 `ghost-import.json` 或 `.zip` 文件
5. 点击 **导入**

### 方法 2: Ghost CLI（如果可用）
```bash
ghost import ghost-import.json
```

## 开发

### 先决条件
- Rust 1.70+
- Cargo

### 构建
```bash
# 克隆仓库
git clone https://github.com/Hansuku/ghost-markdown-importer.git
cd ghost-markdown-importer

# 调试模式构建
cargo build

# 发布模式构建
cargo build --release

# 运行测试
cargo test

# 使用示例数据运行
cargo run -- ./examples/sample-content --verbose
```

### 开发设置
```bash
# 安装开发依赖
cargo install cargo-watch

# 监视模式运行测试
cargo watch -x test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

## 故障排除

### 常见问题

**"未找到 markdown 文件"**
- 确保文件扩展名为 `.md`
- 检查目录路径是否正确
- 对子目录使用 `--recursive` 标志

**"YAML frontmatter 解析失败"**
- 检查 frontmatter 中的 YAML 语法
- 确保 YAML 缩进正确
- 验证字符串中的特殊字符

**"ZIP 中未包含图片"**
- 使用 `--include-images` 标志
- 确保图片为支持的格式（jpg、png、gif、webp、svg）
- 检查图片路径是否可访问

### 调试模式
使用 `--verbose` 标志获取详细的处理信息：
```bash
gmi ./content --verbose
```

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) 用于 markdown 解析
- [serde](https://serde.rs/) 用于 JSON 序列化
- [clap](https://docs.rs/clap/) 用于 CLI 参数解析
- Ghost CMS 团队提供优秀的导入格式文档

## 更新日志

### v0.1.0
- 初始版本
- Markdown 到 Ghost JSON 转换
- 带图片的 ZIP 导出
- 全面的 CLI 界面
- YAML frontmatter 支持
- 标签和作者管理