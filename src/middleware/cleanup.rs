use std::time::Duration;

use sqlx::SqlitePool;

pub fn start_cleanup_task(db: SqlitePool) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(600));

        loop {
            interval.tick().await;

            let result = sqlx::query(
                "DELETE FROM links WHERE link_type = 1 AND expires_at IS NOT NULL AND datetime(expires_at) < datetime('now')"
            )
            .execute(&db)
            .await;

            match result {
                Ok(res) => {
                    let deleted = res.rows_affected();
                    if deleted > 0 {
                        tracing::info!("Cleaned up {} expired links", deleted);
                    }
                }
                Err(e) => {
                    tracing::warn!("Cleanup task failed: {}", e);
                }
            }
        }
    })
}
