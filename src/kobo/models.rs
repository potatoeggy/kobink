#![allow(non_snake_case)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDeviceRequest {
    pub AffiliateName: String,
    pub AppVersion: String,
    pub ClientKey: String,
    pub DeviceId: String,
    pub PlatformId: String,
    pub SerialNumber: String,
    pub UserKey: String,
}

// from calibre-web,
#[derive(Debug, Serialize, Deserialize)]
pub struct _SyncTokenData {
    pub raw_kobo_store_token: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub books_last_modified: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub books_last_created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub archive_last_modified: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub reading_state_last_modified: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub tags_last_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncTokenModel {
    pub version: String,
    pub data: _SyncTokenData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDeviceResponse {
    pub AccessToken: String,
    pub RefreshToken: String,
    pub TokenType: String,
    pub TrackingId: String,
    pub UserKey: String,
}
