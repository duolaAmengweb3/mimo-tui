//! Local token-usage counter (`~/.mimo/usage.db`).
//!
//! Stores every API call's input/output tokens so `/usage` can show daily
//! / monthly trends without depending on the upstream plan API.

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};

use crate::paths;
use mimo_tui_anthropic_client::Usage as ApiUsage;

pub struct UsageDb {
    conn: Connection,
}

impl UsageDb {
    pub fn open() -> Result<Self> {
        paths::ensure_layout()?;
        let conn = Connection::open(paths::usage_db()?).context("open usage db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS calls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts TEXT NOT NULL,
                model TEXT NOT NULL,
                input_tokens INTEGER NOT NULL,
                output_tokens INTEGER NOT NULL,
                cache_creation_tokens INTEGER NOT NULL DEFAULT 0,
                cache_read_tokens INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn record(&self, model: &str, u: &ApiUsage) -> Result<()> {
        let ts = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO calls (ts, model, input_tokens, output_tokens, cache_creation_tokens, cache_read_tokens) VALUES (?, ?, ?, ?, ?, ?)",
            params![
                ts,
                model,
                u.input_tokens,
                u.output_tokens,
                u.cache_creation_input_tokens,
                u.cache_read_input_tokens,
            ],
        )?;
        Ok(())
    }

    pub fn totals_today(&self) -> Result<DailyTotals> {
        let today = Utc::now().date_naive().to_string(); // YYYY-MM-DD
        let mut stmt = self.conn.prepare(
            "SELECT IFNULL(SUM(input_tokens), 0),
                    IFNULL(SUM(output_tokens), 0),
                    IFNULL(SUM(cache_read_tokens), 0),
                    COUNT(*)
             FROM calls WHERE substr(ts, 1, 10) = ?",
        )?;
        let row = stmt.query_row([&today], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })?;
        Ok(DailyTotals {
            date: today,
            input_tokens: row.0 as u64,
            output_tokens: row.1 as u64,
            cache_read_tokens: row.2 as u64,
            call_count: row.3 as u64,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DailyTotals {
    pub date: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub call_count: u64,
}
