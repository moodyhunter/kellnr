use appstate::{CratesIoPrefetchSenderState, DbState, SettingsState};
use axum::{
    extract::{Path, State},
    Json,
};
use common::{original_name::OriginalName, prefetch::Prefetch};
use hyper::{HeaderMap, StatusCode};

use crate::{
    config_json::ConfigJson,
    cratesio_prefetch_api::{prefetch_cratesio, prefetch_len2_cratesio},
    kellnr_prefetch_api::{prefetch_kellnr, prefetch_len2_kellnr},
};

pub async fn config_unified(State(settings): SettingsState) -> Json<ConfigJson> {
    Json(ConfigJson::from((&(*settings), "unified")))
}

pub async fn prefetch_unified(
    path: Path<(String, String, OriginalName)>,
    headers: HeaderMap,
    state: DbState,
    sender_state: CratesIoPrefetchSenderState,
) -> Result<Prefetch, StatusCode> {
    match prefetch_kellnr(
        axum::extract::Path(path.clone()),
        headers.clone(),
        state.clone(),
    )
    .await
    {
        Ok(prefetch) => Ok(prefetch),
        Err(_) => {
            // TODO: check if crates.io is enabled
            prefetch_cratesio(
                axum::extract::Path(path.clone()),
                headers.clone(),
                state.clone(),
                sender_state,
            )
            .await
        }
    }
}

pub async fn prefetch_len2_unified(
    path: Path<(String, OriginalName)>,
    headers: HeaderMap,
    dbstate: DbState,
    sender_state: CratesIoPrefetchSenderState,
) -> Result<Prefetch, StatusCode> {
    match prefetch_len2_kellnr(
        axum::extract::Path(path.clone()),
        headers.clone(),
        dbstate.clone(),
    )
    .await
    {
        Ok(prefetch) => Ok(prefetch),
        Err(_) => {
            prefetch_len2_cratesio(
                axum::extract::Path(path.clone()),
                headers.clone(),
                dbstate.clone(),
                sender_state,
            )
            .await
        }
    }
}
