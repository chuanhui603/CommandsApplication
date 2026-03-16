# Tauri Command Contracts

This document defines the first-pass TypeScript interfaces and Rust command stubs for the Tauri bridge.

The bridge is intentionally narrow. Product logic should stay primarily in Vue and TypeScript.

## Shared TypeScript Types

```ts
export type PlatformKind = 'linux-shell' | 'wsl' | 'windows-powershell';

export type OutputMode = 'command' | 'script';

export type ParamType = 'text' | 'boolean' | 'single-select' | 'path';

export type ViewType = 'tree' | 'graph';

export interface RuntimeError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

export type CommandResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: RuntimeError };
```

## Core DTOs

```ts
export interface MindmapSummary {
  id: string;
  name: string;
  updatedAt: string;
  currentVersion: number;
  lastBuildResultId?: string | null;
}

export interface MindmapRecord {
  id: string;
  name: string;
  description?: string | null;
  rootNodeId?: string | null;
  activePathId?: string | null;
  currentVersion: number;
  lastBuildResultId?: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface NodeParamValue {
  id: string;
  nodeId: string;
  paramKey: string;
  paramType: ParamType;
  paramValue: string | null;
}

export interface MindmapNode {
  id: string;
  mindmapId: string;
  templateId?: string | null;
  title: string;
  notes?: string | null;
  includeInOutput: boolean;
  orderOverride?: number | null;
  createdAt: string;
  updatedAt: string;
  params: NodeParamValue[];
}

export interface MindmapEdge {
  id: string;
  mindmapId: string;
  sourceNodeId: string;
  targetNodeId: string;
  edgeType: string;
  priority?: number | null;
  enabled: boolean;
}

export interface MindmapSnapshot {
  mindmap: MindmapRecord;
  nodes: MindmapNode[];
  edges: MindmapEdge[];
  layouts: {
    tree: Record<string, unknown>;
    graph: Record<string, unknown>;
  };
}

export interface TemplateParamDefinition {
  id: string;
  templateId: string;
  paramKey: string;
  label: string;
  type: ParamType;
  required: boolean;
  defaultValue?: string | null;
  options?: string[];
}

export interface TemplateRecord {
  id: string;
  name: string;
  description?: string | null;
  platformKind: PlatformKind;
  category?: string | null;
  builtIn: boolean;
  commandPattern: string;
  createdAt: string;
  updatedAt: string;
  params: TemplateParamDefinition[];
}

export interface BuildResultRecord {
  id: string;
  mindmapId: string;
  target: PlatformKind;
  outputMode: OutputMode;
  content: string;
  createdAt: string;
}
```

## Runtime API Shape in TypeScript

```ts
export interface TauriRuntimeApi {
  appInitialize(): Promise<CommandResult<{
    appVersion: string;
    dbReady: boolean;
    dataDir: string;
    schemaVersion: number;
  }>>;

  getStorageStatus(): Promise<CommandResult<{
    dbReady: boolean;
    lastError?: RuntimeError | null;
  }>>;

  listMindmaps(): Promise<CommandResult<{ items: MindmapSummary[] }>>;
  createMindmap(input: {
    name: string;
    description?: string;
  }): Promise<CommandResult<{ mindmapId: string; createdAt: string }>>;
  getMindmapDetail(input: {
    mindmapId: string;
  }): Promise<CommandResult<MindmapSnapshot>>;
  saveMindmapSnapshot(input: MindmapSnapshot): Promise<CommandResult<{
    mindmapId: string;
    currentVersion: number;
    updatedAt: string;
  }>>;
  deleteMindmap(input: {
    mindmapId: string;
  }): Promise<CommandResult<{ deleted: true }>>;

  listTemplates(input?: {
    platformKind?: PlatformKind;
    category?: string | null;
    includeBuiltIn?: boolean;
    includeUser?: boolean;
  }): Promise<CommandResult<{ items: TemplateRecord[] }>>;
  createTemplate(input: Omit<TemplateRecord, 'id' | 'createdAt' | 'updatedAt' | 'builtIn'>): Promise<CommandResult<{ templateId: string }>>;
  updateTemplate(input: {
    templateId: string;
    patch: Partial<Omit<TemplateRecord, 'id' | 'builtIn' | 'createdAt' | 'updatedAt'>>;
  }): Promise<CommandResult<{ templateId: string; updatedAt: string }>>;
  cloneBuiltinTemplate(input: {
    templateId: string;
    newName: string;
  }): Promise<CommandResult<{ templateId: string }>>;
  deleteUserTemplate(input: {
    templateId: string;
  }): Promise<CommandResult<{ deleted: true }>>;

  importMindmapFromFile(): Promise<CommandResult<{ fileName: string; payload: unknown }>>;
  exportMindmapToFile(input: {
    mindmapId: string;
    payload: unknown;
  }): Promise<CommandResult<{ saved: true; path: string }>>;
  importTemplateBundleFromFile(): Promise<CommandResult<{ fileName: string; payload: unknown }>>;
  exportTemplateBundleToFile(input: {
    payload: unknown;
  }): Promise<CommandResult<{ saved: true; path: string }>>;

  saveBuildResult(input: {
    mindmapId: string;
    target: PlatformKind;
    outputMode: OutputMode;
    content: string;
  }): Promise<CommandResult<{ buildResultId: string; createdAt: string }>>;
  listRecentBuildResults(input?: {
    limit?: number;
  }): Promise<CommandResult<{ items: BuildResultRecord[] }>>;
  getUserSettings(): Promise<CommandResult<Record<string, unknown>>>;
  updateUserSettings(input: {
    settings: Record<string, unknown>;
  }): Promise<CommandResult<{ saved: true }>>;
}
```

## Suggested Rust Command Stubs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RuntimeErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

pub type CommandResult<T> = Result<T, RuntimeErrorDto>;

#[tauri::command]
pub async fn app_initialize() -> CommandResult<AppInitializeResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn get_storage_status() -> CommandResult<StorageStatusResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn list_mindmaps() -> CommandResult<ListMindmapsResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn create_mindmap(input: CreateMindmapRequestDto) -> CommandResult<CreateMindmapResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn get_mindmap_detail(input: GetMindmapDetailRequestDto) -> CommandResult<MindmapSnapshotDto> {
    todo!()
}

#[tauri::command]
pub async fn save_mindmap_snapshot(input: SaveMindmapSnapshotRequestDto) -> CommandResult<SaveMindmapSnapshotResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn delete_mindmap(input: DeleteMindmapRequestDto) -> CommandResult<DeleteMindmapResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn list_templates(input: Option<ListTemplatesRequestDto>) -> CommandResult<ListTemplatesResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn create_template(input: CreateTemplateRequestDto) -> CommandResult<CreateTemplateResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn update_template(input: UpdateTemplateRequestDto) -> CommandResult<UpdateTemplateResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn clone_builtin_template(input: CloneBuiltinTemplateRequestDto) -> CommandResult<CloneBuiltinTemplateResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn delete_user_template(input: DeleteUserTemplateRequestDto) -> CommandResult<DeleteUserTemplateResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn import_mindmap_from_file() -> CommandResult<FilePayloadResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn export_mindmap_to_file(input: ExportMindmapToFileRequestDto) -> CommandResult<FileSaveResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn import_template_bundle_from_file() -> CommandResult<FilePayloadResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn export_template_bundle_to_file(input: ExportTemplateBundleToFileRequestDto) -> CommandResult<FileSaveResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn save_build_result(input: SaveBuildResultRequestDto) -> CommandResult<SaveBuildResultResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn list_recent_build_results(input: Option<ListRecentBuildResultsRequestDto>) -> CommandResult<ListRecentBuildResultsResponseDto> {
    todo!()
}

#[tauri::command]
pub async fn get_user_settings() -> CommandResult<UserSettingsDto> {
    todo!()
}

#[tauri::command]
pub async fn update_user_settings(input: UpdateUserSettingsRequestDto) -> CommandResult<UpdateUserSettingsResponseDto> {
    todo!()
}
```

## Notes

- Commands should stay use-case oriented.
- Graph saves should prefer full snapshots over tiny field mutations.
- UI code should work against DTOs and domain models rather than database tables.