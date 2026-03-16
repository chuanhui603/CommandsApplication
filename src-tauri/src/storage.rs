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
          target TEXT NOT NULL,
          output_mode TEXT NOT NULL,
          content TEXT NOT NULL,
          created_at TEXT NOT NULL
        );
        INSERT OR IGNORE INTO app_meta(key, value) VALUES('schema_version', '1');
        "#,
    )
    .map_err(|e| format!("failed to init schema: {e}"))?;
    seed_builtin_templates(conn)?;
    Ok(())
}

fn seed_builtin_templates(conn: &Connection) -> Result<(), String> {
    let now = now_iso();
    let defaults = vec![
        ("tpl_builtin_files_ls", "List files", "files and directories", "linux-shell", "files-and-directories", "ls -la"),
        ("tpl_builtin_search_rg", "Search text", "search and filtering", "linux-shell", "search-and-filtering", "rg {{pattern}} {{path}}"),
        ("tpl_builtin_git_status", "Git status", "git", "linux-shell", "git", "git status"),
        ("tpl_builtin_docker_ps", "Docker list", "docker", "linux-shell", "docker", "docker ps -a"),
        ("tpl_builtin_tar", "Tar gzip", "compression", "linux-shell", "compression", "tar -czf {{archive}} {{path}}"),
        ("tpl_builtin_curl", "HTTP check", "networking", "linux-shell", "networking", "curl -I {{url}}"),
    ];
    for item in defaults {
        conn.execute(
            r#"INSERT OR IGNORE INTO templates(
              id,name,description,platform_kind,category,built_in,command_pattern,params_json,created_at,updated_at
            ) VALUES(?1,?2,?3,?4,?5,1,?6,'[]',?7,?7)"#,
            params![item.0, item.1, item.2, item.3, item.4, item.5, now],
        )
        .map_err(|e| format!("failed seeding template {}: {e}", item.0))?;
    }
    Ok(())
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
    let mut conn = connect(app)?;
    let mindmap_id = request.mindmap.id.clone();
    let mut tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("failed creating transaction: {e}"))?;
    let existed_version: Option<i64> = tx
        .query_row(
            "SELECT current_version FROM mindmaps WHERE id=?1",
            params![mindmap_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("failed checking mindmap version: {e}"))?;
    let next_version = existed_version.unwrap_or(request.mindmap.current_version) + 1;
    let now = request.metadata.updated_at.clone();
    let snapshot = serde_json::to_string(&GetMindmapDetailResponse {
        mindmap: serde_json::json!({
          "id": request.mindmap.id,
          "name": request.mindmap.name,
          "description": request.mindmap.description,
          "rootNodeId": request.mindmap.root_node_id,
          "activePathId": request.mindmap.active_path_id,
          "currentVersion": next_version,
          "lastBuildResultId": serde_json::Value::Null,
          "createdAt": now,
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
        ) VALUES(?1,?2,?3,?4,?5,?6,NULL,?7,?7,?8)
        ON CONFLICT(id) DO UPDATE SET
          name=excluded.name,
          description=excluded.description,
          root_node_id=excluded.root_node_id,
          active_path_id=excluded.active_path_id,
          current_version=excluded.current_version,
          updated_at=excluded.updated_at,
          snapshot_json=excluded.snapshot_json"#,
        params![
            mindmap_id,
            request.mindmap.name,
            request.mindmap.description,
            request.mindmap.root_node_id,
            request.mindmap.active_path_id,
            next_version,
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
