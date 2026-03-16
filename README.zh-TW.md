# Command Mindmap Executor 架構與介面規格
本文件提供給產品/技術使用者快速理解本專案的產品結構與 Tauri 介面契約。

## 產品介紹

Command Mindmap Executor 是一個本地優先的桌面工具，讓你用心智圖方式整理可重用的命令片段，並在需要時輸出成可重用的命令或腳本流程。

## 產品結構

1. **編輯層**：Tree View + Graph View 編排命令流程。  
2. **資料層**：以 SQLite 儲存 mindmap、範本、產出結果與設定。  
3. **桌面能力層**：由 Tauri 提供本機檔案操作與桌面封裝。  

## 系統邊界

- Vue / TypeScript：UI、狀態、驗證流程、命令生成流程。
- Tauri / Rust：啟動、SQLite、檔案 I/O、型別化回傳。

## Tauri Command 契約（TypeScript Interface）

```ts
export type PlatformKind = "linux-shell" | "wsl" | "windows-powershell";
export type OutputMode = "command" | "script";
export type IsoDateTime = string;

export interface CommandError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

export type CommandResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: CommandError };

export interface AppInitializeRequest {}
export interface AppInitializeResponse {
  appVersion: string;
  dbReady: boolean;
  dataDir: string;
  schemaVersion: number;
}

export interface GetStorageStatusResponse {
  dbReady: boolean;
  lastError: string | null;
}

export interface MindmapSummary {
  id: string;
  name: string;
  updatedAt: IsoDateTime;
  currentVersion: number;
  lastBuildResultId: string | null;
}
export interface ListMindmapsResponse {
  items: MindmapSummary[];
}

export interface CreateMindmapRequest {
  name: string;
  description?: string;
}
export interface CreateMindmapResponse {
  mindmapId: string;
  createdAt: IsoDateTime;
}

export interface GetMindmapDetailRequest {
  mindmapId: string;
}
export interface SaveMindmapSnapshotRequest {
  mindmap: {
    id: string;
    name: string;
    description?: string;
    rootNodeId: string | null;
    activePathId: string | null;
    currentVersion: number;
  };
  nodes: unknown[];
  edges: unknown[];
  layouts: { tree: Record<string, unknown>; graph: Record<string, unknown> };
  metadata: { updatedAt: IsoDateTime };
}
export interface SaveMindmapSnapshotResponse {
  ok: true;
  mindmapId: string;
  currentVersion: number;
  updatedAt: IsoDateTime;
}
export interface DeleteMindmapRequest {
  mindmapId: string;
}

export interface ListTemplatesRequest {
  platformKind: PlatformKind | null;
  category: string | null;
  includeBuiltIn: boolean;
  includeUser: boolean;
}
export interface ListTemplatesResponse {
  items: unknown[];
}
export interface CreateTemplateRequest {
  name: string;
  description: string;
  platformKind: PlatformKind;
  category: string | null;
  commandPattern: string;
  params: unknown[];
}
export interface UpdateTemplateRequest extends CreateTemplateRequest {
  templateId: string;
}
export interface CloneBuiltinTemplateRequest {
  templateId: string;
  newName: string;
}
export interface DeleteUserTemplateRequest {
  templateId: string;
}

export interface ExportMindmapToFileRequest {
  mindmapId: string;
  payload: unknown;
}
export interface ImportMindmapFromFileResponse {
  fileName: string;
  payload: unknown;
}
export interface ExportTemplateBundleToFileRequest {
  payload: unknown;
}
export interface ImportTemplateBundleFromFileResponse {
  fileName: string;
  payload: unknown;
}

export interface SaveBuildResultRequest {
  mindmapId: string;
  target: PlatformKind;
  outputMode: OutputMode;
  content: string;
}
export interface ListRecentBuildResultsRequest {
  mindmapId: string;
  limit?: number;
}
export interface GetUserSettingsResponse {
  values: Record<string, unknown>;
}
export interface UpdateUserSettingsRequest {
  values: Record<string, unknown>;
}
```

## Rust Command Stub 規格（Tauri）

> 下列為介面 stub 規格，便於 Rust 與 TypeScript 共同對齊；可先以 `todo!()` 補齊實作。

```rust
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInitializeResponse {
    pub app_version: String,
    pub db_ready: bool,
    pub data_dir: String,
    pub schema_version: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMindmapSnapshotRequest {
    pub mindmap: serde_json::Value,
    pub nodes: Vec<serde_json::Value>,
    pub edges: Vec<serde_json::Value>,
    pub layouts: serde_json::Value,
    pub metadata: serde_json::Value,
}

#[tauri::command]
pub fn app_initialize(app: AppHandle) -> Result<AppInitializeResponse, String> { todo!() }

#[tauri::command]
pub fn get_storage_status(app: AppHandle) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn list_mindmaps(app: AppHandle) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn create_mindmap(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn get_mindmap_detail(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn save_mindmap_snapshot(
    app: AppHandle,
    request: SaveMindmapSnapshotRequest,
) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn delete_mindmap(app: AppHandle, request: serde_json::Value) -> Result<(), String> { todo!() }

#[tauri::command]
pub fn list_templates(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn create_template(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn update_template(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn clone_builtin_template(app: AppHandle, request: serde_json::Value) -> Result<(), String> { todo!() }

#[tauri::command]
pub fn delete_user_template(app: AppHandle, request: serde_json::Value) -> Result<(), String> { todo!() }

#[tauri::command]
pub fn export_mindmap_to_file(app: AppHandle, request: serde_json::Value) -> Result<(), String> { todo!() }

#[tauri::command]
pub fn import_mindmap_from_file(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn export_template_bundle_to_file(app: AppHandle, request: serde_json::Value) -> Result<(), String> { todo!() }

#[tauri::command]
pub fn import_template_bundle_from_file(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn save_build_result(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn list_recent_build_results(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn get_user_settings(app: AppHandle) -> Result<serde_json::Value, String> { todo!() }

#[tauri::command]
pub fn update_user_settings(app: AppHandle, request: serde_json::Value) -> Result<serde_json::Value, String> { todo!() }
```

## 現況備註（與現行程式對齊）

- 目前已落地核心子集：`app_initialize`、`get_storage_status`、`list_mindmaps`、`get_mindmap_detail`、`save_mindmap_snapshot`、`list_templates`、`clone_builtin_template`、`export_json_to_file`、`import_json_from_file`。
- 其餘 command 可依本規格逐步補齊，且維持 request/response 的型別化契約。

