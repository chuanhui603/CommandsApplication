## 1. Application Foundation

- [x] 1.1 Create the Vue application shell inside a Tauri desktop project for the command mindmap editor
- [x] 1.2 Define the domain models for mindmaps, nodes, edges, templates, build profiles, build results, and validator diagnostics
- [x] 1.3 Define the Tauri command boundary so Rust only handles native capabilities, SQLite access, and file-dialog workflows
- [x] 1.4 Implement the SQLite-backed persistence adapter used by the Tauri runtime
- [x] 1.5 Integrate a maintained graph-editor package for Graph View and a maintained Tauri SQLite package or plugin for storage access

## 2. Mindmap Editing Experience

- [x] 2.1 Implement shared graph state with separate Tree View and Graph View layout persistence
- [x] 2.2 Add node creation through toolbox drag-and-drop and context-menu actions
- [x] 2.3 Add inspector-based editing for node content, template assignment, parameter values, inclusion state, and active-path selection
- [x] 2.4 Implement debounced autosave for meaningful graph edits with visible save-status feedback

## 3. Template Library

- [x] 3.1 Implement built-in read-only template support and clone-to-user-library behavior
- [x] 3.2 Implement custom template creation and editing with platform-kind selection for Linux shell, WSL, and Windows PowerShell
- [x] 3.3 Seed the built-in library with the initial six categories: files and directories, search and filtering, Git, Docker, compression, and networking
- [x] 3.4 Support user-defined AI command-set templates for GitHub Copilot, Claude, and OpenSpec without shipping them as built-in defaults

## 4. Import / Export

- [x] 4.1 Define and implement the dedicated JSON format for mindmap import/export, including referenced-template packaging
- [x] 4.2 Define and implement the dedicated JSON format for bulk template-library import/export
- [x] 4.3 Implement strict conflict blocking and diagnostic reporting for mindmap and template-library imports

## 5. Generation and Validation

- [x] 5.1 Implement validator rules for root-node presence, active-path validity, cycle detection, required parameters, target compatibility, and import conflicts
- [x] 5.2 Implement live preview of generated output for the selected target and output mode
- [x] 5.3 Implement explicit generation of final command output and script output with persisted last-result metadata

## 6. Persistence and Quality

- [x] 6.1 Persist current version metadata, last generated result, and updated timestamps for each mindmap
- [x] 6.2 Add tests covering import formats, validation failures, active-path generation, template-platform compatibility, and built-in template cloning
- [x] 6.3 Document the Tauri architecture, including which responsibilities stay in Vue/TypeScript and which are handled by the Rust bridge
