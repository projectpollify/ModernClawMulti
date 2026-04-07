use std::path::Path;

use chrono::{DateTime, Utc};

use crate::services::database::Database;
use crate::types::Agent;

pub struct AgentRepository<'a> {
    db: &'a Database,
}

impl<'a> AgentRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, agent: &Agent) -> Result<(), String> {
        self.db.execute(
            r#"
            INSERT INTO agents (
                agent_id,
                name,
                description,
                status,
                workspace_path,
                default_model,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            &[
                &agent.agent_id,
                &agent.name,
                &agent.description,
                &agent.status,
                &agent.workspace_path,
                &agent.default_model,
                &agent.created_at.to_rfc3339(),
                &agent.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Agent>, String> {
        self.db.query_all(
            r#"
            SELECT agent_id, name, description, status, workspace_path, default_model, created_at, updated_at
            FROM agents
            ORDER BY updated_at DESC, name ASC
            "#,
            &[],
            |row| {
                Ok(Agent {
                    agent_id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    workspace_path: row.get(4)?,
                    default_model: row.get(5)?,
                    created_at: parse_rfc3339(row.get(6)?)?,
                    updated_at: parse_rfc3339(row.get(7)?)?,
                })
            },
        )
    }

    pub fn get(&self, agent_id: &str) -> Result<Option<Agent>, String> {
        self.db.query_one(
            r#"
            SELECT agent_id, name, description, status, workspace_path, default_model, created_at, updated_at
            FROM agents
            WHERE agent_id = ?1
            "#,
            &[&agent_id],
            |row| {
                Ok(Agent {
                    agent_id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    workspace_path: row.get(4)?,
                    default_model: row.get(5)?,
                    created_at: parse_rfc3339(row.get(6)?)?,
                    updated_at: parse_rfc3339(row.get(7)?)?,
                })
            },
        )
    }

    pub fn ensure_default_agent(&self, default_workspace_path: &str) -> Result<(), String> {
        let now = Utc::now().to_rfc3339();
        let default_name = "Default Brain";
        let default_status = "active";

        self.db.execute(
            r#"
            INSERT OR IGNORE INTO agents (
                agent_id,
                name,
                description,
                status,
                workspace_path,
                default_model,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            &[
                &"default",
                &default_name,
                &Some("Migrated baseline single-brain workspace".to_string()),
                &default_status,
                &default_workspace_path,
                &Option::<String>::None,
                &now,
                &now,
            ],
        )?;

        self.db.execute(
            r#"
            UPDATE agents
            SET workspace_path = ?1,
                updated_at = ?2
            WHERE agent_id = 'default'
            "#,
            &[&default_workspace_path, &now],
        )?;

        if self.db.get_setting("active_agent_id")?.is_none() {
            self.db.set_setting("active_agent_id", "default")?;
        }

        Ok(())
    }

    pub fn get_active_agent_id(&self) -> Result<String, String> {
        Ok(self
            .db
            .get_setting("active_agent_id")?
            .unwrap_or_else(|| "default".to_string()))
    }

    pub fn set_active_agent(&self, agent_id: &str) -> Result<(), String> {
        if self.get(agent_id)?.is_none() {
            return Err(format!("Agent not found: {}", agent_id));
        }

        self.db.set_setting("active_agent_id", agent_id)
    }

    pub fn get_active_agent(&self, default_workspace_path: &str) -> Result<Agent, String> {
        self.ensure_default_agent(default_workspace_path)?;

        let active_agent_id = self.get_active_agent_id()?;

        if let Some(agent) = self.get(&active_agent_id)? {
            return Ok(agent);
        }

        self.db.set_setting("active_agent_id", "default")?;
        self.get("default")?
            .ok_or_else(|| "Default agent could not be resolved".to_string())
    }

    pub fn resolve_active_workspace_path(&self, default_workspace_path: &str) -> Result<String, String> {
        Ok(self.get_active_agent(default_workspace_path)?.workspace_path)
    }

    pub fn default_workspace_path_for_new_agent(default_workspace_root: &str, agent_id: &str) -> String {
        Path::new(default_workspace_root)
            .join("agents")
            .join(agent_id)
            .to_string_lossy()
            .to_string()
    }
}

fn parse_rfc3339(value: String) -> rusqlite::Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                value.len(),
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })
}
