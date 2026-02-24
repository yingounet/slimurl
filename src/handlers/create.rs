use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::{CreateLinkRequest, CreateLinkResponse, LinkType},
    services::LinkEntry,
    utils::generate_code,
    AppError, AppState, Result,
};

pub async fn create_link(
    State(state): State<AppState>,
    Json(req): Json<CreateLinkRequest>,
) -> Result<impl IntoResponse> {
    if req.url.len() > 2048 {
        return Err(AppError::BadRequest("URL too long (max 2048 chars)".into()));
    }

    if !req.url.starts_with("http://") && !req.url.starts_with("https://") {
        return Err(AppError::BadRequest("URL must start with http:// or https://".into()));
    }

    let (link_type, expires_at) = match req.link_type.as_deref() {
        Some("permanent") => (LinkType::Permanent, None),
        _ => {
            let expires_in = req.expires_in.as_deref().unwrap_or("24h");
            let expires_at = crate::models::parse_expires_in(expires_in)
                .ok_or_else(|| AppError::BadRequest("Invalid expires_in value".into()))?;
            (LinkType::Temporary, Some(expires_at))
        }
    };

    let code = generate_code();
    
    sqlx::query(
        "INSERT INTO links (code, target_url, link_type, expires_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&code)
    .bind(&req.url)
    .bind(link_type as i32)
    .bind(&expires_at)
    .execute(&state.db)
    .await?;

    let short_url = format!("{}/{}", state.config.base_url, code);
    let qr_url = format!("{}/qrcode", short_url);

    let response = CreateLinkResponse {
        code: code.clone(),
        short_url,
        qr_url,
        expires_at,
    };

    state.cache.insert_link(
        code,
        LinkEntry {
            target_url: req.url,
            link_type: link_type as i32,
            expires_at: response.expires_at.clone(),
        },
    ).await;

    Ok((StatusCode::CREATED, Json(response)))
}
