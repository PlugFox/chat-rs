//! # Chat Server
//!
//! WebSocket chat server built on axum + PostgreSQL.
//!
//! ## Architecture
//!
//! - [`config`] — TOML configuration parsing and validation
//! - [`state`] — shared application state (`AppState`, `SessionHandle`)
//! - [`app`] — axum router and middleware setup
//! - [`ws`] — WebSocket upgrade, frame loop, and session management
//! - [`handlers`] — frame handlers (auth, messages, subscriptions)
//! - [`db`] — PostgreSQL connection pool and SQL queries

pub mod app;
pub mod config;
pub mod db;
pub mod handlers;
pub mod state;
pub mod ws;
