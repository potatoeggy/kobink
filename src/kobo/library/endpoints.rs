use anyhow::Result;
use axum::{
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::kobo::models::SyncTokenModel;

use super::models::SyncResponse;

pub fn create_library_router() -> Router {
    Router::new()
        .route("/:book_id/state", put(stub))
        .route("/tags/:book_or_tag_id_idk/items", post(stub))
        .route("/tags/:book_or_tag_id_idk/items/delete", post(stub))
        .route("/sync", get(handle_sync))
}

async fn stub() -> StatusCode {
    StatusCode::OK
}

async fn handle_sync(headers: HeaderMap) -> (StatusCode, Json<SyncResponse>) {
    let sync_token = SyncTokenModel::from_headers(&headers)?;
    (StatusCode::OK, Json(SyncResponse::new(sync_token)))
}

/*
schema provided here for future reference
2023-11-28T00:58:49.911833Z  INFO kobink::server::logging: PUT /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/3c0dcc0f-ae46-4070-a1d5-60f2ef15370c/state: {"ReadingStates": [{"CurrentBookmark": {"ContentSourceProgressPercent": 0,"LastModified": "2023-11-28T00:55:55Z","Location": {"Source": "OEBPS/file0001.xhtml","Type": "KoboSpan","Value": "kobo.1.1"},"ProgressPercent": 0},"EntitlementId": "3c0dcc0f-ae46-4070-a1d5-60f2ef15370c","LastModified": "2023-11-28T00:55:55Z","Statistics": {"LastModified": "2023-11-28T00:55:55Z","RemainingTimeMinutes": 64,"SpentReadingMinutes": 0},"StatusInfo": {"LastModified": "2023-11-28T00:55:55Z","Status": "Reading"}}]}
2023-11-28T00:58:49.912025Z  INFO kobink::server::logging: response /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/3c0dcc0f-ae46-4070-a1d5-60f2ef15370c/state: <empty>
2023-11-28T00:58:50.044994Z  INFO kobink::server::logging: POST /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/492ecec4-8d81-43ab-a62a-a5ceb28bd609/items: {"Items": [{"RevisionId": "4f8277d7-7347-4609-a2d4-8996bccb01ba","Type": "ProductRevisionTagItem"},{"RevisionId": "c6b73322-28c9-4132-9038-2d2a68f17549","Type": "ProductRevisionTagItem"},{"RevisionId": "0f61362f-b028-4422-9638-05a9723c1fc3","Type": "ProductRevisionTagItem"},{"RevisionId": "93571935-524c-445b-9c24-20ba47a1ca62","Type": "ProductRevisionTagItem"},{"RevisionId": "c0c9fad7-23e3-4e12-9fd6-2de6a1d70002","Type": "ProductRevisionTagItem"},{"RevisionId": "30fa7c81-61cc-456c-a9e2-a8601b74f9d6","Type": "ProductRevisionTagItem"},{"RevisionId": "0cb50f1e-5ae5-4efe-a4ab-dd2be298441c","Type": "ProductRevisionTagItem"},{"RevisionId": "a9a7e8eb-642f-4050-b837-38a7b938a675","Type": "ProductRevisionTagItem"},{"RevisionId": "2979f767-fa61-45c0-82d3-47be0338c81c","Type": "ProductRevisionTagItem"},{"RevisionId": "9f8f8c98-95e8-43b1-a3ac-0c7bea546a62","Type": "ProductRevisionTagItem"}]}
2023-11-28T00:58:50.045164Z  INFO kobink::server::logging: response /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/492ecec4-8d81-43ab-a62a-a5ceb28bd609/items: <empty>
2023-11-28T00:58:50.125975Z  INFO kobink::server::logging: POST /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/e339533e-fd07-4a6d-8bfd-c08bd22b4a5d/items/delete: {"Items": [{"RevisionId": "f09b17b5-4d29-42ed-9e4f-b9ccce878a12","Type": "ProductRevisionTagItem"},{"RevisionId": "4739b6d5-fe7d-4b20-ba64-e7547854957e","Type": "ProductRevisionTagItem"},{"RevisionId": "950ce9e3-e261-4500-b481-baec1a604f92","Type": "ProductRevisionTagItem"},{"RevisionId": "c1465dce-63d4-4f2e-859f-8bfb4e7b380a","Type": "ProductRevisionTagItem"},{"RevisionId": "47c00f9a-3715-4b34-ad95-48b87d8f0f56","Type": "ProductRevisionTagItem"},{"RevisionId": "768579cc-2078-445a-b75c-7bc438ce8428","Type": "ProductRevisionTagItem"},{"RevisionId": "5adcb754-4800-4158-8dc5-f0ac63742dd5","Type": "ProductRevisionTagItem"},{"RevisionId": "a7fbe8fd-d66f-4cd4-bf50-eb46970c8be7","Type": "ProductRevisionTagItem"},{"RevisionId": "03dbe74e-33d9-4827-b942-b80802649795","Type": "ProductRevisionTagItem"},{"RevisionId": "2aee3b02-fef4-4ebb-bc59-10df63f10cc1","Type": "ProductRevisionTagItem"},{"RevisionId": "0bfe2405-2dc0-480f-99da-1daf4564c7bb","Type": "ProductRevisionTagItem"},{"RevisionId": "449bda1b-1319-4638-a614-a8ba04025f7e","Type": "ProductRevisionTagItem"},{"RevisionId": "adc8f389-2d2d-4f0d-bdab-44e4a548abc1","Type": "ProductRevisionTagItem"}]}
2023-11-28T00:58:50.126107Z  INFO kobink::server::logging: response /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/e339533e-fd07-4a6d-8bfd-c08bd22b4a5d/items/delete: <empty>
2023-11-28T00:58:50.215432Z  INFO kobink::server::logging: POST /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/9531b8f0-be27-47b5-a82b-724cec5619d7/items: {"Items": [{"RevisionId": "dee9dbed-22b3-4d67-b200-b06162e0354f","Type": "ProductRevisionTagItem"},{"RevisionId": "4bda4dab-d50c-46bc-91ea-ccee33def0d1","Type": "ProductRevisionTagItem"},{"RevisionId": "20da1412-3629-4fd1-859f-873bf8c0ec29","Type": "ProductRevisionTagItem"},{"RevisionId": "e56cae4f-74ca-4e35-9598-84230b56f58f","Type": "ProductRevisionTagItem"},{"RevisionId": "81471b5e-0489-4dba-8a00-92b46f6f3413","Type": "ProductRevisionTagItem"},{"RevisionId": "6635b850-2582-4479-9040-ce7355d32468","Type": "ProductRevisionTagItem"},{"RevisionId": "07da880a-985a-4a4b-a995-0c12dd4cdd20","Type": "ProductRevisionTagItem"},{"RevisionId": "b2a3912b-03a8-4fad-89b5-4d86378b4cfe","Type": "ProductRevisionTagItem"},{"RevisionId": "a9a7e8eb-642f-4050-b837-38a7b938a675","Type": "ProductRevisionTagItem"},{"RevisionId": "0591e567-f975-4dc1-b60d-a5f3b215fd3a","Type": "ProductRevisionTagItem"},{"RevisionId": "23bb3431-e5b5-488d-81f6-f69d83820321","Type": "ProductRevisionTagItem"},{"RevisionId": "c26486a5-63ed-4fa2-ae83-bed799183bc7","Type": "ProductRevisionTagItem"},{"RevisionId": "09443749-a03f-4d2f-b94b-476db8d8e462","Type": "ProductRevisionTagItem"},{"RevisionId": "4f8277d7-7347-4609-a2d4-8996bccb01ba","Type": "ProductRevisionTagItem"},{"RevisionId": "f6d32c90-cbc7-4b19-acde-ea5696837a5f","Type": "ProductRevisionTagItem"},{"RevisionId": "c97b5460-604f-4b5d-9d34-c44bf93a24ea","Type": "ProductRevisionTagItem"},{"RevisionId": "1172a2be-63fa-4626-88ba-c7d9b18728ac","Type": "ProductRevisionTagItem"},{"RevisionId": "62d00d8f-c773-464a-be26-4f08370e70f1","Type": "ProductRevisionTagItem"},{"RevisionId": "3d25b153-63f8-44fe-b749-fa02fc4fe602","Type": "ProductRevisionTagItem"},{"RevisionId": "2316cf41-35f7-483b-b54b-3d40129cae64","Type": "ProductRevisionTagItem"}]}
2023-11-28T00:58:50.215569Z  INFO kobink::server::logging: response /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/tags/9531b8f0-be27-47b5-a82b-724cec5619d7/items: <empty>
2023-11-28T00:58:50.349891Z  INFO kobink::server::logging: GET /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/sync: <empty>
2023-11-28T00:58:50.349959Z  INFO kobink::server::logging: response /kobo/16c0b8d261663e45fc1c4c22207abc32/v1/library/sync: <empty>
 */
