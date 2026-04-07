use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::agent_repo::AgentRepository;
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
