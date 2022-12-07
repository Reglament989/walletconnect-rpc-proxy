use super::{RpcProvider, RpcQueryParams};
use crate::error::{RpcError, RpcResult};
use async_trait::async_trait;
use hyper::{client::HttpConnector, Body, Client, Response};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;
use tracing::info;

#[derive(Clone)]
pub struct BscProvider {
    pub client: Client<HttpsConnector<HttpConnector>>,
    // pub project_id: String,  Not needed
    pub supported_chains: HashMap<String, String>,
}

#[async_trait]
impl RpcProvider for BscProvider {
    async fn proxy(
        &self,
        method: hyper::http::Method,
        _path: warp::path::FullPath,
        query_params: RpcQueryParams,
        _headers: hyper::http::HeaderMap,
        body: hyper::body::Bytes,
    ) -> RpcResult<Response<Body>> {
        let uri = match query_params.chain_id.to_lowercase().as_str() {
            "eip155:56" => Some("https://bsc-dataseed.binance.org"),
            "eip155:97" => Some("https://data-seed-prebsc-1-s1.binance.org:8545"),
            _ => None,
        }
        .ok_or(RpcError::ChainNotFound)?;

        let hyper_request = hyper::http::Request::builder()
            .method(method)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(hyper::body::Body::from(body))?;

        // TODO: map the response error codes properly
        // e.g. HTTP401 from target should map to HTTP500
        Ok(self.client.request(hyper_request).await?)
    }

    fn supports_caip_chainid(&self, chain_id: &str) -> bool {
        self.supported_chains.contains_key(chain_id)
    }

    fn supported_caip_chainids(&self) -> Vec<String> {
        self.supported_chains.keys().cloned().collect()
    }
}
