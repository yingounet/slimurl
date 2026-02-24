use std::io::Cursor;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use image::ImageFormat;
use qrcode::QrCode;
use serde::Deserialize;

use crate::{AppError, AppState, Result};

#[derive(Debug, Deserialize)]
pub struct QrParams {
    #[serde(default = "default_size")]
    pub size: u32,
    #[serde(default)]
    pub format: Option<String>,
}

fn default_size() -> u32 {
    200
}

pub async fn qrcode(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Query(params): Query<QrParams>,
) -> Result<impl IntoResponse> {
    let size = params.size.clamp(100, 1000);

    if state.cache.get_link(&code).await.is_none() {
        let row: Option<(String,)> = sqlx::query_as("SELECT target_url FROM links WHERE code = ?")
            .bind(&code)
            .fetch_optional(&state.db)
            .await?;
        
        if row.is_none() {
            return Err(AppError::NotFound("Link not found".into()));
        }
    }

    let qr_url = format!("{}/{}", state.config.base_url, code);
    
    let format = params.format.as_deref().unwrap_or("png");
    
    if format == "svg" {
        let svg = generate_svg(&qr_url, size)?;
        Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/svg+xml")],
            svg,
        )
            .into_response())
    } else {
        let png = generate_png(&qr_url, size)?;
        Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/png")],
            png,
        )
            .into_response())
    }
}

fn generate_png(url: &str, size: u32) -> Result<Vec<u8>> {
    let code = QrCode::new(url).map_err(|e| AppError::InternalError(e.to_string()))?;
    let image = code
        .render::<image::Luma<u8>>()
        .min_dimensions(size, size)
        .build();

    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|e| AppError::InternalError(e.to_string()))?;
    Ok(buffer)
}

fn generate_svg(url: &str, size: u32) -> Result<String> {
    let code = QrCode::new(url).map_err(|e| AppError::InternalError(e.to_string()))?;
    let svg = code
        .render()
        .min_dimensions(size, size)
        .dark_color(qrcode::render::svg::Color("#000000"))
        .light_color(qrcode::render::svg::Color("#ffffff"))
        .build();
    Ok(svg)
}
