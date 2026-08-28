<div align="center">

![Logo](assets/neko_logo.jpg)

[English](docs/README_en.md) | [日本語](docs/README_ja.md) | [Русский](docs/README_ru.md)
# Project N.E.K.O. :kissing_cat: <br>**听见你的心情，看见你的世界，<br>陪你发现更多喜欢。**

> **N.E.K.O.** = **N**etworked **E**motional **K**nowledging **O**rganism (网络型情感知性生命体)
>
> N.E.K.O，一个渴望理解、建立连接、并与我们共同成长的数字生命。

为你将「快速开始、进阶使用、项目细节、Star History」等模块整合适配到 Tauri 移植版的 `README.md` 中：

```markdown
# N.E.K.O. Tauri

Project N.E.K.O. 的非官方 Tauri 桌面移植版。

本项目基于开源的 [Project N.E.K.O.](https://github.com/Project-N-E-K-O/N.E-K.O) 进行开发，采用 **Tauri + React + TypeScript** 架构重构桌面端窗口与交互实现，提供更轻量的桌面端运行方案。

> **免责声明**：本项目为社区维护的**非官方移植版（Unofficial Port）**，与 Project N.E.K.O. 官方团队无任何隶属、授权或直接关联。

---

## 移植特性

* **轻量化桌面封装**：基于 Tauri 架构构建，大幅缩减桌面运行时内存与体积占用。
* **原生窗口与穿透适配**：深度适配 Windows / macOS 桌面透明置顶、动态鼠标穿透与拖拽交互。
* **核心功能保留**：保留多模态对话、屏幕理解、多形态 Avatar（Live2D / VRM / MMD）与本地/云端模型接入能力。

---

## 快速开始

### Windows / macOS 用户（一键包）

解压 Release 发布的压缩包后，直接运行 `N.E.K.O.exe` 或 `N.E.K.O.app` 即可启动。

> **macOS 用户注意**：如首次打开提示已损坏或无法验证开发者，请在终端执行以下命令解除系统隔离：
> ```bash
> xattr -cr /Applications/N.E.K.O.app
> ```

<details>
<summary>Docker 部署 (Linux)</summary>

### Docker 运行方式
```bash
docker build -t neko-tauri .
docker run -d -p 3000:3000 --name neko-app neko-tauri

```

### 环境要求

* Node.js (>= 18)
* Rust & Cargo (最新稳定版)
* 平台编译工具链（Xcode Command Line Tools / Visual Studio C++ Build Tools）

### 启动调试

```bash
# 安装依赖
npm install

# 启动 Tauri 开发环境
npm run tauri dev

```

### 构建打包

```bash
npm run tauri build

```

构建产物生成在 `src-tauri/target/release/bundle/`。

* **模型接入**：支持配置 OpenAI / Gemini / Qwen / DeepSeek / Ollama 等兼容接口。
* **快捷键与交互**：支持通过快捷键快速切换鼠标穿透、锁定窗口位置与调出控制面板。
* **Live2D / VRM 自定义**：可将模型文件放置于 `assets/models` 目录下进行加载。

* **技术架构**：Tauri 2 + React + TypeScript + Rust Backend
* **窗口行为**：通过 Rust 底层原生 API 实现跨平台无边框透明窗口与穿透检测
* **衍生说明**：本项目核心聚焦于前端桌面端宿主改造，核心后端逻辑沿用原开源项目规范

---

## Star History

---

## 开源协议与致谢

* 原项目：[Project N.E.K.O.](https://github.com/Project-N-E-K-O/N.E-K.O)
* 原项目作者：Hongzhi Wen
* 本项目代码及原项目衍生部分均遵循 [MIT License](https://www.google.com/search?q=./LICENSE)。
* 详细的衍生代码与第三方组件授权说明请参考 [NOTICE.md](https://www.google.com/search?q=./NOTICE.md)。

```

---

**一键更新命令**：

在终端中执行以下命令，直接覆盖并推送到 GitHub：

```bash
cat << 'EOF' > README.md
# N.E.K.O. Tauri

Project N.E.K.O. 的非官方 Tauri 桌面移植版。

本项目基于开源的 [Project N.E.K.O.](https://github.com/Project-N-E-K-O/N.E-K.O) 进行开发，采用 **Tauri + React + TypeScript** 架构重构桌面端窗口与交互实现，提供更轻量的桌面端运行方案。

> **免责声明**：本项目为社区维护的**非官方移植版（Unofficial Port）**，与 Project N.E.K.O. 官方团队无任何隶属、授权或直接关联。

---

## 移植特性

* **轻量化桌面封装**：基于 Tauri 架构构建，大幅缩减桌面运行时内存与体积占用。
* **原生窗口与穿透适配**：深度适配 Windows / macOS 桌面透明置顶、动态鼠标穿透与拖拽交互。
* **核心功能保留**：保留多模态对话、屏幕理解、多形态 Avatar（Live2D / VRM / MMD）与本地/云端模型接入能力。

---

## 快速开始

### Windows / macOS 用户（一键包）

解压 Release 发布的压缩包后，直接运行 `N.E.K.O.exe` 或 `N.E.K.O.app` 即可启动。

> **macOS 用户注意**：如首次打开提示已损坏或无法验证开发者，请在终端执行以下命令解除系统隔离：
> ```bash
> xattr -cr /Applications/N.E.K.O.app
> ```

<details>
<summary>Docker 部署 (Linux)</summary>

### Docker 运行方式
```bash
docker build -t neko-tauri .
docker run -d -p 3000:3000 --name neko-app neko-tauri

```

### 环境要求

* Node.js (>= 18)
* Rust & Cargo (最新稳定版)
* 平台编译工具链（Xcode Command Line Tools / Visual Studio C++ Build Tools）

### 启动调试

```bash
# 安装依赖
npm install

# 启动 Tauri 开发环境
npm run tauri dev

```

### 构建打包

```bash
npm run tauri build

```

构建产物生成在 `src-tauri/target/release/bundle/`。

* **模型接入**：支持配置 OpenAI / Gemini / Qwen / DeepSeek / Ollama 等兼容接口。
* **快捷键与交互**：支持通过快捷键快速切换鼠标穿透、锁定窗口位置与调出控制面板。
* **Live2D / VRM 自定义**：可将模型文件放置于 `assets/models` 目录下进行加载。

* **技术架构**：Tauri 2 + React + TypeScript + Rust Backend
* **窗口行为**：通过 Rust 底层原生 API 实现跨平台无边框透明窗口与穿透检测
* **衍生说明**：本项目核心聚焦于前端桌面端宿主改造，核心后端逻辑沿用原开源项目规范

---

## Star History

---

## 开源协议与致谢

* 原项目：[Project N.E.K.O.](https://github.com/Project-N-E-K-O/N.E-K.O)
* 原项目作者：Hongzhi Wen
* 本项目代码及原项目衍生部分均遵循 [MIT License](https://www.google.com/search?q=./LICENSE)。
* 详细的衍生代码与第三方组件授权说明请参考 [NOTICE.md](https://www.google.com/search?q=./NOTICE.md)。
EOF

git add README.md
git commit -m "docs: update README sections"
git push origin main

```

```
