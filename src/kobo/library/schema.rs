use crate::library::models::Book;

use super::models::{
    ActivePeriod, BookEntitlement, BookMetadata, ContributorRole, CurrentDisplayPrice,
    CurrentLoveDisplayPrice, Entitlement, PhoneticPronunciations, Publisher, SyncEvent,
};
use chrono::SecondsFormat;
use uuid::Uuid;

impl SyncEvent {
    pub fn new(book: &Book) -> Self {
        Self {
            NewEntitlement: Entitlement::new(book),
        }
    }
}

impl Entitlement {
    pub fn new(book: &Book) -> Self {
        let mut new = Self::default(book.id);
        new.BookMetadata.Title = book.title.clone();
        new.BookMetadata.Description = book.description.clone();
        new.BookMetadata.DownloadUrls = vec![book.download_url()];
        new.BookMetadata.Contributors = book.authors.iter().map(|author| author.clone()).collect();
        new.BookMetadata.ContributorRoles = book
            .authors
            .iter()
            .map(|author| ContributorRole {
                Name: author.clone(),
            })
            .collect();
        new
    }

    pub fn default(uuid: Uuid) -> Entitlement {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            BookEntitlement: BookEntitlement {
                Accessibility: "Full".to_string(),
                ActivePeriod: ActivePeriod { From: now.clone() },
                Created: now.clone(),
                CrossRevisionId: uuid,
                Id: uuid,
                IsRemoved: false,
                IsHiddenFromArchive: false,
                IsLocked: false,
                LastModified: now.clone(),
                OriginCategory: "Imported".to_string(),
                RevisionId: uuid,
                Status: "Active".to_string(),
            },
            BookMetadata: BookMetadata {
                Categories: vec!["00000000-0000-0000-000000000001".to_string()],
                Contributors: vec![],
                ContributorRoles: vec![],
                CoverImageId: uuid,
                CrossRevisionId: uuid,
                CurrentDisplayPrice: CurrentDisplayPrice {
                    CurrencyCode: "USD".to_string(),
                    TotalAmount: 0,
                },
                CurrentLoveDisplayPrice: CurrentLoveDisplayPrice { TotalAmount: 0 },
                Description: "".to_string(),
                DownloadUrls: vec![],
                EntitlementId: uuid,
                ExternalIds: vec![],
                Genre: "00000000-0000-0000-000000000001".to_string(),
                IsEligibleForKoboLove: false,
                IsInternetArchive: false,
                IsPreOrder: false,
                IsSocialEnabled: true,
                Language: "en".to_string(),
                PhoneticPronunciations: PhoneticPronunciations {},
                PublicationDate: now.clone(),
                Publisher: Publisher {
                    Imprint: "".to_string(),
                    Name: "".to_string(),
                },
                RevisionId: uuid,
                Series: None,
                Title: "".to_string(),
                WorkId: uuid,
            },
        }
    }
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
