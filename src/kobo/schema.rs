use std::error::Error;

use super::models::{self, AuthDeviceResponse, SyncTokenModel, _SyncTokenData};
use anyhow::{anyhow, Context};
use chrono::{self, DateTime, Utc};

const CURRENT_SYNCTOKEN_VERSION: &'static str = "1-1-0";
const MIN_SYNCTOKEN_VERSION: &'static str = "1-0-0";
const SYNCTOKEN_HEADER: &'static str = "x-kobo-synctoken";

impl AuthDeviceResponse {
    pub fn from_user_key(user_key: impl ToString) -> AuthDeviceResponse {
        AuthDeviceResponse {
            AccessToken: "SAMPLE_ACCESS_TOKEN".to_string(),
            RefreshToken: "SAMPLE_REFRESH_TOKEN".to_string(),
            TokenType: "Bearer".to_string(),
            TrackingId: "SAMPLE_TRACKING_ID".to_string(),
            UserKey: user_key.to_string(),
        }
    }
}

impl SyncTokenModel {
    pub fn new(
        raw_kobo_store_token: impl ToString,
        books_last_modified: Option<DateTime<Utc>>,
        books_last_created: Option<DateTime<Utc>>,
        archive_last_modified: Option<DateTime<Utc>>,
        reading_state_last_modified: Option<DateTime<Utc>>,
        tags_last_modified: Option<DateTime<Utc>>,
    ) -> SyncTokenModel {
        let now = Utc::now();
        SyncTokenModel {
            version: CURRENT_SYNCTOKEN_VERSION.to_string(),
            data: _SyncTokenData {
                raw_kobo_store_token: raw_kobo_store_token.to_string(),
                books_last_modified: books_last_modified.unwrap_or(now),
                books_last_created: books_last_created.unwrap_or(now),
                archive_last_modified: archive_last_modified.unwrap_or(now),
                reading_state_last_modified: reading_state_last_modified.unwrap_or(now),
                tags_last_modified: tags_last_modified.unwrap_or(now),
            },
        }
    }

    pub fn default() -> SyncTokenModel {
        Self::new("", None, None, None, None, None)
    }

    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> anyhow::Result<SyncTokenModel> {
        let header_token = headers
            .get("x-kobo-synctoken")
            .context("sync token header not found")?;
        let header_token_str = header_token.to_str().unwrap_or("");
        if header_token_str == "" {
            return Err(anyhow!("empty header token"));
        }

        // from https://github.com/janeczku/calibre-web/blob/master/cps/services/SyncToken.py
        // line 104

        if header_token_str.contains(".") {
            // this is the kobo store token, we don't care about it
            // as we have our own format taken from calibre
            return Ok(Self::new(header_token_str, None, None, None, None, None));
        }

        Ok(Self::default())
    }
}
