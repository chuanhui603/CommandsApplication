## Why

Managing shell commands across Linux, WSL, and Windows PowerShell is hard to reuse once command knowledge is scattered across notes, chat logs, and ad hoc scripts. A visual mindmap-based command builder would let users compose reusable command flows, preview final output before use, and move those flows or template libraries between machines with a controlled local-first workflow.

## What Changes

- Add a Vue-based visual web interface for creating and editing command mindmaps with both tree and free-graph views backed by the same graph model.
- Add local-first persistence for mindmaps, template libraries, current version metadata, last generated output, and update timestamps.
- Add command generation that resolves a single active output path into either a final command string or a target-specific script preview.
- Add template library management with built-in read-only templates, user-defined templates, platform classification, and JSON import/export for template bundles.
- Add mindmap JSON import/export for moving complete command graphs between hosts, including required template references.
- Add validation rules as a separate capability so graph correctness and generation constraints can evolve independently over time.
- Add a Tauri desktop runtime so the Vue application can access local SQLite and file-system capabilities without requiring the user to manage a separate local service.

## Capabilities

### New Capabilities
- `command-mindmaps`: Visual creation, editing, storage, and JSON import/export of command mindmaps with shared tree and graph views.
- `command-template-library`: Built-in and custom command templates, platform-scoped template definitions, and template-library JSON import/export.
- `command-generation`: Preview and generation of final command or script output from a selected active path, including local result metadata.
- `validator-rules`: Validation of graph structure, template compatibility, parameter completeness, and generation readiness with a separately evolvable spec.
- `tauri-desktop-runtime`: Desktop packaging, SQLite access, file import/export bridging, and a minimal Rust command boundary for the Vue application.

### Modified Capabilities
- None.

## Impact

- Adds a new Vue front-end application for visual command composition.
- Requires a local persistence strategy for structured graph and template data.
- Introduces JSON exchange formats for mindmaps and template libraries.
- Defines command-generation rules for Linux shell, WSL, and Windows PowerShell.
- Establishes a validation layer that gates generation when required constraints are not met.
- Introduces a Tauri runtime with a constrained Rust bridge for local desktop capabilities.