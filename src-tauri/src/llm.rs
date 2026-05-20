//! Local LLM features disabled - using Gemini API only

use std::path::Path;

pub fn load_model(_model_path: &Path) -> Result<(), String> {
    Err("Local LLM features are disabled. Please use Gemini API instead.".to_string())
}

pub fn generate(_prompt: &str, _max_tokens: u32) -> Result<String, String> {
    Err("Local LLM features are disabled. Please use Gemini API instead.".to_string())
}
