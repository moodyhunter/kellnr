use appstate::{AppState, CrateIoStorageState, DbState};
use axum::{extract::Path, Json};
use common::{original_name::OriginalName, version::Version};
use error::api_error::{ApiError, ApiResult};
use hyper::StatusCode;
use serde::Serialize;

use crate::{cratesio_api, kellnr_api, search_params::SearchParams};

pub async fn search(dbstate: DbState, params: SearchParams) -> ApiResult<String> {
    match kellnr_api::search(dbstate, params.clone()).await {
        Ok(Json(result)) => {
            let mut serializer = serde_json::Serializer::new(Vec::new());
            result.serialize(&mut serializer).or_else(|_| {
                Err(ApiError::new(
                    "Error serializing search result",
                    &"Serialization error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            })?;

            let json = String::from_utf8(serializer.into_inner()).or_else(|_| {
                Err(ApiError::new(
                    "Error converting search result to string",
                    &"String conversion error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            })?;

            Ok(json)
        }
        Err(_) => cratesio_api::search(params).await,
    }
}

pub async fn download(
    appstate: AppState,
    path: Path<(OriginalName, Version)>,
    crateio_storage_state: CrateIoStorageState,
    dbstate: DbState,
) -> ApiResult<Vec<u8>> {
    match kellnr_api::download(appstate, axum::extract::Path(path.clone())).await {
        Ok(result) => Ok(result),
        Err(_) => cratesio_api::download(
            axum::extract::Path(path.clone()),
            crateio_storage_state,
            dbstate,
        )
        .await
        .or_else(|err| {
            Err(ApiError::new(
                "Crates.io download error",
                &err,
                StatusCode::NOT_FOUND,
            ))
        }),
    }
}
