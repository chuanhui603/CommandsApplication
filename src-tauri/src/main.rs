mod storage;

use serde_json::Value;
use tauri::AppHandle;

#[tauri::command]
fn app_initialize(app: AppHandle) -> Result<storage::AppInitializeResponse, String> {
    storage::app_initialize(&app)
}

#[tauri::command]
fn get_storage_status(app: AppHandle) -> storage::StorageStatusResponse {
    storage::get_storage_status(&app)
}

#[tauri::command]
fn list_mindmaps(app: AppHandle) -> Result<storage::ListMindmapsResponse, String> {
    storage::list_mindmaps(&app)
}

#[tauri::command]
fn get_mindmap_detail(app: AppHandle, mindmap_id: String) -> Result<storage::GetMindmapDetailResponse, String> {
    storage::get_mindmap_detail(&app, mindmap_id)
}

#[tauri::command]
fn save_mindmap_snapshot(
    app: AppHandle,
    request: storage::SaveMindmapSnapshotRequest,
) -> Result<storage::SaveMindmapSnapshotResponse, String> {
    storage::save_mindmap_snapshot(&app, request)
}

#[tauri::command]
fn list_templates(
    app: AppHandle,
    request: storage::ListTemplatesRequest,
) -> Result<storage::ListTemplatesResponse, String> {
    storage::list_templates(&app, request)
}

#[tauri::command]
fn clone_builtin_template(
    app: AppHandle,
    request: storage::CloneBuiltinTemplateRequest,
) -> Result<(), String> {
    storage::clone_builtin_template(&app, request)
}

#[tauri::command]
fn export_json_to_file(path: String, payload: Value) -> Result<(), String> {
    storage::export_json_to_file(path, payload)
}

#[tauri::command]
fn import_json_from_file(path: String) -> Result<storage::JsonImportResponse, String> {
    storage::import_json_from_file(path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            app_initialize,
            get_storage_status,
            list_mindmaps,
            get_mindmap_detail,
            save_mindmap_snapshot,
            list_templates,
            clone_builtin_template,
            export_json_to_file,
            import_json_from_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
