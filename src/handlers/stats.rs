use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{models::StatsResponse, AppError, AppState, Result};

#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "7d".to_string()
}

pub async fn stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Query(query): Query<StatsQuery>,
) -> Result<impl IntoResponse> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT id FROM links WHERE code = ?")
        .bind(&code)
        .fetch_optional(&state.db)
        .await?;

    let link_id = row
        .map(|r| r.0)
        .ok_or_else(|| AppError::NotFound("Link not found".into()))?;

    let time_filter = match query.period.as_str() {
        "1d" => "datetime('now', '-1 day')",
        "7d" => "datetime('now', '-7 days')",
        "30d" => "datetime('now', '-30 days')",
        "all" => "datetime('1970-01-01')",
        _ => "datetime('now', '-7 days')",
    };

    let pv: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM stats WHERE link_id = ? AND accessed_at >= {}",
        time_filter
    ))
    .bind(link_id)
    .fetch_one(&state.db)
    .await?;

    let uv: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(DISTINCT ip_hash) FROM stats WHERE link_id = ? AND accessed_at >= {}",
        time_filter
    ))
    .bind(link_id)
    .fetch_one(&state.db)
    .await?;

    let countries = get_grouped_stats(&state.db, link_id, "country", time_filter).await;
    let devices = get_grouped_stats(&state.db, link_id, "device_type", time_filter).await;
    let browsers = get_grouped_stats(&state.db, link_id, "browser", time_filter).await;
    let referer = get_grouped_stats(&state.db, link_id, "referer", time_filter).await;

    let response = StatsResponse {
        pv: pv.0,
        uv: uv.0,
        countries,
        devices,
        browsers,
        referer,
    };

    Ok((StatusCode::OK, Json(response)))
}

async fn get_grouped_stats(
    db: &sqlx::SqlitePool,
    link_id: i64,
    field: &str,
    time_filter: &str,
) -> HashMap<String, i64> {
    let query = format!(
        "SELECT {} as key, COUNT(*) as count FROM stats WHERE link_id = ? AND accessed_at >= {} GROUP BY {}",
        field, time_filter, field
    );
    
    let result: Vec<(Option<String>, i64)> = sqlx::query_as(&query)
        .bind(link_id)
        .fetch_all(db)
        .await
        .unwrap_or_default();

    result
        .into_iter()
        .filter_map(|(key, count)| key.map(|k| (k, count)))
        .collect()
}
