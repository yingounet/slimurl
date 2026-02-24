use sqlx::SqlitePool;

use crate::utils::DeviceInfo;

pub struct StatsService {
    db: SqlitePool,
}

impl StatsService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn record_visit(
        &self,
        link_id: i64,
        ip_hash: Option<String>,
        device_info: &DeviceInfo,
        referer: Option<String>,
    ) {
        let query = sqlx::query(
            "INSERT INTO stats (link_id, accessed_at, ip_hash, country, device_type, browser, os, referer) VALUES (?, datetime('now'), ?, ?, ?, ?, ?, ?)"
        )
        .bind(link_id)
        .bind(&ip_hash)
        .bind(None::<String>)
        .bind(&device_info.device_type)
        .bind(&device_info.browser)
        .bind(&device_info.os)
        .bind(&referer);

        if let Err(e) = query.execute(&self.db).await {
            tracing::warn!("Failed to record stats: {}", e);
        }
    }
}

pub fn hash_ip(ip: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..8])
}
