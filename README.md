# UNPM

> **UN**ified **P**ackage **M**anager for the Node.js ecosystem, a.k.a. "unpm" (pronounced "un-pm").

[English](#unpm) | [中文](#中文版)

`unpm` is a unified command-line wrapper for Node.js package managers.
Use a single command style across npm, pnpm, yarn, yarn-berry, and bun.

It also provides `unpx`, an alias for `unpm dlx`.

Inspired by [unjs/nypm](https://github.com/unjs/nypm) and [antfu/ni](https://github.com/antfu-collective/ni).

This Rust edition is a rewrite of the TypeScript project [GoodbyeNJN/unpm](https://github.com/GoodbyeNJN/unpm), with some behavior and implementation differences.

---

## Features

- Auto-detects the package manager from lockfiles for local project commands
- Provides one unified command surface across supported package managers
- Forwards unknown options and arguments to the underlying package manager
- Supports JSON config for package manager, runner, and registry defaults
- Supports `--dry-run` to preview resolved commands

## Supported package managers

| Package manager | Identifier   |
| --------------- | ------------ |
| npm             | `npm`        |
| pnpm            | `pnpm`       |
| yarn (v1)       | `yarn`       |
| yarn (v2+)      | `yarn-berry` |
| bun             | `bun`        |

## Installation

Build and install the binaries into Cargo's bin directory:

```bash
./scripts/prepare.sh # initialize submodules and apply patches
cargo build --release
cargo install --path ./
```

Run installed binaries:

```bash
unpm
unpx tsx --version
```

## Usage

```text
unpm [--config <path>] [--pm <package-manager>] [--runner <script-runner>] [--dry-run] <command> [arguments]
unpx [--config <path>] [--pm <package-manager>] [--runner <script-runner>] [--dry-run] <command> [arguments]
```

## Global options

| Option                     | Description                              |
| -------------------------- | ---------------------------------------- |
| `-c, --config <path>`      | Path to a config file                    |
| `--pm <package-manager>`   | Override resolved package manager        |
| `--runner <script-runner>` | Override the script runner for `run`     |
| `--dry-run`                | Print resolved command without executing |

## Pass-through arguments

For most commands, unknown options and arguments are forwarded to the underlying package manager.
Use `--` only when you need to disambiguate options that conflict with `unpm` options.

```bash
unpm add --optional --os linux -- lodash
```

## Commands

### `add` (alias: `a`)

Add packages to the project or global scope.

```bash
unpm add <package...> [options]
```

| Option           | Description                     |
| ---------------- | ------------------------------- |
| `-g, --global`   | Add packages globally           |
| `-d, --dev`      | Add as development dependencies |
| `-o, --optional` | Add as optional dependencies    |
| `-p, --peer`     | Add as peer dependencies        |
| `-e, --exact`    | Use exact versions              |

```bash
unpm add -d lodash
unpm --dry-run add lodash
unpm add -o --os linux lodash
unpm add --dev -- --dev lodash
```

### `remove` (alias: `rm`)

Remove packages from the project or global scope.

```bash
unpm remove <package...> [options]
```

| Option         | Description              |
| -------------- | ------------------------ |
| `-g, --global` | Remove packages globally |

```bash
unpm remove -g tsx
unpm --dry-run remove lodash
unpm remove --dry-run lodash
```

### `install` (alias: `i`)

Install project dependencies.

```bash
unpm install [options]
```

| Option              | Description                          |
| ------------------- | ------------------------------------ |
| `-p, --prod`        | Install production dependencies only |
| `-O, --no-optional` | Exclude optional dependencies        |
| `-P, --no-peer`     | Exclude peer dependencies            |
| `--frozen`          | Do not update the lockfile           |

```bash
unpm install -p --frozen
unpm install --os linux
unpm install -- --no-optional
```

### `update` (alias: `up`)

Update installed packages.

```bash
unpm update [package...] [options]
```

| Option              | Description                          |
| ------------------- | ------------------------------------ |
| `-g, --global`      | Update packages globally             |
| `-p, --prod`        | Update production dependencies only  |
| `-d, --dev`         | Update development dependencies only |
| `-O, --no-optional` | Exclude optional dependencies        |
| `-P, --no-peer`     | Exclude peer dependencies            |
| `-i, --interactive` | Use interactive mode                 |

```bash
unpm update lodash
unpm --dry-run update -i
unpm update --recursive
```

### `list` (alias: `ls`)

List installed packages.

```bash
unpm list [package...] [options]
```

| Option              | Description                        |
| ------------------- | ---------------------------------- |
| `-g, --global`      | List globally installed packages   |
| `-p, --prod`        | List production dependencies only  |
| `-d, --dev`         | List development dependencies only |
| `-O, --no-optional` | Exclude optional dependencies      |
| `-P, --no-peer`     | Exclude peer dependencies          |

```bash
unpm list -O -P
unpm list --json
```

### `exec` (alias: `x`)

Execute a command from installed packages.

```bash
unpm exec <command> [arguments...]
```

```bash
unpm exec tsx --no-cache hello.ts
unpm --dry-run exec -- tsx --no-cache hello.ts
```

### `dlx`

Execute a package command without prior installation.
`unpx` is a direct alias of this command.

```bash
unpm dlx <command> [arguments...]
unpx <command> [arguments...]
```

| Option            | Description                       |
| ----------------- | --------------------------------- |
| `-G, --no-global` | Do not use global package manager |

```bash
unpm dlx -G tsx --no-cache hello.ts
unpm --dry-run dlx -- tsx --no-cache hello.ts
unpx tsx --no-cache hello.ts
```

### `run`

Run scripts from `package.json`.

```bash
unpm run <script> [arguments...]
```

```bash
unpm run build
unpm --dry-run run -- build --watch
```

### `pm`

Forward arguments directly to the resolved package manager.

```bash
unpm pm [arguments...]
```

| Option         | Description                |
| -------------- | -------------------------- |
| `-g, --global` | Use global package manager |

```bash
unpm pm --version
unpm pm -g -- --version
```

### `complete`

Generate shell completion scripts for `unpm` and `unpx`.

```bash
unpm complete <shell> [--unpm <name>] [--unpx <name>]
```

```bash
unpm complete bash >> ~/.bashrc
unpm complete zsh >> ~/.zshrc
unpm complete zsh --unpm my-unpm --unpx my-unpx >> ~/.zshrc
unpm complete fish >> ~/.config/fish/completions/unpm.fish
```

## Configuration

`unpm` supports a JSON config file for package manager defaults, runner selection, and registry URL.

Supported keys:

- `pm.local`: package manager for local project commands
- `pm.global`: package manager for global operations
- `runner`: script runner for `run`
- `registry`: npm registry base URL

Supported values:

- `pm.local` / `pm.global`: `npm`, `pnpm`, `yarn`, `yarn-berry`, `bun`
- `runner`: `node`
- `registry`: any valid URL (for example `https://registry.npmjs.org` or `https://registry.npmmirror.com`)

Example `config.json`:

```json
{
    "pm": {
        "local": "pnpm",
        "global": "npm"
    },
    "runner": "node",
    "registry": "https://registry.npmjs.org"
}
```

## Resolution order

Config file loading order:

1. `--config <PATH>`
2. `UNPM_CONFIG_FILE`
3. default platform config path (`config.json`)

Default config file path by platform:

- Linux: `~/.config/unpm/config.json`
- macOS: `~/Library/Application Support/unpm/config.json`
- Windows: `%APPDATA%\\org\\unpm\\unpm\\config\\config.json`

Package manager resolution:

- Local commands: `--pm` -> auto-detect -> `pm.local`
- Global commands: `--pm` -> `pm.global`

If the package manager is still undetermined and running in an interactive terminal, `unpm` prompts for selection.

---

## Contributor Guide

### Setup

This repository includes a local `clap` crate under `crates/clap`.
Before building, initialize submodules and apply required patches:

```bash
./scripts/prepare.sh
```

### Project structure

- `src/lib.rs`: crate entrypoint and exports
- `src/cli/`: CLI parsing and subcommands
- `src/cli/main/unpm.rs`: `unpm` command processor
- `src/cli/main/unpx.rs`: `unpx` alias processor
- `src/cli/sub/*.rs`: individual subcommand handlers
- `src/cli/completion.rs`: completion adapter logic
- `src/context/config.rs`: config loading and parsing
- `src/package_manager/`: package manager resolution and adaptors

### Testing and development

- Use `cargo test` for the full suite
- Use `cargo test --lib` for library-level checks
- Keep parser behavior stable and add regression tests for CLI changes

---

## 中文版

[返回 English](#unpm)

> **UN**ified **P**ackage **M**anager for the Node.js ecosystem，也就是 “unpm”（读作 “un-pm”）。

`unpm` 是一个统一的 Node.js 包管理器命令行封装。
无论项目使用 npm、pnpm、yarn、yarn-berry 还是 bun，都可以使用同一套命令。

同时提供 `unpx`，它是 `unpm dlx` 的别名。

灵感来源于 [unjs/nypm](https://github.com/unjs/nypm) 与 [antfu/ni](https://github.com/antfu-collective/ni)。

本 Rust 版本重写自 TypeScript 项目 [GoodbyeNJN/unpm](https://github.com/GoodbyeNJN/unpm)，在行为和实现上有一些差异。

## 特性

- 对本地项目命令，可根据 lockfile 自动检测包管理器
- 在多种包管理器上提供统一命令接口
- 未知选项和参数会透传到底层包管理器
- 支持 JSON 配置默认包管理器、runner 与 registry
- 支持 `--dry-run` 预览最终执行命令

## 支持的包管理器

| 包管理器   | 标识符       |
| ---------- | ------------ |
| npm        | `npm`        |
| pnpm       | `pnpm`       |
| yarn (v1)  | `yarn`       |
| yarn (v2+) | `yarn-berry` |
| bun        | `bun`        |

## 安装

构建并安装二进制到 Cargo 的 bin 目录：

```bash
./scripts/prepare.sh # 初始化子模块并应用补丁
cargo build --release
cargo install --path ./
```

运行已安装命令：

```bash
unpm
unpx tsx --version
```

## 用法

```text
unpm [--config <path>] [--pm <package-manager>] [--runner <script-runner>] [--dry-run] <command> [arguments]
unpx [--config <path>] [--pm <package-manager>] [--runner <script-runner>] [--dry-run] <command> [arguments]
```

## 全局选项

| 选项                       | 说明                        |
| -------------------------- | --------------------------- |
| `-c, --config <path>`      | 配置文件路径                |
| `--pm <package-manager>`   | 覆盖解析后的包管理器        |
| `--runner <script-runner>` | 覆盖 `run` 使用的脚本运行器 |
| `--dry-run`                | 打印命令但不执行            |

## 参数透传

大多数命令里，未知选项和参数会透传到底层包管理器。
当参数与 `unpm` 自身选项冲突时，使用 `--` 做分隔。

```bash
unpm add --optional --os linux -- lodash
```

## 命令

### `add`（别名：`a`）

向项目或全局安装依赖。

```bash
unpm add <package...> [options]
```

| 选项             | 说明               |
| ---------------- | ------------------ |
| `-g, --global`   | 全局安装           |
| `-d, --dev`      | 作为开发依赖安装   |
| `-o, --optional` | 作为可选依赖安装   |
| `-p, --peer`     | 作为 peer 依赖安装 |
| `-e, --exact`    | 使用精确版本       |

```bash
unpm add -d lodash
unpm --dry-run add lodash
unpm add -o --os linux lodash
unpm add --dev -- --dev lodash
```

### `remove`（别名：`rm`）

从项目或全局移除依赖。

```bash
unpm remove <package...> [options]
```

| 选项           | 说明     |
| -------------- | -------- |
| `-g, --global` | 全局移除 |

```bash
unpm remove -g tsx
unpm --dry-run remove lodash
unpm remove --dry-run lodash
```

### `install`（别名：`i`）

安装项目依赖。

```bash
unpm install [options]
```

| 选项                | 说明            |
| ------------------- | --------------- |
| `-p, --prod`        | 仅安装生产依赖  |
| `-O, --no-optional` | 排除可选依赖    |
| `-P, --no-peer`     | 排除 peer 依赖  |
| `--frozen`          | 不更新 lockfile |

```bash
unpm install -p --frozen
unpm install --os linux
unpm install -- --no-optional
```

### `update`（别名：`up`）

更新已安装依赖。

```bash
unpm update [package...] [options]
```

| 选项                | 说明           |
| ------------------- | -------------- |
| `-g, --global`      | 全局更新       |
| `-p, --prod`        | 仅更新生产依赖 |
| `-d, --dev`         | 仅更新开发依赖 |
| `-O, --no-optional` | 排除可选依赖   |
| `-P, --no-peer`     | 排除 peer 依赖 |
| `-i, --interactive` | 交互模式       |

```bash
unpm update lodash
unpm --dry-run update -i
unpm update --recursive
```

### `list`（别名：`ls`）

列出已安装依赖。

```bash
unpm list [package...] [options]
```

| 选项                | 说明             |
| ------------------- | ---------------- |
| `-g, --global`      | 列出全局安装依赖 |
| `-p, --prod`        | 仅列出生产依赖   |
| `-d, --dev`         | 仅列出开发依赖   |
| `-O, --no-optional` | 排除可选依赖     |
| `-P, --no-peer`     | 排除 peer 依赖   |

```bash
unpm list -O -P
unpm list --json
```

### `exec`（别名：`x`）

执行已安装包提供的命令。

```bash
unpm exec <command> [arguments...]
```

```bash
unpm exec tsx --no-cache hello.ts
unpm --dry-run exec -- tsx --no-cache hello.ts
```

### `dlx`

无需预先安装，直接执行包命令。
`unpx` 是该命令的直接别名。

```bash
unpm dlx <command> [arguments...]
unpx <command> [arguments...]
```

| 选项              | 说明               |
| ----------------- | ------------------ |
| `-G, --no-global` | 不使用全局包管理器 |

```bash
unpm dlx -G tsx --no-cache hello.ts
unpm --dry-run dlx -- tsx --no-cache hello.ts
unpx tsx --no-cache hello.ts
```

### `run`

运行 `package.json` 中定义的脚本。

```bash
unpm run <script> [arguments...]
```

```bash
unpm run build
unpm --dry-run run -- build --watch
```

### `pm`

将参数直接转发给解析后的包管理器。

```bash
unpm pm [arguments...]
```

| 选项           | 说明             |
| -------------- | ---------------- |
| `-g, --global` | 使用全局包管理器 |

```bash
unpm pm --version
unpm pm -g -- --version
```

### `complete`

为 `unpm` 和 `unpx` 生成 shell 补全脚本。

```bash
unpm complete <shell> [--unpm <name>] [--unpx <name>]
```

```bash
unpm complete bash >> ~/.bashrc
unpm complete zsh >> ~/.zshrc
unpm complete zsh --unpm my-unpm --unpx my-unpx >> ~/.zshrc
unpm complete fish >> ~/.config/fish/completions/unpm.fish
```

## 配置

`unpm` 支持 JSON 配置文件，用于设置默认包管理器、runner 和 registry。

支持的键：

- `pm.local`：本地项目命令使用的包管理器
- `pm.global`：全局操作使用的包管理器
- `runner`：`run` 使用的脚本运行器
- `registry`：npm registry 基础 URL

支持的值：

- `pm.local` / `pm.global`：`npm`、`pnpm`、`yarn`、`yarn-berry`、`bun`
- `runner`：`node`
- `registry`：任意合法 URL（例如 `https://registry.npmjs.org` 或 `https://registry.npmmirror.com`）

示例 `config.json`：

```json
{
    "pm": {
        "local": "pnpm",
        "global": "npm"
    },
    "runner": "node",
    "registry": "https://registry.npmjs.org"
}
```

## 解析顺序

配置文件加载顺序：

1. `--config <PATH>`
2. `UNPM_CONFIG_FILE`
3. 平台默认配置路径（`config.json`）

三大平台默认配置路径：

- Linux：`~/.config/unpm/config.json`
- macOS：`~/Library/Application Support/unpm/config.json`
- Windows：`%APPDATA%\\org\\unpm\\unpm\\config\\config.json`

包管理器解析顺序：

- 本地命令：`--pm` -> 自动检测 -> `pm.local`
- 全局命令：`--pm` -> `pm.global`

如果仍无法确定包管理器，且当前运行在交互式终端中，`unpm` 会提示选择。

---

## 贡献者指南

### 环境准备

本仓库包含本地 `clap` crate（位于 `crates/clap`）。
构建前请先初始化子模块并应用补丁：

```bash
./scripts/prepare.sh
```

### 项目结构

- `src/lib.rs`：crate 入口与导出
- `src/cli/`：CLI 解析与子命令
- `src/cli/main/unpm.rs`：`unpm` 主命令处理
- `src/cli/main/unpx.rs`：`unpx` 别名命令处理
- `src/cli/sub/*.rs`：各子命令处理逻辑
- `src/cli/completion.rs`：补全适配逻辑
- `src/context/config.rs`：配置加载与解析
- `src/package_manager/`：包管理器解析与适配层

### 测试与开发

- 全量测试：`cargo test`
- 仅库测试：`cargo test --lib`
- 变更 CLI 解析行为时，请补充回归测试
