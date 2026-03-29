-- M1: Initial schema
-- All timestamps are BIGINT (Unix seconds).
-- All IDs are INTEGER (u32 in Rust, i32 in PostgreSQL).

-- ============================================================================
-- Users
-- ============================================================================

CREATE TABLE users (
    id          SERIAL      PRIMARY KEY,
    external_id TEXT        NOT NULL UNIQUE,
    flags       SMALLINT    NOT NULL DEFAULT 0,
    created_at  BIGINT      NOT NULL,
    updated_at  BIGINT      NOT NULL
);

CREATE TABLE user_info (
    user_id    INTEGER     PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    username   VARCHAR(32),
    first_name VARCHAR(64),
    last_name  VARCHAR(64),
    avatar_url TEXT,
    updated_at BIGINT      NOT NULL
);

-- Partial unique index: only non-null usernames must be unique.
CREATE UNIQUE INDEX idx_user_info_username ON user_info(username)
    WHERE username IS NOT NULL;

-- ============================================================================
-- Device sessions
-- ============================================================================

CREATE TABLE sessions (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     INTEGER     NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id   UUID        NOT NULL,
    last_seen   BIGINT      NOT NULL,
    created_at  BIGINT      NOT NULL,

    CONSTRAINT uq_sessions_user_device UNIQUE (user_id, device_id)
);

CREATE INDEX idx_sessions_user_active ON sessions(user_id)
    WHERE id IS NOT NULL; -- all rows; used for listing a user's sessions

-- ============================================================================
-- Chats
-- ============================================================================

CREATE TABLE chats (
    id          SERIAL      PRIMARY KEY,
    parent_id   INTEGER     REFERENCES chats(id),
    kind        SMALLINT    NOT NULL,
    title       TEXT,
    avatar_url  TEXT,
    last_msg_id INTEGER     NOT NULL DEFAULT 0,
    created_at  BIGINT      NOT NULL,
    updated_at  BIGINT      NOT NULL,

    CONSTRAINT channel_requires_parent CHECK (kind != 2 OR parent_id IS NOT NULL),
    CONSTRAINT dm_no_parent            CHECK (kind != 0 OR parent_id IS NULL)
);

-- ============================================================================
-- Chat members
-- ============================================================================

CREATE TABLE chat_members (
    chat_id     INTEGER     NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id     INTEGER     NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role        SMALLINT    NOT NULL DEFAULT 0,
    permissions INTEGER,
    joined_at   BIGINT      NOT NULL,
    updated_at  BIGINT      NOT NULL,

    PRIMARY KEY (chat_id, user_id)
);

-- ============================================================================
-- DM index — guarantees at most one DM per user pair
-- ============================================================================

CREATE TABLE dm_index (
    chat_id INTEGER PRIMARY KEY REFERENCES chats(id) ON DELETE CASCADE,
    user_a  INTEGER NOT NULL,
    user_b  INTEGER NOT NULL,

    UNIQUE  (user_a, user_b),
    CHECK   (user_a < user_b)
);

-- ============================================================================
-- Messages (append-only — never DELETE, only UPDATE for soft-delete)
-- ============================================================================

CREATE TABLE messages (
    chat_id     INTEGER     NOT NULL,
    id          INTEGER     NOT NULL,
    sender_id   INTEGER     NOT NULL,
    created_at  BIGINT      NOT NULL,
    updated_at  BIGINT      NOT NULL,
    kind        SMALLINT    NOT NULL,
    flags       SMALLINT    NOT NULL DEFAULT 0,
    reply_to_id INTEGER,
    content     TEXT        NOT NULL,
    rich_content BYTEA,
    extra       TEXT,

    PRIMARY KEY (chat_id, id)
);

CREATE INDEX idx_messages_updated ON messages(chat_id, updated_at);

-- ============================================================================
-- Read receipts
-- ============================================================================

CREATE TABLE read_receipts (
    chat_id       INTEGER NOT NULL,
    user_id       INTEGER NOT NULL,
    last_read_id  INTEGER NOT NULL,

    PRIMARY KEY (chat_id, user_id)
);

-- ============================================================================
-- Reactions
-- ============================================================================

CREATE TABLE reactions (
    chat_id       INTEGER  NOT NULL,
    message_id    INTEGER  NOT NULL,
    user_id       INTEGER  NOT NULL,
    emoji_pack_id INTEGER  NOT NULL,
    emoji_idx     SMALLINT NOT NULL,

    PRIMARY KEY (chat_id, message_id, user_id, emoji_pack_id, emoji_idx)
);

-- ============================================================================
-- Idempotency keys (24h TTL, cleaned up by background job)
-- ============================================================================

CREATE TABLE idempotency_keys (
    key         UUID    PRIMARY KEY,
    chat_id     INTEGER NOT NULL,
    message_id  INTEGER NOT NULL,
    created_at  BIGINT  NOT NULL
);

CREATE INDEX idx_idempotency_created ON idempotency_keys(created_at);
