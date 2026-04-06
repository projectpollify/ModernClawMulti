use crate::services::database::Database;
use crate::types::Conversation;
use chrono::{DateTime, Utc};

pub struct ConversationRepository<'a> {
    db: &'a Database,
}

impl<'a> ConversationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, conversation: &Conversation) -> Result<(), String> {
        self.db.execute(
            r#"
            INSERT INTO conversations (id, title, model, message_count, preview, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            &[
                &conversation.id,
                &conversation.title,
                &conversation.model,
                &conversation.message_count,
                &conversation.preview,
                &conversation.created_at.to_rfc3339(),
                &conversation.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<Conversation>, String> {
        self.db.query_one(
            "SELECT id, title, model, message_count, preview, created_at, updated_at FROM conversations WHERE id = ?1",
            &[&id],
            |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model: row.get(2)?,
                    message_count: row.get(3)?,
                    preview: row.get(4)?,
                    created_at: parse_rfc3339(row.get(5)?)?,
                    updated_at: parse_rfc3339(row.get(6)?)?,
                })
            },
        )
    }

    pub fn list(&self, limit: Option<i32>) -> Result<Vec<Conversation>, String> {
        let limit = limit.unwrap_or(100);

        self.db.query_all(
            r#"
            SELECT id, title, model, message_count, preview, created_at, updated_at
            FROM conversations
            ORDER BY updated_at DESC
            LIMIT ?1
            "#,
            &[&limit],
            |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model: row.get(2)?,
                    message_count: row.get(3)?,
                    preview: row.get(4)?,
                    created_at: parse_rfc3339(row.get(5)?)?,
                    updated_at: parse_rfc3339(row.get(6)?)?,
                })
            },
        )
    }

    pub fn update(
        &self,
        id: &str,
        title: Option<&str>,
        preview: Option<&str>,
        message_count: Option<i32>,
    ) -> Result<(), String> {
        let now = Utc::now().to_rfc3339();

        if let Some(title) = title {
            self.db.execute(
                "UPDATE conversations SET title = ?1, updated_at = ?2 WHERE id = ?3",
                &[&title, &now, &id],
            )?;
        }

        if let Some(preview) = preview {
            self.db.execute(
                "UPDATE conversations SET preview = ?1, updated_at = ?2 WHERE id = ?3",
                &[&preview, &now, &id],
            )?;
        }

        if let Some(message_count) = message_count {
            self.db.execute(
                "UPDATE conversations SET message_count = ?1, updated_at = ?2 WHERE id = ?3",
                &[&message_count, &now, &id],
            )?;
        }

        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        self.db
            .execute("DELETE FROM conversations WHERE id = ?1", &[&id])?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: Option<i32>) -> Result<Vec<Conversation>, String> {
        let limit = limit.unwrap_or(20);
        let pattern = format!("%{}%", query);

        self.db.query_all(
            r#"
            SELECT id, title, model, message_count, preview, created_at, updated_at
            FROM conversations
            WHERE title LIKE ?1 OR preview LIKE ?1
            ORDER BY updated_at DESC
            LIMIT ?2
            "#,
            &[&pattern, &limit],
            |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    model: row.get(2)?,
                    message_count: row.get(3)?,
                    preview: row.get(4)?,
                    created_at: parse_rfc3339(row.get(5)?)?,
                    updated_at: parse_rfc3339(row.get(6)?)?,
                })
            },
        )
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
