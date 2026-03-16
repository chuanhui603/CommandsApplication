# Product Architecture Overview

This document explains the product structure of Command Mindmap Executor at a high level.

It is meant for readers who want to understand how the application is organized without reading implementation code.

## Product Layers

The first release is organized around three main layers.

### 1. Editor Layer

This is where users create and arrange command workflows.

- Tree View for readable hierarchy
- Graph View for free-form connections
- Inspector editing for node details and parameters
- Active-path selection for final output

### 2. Template and Validation Layer

This layer makes command pieces reusable and keeps output predictable.

- Built-in templates for common command categories
- User-defined templates for custom workflows
- Platform-scoped templates for Linux shell, WSL, and Windows PowerShell
- Validation before final generation

### 3. Persistence and Exchange Layer

This layer keeps data local and portable.

- Local SQLite persistence
- Autosave after meaningful edits
- JSON export for mindmaps
- JSON export for template bundles
- JSON import for migration between machines

## Runtime Structure

The product is designed as a desktop application:

- Vue for the interface
- Tauri for the desktop runtime
- SQLite for local data storage

This gives the application a web-style interface while still supporting local files and local persistence.

## Why Two Visual Modes Exist

The application uses both Tree View and Graph View because they solve different user needs.

- Tree View makes command order easier to read
- Graph View makes relationships and experimentation easier to manage

Both views reflect the same underlying command graph.

## Output Model

The first release generates output from one active path at a time.

This means:

- a mindmap may contain multiple branches
- only one selected path is used for final output
- unused branches can still stay in the graph for reuse later

## Local-First Data Model

The product is intentionally local-first.

- data is saved on the user's machine
- cloud services are not required
- JSON files are used for backup and movement between hosts

## Related References

- [../reference/tauri-command-contracts.md](../reference/tauri-command-contracts.md)
- [../reference/sqlite-schema.md](../reference/sqlite-schema.md)