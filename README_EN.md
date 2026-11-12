# Volcengine Agent

A focused desktop manager based on [Codex++](https://github.com/BigPizzaV3/CodexPlusPlus), customized for the `volcengine.org` relay service.

The goal is simple: install the package, configure one Volcengine API key, and start using the bundled Codex CLI immediately.

## Features

- Supports only the `https://api.volcengine.org/` relay API.
- Uses `https://api.volcengine.org/v1` by default and does not expose other relay providers in the UI.
- Bundles the Codex CLI in release installers.
- Builds release packages for Windows, macOS, and Linux.
- Keeps API keys, models, and local settings on the local machine.
- Includes the desktop management and launch features inherited from Codex++.

## Quick start

1. Download the package for your system from [Releases](https://github.com/962468832/volcengine-agent/releases).
2. Install and open Volcengine Agent.
3. Enter your Volcengine API key and choose an available model.
4. Launch the bundled Codex and start coding.

The relay endpoint is fixed to:

```text
https://api.volcengine.org/v1
```

Get API keys, model availability, quota, and pricing from `volcengine.org`.

## Development

The project requires Node.js, Rust, and a Tauri development environment.

```bash
npm ci
npm run check
npm run vite:build
```

Use the Tauri configuration and GitHub Actions workflows for desktop packaging. The release workflow downloads and packages the Codex CLI.

## Sponsor

Thanks to [volcengine.org](https://api.volcengine.org/) for supporting this project.

## License and source

This project is based on [Codex++](https://github.com/BigPizzaV3/CodexPlusPlus) and is distributed under the [AGPL-3.0 license](LICENSE). Third-party notices are available in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Please use this repository's [Issues](https://github.com/962468832/volcengine-agent/issues) for bug reports and include your OS, app version, and redacted logs.
