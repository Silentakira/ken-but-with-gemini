//! Local LLM features disabled - using Gemini API only

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAskRequest {
    pub prompt: String,
    pub model: String,
    #[serde(default)]
    pub history: Vec<HistoryTurn>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct LocalAskResponse {
    pub text: String,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct LocalModelListEntry {
    pub id: String,
    pub name: String,
    pub is_downloaded: bool,
    pub source: String,
}

#[tauri::command]
pub fn list_local_models(_app: AppHandle) -> Result<Vec<LocalModelListEntry>, String> {
    Ok(Vec::new()) // Return empty list - local models disabled
}

#[tauri::command]
pub async fn ask_local_model(
    _app: AppHandle,
    _request: LocalAskRequest,
) -> Result<LocalAskResponse, String> {
    Err("Local LLM features are disabled. Please use Gemini API instead.".to_string())
}
