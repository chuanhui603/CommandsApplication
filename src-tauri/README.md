# Tauri Runtime Notes

This folder contains the desktop runtime layer for the application.

The Tauri side should stay intentionally small and focused on native capabilities.

Primary responsibilities:

- application startup
- SQLite initialization and persistence
- file dialogs for import and export
- file read and write operations
- typed command responses back to Vue

This layer should avoid owning most product logic.

In the first release, Rust should not become the main place for:

- graph-editing rules
- active-path resolution
- template rendering
- validator business rules
- command-preview composition

Detailed request and response contracts:

- [../docs/reference/tauri-command-contracts.md](../docs/reference/tauri-command-contracts.md)

Data model reference:

- [../docs/reference/sqlite-schema.md](../docs/reference/sqlite-schema.md)