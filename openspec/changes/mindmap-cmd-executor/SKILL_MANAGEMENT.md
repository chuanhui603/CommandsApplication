# Skill Management (Internal)

This file is for implementation guidance and internal contributor alignment.

## Internal Documentation Placement

- Root README files are user-facing and should stay focused on product introduction and usage.
- Product structure documents belong under `docs/architecture/`.
- Runtime contracts and persistence references belong under `docs/reference/`.
- Folder-specific implementation notes belong in local README files such as `src/README.md` and `src-tauri/README.md`.

## Recommended Implementation Sequence

1. Define TypeScript domain models and DTOs.
2. Define and freeze Tauri command contracts.
3. Create SQLite schema and initialization flow.
4. Implement core persistence commands first:
   - `app_initialize`
   - `list_mindmaps`
   - `get_mindmap_detail`
   - `save_mindmap_snapshot`
5. Add template management commands.
6. Add import/export command set.
7. Add build-result persistence and user-settings commands.

## Notes for Maintainers New to Rust

1. Build most product features in Vue + TypeScript first.
2. Keep each Tauri command focused and explicit.
3. Keep Rust focused on desktop capabilities, storage, and file bridge concerns.
4. Do not move editor/validator/generation logic to Rust unless there is a clear technical requirement.
