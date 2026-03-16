# Front-End Notes

This folder contains the Vue and TypeScript application.

Recommended ownership for this side of the project:

- workspace UI and page structure
- editor state and autosave orchestration
- mindmap interaction logic
- template selection and parameter editing
- validator execution and diagnostic presentation
- preview generation and output presentation
- calling the Tauri runtime through typed contracts

This layer should prefer domain models and DTOs over direct persistence assumptions.

Related references:

- [../docs/reference/tauri-command-contracts.md](../docs/reference/tauri-command-contracts.md)
- [../docs/architecture/overview.md](../docs/architecture/overview.md)