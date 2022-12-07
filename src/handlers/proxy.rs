use dotenv_codegen::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::analytics::MessageInfo;
use tap::TapFallible;
use tracing::{info, warn};

use crate::handlers::{handshake_error, RpcQueryParams};
use crate::State;

use super::field_validation_error;

pub async fn handler(
    state: Arc<State>,
    sender: Option<SocketAddr>,
    method: hyper::http::Method,
    path: warp::path::FullPath,
    query_params: RpcQueryParams,
    headers: hyper::http::HeaderMap,
    body: hyper::body::Bytes,
) -> Result<impl warp::Reply, warp::Rejection> {
    // if query_params.project_id.is_empty() {
    //     return Ok(field_validation_error(
    //         "projectId",
    //         "No project id provided",
    //     ));
    // }

    // Project auth not need now but with time can be implemented, now just env
    // match state.registry.project_data(&query_params.project_id).await {
    //     Ok(project) => {
    //         if let Err(access_err) = project.validate_access(&query_params.project_id, None) {
    //             state.metrics.add_rejected_project();
    //             return Ok(handshake_error("projectId", format!("{:?}", access_err)));
    //         }
    //     }

    //     Err(err) => {
    //         state.metrics.add_rejected_project();
    //         return Ok(handshake_error("projectId", format!("{:?}", err)));
    //     }
    // }
    if query_params.project_id != dotenv!("PROJECT_ID") {
        return Ok(handshake_error("projectId", "Closed beta at now"));
    }
    let chain_id = query_params.chain_id.to_lowercase();
    let provider = match state.providers.get_provider_for_chain_id(&chain_id) {
        Some(provider) => provider,
        _ => {
            return Ok(field_validation_error(
                "chainId",
                format!("We don't support the chainId you provided: {}", chain_id),
            ))
        }
    };

    // TODO: map the response error codes properly
    // e.g. HTTP401 from target should map to HTTP500
    provider
        .proxy(method, path, query_params, headers, body)
        .await
        .map_err(|err| {
            warn!("{}", err);
            warp::reject::reject()
        })
}
