# local-oss-rs

一个基于 Rust 实现的本地对象存储服务器，提供简单的文件上传和管理功能。

## 功能特性

- 文件上传：支持单文件上传
- 路径管理：支持指定上传路径
- 压缩包处理：支持自动解压（zip、tar、tar.gz）
- 安全性：路径检查和清理
- 高性能：异步处理，支持大文件上传

## 快速开始

### 安装

```bash
git clone https://github.com/sosohime/local-oss-rs.git
cd local-oss-rs
cargo build --release
```

### 配置

修改`config.toml`配置文件:

```toml
storage_dir = "./storage"  # 存储目录

[server]
host = "127.0.0.1"
port = 8080
```

### 运行

```bash
cargo run --release
```

## API使用说明

### 文件上传

基础上传

```bash
curl -X POST http://localhost:8080/upload \
  -F "file=@/path/to/your/file.txt"
```

基础上传

```bash
curl -X POST http://localhost:8080/upload \
  -F "file=@/path/to/your/file.txt" \
  -F "path=custom/path"
```

基础上传

```bash
curl -X POST http://localhost:8080/upload \
  -F "file=@/path/to/your/archive.zip" \
  -F "should_unzip=true"
```

## 开发说明

```plaintext
src/
├── main.rs        # 程序入口
├── config.rs      # 配置管理
├── error.rs       # 错误处理
└── storage.rs     # 存储实现
```

### 构建要求

- Rust 1.56.0 或更高版本
- 依赖项：
  - actix-web：Web 框架
  - actix-multipart：文件上传处理
  - zip：ZIP 文件处理
  - tar：TAR 文件处理
  - flate2：压缩文件处理
  - serde：序列化支持
  - toml：配置文件解析

## LICENSE

MIT

## 注意事项

- 确保配置文件中的存储目录具有正确的读写权限
- 大文件上传可能需要调整服务器配置
- 解压功能仅支持指定的压缩格式
