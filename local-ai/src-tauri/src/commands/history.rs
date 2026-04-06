use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::{conversation_repo::ConversationRepository, message_repo::MessageRepository};
use crate::types::{Conversation, Message};
use crate::DatabaseState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub tokens_used: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl From<MessageDto> for Message {
    fn from(value: MessageDto) -> Self {
        Self {
            id: value.id,
            conversation_id: value.conversation_id,
            role: value.role,
            content: value.content,
            tokens_used: value.tokens_used,
            created_at: value.created_at,
        }
    }
}

impl From<Message> for MessageDto {
    fn from(value: Message) -> Self {
        Self {
            id: value.id,
            conversation_id: value.conversation_id,
            role: value.role,
            content: value.content,
            tokens_used: value.tokens_used,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDto {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub model: String,
    pub message_count: i32,
    pub preview: Option<String>,
}

impl From<ConversationDto> for Conversation {
    fn from(value: ConversationDto) -> Self {
        Self {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
            model: value.model,
            message_count: value.message_count,
            preview: value.preview,
        }
    }
}

impl From<Conversation> for ConversationDto {
    fn from(value: Conversation) -> Self {
        Self {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
            model: value.model,
            message_count: value.message_count,
            preview: value.preview,
        }
    }
}

#[tauri::command]
pub async fn conversation_create(
    state: State<'_, DatabaseState>,
    conversation: ConversationDto,
) -> Result<(), String> {
    let repo = ConversationRepository::new(&state.db);
    repo.create(&conversation.into())
}

#[tauri::command]
pub async fn conversation_list(
    state: State<'_, DatabaseState>,
    limit: Option<i32>,
) -> Result<Vec<ConversationDto>, String> {
    let repo = ConversationRepository::new(&state.db);
    Ok(repo.list(limit)?.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn conversation_get(
    state: State<'_, DatabaseState>,
    id: String,
) -> Result<Option<ConversationDto>, String> {
    let repo = ConversationRepository::new(&state.db);
    Ok(repo.get(&id)?.map(Into::into))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn conversation_update(
    state: State<'_, DatabaseState>,
    id: String,
    title: Option<String>,
    preview: Option<String>,
    messageCount: Option<i32>,
) -> Result<(), String> {
    let repo = ConversationRepository::new(&state.db);
    repo.update(&id, title.as_deref(), preview.as_deref(), messageCount)
}

#[tauri::command]
pub async fn conversation_delete(
    state: State<'_, DatabaseState>,
    id: String,
) -> Result<(), String> {
    let repo = ConversationRepository::new(&state.db);
    repo.delete(&id)
}

#[tauri::command]
pub async fn conversation_search(
    state: State<'_, DatabaseState>,
    query: String,
    limit: Option<i32>,
) -> Result<Vec<ConversationDto>, String> {
    let repo = ConversationRepository::new(&state.db);
    Ok(repo.search(&query, limit)?.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn message_create(
    state: State<'_, DatabaseState>,
    message: MessageDto,
) -> Result<(), String> {
    let repo = MessageRepository::new(&state.db);
    repo.create(&message.into())
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn messages_get(
    state: State<'_, DatabaseState>,
    conversationId: String,
) -> Result<Vec<MessageDto>, String> {
    let repo = MessageRepository::new(&state.db);
    Ok(repo
        .get_for_conversation(&conversationId)?
        .into_iter()
        .map(Into::into)
        .collect())
}
