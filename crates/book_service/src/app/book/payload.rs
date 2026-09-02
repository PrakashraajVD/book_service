use jiff::civil::Date;
use serde::Deserialize;

use crate::models::BookStatus;

#[derive(Debug, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub published_date: Date,
    pub status: BookStatus,
}
