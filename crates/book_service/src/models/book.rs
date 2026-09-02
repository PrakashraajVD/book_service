use jiff::{Timestamp, civil::Date};
use serde::{Deserialize, Serialize};
use toasty::{Embed, Model};
use uuid::Uuid;

#[derive(Debug, Model, Serialize)]
pub struct Book {
    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,

    #[key]
    #[auto]
    pub id: Uuid,

    pub published_date: Date,
    pub status: BookStatus,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Embed, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[column(type=u8)]
pub enum BookStatus {
    #[column(variant = 0)]
    Pending,

    #[column(variant = 1)]
    Verified,
}
