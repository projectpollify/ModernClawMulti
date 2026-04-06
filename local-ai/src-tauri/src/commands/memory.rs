use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::services::memory::MemoryService;
use crate::types::{CuratorPackage, DailyLog, MemoryContext, MemoryFile};

pub struct MemoryState {
    pub service: Arc<Mutex<MemoryService>>,
}

#[tauri::command]
pub async fn memory_initialize(state: State<'_, MemoryState>) -> Result<(), String> {
    let service = state.service.lock().await;
    service.initialize()
}

#[tauri::command]
pub async fn memory_read_file(
    state: State<'_, MemoryState>,
    filename: String,
) -> Result<MemoryFile, String> {
    let service = state.service.lock().await;
    service.read_file(&filename)
}

#[tauri::command]
pub async fn memory_write_file(
    state: State<'_, MemoryState>,
    filename: String,
    content: String,
) -> Result<(), String> {
    let service = state.service.lock().await;
    service.write_file(&filename, &content)
}

#[tauri::command]
pub async fn memory_get_today_log(state: State<'_, MemoryState>) -> Result<DailyLog, String> {
    let service = state.service.lock().await;
    service.get_today_log()
}

#[tauri::command]
pub async fn memory_append_log(
    state: State<'_, MemoryState>,
    entry: String,
) -> Result<(), String> {
    let service = state.service.lock().await;
    service.append_to_today_log(&entry)
}

#[tauri::command]
pub async fn memory_list_daily_logs(state: State<'_, MemoryState>) -> Result<Vec<String>, String> {
    let service = state.service.lock().await;
    service.list_daily_logs()
}

#[tauri::command]
pub async fn memory_list_knowledge(state: State<'_, MemoryState>) -> Result<Vec<String>, String> {
    let service = state.service.lock().await;
    service.list_knowledge_files()
}

#[tauri::command]
pub async fn memory_list_curator_staged(
    state: State<'_, MemoryState>,
) -> Result<Vec<CuratorPackage>, String> {
    let service = state.service.lock().await;
    service.list_curator_staged_packages()
}

#[tauri::command]
pub async fn memory_import_curator_package(
    state: State<'_, MemoryState>,
    folder_name: String,
) -> Result<String, String> {
    let service = state.service.lock().await;
    service.import_curator_package(&folder_name)
}

#[tauri::command]
pub async fn memory_reject_curator_package(
    state: State<'_, MemoryState>,
    folder_name: String,
) -> Result<(), String> {
    let service = state.service.lock().await;
    service.reject_curator_package(&folder_name)
}

#[tauri::command]
pub async fn memory_load_context(state: State<'_, MemoryState>) -> Result<MemoryContext, String> {
    let service = state.service.lock().await;
    service.load_context()
}

#[tauri::command]
pub async fn memory_get_base_path(state: State<'_, MemoryState>) -> Result<String, String> {
    let service = state.service.lock().await;
    Ok(service.get_base_path())
}

#[tauri::command]
pub async fn memory_open_folder(state: State<'_, MemoryState>) -> Result<(), String> {
    let service = state.service.lock().await;
    service.open_base_path()
}
