# SQLite Schema Draft

This document contains the first-pass SQLite schema for the local Tauri runtime.

## Design Goals

- keep local persistence structured and predictable
- support autosave with transactional snapshot writes
- store Tree View and Graph View layouts separately
- keep templates reusable across multiple mindmaps
- keep generated output history available without blocking core editing flows

## Tables

### `mindmaps`

- `id` TEXT PRIMARY KEY
- `name` TEXT NOT NULL
- `description` TEXT
- `root_node_id` TEXT
- `active_path_id` TEXT
- `current_version` INTEGER NOT NULL DEFAULT 1
- `last_build_result_id` TEXT
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

### `mindmap_nodes`

- `id` TEXT PRIMARY KEY
- `mindmap_id` TEXT NOT NULL
- `template_id` TEXT
- `title` TEXT NOT NULL
- `notes` TEXT
- `include_in_output` INTEGER NOT NULL DEFAULT 1
- `order_override` INTEGER
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

### `mindmap_node_params`

- `id` TEXT PRIMARY KEY
- `node_id` TEXT NOT NULL
- `param_key` TEXT NOT NULL
- `param_type` TEXT NOT NULL
- `param_value` TEXT

Expected `param_type` values:

- `text`
- `boolean`
- `single-select`
- `path`

### `mindmap_edges`

- `id` TEXT PRIMARY KEY
- `mindmap_id` TEXT NOT NULL
- `source_node_id` TEXT NOT NULL
- `target_node_id` TEXT NOT NULL
- `edge_type` TEXT NOT NULL
- `priority` INTEGER
- `enabled` INTEGER NOT NULL DEFAULT 1

Expected `edge_type` values:

- `sequence`
- `dependency`

### `mindmap_layouts`

- `id` TEXT PRIMARY KEY
- `mindmap_id` TEXT NOT NULL
- `view_type` TEXT NOT NULL
- `layout_json` TEXT NOT NULL

Expected `view_type` values:

- `tree`
- `graph`

### `templates`

- `id` TEXT PRIMARY KEY
- `name` TEXT NOT NULL
- `description` TEXT
- `platform_kind` TEXT NOT NULL
- `category` TEXT
- `built_in` INTEGER NOT NULL DEFAULT 0
- `command_pattern` TEXT NOT NULL
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

Expected `platform_kind` values:

- `linux-shell`
- `wsl`
- `windows-powershell`

### `template_params`

- `id` TEXT PRIMARY KEY
- `template_id` TEXT NOT NULL
- `param_key` TEXT NOT NULL
- `label` TEXT NOT NULL
- `type` TEXT NOT NULL
- `required` INTEGER NOT NULL DEFAULT 0
- `default_value` TEXT
- `options_json` TEXT

### `build_results`

- `id` TEXT PRIMARY KEY
- `mindmap_id` TEXT NOT NULL
- `target` TEXT NOT NULL
- `output_mode` TEXT NOT NULL
- `content` TEXT NOT NULL
- `created_at` TEXT NOT NULL

Expected `output_mode` values:

- `command`
- `script`

### `app_settings`

- `key` TEXT PRIMARY KEY
- `value_json` TEXT NOT NULL

## Relationship Summary

```text
mindmaps
  -> mindmap_nodes
  -> mindmap_edges
  -> mindmap_layouts
  -> build_results

templates
  -> template_params

mindmap_nodes
  -> mindmap_node_params
  -> templates
```

## Save Strategy

The preferred save flow should run in one transaction:

1. update the `mindmaps` row
2. reconcile node rows for the target mindmap
3. reconcile edge rows for the target mindmap
4. reconcile node parameter rows
5. reconcile tree and graph layout rows
6. commit