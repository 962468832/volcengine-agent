# Volcengine Agent

基于 [Codex++](https://github.com/BigPizzaV3/CodexPlusPlus) 定制的桌面管理器，专门连接 `volcengine.org` 中转站。

这个版本的目标很简单：下载安装包后直接使用内置 Codex，配置一次 Volcengine API Key 即可开始编程。

## 主要功能

- 仅支持 `https://api.volcengine.org/` 中转 API。
- 默认接口地址为 `https://api.volcengine.org/v1`，应用界面不会提供其他中转站选项。
- 安装包内置 Codex CLI，安装完成后即可启动使用。
- 支持 Windows、macOS 和 Linux 安装包构建。
- API Key、模型和本地配置保存在本机。
- 基于 Codex++ 的桌面管理、启动和配置能力。

## 快速开始

1. 在 [Releases](https://github.com/962468832/volcengine-agent/releases) 下载对应系统的安装包。
2. 安装并打开 Volcengine Agent。
3. 在连接设置中填写 Volcengine API Key，选择可用模型。
4. 启动内置 Codex，开始使用。

连接地址固定为：

```text
https://api.volcengine.org/v1
```

请从 `volcengine.org` 获取 API Key、模型列表和服务说明。具体可用模型、额度和价格以服务商页面为准。

## 开发构建

需要 Node.js、Rust 和 Tauri 开发环境。

```bash
npm ci
npm run check
npm run vite:build
```

桌面应用构建命令请以仓库中的 Tauri 配置和 GitHub Actions 工作流为准。发布工作流会下载并打包 Codex CLI。

## 赞助商

感谢 [volcengine.org](https://api.volcengine.org/) 对本项目的支持。

## 许可证与来源

本项目基于 [Codex++](https://github.com/BigPizzaV3/CodexPlusPlus) 修改，遵循仓库中的 [AGPL-3.0 许可证](LICENSE)。第三方组件和上游版权信息见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

问题反馈请提交到本仓库的 [Issues](https://github.com/962468832/volcengine-agent/issues)，并附上系统版本、应用版本和已脱敏的日志。
