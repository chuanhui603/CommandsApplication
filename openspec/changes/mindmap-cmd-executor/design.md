## Context

This change introduces a new local-first command-management application for users who build and reuse shell commands visually instead of maintaining ad hoc notes and scripts. The system must support Linux shell, WSL, and Windows PowerShell command sets, while keeping the editing experience centered on one shared graph model that can be rendered as both a tree and a free-form graph.

The requested implementation direction is now explicit: a Vue front end packaged inside a Tauri desktop application with SQLite-backed local persistence. Tauri is used to provide desktop packaging, local file access, and a narrow native bridge while keeping the majority of product logic in Vue and TypeScript. The design therefore treats Rust as infrastructure glue, not as the place where core editor or generation behavior should live.

## Goals / Non-Goals

**Goals:**
- Provide a Vue visual editor for command mindmaps with both Tree View and Graph View over one shared content model.
- Store mindmaps, templates, generation metadata, and validation-relevant state in a structured local-first format.
- Package the application as a Tauri desktop app with a minimal native bridge and SQLite local storage.
- Support built-in read-only templates plus user-defined templates classified by platform type: Linux shell, WSL, and Windows PowerShell.
- Generate either a final command string or a script output from one active output path.
- Support JSON import/export for complete mindmaps and for template-library bundles, including bulk operations.
- Define validator behavior as an independent capability so future changes can evolve generation rules without rewriting unrelated specs.

**Non-Goals:**
- Direct execution of generated commands.
- Multi-user collaboration or cloud synchronization.
- Windows cmd.exe support in the first release.
- Full version history or rollback of all graph edits.
- Markdown files as the primary persistence model.
- A large Rust domain layer that duplicates editor, validator, or generation logic already owned by Vue and TypeScript.

## Decisions

### 1. Use one graph content model with two view-specific layouts

The editor SHALL maintain a single underlying command graph and expose it as Tree View and Graph View. Node content, edges, active path state, and generation semantics remain shared. View-specific positions are stored separately so users can optimize readability in each visualization without causing layout conflicts.

Alternatives considered:
- One shared coordinate system for both views: rejected because tree layout and free-form graph layout optimize for different reading patterns and would constantly overwrite each other.
- Two fully separate editors and storage models: rejected because it duplicates logic and makes import/export, validation, and generation harder to keep consistent.

### 2. Model nodes as single command fragments referencing templates

Each command node represents one command fragment, not an entire script block. Nodes reference templates and supply parameter values, inclusion state, notes, and optional ordering overrides. This keeps generation predictable and allows validation to reason about missing parameters or incompatible platform types.

Alternatives considered:
- Allow arbitrary script bodies inside nodes: rejected because it collapses the graph into unmanaged text blobs and weakens validation.
- Represent parameters as separate graph nodes: rejected for the first release because it significantly increases UI and data-model complexity.

### 3. Limit output generation to one active path per build

The graph may contain multiple branches, but the system SHALL generate output from only one active path at a time. Additional branches may remain in the graph as alternatives, dependencies, or disabled fragments. This keeps final output deterministic while preserving exploratory modeling.

Alternatives considered:
- Flatten every branch into one command: rejected because branch meaning becomes ambiguous and often incorrect.
- Generate one output per branch by default: rejected for the first release because it complicates UI, export semantics, and validation scope.

### 4. Split import/export into two JSON contracts

The system SHALL define separate JSON formats for mindmaps and template libraries. Mindmap exports may include referenced templates required to reconstruct the graph on another machine. Template-library imports and exports support bulk transport of built-in clones and user-defined templates. Import conflicts are blocked instead of auto-merged or auto-renamed.

Alternatives considered:
- A single universal JSON package for every entity: rejected because it blurs import semantics and makes validation less predictable.
- Automatic rename or overwrite during import: rejected because it hides conflicts and creates hard-to-audit local state.

### 5. Keep built-in templates read-only and extend through cloning

Built-in templates SHALL be immutable. Users who want to customize them must clone them into the user template library and edit the clone. This preserves a stable baseline for documentation, validation, and future upgrades.

Alternatives considered:
- Allow direct editing of built-in templates: rejected because it makes upgrades and support harder and blurs the line between platform defaults and user customizations.

### 6. Treat platform type as explicit template metadata

Templates SHALL declare one platform kind: Linux shell, WSL, or Windows PowerShell. Command generation validates compatibility between the current build target and the templates used by nodes on the active path. This is simpler and more predictable than attempting broad automatic shell translation in the first release.

Alternatives considered:
- Fully automatic shell translation from one abstract template: rejected because quoting, paths, flags, and shell semantics vary enough to create misleading output.
- Store separate inline command strings directly in each node: rejected because it weakens template reuse and template-library import/export.

### 7. Use Tauri with SQLite as the canonical runtime and persistence model

The application SHALL be delivered as a Tauri desktop app. Vue and TypeScript own the UI, state management, graph editing, generation orchestration, and validator presentation. Tauri provides the native shell, application packaging, file-system bridge, and invocation boundary to SQLite-backed storage. Rust code SHALL stay minimal and focused on native capabilities, schema initialization, and well-scoped command handlers.

Alternatives considered:
- Vue plus a separate local API service: rejected for the initial design because it adds another moving process, another startup dependency, and more deployment complexity for a single-user desktop tool.
- Markdown files as the primary data store: rejected because graph topology, validation state, template libraries, and bulk import/export are all awkward to represent, diff, and validate in Markdown frontmatter and nested documents.
- IndexedDB-only as the long-term canonical store: rejected because the requested cross-machine data model and future local-runtime portability fit SQLite better.

### 8. Keep Rust small and place most product logic in TypeScript

The native layer SHALL expose a constrained set of commands such as database initialization, CRUD-oriented storage operations, import/export file selection, and future native integrations. Domain rules such as active-path resolution, template selection, preview generation, and validator result interpretation SHALL remain in TypeScript unless a native implementation becomes necessary later.

Alternatives considered:
- Put validator and generation logic in Rust from the start: rejected because it would raise the implementation barrier, duplicate domain logic, and make ordinary feature iteration harder for a Vue-focused codebase.
- Use Rust only as an opaque SQLite proxy with large ad hoc APIs: rejected because that tends to create an unstable bridge contract and pushes avoidable complexity into native code.

### 9. Prefer established packages for graph editing and SQLite integration

The implementation SHALL prefer maintained ecosystem packages over custom infrastructure for the free-graph editor and SQLite bridge. The current recommended direction is a Vue-native graph editor package for Graph View and an established Tauri SQLite integration package or plugin for the local database layer. Custom implementations should be introduced only if package limitations materially block required product behavior.

Alternatives considered:
- Build a custom graph editor from scratch: rejected because drag, zoom, pan, edge rendering, and node selection are already solved problems and would slow delivery.
- Write a fully custom Rust-side SQLite bridge before evaluating maintained plugins: rejected because it increases native complexity without product benefit in the first release.

### 10. Use autosave as the default persistence behavior

The editor SHALL autosave after meaningful edits to the graph, templates, and related metadata. Autosave should be debounced and transactional so the user does not need to manually manage save cycles during normal editing.

Alternatives considered:
- Manual save only: rejected because this product is built around iterative visual editing and frequent small changes.
- Immediate save on every keystroke without debouncing: rejected because it would create unnecessary write pressure and noisier failure modes.

### 11. Allow Markdown only as a derived or auxiliary export surface

Markdown may be added later as a readable export for documentation, notes, or sharing, but it SHALL NOT be the canonical source of truth. If introduced, Markdown exports should be generated from structured graph and template data, not parsed as the primary editing store.

Alternatives considered:
- Primary Markdown authoring with parsers feeding the app: rejected because it complicates round-tripping and makes visual editing fragile.

## Risks / Trade-offs

- [Tauri introduces a Rust layer the user is not familiar with] -> Mitigation: keep Rust limited to a documented command boundary, with product logic and most tests living in TypeScript.
- [Desktop packaging adds release and signing complexity] -> Mitigation: keep the initial runtime local-first and single-user, and document packaging as a separate delivery concern from core features.
- [Autosave can hide persistence failures until later] -> Mitigation: expose last-saved state in the UI, debounce writes, and surface save failures immediately.
- [Platform-specific templates can duplicate similar commands] -> Mitigation: support cloning, bulk template import/export, and consistent template categories to keep duplication manageable.
- [Single active path can feel restrictive in complex graphs] -> Mitigation: preserve alternative branches in the graph and allow future expansion through the separate validator and generation specs.
- [Blocking imports on name or id conflict adds manual work] -> Mitigation: return explicit conflict diagnostics so the user can fix JSON intentionally before retrying.
- [No primary Markdown storage reduces human-editable portability] -> Mitigation: keep JSON import/export simple and add Markdown export later if documentation use cases become important.

## Migration Plan

Because this is a new capability set, there is no existing runtime migration. The initial implementation should:

1. Create the Vue application shell and storage abstraction.
2. Create the Tauri desktop shell and define the minimal Rust command surface for storage and file interactions.
3. Define the SQLite schema for graphs, nodes, edges, templates, build profiles, and build results.
4. Integrate the selected graph editing package and SQLite package/plugin into the runtime boundary.
5. Implement autosave with debounced snapshot persistence and save-status feedback.
6. Implement JSON import/export contracts for mindmaps and template bundles.
7. Add validator and generation logic before wiring full editor interactions.
8. Ship built-in base template categories and allow user-owned AI command-set templates as custom imports or custom creations, not as defaults.

Rollback is file-level removal of the new application code and change artifacts because no previous production interface exists.

## Open Questions

- Whether AI-related command-set templates should be distributed as example import bundles within the repository or generated by an in-app starter wizard.
- Whether validator output in the first release should expose only blocking errors and warnings or also informational notices.