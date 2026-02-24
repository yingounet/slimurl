use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};

use crate::{
    middleware::{hash_ip, StatsService},
    services::LinkEntry,
    utils::parse_user_agent,
    AppError, AppState, Result,
};

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    let entry = get_link(&state, &code).await?;

    if entry.link_type == 1 {
        if let Some(ref expires_at) = entry.expires_at {
            if let Ok(expires) = DateTime::parse_from_rfc3339(expires_at) {
                if expires.with_timezone(&Utc) < Utc::now() {
                    state.cache.remove_link(&code).await;
                    return Err(AppError::Expired);
                }
            }
        }
    }

    let link_id = get_link_id(&state, &code).await;
    
    if let Some(id) = link_id {
        let stats_service = StatsService::new(state.db.clone());
        
        let ip = headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .split(',')
            .next()
            .unwrap_or("unknown")
            .trim();
        
        let ip_hash = hash_ip(ip);
        
        let user_agent = headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        
        let device_info = parse_user_agent(user_agent);
        
        let referer = headers
            .get("referer")
            .and_then(|v| v.to_str().ok())
            .and_then(|r| {
                url::Url::parse(r).ok().and_then(|u| {
                    u.host_str().map(|h| h.to_string())
                })
            });

        tokio::spawn(async move {
            stats_service.record_visit(id, Some(ip_hash), &device_info, referer).await;
        });
    }

    Ok((
        StatusCode::FOUND,
        [(header::LOCATION, entry.target_url)],
    ))
}

async fn get_link(state: &AppState, code: &str) -> Result<LinkEntry> {
    if let Some(entry) = state.cache.get_link(code).await {
        return Ok(entry);
    }

    let row = sqlx::query_as::<_, LinkEntry>(
        "SELECT target_url, link_type, expires_at FROM links WHERE code = ?"
    )
    .bind(code)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Link not found".into()))?;

    state.cache.insert_link(code.to_string(), row.clone()).await;
    Ok(row)
}

async fn get_link_id(state: &AppState, code: &str) -> Option<i64> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT id FROM links WHERE code = ?")
        .bind(code)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();
    row.map(|r| r.0)
}
