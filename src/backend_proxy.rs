use async_trait::async_trait;

use pingora_core::prelude::HttpPeer;

use pingora_core::BError;

use pingora_core::Result;

use pingora_proxy::ProxyHttp;
use pingora_proxy::Session;

pub struct ReverseProxy;

#[async_trait]
impl ProxyHttp for ReverseProxy {
    type CTX = ();

    /// Define how the `ctx` should be created.
    fn new_ctx(&self) -> Self::CTX {}

    /// Define where the proxy should send the request to.

    /// The returned [HttpPeer] contains the information regarding where and how this request should
    /// be forwarded to.
    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora_core::Result<Box<HttpPeer>, BError> {
        let peer = HttpPeer::new(
            // target URL
            String::from("jsonplaceholder.typicode.com:443"),
            // Enable TLS
            true,
            // Set SNI hostname for TLS handshake
            "jsonplaceholder.typicode.com".to_string(),
        );

        Ok(Box::from(peer))
    }
    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "jsonplaceholder.typicode.com")
            .unwrap();
        Ok(())
    }
}
