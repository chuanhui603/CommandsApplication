# Command Mindmap Executor

[繁體中文說明](README.zh-TW.md)

Command Mindmap Executor is a local-first desktop application for building reusable shell commands with a visual mindmap workflow.

Instead of keeping commands in scattered notes, chat logs, or one-off scripts, you can organize them as reusable nodes, connect them visually, and export the result when needed.

## Highlights

- Build command flows with Tree View and Graph View
- Reuse command templates for Linux shell, WSL, and Windows PowerShell
- Preview generated command or script output before using it
- Save data locally on your machine
- Import and export mindmaps and template bundles as JSON

## Who It Is For

This project is intended for people who frequently work with command-line tools and want a more structured way to organize, reuse, and move command workflows between environments.

Typical use cases include:

- managing personal command libraries
- building repeatable shell workflows
- keeping Linux, WSL, and PowerShell command sets organized
- moving command collections between machines

## How It Works

1. Create a mindmap for a command workflow.
2. Add or reuse command templates.
3. Connect nodes and choose the active output path.
4. Review the generated command or script preview.
5. Save locally or export as JSON.

## Platform Model

The first release is designed around three command targets:

- Linux shell
- WSL
- Windows PowerShell

## Local-First Behavior

The application is designed to keep your data on your own machine.

- local persistence is built in
- cloud synchronization is not required
- import/export is used for backup and moving data between hosts

## Development Quick Start

If you want to run the project locally during development:

```bash
npm install
npm run tauri:dev
```

To build the desktop application:

```bash
npm run tauri:build
```

## Documentation

- Product architecture overview: [docs/architecture/overview.md](docs/architecture/overview.md)
- Tauri runtime command contracts: [docs/reference/tauri-command-contracts.md](docs/reference/tauri-command-contracts.md)
- SQLite schema draft: [docs/reference/sqlite-schema.md](docs/reference/sqlite-schema.md)
