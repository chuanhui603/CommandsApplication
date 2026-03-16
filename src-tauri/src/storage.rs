use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInitializeResponse {
    pub app_version: String,
    pub db_ready: bool,
    pub data_dir: String,
    pub schema_version: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStatusResponse {
    pub db_ready: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapSummary {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub current_version: i64,
    pub last_build_result_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListMindmapsResponse {
    pub items: Vec<MindmapSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMindmapSnapshotRequest {
    pub mindmap: SaveMindmapMetadata,
    pub nodes: Vec<Value>,
    pub edges: Vec<Value>,
    pub layouts: Value,
    pub metadata: SaveMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMindmapMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub root_node_id: Option<String>,
    pub active_path_id: Option<String>,
    pub current_version: i64,
    pub last_build_result_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMetadata {
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMindmapSnapshotResponse {
    pub ok: bool,
    pub mindmap_id: String,
    pub current_version: i64,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMindmapDetailResponse {
    pub mindmap: Value,
    pub nodes: Vec<Value>,
    pub edges: Vec<Value>,
    pub layouts: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplatesRequest {
    pub platform_kind: Option<String>,
    pub category: Option<String>,
    pub include_built_in: bool,
    pub include_user: bool,
}

#[derive(Debug, Serialize)]
pub struct ListTemplatesResponse {
    pub items: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneBuiltinTemplateRequest {
    pub template_id: String,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertTemplateRequest {
    pub name: String,
    pub description: String,
    pub platform_kind: String,
    pub category: Option<String>,
    pub command_pattern: String,
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateRequest {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub platform_kind: String,
    pub category: Option<String>,
    pub command_pattern: String,
    pub params: Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMutationResponse {
    pub template_id: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserTemplateRequest {
    pub template_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveBuildResultRequest {
    pub mindmap_id: String,
    pub target: String,
    pub output_mode: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveBuildResultResponse {
    pub build_result_id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRecentBuildResultsRequest {
    pub mindmap_id: String,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListRecentBuildResultsResponse {
    pub items: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonImportResponse {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub payload: Value,
}

pub fn db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
    fs::create_dir_all(&data_dir).map_err(|e| format!("failed to create data dir: {e}"))?;
    Ok(data_dir.join("command_mindmaps.db"))
}

pub fn connect(app: &AppHandle) -> Result<Connection, String> {
    let path = db_path(app)?;
    Connection::open(path).map_err(|e| format!("failed to open sqlite: {e}"))
}

pub fn initialize_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS app_meta (
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS mindmaps (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          description TEXT,
          root_node_id TEXT,
          active_path_id TEXT,
          current_version INTEGER NOT NULL DEFAULT 1,
          last_build_result_id TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL,
          snapshot_json TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS templates (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL UNIQUE,
          description TEXT NOT NULL,
          platform_kind TEXT NOT NULL,
          category TEXT,
          built_in INTEGER NOT NULL,
          command_pattern TEXT NOT NULL,
          params_json TEXT NOT NULL,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS build_results (
          id TEXT PRIMARY KEY,
          mindmap_id TEXT NOT NULL,
                    mindmap_version INTEGER NOT NULL DEFAULT 0,
          target TEXT NOT NULL,
          output_mode TEXT NOT NULL,
          content TEXT NOT NULL,
          created_at TEXT NOT NULL
        );
        INSERT OR IGNORE INTO app_meta(key, value) VALUES('schema_version', '1');
        "#,
    )
    .map_err(|e| format!("failed to init schema: {e}"))?;
    ensure_column_exists(
        conn,
        "build_results",
        "mindmap_version",
        "ALTER TABLE build_results ADD COLUMN mindmap_version INTEGER NOT NULL DEFAULT 0",
    )?;
    seed_builtin_templates(conn)?;
    seed_sample_mindmaps(conn)?;
    Ok(())
}

fn ensure_column_exists(
    conn: &Connection,
    table_name: &str,
    column_name: &str,
    alter_sql: &str,
) -> Result<(), String> {
    let pragma = format!("PRAGMA table_info({table_name})");
    let mut stmt = conn
        .prepare(&pragma)
        .map_err(|e| format!("failed preparing table info query for {table_name}: {e}"))?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| format!("failed reading table info for {table_name}: {e}"))?;

    for column in columns {
        if column.map_err(|e| format!("failed mapping table column for {table_name}: {e}"))?
            == column_name
        {
            return Ok(());
        }
    }

    conn.execute(alter_sql, [])
        .map_err(|e| format!("failed altering {table_name} for column {column_name}: {e}"))?;
    Ok(())
}

fn seed_builtin_templates(conn: &Connection) -> Result<(), String> {
    let now = now_iso();
    let defaults = vec![
                (
                        "tpl_builtin_files_ls",
                        "List files",
                        "List files and directories under a target path.",
                        "linux-shell",
                        "files-and-directories",
                        "ls -la {{path}}",
                        serde_json::json!([
                            {
                                "id": "param_ls_path",
                                "paramKey": "path",
                                "label": "Target path",
                                "type": "path",
                                "required": false,
                                "defaultValue": ".",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_search_rg",
                        "Search text",
                        "Search files using ripgrep.",
                        "linux-shell",
                        "search-and-filtering",
                        "rg {{pattern}} {{path}}",
                        serde_json::json!([
                            {
                                "id": "param_rg_pattern",
                                "paramKey": "pattern",
                                "label": "Search pattern",
                                "type": "text",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            },
                            {
                                "id": "param_rg_path",
                                "paramKey": "path",
                                "label": "Search path",
                                "type": "path",
                                "required": false,
                                "defaultValue": ".",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_git_status",
                        "Git status",
                        "Inspect the current Git working tree state.",
                        "linux-shell",
                        "git",
                        "git status",
                        "[]".to_string(),
                ),
                (
                        "tpl_builtin_docker_ps",
                        "Docker list",
                        "List Docker containers.",
                        "linux-shell",
                        "docker",
                        "docker ps {{flags}}",
                        serde_json::json!([
                            {
                                "id": "param_docker_flags",
                                "paramKey": "flags",
                                "label": "Extra flags",
                                "type": "text",
                                "required": false,
                                "defaultValue": "-a",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_tar",
                        "Tar gzip",
                        "Compress a file or directory into a tar.gz archive.",
                        "linux-shell",
                        "compression",
                        "tar -czf {{archive}} {{path}}",
                        serde_json::json!([
                            {
                                "id": "param_tar_archive",
                                "paramKey": "archive",
                                "label": "Archive output",
                                "type": "path",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            },
                            {
                                "id": "param_tar_path",
                                "paramKey": "path",
                                "label": "Source path",
                                "type": "path",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_curl",
                        "HTTP check",
                        "Send a quick HTTP HEAD request.",
                        "linux-shell",
                        "networking",
                        "curl -I {{url}}",
                        serde_json::json!([
                            {
                                "id": "param_curl_url",
                                "paramKey": "url",
                                "label": "Target URL",
                                "type": "text",
                                "required": true,
                                "defaultValue": "https://example.com",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_ps_ls",
                        "PowerShell list files",
                        "List files in a folder using PowerShell.",
                        "windows-powershell",
                        "files-and-directories",
                        "Get-ChildItem {{path}} -Force",
                        serde_json::json!([
                            {
                                "id": "param_ps_path",
                                "paramKey": "path",
                                "label": "Folder path",
                                "type": "path",
                                "required": false,
                                "defaultValue": ".",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_ps_select_string",
                        "PowerShell search text",
                        "Search file contents using Select-String.",
                        "windows-powershell",
                        "search-and-filtering",
                        "Select-String -Path {{path}} -Pattern {{pattern}}",
                        serde_json::json!([
                            {
                                "id": "param_ps_ss_path",
                                "paramKey": "path",
                                "label": "File path or glob",
                                "type": "path",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            },
                            {
                                "id": "param_ps_ss_pattern",
                                "paramKey": "pattern",
                                "label": "Pattern",
                                "type": "text",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_wsl_apt_update",
                        "WSL apt update",
                        "Refresh package metadata inside WSL.",
                        "wsl",
                        "system",
                        "wsl sudo apt update && wsl sudo apt upgrade -y",
                        "[]".to_string(),
                ),
                (
                        "tpl_builtin_wsl_docker_logs",
                        "WSL docker logs",
                        "Inspect Docker container logs inside WSL.",
                        "wsl",
                        "docker",
                        "wsl docker logs {{container}} {{flags}}",
                        serde_json::json!([
                            {
                                "id": "param_wsl_container",
                                "paramKey": "container",
                                "label": "Container name",
                                "type": "text",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            },
                            {
                                "id": "param_wsl_flags",
                                "paramKey": "flags",
                                "label": "Extra flags",
                                "type": "text",
                                "required": false,
                                "defaultValue": "--tail 100",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_git_pull_rebase",
                        "Git pull with rebase",
                        "Update a branch from origin using rebase so history stays linear.",
                        "linux-shell",
                        "git",
                        "git pull --rebase origin {{branch}}",
                        serde_json::json!([
                            {
                                "id": "param_git_rebase_branch",
                                "paramKey": "branch",
                                "label": "Branch",
                                "type": "text",
                                "required": true,
                                "defaultValue": "main",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_docker_logs_tail",
                        "Docker logs tail",
                        "Tail the latest logs from a specific container.",
                        "linux-shell",
                        "docker",
                        "docker logs {{container}} {{flags}}",
                        serde_json::json!([
                            {
                                "id": "param_docker_logs_container",
                                "paramKey": "container",
                                "label": "Container name",
                                "type": "text",
                                "required": true,
                                "defaultValue": null,
                                "options": []
                            },
                            {
                                "id": "param_docker_logs_flags",
                                "paramKey": "flags",
                                "label": "Extra flags",
                                "type": "text",
                                "required": false,
                                "defaultValue": "--tail 200",
                                "options": []
                            }
                        ]).to_string(),
                ),
                (
                        "tpl_builtin_ps_test_net_connection",
                        "PowerShell network check",
                        "Verify a TCP endpoint from PowerShell.",
                        "windows-powershell",
                        "networking",
                        "Test-NetConnection {{host}} -Port {{port}}",
                        serde_json::json!([
                            {
                                "id": "param_ps_tnc_host",
                                "paramKey": "host",
                                "label": "Host",
                                "type": "text",
                                "required": true,
                                "defaultValue": "localhost",
                                "options": []
                            },
                            {
                                "id": "param_ps_tnc_port",
                                "paramKey": "port",
                                "label": "Port",
                                "type": "text",
                                "required": true,
                                "defaultValue": "80",
                                "options": []
                            }
                        ]).to_string(),
                ),
    ];
    for item in defaults {
        conn.execute(
                        r#"INSERT INTO templates(
              id,name,description,platform_kind,category,built_in,command_pattern,params_json,created_at,updated_at
                        ) VALUES(?1,?2,?3,?4,?5,1,?6,?7,?8,?8)
                        ON CONFLICT(id) DO UPDATE SET
                            name=excluded.name,
                            description=excluded.description,
                            platform_kind=excluded.platform_kind,
                            category=excluded.category,
                            built_in=1,
                            command_pattern=excluded.command_pattern,
                            params_json=excluded.params_json,
                            updated_at=excluded.updated_at"#,
                        params![item.0, item.1, item.2, item.3, item.4, item.5, item.6, now],
        )
        .map_err(|e| format!("failed seeding template {}: {e}", item.0))?;
    }
    Ok(())
}

fn seed_sample_mindmaps(conn: &Connection) -> Result<(), String> {
        insert_sample_mindmap(
                conn,
                "mm_sample_linux_project_review",
                "範例 01 Linux 專案巡檢",
                "先列出 src，再搜尋 TODO，最後確認 Git 狀態。這張範例適合看 active path、模板套用與參數怎麼填。",
                "sample_linux_list",
                "sample_linux_git_status",
                vec![
                        serde_json::json!({
                            "id": "sample_linux_list",
                            "templateId": "tpl_builtin_files_ls",
                            "title": "列出 src 目錄",
                            "notes": "第一步先確認要巡檢的資料夾。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_linux_list_path",
                                    "paramKey": "path",
                                    "paramType": "path",
                                    "value": "src"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_linux_search",
                            "templateId": "tpl_builtin_search_rg",
                            "title": "搜尋 TODO / FIXME",
                            "notes": "把常見待處理標記先掃過一遍。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_linux_search_pattern",
                                    "paramKey": "pattern",
                                    "paramType": "text",
                                    "value": "TODO|FIXME"
                                },
                                {
                                    "id": "sample_linux_search_path",
                                    "paramKey": "path",
                                    "paramType": "path",
                                    "value": "src"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_linux_git_status",
                            "templateId": "tpl_builtin_git_status",
                            "title": "確認 Git 狀態",
                            "notes": "最後看工作樹是否乾淨。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        })
                ],
                vec![
                        serde_json::json!({
                            "id": "edge_sample_linux_list_search",
                            "sourceNodeId": "sample_linux_list",
                            "targetNodeId": "sample_linux_search",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        }),
                        serde_json::json!({
                            "id": "edge_sample_linux_search_git",
                            "sourceNodeId": "sample_linux_search",
                            "targetNodeId": "sample_linux_git_status",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        })
                ],
                serde_json::json!({
                    "tree": {
                        "sample_linux_list": { "x": 0, "y": 0 },
                        "sample_linux_search": { "x": 0, "y": 56 },
                        "sample_linux_git_status": { "x": 0, "y": 112 }
                    },
                    "graph": {
                        "sample_linux_list": { "x": 80, "y": 80 },
                        "sample_linux_search": { "x": 320, "y": 80 },
                        "sample_linux_git_status": { "x": 560, "y": 80 }
                    }
                }),
        )?;

        insert_sample_mindmap(
                conn,
                "mm_sample_docker_debug",
                "範例 02 Docker 偵錯流程",
                "先列出容器，再看 api logs，最後打健康檢查。這張範例適合看 Docker 相關模板與多參數設定。",
                "sample_docker_list",
                "sample_docker_health",
                vec![
                        serde_json::json!({
                            "id": "sample_docker_list",
                            "templateId": "tpl_builtin_docker_ps",
                            "title": "列出全部容器",
                            "notes": "先確認容器名稱與狀態。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_docker_list_flags",
                                    "paramKey": "flags",
                                    "paramType": "text",
                                    "value": "-a"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_docker_logs",
                            "templateId": "tpl_builtin_docker_logs_tail",
                            "title": "查看 api 容器 logs",
                            "notes": "container 參數通常填 docker compose service 名稱。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_docker_logs_container",
                                    "paramKey": "container",
                                    "paramType": "text",
                                    "value": "api"
                                },
                                {
                                    "id": "sample_docker_logs_flags",
                                    "paramKey": "flags",
                                    "paramType": "text",
                                    "value": "--tail 200"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_docker_health",
                            "templateId": "tpl_builtin_curl",
                            "title": "打健康檢查",
                            "notes": "如果服務有 health endpoint，可以接在 logs 後面。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_docker_health_url",
                                    "paramKey": "url",
                                    "paramType": "text",
                                    "value": "http://localhost:8080/healthz"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        })
                ],
                vec![
                        serde_json::json!({
                            "id": "edge_sample_docker_list_logs",
                            "sourceNodeId": "sample_docker_list",
                            "targetNodeId": "sample_docker_logs",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        }),
                        serde_json::json!({
                            "id": "edge_sample_docker_logs_health",
                            "sourceNodeId": "sample_docker_logs",
                            "targetNodeId": "sample_docker_health",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        })
                ],
                serde_json::json!({
                    "tree": {
                        "sample_docker_list": { "x": 0, "y": 0 },
                        "sample_docker_logs": { "x": 0, "y": 56 },
                        "sample_docker_health": { "x": 0, "y": 112 }
                    },
                    "graph": {
                        "sample_docker_list": { "x": 80, "y": 80 },
                        "sample_docker_logs": { "x": 340, "y": 80 },
                        "sample_docker_health": { "x": 620, "y": 80 }
                    }
                }),
        )?;

        insert_sample_mindmap(
                conn,
                "mm_sample_powershell_logs",
                "範例 03 PowerShell 日誌掃描",
                "列出 logs 資料夾、搜尋錯誤、最後測試內網服務連線。這張範例適合看 Windows PowerShell 平台怎麼設定。",
                "sample_ps_list",
                "sample_ps_network_check",
                vec![
                        serde_json::json!({
                            "id": "sample_ps_list",
                            "templateId": "tpl_builtin_ps_ls",
                            "title": "列出 logs 資料夾",
                            "notes": "path 可以改成任何你要掃描的資料夾。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_ps_list_path",
                                    "paramKey": "path",
                                    "paramType": "path",
                                    "value": ".\\logs"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_ps_search",
                            "templateId": "tpl_builtin_ps_select_string",
                            "title": "搜尋 error / exception",
                            "notes": "Pattern 可以填 regex；Path 也可以用 glob。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_ps_search_path",
                                    "paramKey": "path",
                                    "paramType": "path",
                                    "value": ".\\logs\\*.log"
                                },
                                {
                                    "id": "sample_ps_search_pattern",
                                    "paramKey": "pattern",
                                    "paramType": "text",
                                    "value": "error|exception"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        }),
                        serde_json::json!({
                            "id": "sample_ps_network_check",
                            "templateId": "tpl_builtin_ps_test_net_connection",
                            "title": "測試內網服務連線",
                            "notes": "這一步示範同一張心智圖也可以接上網路檢查。",
                            "includeInOutput": true,
                            "orderOverride": null,
                            "params": [
                                {
                                    "id": "sample_ps_network_host",
                                    "paramKey": "host",
                                    "paramType": "text",
                                    "value": "intranet-api"
                                },
                                {
                                    "id": "sample_ps_network_port",
                                    "paramKey": "port",
                                    "paramType": "text",
                                    "value": "443"
                                }
                            ],
                            "createdAt": now_iso(),
                            "updatedAt": now_iso()
                        })
                ],
                vec![
                        serde_json::json!({
                            "id": "edge_sample_ps_list_search",
                            "sourceNodeId": "sample_ps_list",
                            "targetNodeId": "sample_ps_search",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        }),
                        serde_json::json!({
                            "id": "edge_sample_ps_search_network",
                            "sourceNodeId": "sample_ps_search",
                            "targetNodeId": "sample_ps_network_check",
                            "edgeType": "flow",
                            "priority": null,
                            "enabled": true
                        })
                ],
                serde_json::json!({
                    "tree": {
                        "sample_ps_list": { "x": 0, "y": 0 },
                        "sample_ps_search": { "x": 0, "y": 56 },
                        "sample_ps_network_check": { "x": 0, "y": 112 }
                    },
                    "graph": {
                        "sample_ps_list": { "x": 80, "y": 80 },
                        "sample_ps_search": { "x": 360, "y": 80 },
                        "sample_ps_network_check": { "x": 680, "y": 80 }
                    }
                }),
        )?;

        Ok(())
}

fn insert_sample_mindmap(
        conn: &Connection,
        id: &str,
        name: &str,
        description: &str,
        root_node_id: &str,
        active_path_id: &str,
        nodes: Vec<Value>,
        edges: Vec<Value>,
        layouts: Value,
) -> Result<(), String> {
        let now = now_iso();
        let snapshot = serde_json::to_string(&GetMindmapDetailResponse {
                mindmap: serde_json::json!({
                    "id": id,
                    "name": name,
                    "description": description,
                    "rootNodeId": root_node_id,
                    "activePathId": active_path_id,
                    "currentVersion": 1,
                    "lastBuildResultId": serde_json::Value::Null,
                    "createdAt": now,
                    "updatedAt": now
                }),
                nodes,
                edges,
                layouts,
        })
        .map_err(|e| format!("failed serializing sample mindmap {id}: {e}"))?;

        conn.execute(
                r#"INSERT OR IGNORE INTO mindmaps(
                    id,name,description,root_node_id,active_path_id,current_version,last_build_result_id,created_at,updated_at,snapshot_json
                ) VALUES(?1,?2,?3,?4,?5,1,NULL,?6,?6,?7)"#,
                params![id, name, description, root_node_id, active_path_id, now, snapshot],
        )
        .map_err(|e| format!("failed inserting sample mindmap {id}: {e}"))?;
        Ok(())
}

fn clone_builtin_template_in_conn(
    conn: &Connection,
    request: CloneBuiltinTemplateRequest,
) -> Result<(), String> {
    let source: Option<(String, String, String, String, Option<String>, String, String)> = conn
        .query_row(
            "SELECT id,name,description,platform_kind,category,command_pattern,params_json
             FROM templates WHERE id=?1 AND built_in=1",
            params![request.template_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )
        .optional()
        .map_err(|e| format!("failed reading builtin template: {e}"))?;
    let Some(template) = source else {
        return Err("builtin template not found".to_string());
    };

    let now = now_iso();
    let new_id = format!("tpl_user_{}", Uuid::new_v4().simple());
    conn.execute(
        r#"INSERT INTO templates(
          id,name,description,platform_kind,category,built_in,command_pattern,params_json,created_at,updated_at
        ) VALUES(?1,?2,?3,?4,?5,0,?6,?7,?8,?8)"#,
        params![new_id, request.new_name, template.2, template.3, template.4, template.5, template.6, now],
    )
    .map_err(|e| format!("failed cloning template: {e}"))?;
    Ok(())
}

fn update_snapshot_generation_metadata(
    snapshot_json: &str,
    build_result_id: &str,
    updated_at: &str,
) -> Result<String, String> {
    let mut snapshot = serde_json::from_str::<Value>(snapshot_json)
        .map_err(|e| format!("failed parsing existing snapshot json: {e}"))?;
    if let Some(mindmap) = snapshot.get_mut("mindmap").and_then(Value::as_object_mut) {
        mindmap.insert(
            "lastBuildResultId".to_string(),
            Value::String(build_result_id.to_string()),
        );
        mindmap.insert("updatedAt".to_string(), Value::String(updated_at.to_string()));
    }
    serde_json::to_string(&snapshot)
        .map_err(|e| format!("failed serializing updated snapshot json: {e}"))
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn app_initialize(app: &AppHandle) -> Result<AppInitializeResponse, String> {
    let conn = connect(app)?;
    initialize_schema(&conn)?;
    let schema_version: i64 = conn
        .query_row(
            "SELECT value FROM app_meta WHERE key='schema_version'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| format!("failed reading schema version: {e}"))?
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1);
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve data dir: {e}"))?;
    Ok(AppInitializeResponse {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        db_ready: true,
        data_dir: data_dir.to_string_lossy().to_string(),
        schema_version,
    })
}

pub fn get_storage_status(app: &AppHandle) -> StorageStatusResponse {
    match connect(app).and_then(|conn| initialize_schema(&conn)) {
        Ok(_) => StorageStatusResponse {
            db_ready: true,
            last_error: None,
        },
        Err(e) => StorageStatusResponse {
            db_ready: false,
            last_error: Some(e),
        },
    }
}

pub fn list_mindmaps(app: &AppHandle) -> Result<ListMindmapsResponse, String> {
    let conn = connect(app)?;
    let mut stmt = conn
        .prepare(
            "SELECT id,name,updated_at,current_version,last_build_result_id
             FROM mindmaps ORDER BY updated_at DESC",
        )
        .map_err(|e| format!("failed preparing list mindmaps: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(MindmapSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                updated_at: row.get(2)?,
                current_version: row.get(3)?,
                last_build_result_id: row.get(4)?,
            })
        })
        .map_err(|e| format!("failed querying list mindmaps: {e}"))?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| format!("failed mapping mindmap row: {e}"))?);
    }
    Ok(ListMindmapsResponse { items })
}

pub fn get_mindmap_detail(app: &AppHandle, mindmap_id: String) -> Result<GetMindmapDetailResponse, String> {
    let conn = connect(app)?;
    let snapshot_json = conn
        .query_row(
            "SELECT snapshot_json FROM mindmaps WHERE id=?1",
            params![mindmap_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| format!("failed reading mindmap snapshot: {e}"))?;
    if let Some(raw) = snapshot_json {
        return serde_json::from_str::<GetMindmapDetailResponse>(&raw)
            .map_err(|e| format!("failed parsing snapshot json: {e}"));
    }
    Err("mindmap not found".to_string())
}

pub fn save_mindmap_snapshot(
    app: &AppHandle,
    request: SaveMindmapSnapshotRequest,
) -> Result<SaveMindmapSnapshotResponse, String> {
    let conn = connect(app)?;
    let mindmap_id = request.mindmap.id.clone();
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("failed creating transaction: {e}"))?;
    let existing_record: Option<(i64, String)> = tx
        .query_row(
            "SELECT current_version, created_at FROM mindmaps WHERE id=?1",
            params![mindmap_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| format!("failed checking mindmap version: {e}"))?;
    let next_version = existing_record
        .as_ref()
        .map(|record| record.0)
        .unwrap_or(request.mindmap.current_version)
        + 1;
    let now = request.metadata.updated_at.clone();
    let created_at = existing_record
        .map(|record| record.1)
        .unwrap_or_else(|| now.clone());
    let snapshot = serde_json::to_string(&GetMindmapDetailResponse {
        mindmap: serde_json::json!({
          "id": request.mindmap.id,
          "name": request.mindmap.name,
          "description": request.mindmap.description,
          "rootNodeId": request.mindmap.root_node_id,
          "activePathId": request.mindmap.active_path_id,
          "currentVersion": next_version,
          "lastBuildResultId": request.mindmap.last_build_result_id,
          "createdAt": created_at,
          "updatedAt": now
        }),
        nodes: request.nodes.clone(),
        edges: request.edges.clone(),
        layouts: request.layouts.clone(),
    })
    .map_err(|e| format!("failed serializing snapshot: {e}"))?;
    tx.execute(
        r#"INSERT INTO mindmaps(
          id,name,description,root_node_id,active_path_id,current_version,last_build_result_id,created_at,updated_at,snapshot_json
                ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
        ON CONFLICT(id) DO UPDATE SET
          name=excluded.name,
          description=excluded.description,
          root_node_id=excluded.root_node_id,
          active_path_id=excluded.active_path_id,
          current_version=excluded.current_version,
                    last_build_result_id=excluded.last_build_result_id,
          updated_at=excluded.updated_at,
          snapshot_json=excluded.snapshot_json"#,
        params![
            mindmap_id,
            request.mindmap.name,
            request.mindmap.description,
            request.mindmap.root_node_id,
            request.mindmap.active_path_id,
            next_version,
                        request.mindmap.last_build_result_id,
                        created_at,
                        now,
            snapshot
        ],
    )
    .map_err(|e| format!("failed upserting mindmap: {e}"))?;
    tx.commit().map_err(|e| format!("failed committing transaction: {e}"))?;
    Ok(SaveMindmapSnapshotResponse {
        ok: true,
        mindmap_id: request.mindmap.id,
        current_version: next_version,
        updated_at: request.metadata.updated_at,
    })
}

pub fn list_templates(app: &AppHandle, request: ListTemplatesRequest) -> Result<ListTemplatesResponse, String> {
    let conn = connect(app)?;
    let mut sql = String::from(
        "SELECT id,name,description,platform_kind,category,built_in,command_pattern,params_json,created_at,updated_at
         FROM templates WHERE 1=1",
    );
    if let Some(platform_kind) = request.platform_kind.clone() {
        sql.push_str(" AND platform_kind = '");
        sql.push_str(&platform_kind);
        sql.push('\'');
    }
    if let Some(category) = request.category.clone() {
        sql.push_str(" AND category = '");
        sql.push_str(&category);
        sql.push('\'');
    }
    if !request.include_built_in {
        sql.push_str(" AND built_in = 0");
    }
    if !request.include_user {
        sql.push_str(" AND built_in = 1");
    }
    sql.push_str(" ORDER BY built_in DESC, name ASC");
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("failed preparing template query: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
              "id": row.get::<_, String>(0)?,
              "name": row.get::<_, String>(1)?,
              "description": row.get::<_, String>(2)?,
              "platformKind": row.get::<_, String>(3)?,
              "category": row.get::<_, Option<String>>(4)?,
              "builtIn": row.get::<_, i64>(5)? == 1,
              "commandPattern": row.get::<_, String>(6)?,
              "params": serde_json::from_str::<Value>(&row.get::<_, String>(7)?).unwrap_or(Value::Array(vec![])),
              "createdAt": row.get::<_, String>(8)?,
              "updatedAt": row.get::<_, String>(9)?
            }))
        })
        .map_err(|e| format!("failed querying templates: {e}"))?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| format!("failed mapping template row: {e}"))?);
    }
    Ok(ListTemplatesResponse { items })
}

pub fn clone_builtin_template(
    app: &AppHandle,
    request: CloneBuiltinTemplateRequest,
) -> Result<(), String> {
    let conn = connect(app)?;
    clone_builtin_template_in_conn(&conn, request)
}

pub fn create_template(
    app: &AppHandle,
    request: UpsertTemplateRequest,
) -> Result<TemplateMutationResponse, String> {
    let conn = connect(app)?;
    let now = now_iso();
    let template_id = format!("tpl_user_{}", Uuid::new_v4().simple());
    let params_json = serde_json::to_string(&request.params)
        .map_err(|e| format!("failed serializing template params: {e}"))?;
    conn.execute(
        r#"INSERT INTO templates(
          id,name,description,platform_kind,category,built_in,command_pattern,params_json,created_at,updated_at
        ) VALUES(?1,?2,?3,?4,?5,0,?6,?7,?8,?8)"#,
        params![
            template_id,
            request.name,
            request.description,
            request.platform_kind,
            request.category,
            request.command_pattern,
            params_json,
            now
        ],
    )
    .map_err(|e| format!("failed creating template: {e}"))?;
    Ok(TemplateMutationResponse {
        template_id,
        updated_at: now,
    })
}

pub fn update_template(
    app: &AppHandle,
    request: UpdateTemplateRequest,
) -> Result<TemplateMutationResponse, String> {
    let conn = connect(app)?;
    let now = now_iso();
    let params_json = serde_json::to_string(&request.params)
        .map_err(|e| format!("failed serializing template params: {e}"))?;
    let affected = conn
        .execute(
            r#"UPDATE templates SET
              name=?2,
              description=?3,
              platform_kind=?4,
              category=?5,
              command_pattern=?6,
              params_json=?7,
              updated_at=?8
            WHERE id=?1 AND built_in=0"#,
            params![
                request.template_id,
                request.name,
                request.description,
                request.platform_kind,
                request.category,
                request.command_pattern,
                params_json,
                now
            ],
        )
        .map_err(|e| format!("failed updating template: {e}"))?;
    if affected == 0 {
        return Err("user template not found".to_string());
    }
    Ok(TemplateMutationResponse {
        template_id: request.template_id,
        updated_at: now,
    })
}

pub fn delete_user_template(
    app: &AppHandle,
    request: DeleteUserTemplateRequest,
) -> Result<(), String> {
    let conn = connect(app)?;
    conn.execute(
        "DELETE FROM templates WHERE id=?1 AND built_in=0",
        params![request.template_id],
    )
    .map_err(|e| format!("failed deleting template: {e}"))?;
    Ok(())
}

pub fn save_build_result(
    app: &AppHandle,
    request: SaveBuildResultRequest,
) -> Result<SaveBuildResultResponse, String> {
    let conn = connect(app)?;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("failed creating build result transaction: {e}"))?;
    let now = now_iso();
    let build_result_id = format!("build_{}", Uuid::new_v4().simple());
    let (mindmap_version, snapshot_json): (i64, String) = tx
        .query_row(
            "SELECT current_version, snapshot_json FROM mindmaps WHERE id=?1",
            params![request.mindmap_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| format!("failed reading mindmap before saving build result: {e}"))?
        .ok_or_else(|| "mindmap not found for build result".to_string())?;
    let updated_snapshot = update_snapshot_generation_metadata(&snapshot_json, &build_result_id, &now)?;
    tx.execute(
        r#"INSERT INTO build_results(
          id,mindmap_id,mindmap_version,target,output_mode,content,created_at
        ) VALUES(?1,?2,?3,?4,?5,?6,?7)"#,
        params![
            build_result_id,
            request.mindmap_id,
            mindmap_version,
            request.target,
            request.output_mode,
            request.content,
            now
        ],
    )
    .map_err(|e| format!("failed saving build result: {e}"))?;
    tx.execute(
        "UPDATE mindmaps SET last_build_result_id=?2, updated_at=?3, snapshot_json=?4 WHERE id=?1",
        params![request.mindmap_id, build_result_id, now, updated_snapshot],
    )
    .map_err(|e| format!("failed updating mindmap build metadata: {e}"))?;
    tx.commit()
        .map_err(|e| format!("failed committing build result transaction: {e}"))?;
    Ok(SaveBuildResultResponse {
        build_result_id,
        created_at: now,
    })
}

pub fn list_recent_build_results(
    app: &AppHandle,
    request: ListRecentBuildResultsRequest,
) -> Result<ListRecentBuildResultsResponse, String> {
    let conn = connect(app)?;
    let limit = request.limit.unwrap_or(10);
    let mut stmt = conn
        .prepare(
            "SELECT id,mindmap_id,mindmap_version,target,output_mode,content,created_at FROM build_results WHERE mindmap_id=?1 ORDER BY created_at DESC LIMIT ?2",
        )
        .map_err(|e| format!("failed preparing recent build results query: {e}"))?;
    let rows = stmt
        .query_map(params![request.mindmap_id, limit], |row| {
            Ok(serde_json::json!({
              "id": row.get::<_, String>(0)?,
              "mindmapId": row.get::<_, String>(1)?,
              "mindmapVersion": row.get::<_, i64>(2)?,
              "target": row.get::<_, String>(3)?,
              "outputMode": row.get::<_, String>(4)?,
              "content": row.get::<_, String>(5)?,
              "createdAt": row.get::<_, String>(6)?
            }))
        })
        .map_err(|e| format!("failed querying recent build results: {e}"))?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| format!("failed mapping build result row: {e}"))?);
    }
    Ok(ListRecentBuildResultsResponse { items })
}

pub fn export_json_to_file(path: String, payload: Value) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&payload).map_err(|e| format!("failed serializing export json: {e}"))?;
    fs::write(path, json).map_err(|e| format!("failed writing export file: {e}"))
}

pub fn import_json_from_file(path: String) -> Result<JsonImportResponse, String> {
    let raw = fs::read_to_string(&path).map_err(|e| format!("failed reading import file: {e}"))?;
    let payload = serde_json::from_str::<Value>(&raw).map_err(|e| format!("invalid json file: {e}"))?;
    let file_name = PathBuf::from(path)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown.json".to_string());
    Ok(JsonImportResponse { file_name, payload })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_builtin_template_creates_user_owned_copy() {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        initialize_schema(&conn).expect("initialize schema");

        clone_builtin_template_in_conn(
            &conn,
            CloneBuiltinTemplateRequest {
                template_id: "tpl_builtin_git_status".to_string(),
                new_name: "Git status custom".to_string(),
            },
        )
        .expect("clone builtin template");

        let cloned = conn
            .query_row(
                "SELECT built_in, description, platform_kind, category, command_pattern, params_json FROM templates WHERE name=?1",
                params!["Git status custom"],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )
            .expect("read cloned template");

        assert_eq!(cloned.0, 0);
        assert_eq!(cloned.1, "Inspect the current Git working tree state.");
        assert_eq!(cloned.2, "linux-shell");
        assert_eq!(cloned.3.as_deref(), Some("git"));
        assert_eq!(cloned.4, "git status");
        assert_eq!(cloned.5, "[]");
    }
}
