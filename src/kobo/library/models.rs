#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type SyncResponse = Vec<SyncEvent>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncEvent {
    pub NewEntitlement: Entitlement, // can also be ChangedEntitlement
                                     // pub ChangedReadingState: // on calibre
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entitlement {
    pub BookEntitlement: BookEntitlement,
    pub BookMetadata: BookMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookEntitlement {
    pub Accessibility: String,
    pub ActivePeriod: ActivePeriod,
    pub Created: String,
    pub CrossRevisionId: Uuid,
    pub Id: Uuid,
    pub IsRemoved: bool,
    pub IsHiddenFromArchive: bool,
    pub IsLocked: bool,
    pub LastModified: String,
    pub OriginCategory: String,
    pub RevisionId: Uuid,
    pub Status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivePeriod {
    pub From: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookMetadata {
    pub Categories: Vec<String>,
    pub Contributors: Vec<Contributor>,
    pub ContributorRoles: Vec<ContributorRole>,
    pub CoverImageId: Uuid,
    pub CrossRevisionId: Uuid,
    pub CurrentDisplayPrice: CurrentDisplayPrice,
    pub CurrentLoveDisplayPrice: CurrentLoveDisplayPrice,
    pub Description: String,
    pub DownloadUrls: Vec<DownloadUrl>,
    pub EntitlementId: Uuid,
    pub ExternalIds: Vec<ExternalId>,
    pub Genre: String,
    pub IsEligibleForKoboLove: bool,
    pub IsInternetArchive: bool,
    pub IsPreOrder: bool,
    pub IsSocialEnabled: bool,
    pub Language: String,
    pub PhoneticPronunciations: PhoneticPronunciations,
    pub PublicationDate: String,
    pub Publisher: Publisher,
    pub RevisionId: Uuid,
    pub Series: Option<Series>,
    pub Title: String,
    pub WorkId: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contributor {
    pub Name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributorRole {
    pub Name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentDisplayPrice {
    pub CurrencyCode: String,
    pub TotalAmount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentLoveDisplayPrice {
    pub TotalAmount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadUrl {
    pub Format: String,
    pub Size: u64,
    pub Url: String,
    pub Platform: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalId {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneticPronunciations {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub Imprint: String,
    pub Name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub Name: String,
    pub Number: u64,
    pub NumberFloat: f64,
    pub Id: Uuid,
}

/*
 * {
    "BookEntitlement": {
        "Accessibility": "Full",
        "ActivePeriod": {
            "From": "timestamp, datetime.now"
        },
        "Created": "timestamp, book creation date",
        "CrossRevisionId": "book uuid",
        "Id": "book uuid",
        "IsRemoved": "bool",
        "IsHiddenFromArchive": false,
        "IsLocked": false,
        "LastModified": "timestamp, book last modified date",
        "OriginCategory": "Imported",
        "RevisionId": "book uuid",
        "Status": "Active"
    },
    "BookMetadata": {
        "Categories": ["00000000-0000-0000-000000000001"],
        "Contributors": [{"Name": "author name"}],
        "ContributorRoles": [{"Name": "author name"}],
        "CoverImageId": "book uuid",
        "CrossRevisionId":"book uuid",
        "CurrentDisplayPrice": {"CurrencyCode": "USD", "TotalAmount": 0},
        "CurrentLoveDisplayPrice": {"TotalAmount": 0},
        "Description": "description",
        "DownloadUrls": [
            {
                "Format": "EPUB",
                "Size": "book file size in bytes",
                "Url": "book download url",
                "Platform": "Generic"
            }
        ],
        "EntitlementId": "book uuid",
        "ExternalIds": [],
        "Genre": "00000000-0000-0000-000000000001",
        "IsEligibleForKoboLove": false,
        "IsInternetArchive": false,
        "IsPreOrder": false,
        "IsSocialEnabled": true,
        "Language": "en",
        "Phonetic-Pronunciations": {},
        "PublicationDate": "timestamp, book publication date",
        "Publisher": {"Imprint": "", "Name": "publisher name"},
        "RevisionId":"book uuid",
        "Series": {
            "Name": "series name",
            "Number": 1,
            "NumberFloat": 1.0,
            "Id": "series uuid"
        }, // optional, can be omitted
        "Title": "book title",
        "WorkId": "book uuid"
    }
}
 */
