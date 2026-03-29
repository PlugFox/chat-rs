//! TOML configuration for the chat server.

use std::path::Path;

use anyhow::{Context, bail};
use chat_protocol::types::ServerLimits;
use serde::Deserialize;

/// Top-level server configuration, deserialized from TOML.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default)]
    pub server: ServerSection,
    pub database: DatabaseSection,
    pub auth: AuthSection,
    #[serde(default)]
    pub limits: LimitsSection,
    #[serde(default)]
    pub rate_limits: RateLimitsSection,
}

#[derive(Debug, Deserialize)]
pub struct ServerSection {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_ws_send_buffer_size")]
    pub ws_send_buffer_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSection {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct AuthSection {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct LimitsSection {
    #[serde(default = "default_max_message_content_length")]
    pub max_message_content_length: u32,
    #[serde(default = "default_max_extra_size")]
    pub max_extra_size: u32,
    #[serde(default = "default_max_frame_size")]
    pub max_frame_size: u32,
    #[serde(default = "default_max_rich_content_size")]
    pub max_rich_content_size: u32,
    #[serde(default = "default_max_attachment_size")]
    pub max_attachment_size: u64,
    #[serde(default = "default_max_attachments_per_message")]
    pub max_attachments_per_message: u32,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitsSection {
    #[serde(default = "default_connections_per_minute_per_ip")]
    pub connections_per_minute_per_ip: u32,
    #[serde(default = "default_connections_burst")]
    pub connections_burst: u32,
    #[serde(default = "default_messages_per_minute_per_chat")]
    pub messages_per_minute_per_chat: u32,
    #[serde(default = "default_messages_burst")]
    pub messages_burst: u32,
    #[serde(default = "default_commands_per_minute")]
    pub commands_per_minute: u32,
    #[serde(default = "default_commands_burst")]
    pub commands_burst: u32,
}

// ---------------------------------------------------------------------------
// Defaults
// ---------------------------------------------------------------------------

fn default_host() -> String {
    "0.0.0.0".to_owned()
}
fn default_port() -> u16 {
    8080
}
fn default_ws_send_buffer_size() -> u32 {
    256
}
fn default_max_connections() -> u32 {
    20
}
fn default_max_message_content_length() -> u32 {
    4096
}
fn default_max_extra_size() -> u32 {
    4096
}
fn default_max_frame_size() -> u32 {
    32768
}
fn default_max_rich_content_size() -> u32 {
    8192
}
fn default_max_attachment_size() -> u64 {
    52_428_800
}
fn default_max_attachments_per_message() -> u32 {
    10
}
fn default_connections_per_minute_per_ip() -> u32 {
    10
}
fn default_connections_burst() -> u32 {
    3
}
fn default_messages_per_minute_per_chat() -> u32 {
    30
}
fn default_messages_burst() -> u32 {
    5
}
fn default_commands_per_minute() -> u32 {
    60
}
fn default_commands_burst() -> u32 {
    10
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            ws_send_buffer_size: default_ws_send_buffer_size(),
        }
    }
}

impl Default for LimitsSection {
    fn default() -> Self {
        Self {
            max_message_content_length: default_max_message_content_length(),
            max_extra_size: default_max_extra_size(),
            max_frame_size: default_max_frame_size(),
            max_rich_content_size: default_max_rich_content_size(),
            max_attachment_size: default_max_attachment_size(),
            max_attachments_per_message: default_max_attachments_per_message(),
        }
    }
}

impl Default for RateLimitsSection {
    fn default() -> Self {
        Self {
            connections_per_minute_per_ip: default_connections_per_minute_per_ip(),
            connections_burst: default_connections_burst(),
            messages_per_minute_per_chat: default_messages_per_minute_per_chat(),
            messages_burst: default_messages_burst(),
            commands_per_minute: default_commands_per_minute(),
            commands_burst: default_commands_burst(),
        }
    }
}

// ---------------------------------------------------------------------------
// Loading & validation
// ---------------------------------------------------------------------------

impl ServerConfig {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path).with_context(|| format!("reading config: {}", path.display()))?;
        let config: Self = toml::from_str(&content).with_context(|| format!("parsing config: {}", path.display()))?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration for required invariants.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.auth.jwt_secret.is_empty() {
            bail!("auth.jwt_secret must not be empty");
        }
        if self.database.url.is_empty() {
            bail!("database.url must not be empty");
        }
        Ok(())
    }
}

impl LimitsSection {
    /// Convert to the wire `ServerLimits` sent in the Welcome frame.
    pub fn to_server_limits(&self, rate_limits: &RateLimitsSection) -> ServerLimits {
        ServerLimits {
            ping_interval_ms: 30_000,
            ping_timeout_ms: 10_000,
            max_message_size: self.max_message_content_length,
            max_extra_size: self.max_extra_size,
            max_frame_size: self.max_frame_size,
            messages_per_sec: (rate_limits.messages_per_minute_per_chat / 60).max(1) as u16,
            connections_per_ip: rate_limits.connections_per_minute_per_ip.min(u16::MAX as u32) as u16,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let toml = r#"
[database]
url = "postgres://chat:chat@localhost/chat_db"

[auth]
jwt_secret = "test-secret"
"#;
        let config: ServerConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.limits.max_message_content_length, 4096);
        config.validate().unwrap();
    }

    #[test]
    fn parse_full_config() {
        let toml = r#"
[server]
host = "127.0.0.1"
port = 9090
ws_send_buffer_size = 128

[database]
url = "postgres://chat:chat@localhost/chat_db"
max_connections = 5

[auth]
jwt_secret = "my-secret"

[limits]
max_message_content_length = 2048
max_extra_size = 1024
max_frame_size = 16384
max_rich_content_size = 4096
max_attachment_size = 10485760
max_attachments_per_message = 5

[rate_limits]
connections_per_minute_per_ip = 20
connections_burst = 5
messages_per_minute_per_chat = 60
messages_burst = 10
commands_per_minute = 120
commands_burst = 20
"#;
        let config: ServerConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 9090);
        assert_eq!(config.database.max_connections, 5);
        assert_eq!(config.limits.max_message_content_length, 2048);
        assert_eq!(config.rate_limits.messages_per_minute_per_chat, 60);
    }

    #[test]
    fn defaults_for_missing_fields() {
        let toml = r#"
[database]
url = "postgres://localhost/test"

[auth]
jwt_secret = "secret"
"#;
        let config: ServerConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.ws_send_buffer_size, 256);
        assert_eq!(config.database.max_connections, 20);
        assert_eq!(config.limits.max_frame_size, 32768);
        assert_eq!(config.rate_limits.connections_burst, 3);
    }

    #[test]
    fn empty_jwt_secret_fails_validation() {
        let toml = r#"
[database]
url = "postgres://localhost/test"

[auth]
jwt_secret = ""
"#;
        let config: ServerConfig = toml::from_str(toml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn empty_database_url_fails_validation() {
        let toml = r#"
[database]
url = ""

[auth]
jwt_secret = "secret"
"#;
        let config: ServerConfig = toml::from_str(toml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn server_limits_conversion() {
        let limits = LimitsSection::default();
        let rate_limits = RateLimitsSection::default();
        let server_limits = limits.to_server_limits(&rate_limits);
        assert_eq!(server_limits.ping_interval_ms, 30_000);
        assert_eq!(server_limits.ping_timeout_ms, 10_000);
        assert_eq!(server_limits.max_message_size, 4096);
        assert_eq!(server_limits.max_extra_size, 4096);
        assert_eq!(server_limits.max_frame_size, 32768);
    }
}
