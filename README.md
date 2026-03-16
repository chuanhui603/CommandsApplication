# Command Mindmap Executor

[繁體中文架構文件](README.zh-TW.md)

Command Mindmap Executor is a local-first desktop app for organizing shell commands as a visual mindmap, then exporting reusable JSON snapshots.

## What it does

- Build command flows with a tree view and graph view.
- Edit command nodes and parameters in an inspector panel.
- Autosave to local SQLite through Tauri.
- Import and export mindmaps as JSON files.
- Keep everything local on your machine (no required cloud service).

## Product structure

The app is organized around three user-facing layers:

1. **Editor**: create and connect command nodes visually.
2. **Template + Validation**: map nodes to reusable templates and check readiness.
3. **Output + Exchange**: save snapshots, generate output, and move data with JSON import/export.

## Tech stack

- Frontend: Vue 3 + TypeScript
- Desktop runtime: Tauri
- Local data store: SQLite

## Quick start

### Prerequisites

- Node.js 20+
- Rust toolchain
- Tauri build prerequisites for your OS

### Run in development

```bash
npm install
npm run tauri:dev
```

### Build desktop app

```bash
npm run tauri:build
```

## Basic usage

1. Launch the app.
2. Add command nodes from the tree panel or canvas.
3. Connect nodes in graph view and select active path.
4. Edit node details in the inspector.
5. Use **Export JSON** / **Import JSON** for sharing and migration.

## Documentation

- Product architecture and command contract details: `README.zh-TW.md`
