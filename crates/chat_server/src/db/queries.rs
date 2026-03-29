//! SQL queries for the chat server.
//!
//! Uses runtime-checked queries (`sqlx::query`) for M1. Will be migrated to
//! compile-time checked `sqlx::query!` once `sqlx prepare` is set up.

use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

/// Find a user by external_id. Returns `(id, flags)` if found.
pub async fn find_user_by_external_id(pool: &PgPool, external_id: &str) -> anyhow::Result<Option<(i32, i16)>> {
    let row = sqlx::query_as::<_, (i32, i16)>("SELECT id, flags FROM users WHERE external_id = $1")
        .bind(external_id)
        .fetch_optional(pool)
        .await
        .context("find_user_by_external_id")?;
    Ok(row)
}

/// Create a new user with the given external_id. Returns the new user's internal ID.
pub async fn create_user(pool: &PgPool, external_id: &str, now: i64) -> anyhow::Result<i32> {
    let (user_id,) = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO users (external_id, created_at, updated_at) VALUES ($1, $2, $2) RETURNING id",
    )
    .bind(external_id)
    .bind(now)
    .fetch_one(pool)
    .await
    .context("create_user")?;

    // Create empty user_info row.
    sqlx::query("INSERT INTO user_info (user_id, updated_at) VALUES ($1, $2)")
        .bind(user_id)
        .bind(now)
        .execute(pool)
        .await
        .context("create_user_info")?;

    Ok(user_id)
}

/// Find or create a user by external_id. Returns `(id, flags)`.
pub async fn find_or_create_user(pool: &PgPool, external_id: &str, now: i64) -> anyhow::Result<(i32, i16)> {
    if let Some(row) = find_user_by_external_id(pool, external_id).await? {
        return Ok(row);
    }
    let id = create_user(pool, external_id, now).await?;
    Ok((id, 0))
}

/// Upsert a device session. Returns the session UUID.
pub async fn upsert_device_session(pool: &PgPool, user_id: i32, device_id: Uuid, now: i64) -> anyhow::Result<Uuid> {
    let (session_id,) = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO sessions (user_id, device_id, last_seen, created_at) \
         VALUES ($1, $2, $3, $3) \
         ON CONFLICT ON CONSTRAINT uq_sessions_user_device \
         DO UPDATE SET last_seen = $3 \
         RETURNING id",
    )
    .bind(user_id)
    .bind(device_id)
    .bind(now)
    .fetch_one(pool)
    .await
    .context("upsert_device_session")?;
    Ok(session_id)
}

/// Check if a user is a member of a chat.
pub async fn check_chat_membership(pool: &PgPool, chat_id: i32, user_id: i32) -> anyhow::Result<bool> {
    let row = sqlx::query_scalar::<_, i32>("SELECT 1 FROM chat_members WHERE chat_id = $1 AND user_id = $2")
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .context("check_chat_membership")?;
    Ok(row.is_some())
}

/// Check if an idempotency key exists. Returns `(chat_id, message_id)` if found.
pub async fn check_idempotency_key(pool: &PgPool, key: Uuid) -> anyhow::Result<Option<(i32, i32)>> {
    let row = sqlx::query_as::<_, (i32, i32)>("SELECT chat_id, message_id FROM idempotency_keys WHERE key = $1")
        .bind(key)
        .fetch_optional(pool)
        .await
        .context("check_idempotency_key")?;
    Ok(row)
}

/// Insert a new idempotency key.
pub async fn insert_idempotency_key(
    pool: &PgPool,
    key: Uuid,
    chat_id: i32,
    message_id: i32,
    now: i64,
) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO idempotency_keys (key, chat_id, message_id, created_at) VALUES ($1, $2, $3, $4)")
        .bind(key)
        .bind(chat_id)
        .bind(message_id)
        .bind(now)
        .execute(pool)
        .await
        .context("insert_idempotency_key")?;
    Ok(())
}

/// Atomically increment chat's last_msg_id and insert a new message.
/// Returns `(message_id, created_at)`.
#[allow(clippy::too_many_arguments)]
pub async fn insert_message_atomic(
    pool: &PgPool,
    chat_id: i32,
    sender_id: i32,
    kind: i16,
    flags: i16,
    reply_to_id: Option<i32>,
    content: &str,
    rich_content: Option<&[u8]>,
    extra: Option<&str>,
    now: i64,
) -> anyhow::Result<(i32, i64)> {
    let row = sqlx::query_as::<_, (i32, i64)>(
        "WITH next AS ( \
             UPDATE chats SET last_msg_id = last_msg_id + 1, updated_at = $9 \
             WHERE id = $1 \
             RETURNING last_msg_id \
         ) \
         INSERT INTO messages (chat_id, id, sender_id, created_at, updated_at, kind, flags, reply_to_id, content, rich_content, extra) \
         SELECT $1, last_msg_id, $2, $9, $9, $3, $4, $5, $6, $7, $8 \
         FROM next \
         RETURNING id, created_at",
    )
    .bind(chat_id)      // $1
    .bind(sender_id)    // $2
    .bind(kind)         // $3
    .bind(flags)        // $4
    .bind(reply_to_id)  // $5
    .bind(content)      // $6
    .bind(rich_content) // $7
    .bind(extra)        // $8
    .bind(now)          // $9
    .fetch_one(pool)
    .await
    .context("insert_message_atomic")?;
    Ok(row)
}
