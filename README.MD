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

Project N.E.K.O. 的非官方 Tauri 桌面移植版（macOS 专用）。

本项目基于开源的 [Project N.E.K.O.](https://github.com/Project-N-E-K-O/N.E-K.O) 进行开发，采用 **Tauri + React + TypeScript** 架构重构桌面端窗口与交互实现。

> 免责声明：本项目为社区维护的 非官方移植版（Unofficial Port） ，仅供学习与交流使用，与 Project N.E.K.O. 官方团队无任何隶属、授权或直接关联。

---

## 移植特性

* 轻量化桌面封装：基于 Tauri 构建，优化桌面端资源占用与渲染开销。
* macOS 窗口穿透与置顶：适配 macOS 桌面透明置顶、动态鼠标穿透与拖拽交互。
* 双进程架构：Python 后端驱动多模态交互，Tauri 负责原生桌面渲染。

---

## 运行环境要求

* 系统：macOS
* Node.js (>= 18)
* Python (3.10+)
* Rust & Cargo (最新稳定版)

---

## 本地启动指南

本项目需要分别启动 Python 后端服务与 Tauri 前端客户端：

### 终端 1：启动 Python 后端
在项目根目录下打开终端，激活虚拟环境并运行后端：
```bash
source .venv/bin/activate
python3 launcher.py
终端 2：启动 Tauri 客户端
新建另一个终端窗口，进入 neko-tauri 目录启动客户端：
Bash
cd neko-tauri
npm run tauri dev
开源协议与致谢
原项目：Project N.E.K.O.
原项目作者：Hongzhi Wen
本项目代码及原项目衍生部分遵循 MIT License。
详细说明请查看 NOTICE.md。

---

