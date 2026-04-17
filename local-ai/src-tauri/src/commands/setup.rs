use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use reqwest::Client;
use tauri::State;
use tokio::time::sleep;

use crate::services::llama_cpp::resolve_local_model_path;
use crate::DatabaseState;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[tauri::command]
pub async fn setup_open_external(target: String) -> Result<(), String> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Err("No external target was provided.".to_string());
    }

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("cmd");
        command.arg("/C").arg("start").arg("").arg(trimmed);
        command.creation_flags(CREATE_NO_WINDOW);
        command
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(trimmed);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(trimmed);
        command
    };

    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Failed to open external target: {}", error))?;

    Ok(())
}

#[tauri::command]
pub async fn setup_start_ollama(state: State<'_, DatabaseState>) -> Result<(), String> {
    let model_path = read_string_setting(&state, "directEngineModelPath")?
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            "No GGUF model path is configured yet. Add it in Settings under llama-server Executable / GGUF Model Path, then try Start Engine again.".to_string()
        })?;

    stop_llama_server(&state);
    sleep(Duration::from_millis(300)).await;
    start_llama_server(&state, &model_path)?;
    wait_for_direct_engine().await
}

#[tauri::command]
pub async fn setup_switch_direct_engine_model(
    state: State<'_, DatabaseState>,
    model_name: String,
) -> Result<String, String> {
    let trimmed_name = model_name.trim();
    if trimmed_name.is_empty() {
        return Err("No model name was provided.".to_string());
    }

    let model_path = resolve_local_model_path(trimmed_name).ok_or_else(|| {
        format!(
            "Could not find a local GGUF file for {}. Make sure that model exists on disk first.",
            trimmed_name
        )
    })?;

    write_json_setting(&state, "directEngineModelPath", &model_path)?;
    write_json_setting(&state, "defaultModel", trimmed_name)?;

    stop_llama_server(&state);
    sleep(Duration::from_millis(500)).await;
    start_llama_server(&state, &model_path)?;
    wait_for_direct_engine().await?;

    Ok(model_path)
}

fn read_string_setting(state: &State<'_, DatabaseState>, key: &str) -> Result<Option<String>, String> {
    let value = state.db.get_setting(key)?;

    Ok(value.and_then(|raw| {
        serde_json::from_str::<String>(&raw).ok().or_else(|| {
            let trimmed = raw.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
    }))
}

fn write_json_setting(state: &State<'_, DatabaseState>, key: &str, value: &str) -> Result<(), String> {
    let serialized = serde_json::to_string(value).map_err(|error| error.to_string())?;
    state.db.set_setting(key, &serialized)
}

fn resolve_llama_server_path(configured: Option<&str>) -> Result<String, String> {
    let mut candidates = configured
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    if let Some(found_in_path) = find_in_path("llama-server.exe") {
        candidates.push(found_in_path);
    }

    candidates.extend(common_llama_server_locations());

    for candidate in candidates {
        let path = PathBuf::from(&candidate);
        if path.exists() {
            return Ok(candidate);
        }
    }

    Err(
        "Could not find llama-server.exe. Install llama.cpp first, or set the full llama-server path in Settings."
            .to_string(),
    )
}

fn common_llama_server_locations() -> Vec<String> {
    let mut candidates = Vec::new();

    if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
        let local_app_data = PathBuf::from(local_app_data);
        candidates.push(
            local_app_data
                .join("Programs")
                .join("llama.cpp")
                .join("llama-server.exe")
                .to_string_lossy()
                .to_string(),
        );
        candidates.push(
            local_app_data
                .join("Programs")
                .join("llama.cpp")
                .join("bin")
                .join("llama-server.exe")
                .to_string_lossy()
                .to_string(),
        );
        candidates.push(
            local_app_data
                .join("Programs")
                .join("LM Studio")
                .join("llama-server.exe")
                .to_string_lossy()
                .to_string(),
        );
    }

    if let Some(program_files) = std::env::var_os("ProgramFiles") {
        candidates.push(
            PathBuf::from(program_files)
                .join("llama.cpp")
                .join("llama-server.exe")
                .to_string_lossy()
                .to_string(),
        );
    }

    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        candidates.push(
            PathBuf::from(user_profile)
                .join("scoop")
                .join("shims")
                .join("llama-server.exe")
                .to_string_lossy()
                .to_string(),
        );
    }

    candidates
}

fn find_in_path(file_name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;

    for segment in std::env::split_paths(&path) {
        let candidate = segment.join(file_name);
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    None
}

fn start_llama_server(state: &State<'_, DatabaseState>, model_path: &str) -> Result<(), String> {
    let configured_executable = read_string_setting(state, "directEngineExecutablePath")?;
    let executable = resolve_llama_server_path(configured_executable.as_deref())?;

    if !Path::new(model_path).exists() {
        return Err(format!(
            "The configured GGUF model was not found at {}. Update the GGUF Model Path in Settings and try again.",
            model_path
        ));
    }

    let mut command = Command::new(&executable);
    command.arg("-m").arg(model_path);

    if let Some(mmproj_path) = infer_mmproj_path(model_path) {
        command.arg("--mmproj").arg(mmproj_path);
    }

    if let Some(alias) = infer_model_alias(model_path) {
        command.arg("--alias").arg(alias);
    }

    command
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg("8080")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let child = command
        .spawn()
        .map_err(|error| format!("Failed to start llama.cpp server: {}", error))?;

    let serialized_pid = serde_json::to_string(&child.id()).map_err(|error| error.to_string())?;
    state.db.set_setting("directEnginePid", &serialized_pid)?;

    Ok(())
}

fn stop_llama_server(state: &State<'_, DatabaseState>) {
    let tracked_pid = state
        .db
        .get_setting("directEnginePid")
        .ok()
        .flatten()
        .and_then(|raw| serde_json::from_str::<u32>(&raw).ok());

    if let Some(pid) = tracked_pid {
        let mut command = Command::new("taskkill");
        command.arg("/PID").arg(pid.to_string()).arg("/T").arg("/F");
        #[cfg(target_os = "windows")]
        {
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let _ = command.stdout(Stdio::null()).stderr(Stdio::null()).status();
    } else {
        let mut command = Command::new("taskkill");
        command.arg("/IM").arg("llama-server.exe").arg("/F");
        #[cfg(target_os = "windows")]
        {
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let _ = command.stdout(Stdio::null()).stderr(Stdio::null()).status();
    }

    let _ = state.db.delete_setting("directEnginePid");
}

async fn wait_for_direct_engine() -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(|error| format!("Failed to create direct-engine status client: {}", error))?;

    let url = "http://127.0.0.1:8080/v1/models";

    for _ in 0..30 {
        if let Ok(response) = client.get(url).send().await {
            if response.status().is_success() {
                return Ok(());
            }
        }

        sleep(Duration::from_millis(500)).await;
    }

    Err("The direct engine did not come online after starting. Check the executable path, GGUF model path, and local engine logs, then try again.".to_string())
}

fn infer_mmproj_path(model_path: &str) -> Option<String> {
    let model = Path::new(model_path);
    let parent = model.parent()?;
    let mut entries = std::fs::read_dir(parent).ok()?;

    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        let file_name = path.file_name()?.to_str()?.to_ascii_lowercase();
        if file_name.ends_with(".gguf") && file_name.starts_with("mmproj-") {
            return Some(path.to_string_lossy().to_string());
        }
    }

    None
}

fn infer_model_alias(model_path: &str) -> Option<&'static str> {
    let lower = Path::new(model_path)
        .file_name()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())?;

    if lower.contains("gemma-4-e4b") {
        Some("google/gemma-4-e4b")
    } else if lower.contains("gemma-4-e2b") {
        Some("google/gemma-4-e2b")
    } else {
        None
    }
}
