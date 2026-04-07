use std::fs;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::agent_repo::{AgentRepository, DEFAULT_AGENT_ID};
use crate::services::conversation_repo::ConversationRepository;
use crate::types::Agent;
use crate::{DatabaseState, MemoryState};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDto {
    pub agent_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub workspace_path: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Agent> for AgentDto {
    fn from(value: Agent) -> Self {
        Self {
            agent_id: value.agent_id,
            name: value.name,
            description: value.description,
            status: Some(value.status),
            workspace_path: Some(value.workspace_path),
            default_model: value.default_model,
            created_at: Some(value.created_at),
            updated_at: Some(value.updated_at),
        }
    }
}

#[tauri::command]
pub async fn agent_list(state: State<'_, DatabaseState>) -> Result<Vec<AgentDto>, String> {
    let repo = AgentRepository::new(&state.db);
    Ok(repo.list()?.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn agent_get_active(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
) -> Result<AgentDto, String> {
    let repo = AgentRepository::new(&db_state.db);
    Ok(repo.get_active_agent(&memory_state.root_path)?.into())
}

#[tauri::command]
pub async fn agent_set_active(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
    agent_id: String,
) -> Result<(), String> {
    let repo = AgentRepository::new(&db_state.db);
    repo.ensure_default_agent(&memory_state.root_path)?;
    repo.set_active_agent(&agent_id)
}

#[tauri::command]
pub async fn agent_create(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
    agent: AgentDto,
) -> Result<(), String> {
    let repo = AgentRepository::new(&db_state.db);
    repo.ensure_default_agent(&memory_state.root_path)?;

    let now = Utc::now();
    let workspace_path = agent.workspace_path.unwrap_or_else(|| {
        AgentRepository::default_workspace_path_for_new_agent(&memory_state.root_path, &agent.agent_id)
    });

    let new_agent = Agent {
        agent_id: agent.agent_id,
        name: agent.name,
        description: agent.description,
        status: agent.status.unwrap_or_else(|| "active".to_string()),
        workspace_path: workspace_path.clone(),
        default_model: agent.default_model,
        created_at: agent.created_at.unwrap_or(now),
        updated_at: agent.updated_at.unwrap_or(now),
    };

    repo.create(&new_agent)?;

    let service = crate::services::memory::MemoryService::new(&workspace_path);
    service.initialize()?;

    Ok(())
}

#[tauri::command]
pub async fn agent_delete(
    db_state: State<'_, DatabaseState>,
    memory_state: State<'_, MemoryState>,
    agent_id: String,
) -> Result<(), String> {
    let repo = AgentRepository::new(&db_state.db);
    repo.ensure_default_agent(&memory_state.root_path)?;

    if agent_id == DEFAULT_AGENT_ID {
        return Err("The Rosie baseline brain cannot be deleted in this version.".to_string());
    }

    let agents = repo.list()?;
    if agents.len() <= 1 {
        return Err("You cannot delete the only remaining brain.".to_string());
    }

    let target = repo
        .get(&agent_id)?
        .ok_or_else(|| format!("Agent not found: {}", agent_id))?;

    let active_agent_id = repo.get_active_agent_id()?;
    if active_agent_id == agent_id {
        let replacement = agents
            .iter()
            .find(|agent| agent.agent_id != agent_id)
            .ok_or_else(|| "No replacement brain available.".to_string())?;
        repo.set_active_agent(&replacement.agent_id)?;
    }

    let conversation_repo = ConversationRepository::new(&db_state.db);
    conversation_repo.delete_for_agent(&agent_id)?;
    repo.delete(&agent_id)?;

    if is_managed_agent_workspace(&memory_state.root_path, &target.workspace_path) {
        fs::remove_dir_all(&target.workspace_path)
            .map_err(|error| format!("Failed to remove brain workspace: {}", error))?;
    }

    Ok(())
}

fn is_managed_agent_workspace(root_path: &str, workspace_path: &str) -> bool {
    let managed_root = Path::new(root_path).join("agents");
    Path::new(workspace_path).starts_with(managed_root)
}
