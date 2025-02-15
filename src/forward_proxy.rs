use async_trait::async_trait;

use pingora_core::prelude::HttpPeer;

use pingora_core::BError;

use pingora_proxy::ProxyHttp;
use pingora_proxy::Session;

pub struct ForwardProxy;

fn normalize_host(host: &str) -> String {
    if host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1".to_string()
    } else {
        host.to_string()
    }
}

#[async_trait]
impl ProxyHttp for ForwardProxy {
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
        let host_header = session
            .req_header()
            .headers
            .get("host")
            .and_then(|h| h.to_str().ok()) // Convert to str safely
            .unwrap_or_default(); // Use empty string if missing

        let (host, port) = if let Some((h, p)) = host_header.rsplit_once(':') {
            if let Ok(p) = p.parse::<u16>() {
                if p == 443 {
                    (h.to_string(), 80); // Change 443 to 80
                }
                (h.to_string(), p) // Use provided port
            } else {
                (host_header.to_string(), 80) // Default to port 80 if parsing fails
            }
        } else {
            (host_header.to_string(), 80) // Default to port 80 if none provided
        };

        let resolved_addr = format!("{}:{}", normalize_host(&host), port);

        println!("  proxy routing to {}", &resolved_addr);

        let peer = HttpPeer::new(
            // target URL
            resolved_addr,
            // TLS : false
            false,        // Enable TLS if needed
            host.clone(), // SNI hostname for TLS
        );

        Ok(Box::from(peer))
    }
}
