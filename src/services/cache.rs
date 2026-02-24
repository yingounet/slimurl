use std::time::Duration;

use moka::future::Cache as MokaCache;

use crate::models::Link;

#[derive(Clone, sqlx::FromRow)]
pub struct LinkEntry {
    pub target_url: String,
    pub link_type: i32,
    pub expires_at: Option<String>,
}

impl From<Link> for LinkEntry {
    fn from(link: Link) -> Self {
        Self {
            target_url: link.target_url,
            link_type: link.link_type as i32,
            expires_at: link.expires_at,
        }
    }
}

#[derive(Clone)]
pub struct Cache {
    links: MokaCache<String, LinkEntry>,
}

impl Cache {
    pub fn new(capacity: u64) -> Self {
        Self {
            links: MokaCache::builder()
                .max_capacity(capacity)
                .time_to_live(Duration::from_secs(300))
                .build(),
        }
    }

    pub async fn get_link(&self, code: &str) -> Option<LinkEntry> {
        self.links.get(code).await
    }

    pub async fn insert_link(&self, code: String, entry: LinkEntry) {
        self.links.insert(code, entry).await;
    }

    pub async fn remove_link(&self, code: &str) {
        self.links.invalidate(code).await;
    }
}
