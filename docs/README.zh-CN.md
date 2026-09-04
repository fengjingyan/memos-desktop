# Memos Desktop（中文）

> [Memos](https://github.com/usememos/memos) 的原生桌面应用 —— 自托管笔记，一键启动。

Memos Desktop 用轻量的 [Tauri](https://tauri.app) 窗口封装了官方 [Memos](https://usememos.com) 服务端。它把 `memos` 二进制作为 sidecar 打包进应用，启动时自动拉起服务端、等待端口就绪后打开 Memos 网页界面 —— 不需要 Docker，不需要手动执行 `memos start`，也不用在浏览器里翻标签页。

[English](../README.md) | **中文**

## 为什么选择 Memos Desktop？

- **零配置启动** —— Memos 服务端已打包并由应用托管，双击即用。
- **本地优先** —— 数据以 SQLite 数据库形式保存在本机（默认与可执行文件同目录），完全由你掌控。
- **可配置** —— 端口、数据目录、窗口大小、额外服务端参数，全部由一个 `config.yaml` 控制。
- **轻量** —— Tauri 外壳（约 4.5 MB），而非 Electron 运行时。

## 快速开始

### 下载发布版

从 [Releases](../../releases) 页面获取最新构建：

| 产物 | 说明 |
| --- | --- |
| `memos-desktop_x.y.z_x64-setup.exe` | NSIS 安装包（推荐） |
| `memos-desktop_x.y.z_x64_en-US.msi` | MSI 安装包，适合企业部署 |
| `memos-desktop_x.y.z-win-x64.7z` / `.zip` | 绿色版 —— 解压即用 |

运行 `memos-desktop.exe`，Memos 界面会自动打开。首次启动时服务端会初始化数据库，可能需要几秒钟。

### 从源码运行

前置条件：[Rust](https://rustup.rs)、[Node.js](https://nodejs.org)，以及一个 `memos` 二进制。

1. 将 Memos 服务端二进制放到 `binaries/memos-x86_64-pc-windows-msvc.exe`
   （sidecar 文件名必须带目标平台三元组；其他平台使用对应三元组，如 `memos-x86_64-apple-darwin`）。
2. 安装依赖并以开发模式启动：

   ```bash
   npm install
   npm run tauri dev
   ```

3. 构建发布包（NSIS + MSI）：

   ```bash
   npm run tauri build
   ```

   产物位于 `src-tauri/target/release/bundle/`。

## 配置

应用按以下顺序查找 `config.yaml`，取第一个存在的：

1. `memos-desktop.exe` 同目录（如安装目录或绿色版目录）
2. `%APPDATA%\com.admin.memos-desktop\config.yaml`（Windows）

找不到文件时使用内置默认值。可复制 `src-tauri/config.yaml.example` 作为起点：

```yaml
# Memos 服务端监听地址（窗口将加载 http://{host}:{port}）
host: 127.0.0.1

# Memos 服务端监听端口
port: 5230

# 数据目录（SQLite 数据库存放位置）。
# 留空或删除此项时，默认为可执行文件所在目录。
# data: c:/users/user/memos

# 窗口大小（逻辑像素）
width: 1920
height: 1080

# 额外透传给 memos 服务端的参数（可选）
extra_args:
  # - "--verbose"
```

修改配置后需重启应用生效。

## 工作原理

```
memos-desktop.exe（Tauri 外壳）
├── 读取 config.yaml
├── 启动 memos.exe sidecar  →  --port <port> --data <data> [extra_args...]
├── 轮询端口，等待服务端就绪（最长 15 秒）
└── 在 http://<host>:<port> 打开 webview 窗口
```

## 项目结构

```
├── binaries/                  # memos sidecar 二进制（文件名带目标三元组）
├── dist/                      # 绿色版构建产物（7z / zip）
├── docs/                      # 文档（含本中文说明）
├── icons/
└── src-tauri/
    ├── config.yaml.example    # 配置模板
    ├── src/lib.rs             # 配置加载、sidecar 启动、窗口创建
    └── tauri.conf.json
```

## 许可证

Memos Desktop 按现状提供，供个人使用。内置的 [Memos](https://github.com/usememos/memos) 服务端采用 [MIT](https://github.com/usememos/memos/blob/main/LICENSE) 许可证。
