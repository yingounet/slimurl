use sqlx::SqlitePool;

use anyhow::Result;

pub async fn run_migrations(db: &SqlitePool) -> Result<()> {
    create_links_table(db).await?;
    create_users_table(db).await?;
    create_stats_table(db).await?;
    Ok(())
}

async fn create_links_table(db: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS links (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            code        TEXT NOT NULL UNIQUE,
            target_url  TEXT NOT NULL,
            link_type   INTEGER NOT NULL DEFAULT 1,
            expires_at  TEXT,
            user_id     INTEGER REFERENCES users(id),
            created_at  TEXT NOT NULL DEFAULT (datetime('now')),
            pv_count    INTEGER NOT NULL DEFAULT 0
        );
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_links_code ON links(code);")
        .execute(db)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_links_expires ON links(expires_at) WHERE link_type = 1 AND expires_at IS NOT NULL;"
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn create_users_table(db: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            email       TEXT NOT NULL UNIQUE,
            password    TEXT NOT NULL,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);")
        .execute(db)
        .await?;

    Ok(())
}

async fn create_stats_table(db: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS stats (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            link_id     INTEGER NOT NULL REFERENCES links(id),
            accessed_at TEXT NOT NULL,
            ip_hash     TEXT,
            country     TEXT,
            device_type TEXT,
            browser     TEXT,
            os          TEXT,
            referer     TEXT
        );
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_stats_link ON stats(link_id, accessed_at);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_stats_date ON stats(accessed_at);")
        .execute(db)
        .await?;

    Ok(())
}
