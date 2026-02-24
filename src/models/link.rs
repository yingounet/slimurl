use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "INTEGER")]
#[repr(i32)]
pub enum LinkType {
    Temporary = 1,
    Permanent = 2,
}

impl Default for LinkType {
    fn default() -> Self {
        Self::Temporary
    }
}

impl From<i32> for LinkType {
    fn from(value: i32) -> Self {
        match value {
            2 => Self::Permanent,
            _ => Self::Temporary,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: i64,
    pub code: String,
    pub target_url: String,
    pub link_type: LinkType,
    pub expires_at: Option<String>,
    pub user_id: Option<i64>,
    pub created_at: String,
    pub pv_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
    #[serde(default)]
    pub link_type: Option<String>,
    #[serde(default)]
    pub expires_in: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkResponse {
    pub code: String,
    pub short_url: String,
    pub qr_url: String,
    pub expires_at: Option<String>,
}

impl Link {
    pub fn is_expired(&self) -> bool {
        if self.link_type == LinkType::Permanent {
            return false;
        }
        if let Some(ref expires_at) = self.expires_at {
            if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at) {
                return expires.with_timezone(&Utc) < Utc::now();
            }
        }
        false
    }
}

pub fn parse_expires_in(expires_in: &str) -> Option<String> {
    let now = Utc::now();
    let expires = match expires_in {
        "1h" => now + chrono::Duration::hours(1),
        "24h" => now + chrono::Duration::hours(24),
        "7d" => now + chrono::Duration::days(7),
        "30d" => now + chrono::Duration::days(30),
        _ => return None,
    };
    Some(expires.to_rfc3339())
}
