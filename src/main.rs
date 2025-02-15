use async_trait::async_trait;

use pingora_core::prelude::HttpPeer;
use pingora_core::prelude::Server;
use pingora_core::upstreams::peer::Peer;
use pingora_core::BError;
use pingora_core::Error;
use pingora_core::ErrorType;

use pingora_core::OkOrErr;
use pingora_core::Result;
use pingora_http::RequestHeader;
use pingora_http::ResponseHeader;
use pingora_proxy::ProxyHttp;
use pingora_proxy::Session;
use std::net::ToSocketAddrs;

struct ReverseProxy;

struct ForwardProxy;

fn normalize_host(host: &str) -> String {
    if host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1".to_string()
    } else {
        host.to_string()
    }
}

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

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    // REVERSE PROXY

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, ReverseProxy);

    proxy.add_tcp("0.0.0.0:6193");

    server.add_service(proxy);

    println!("Reverse Proxy (for google) running on port 6193");

    // FORWARD PROXY
    let mut fproxy = pingora_proxy::http_proxy_service(&server.configuration, ForwardProxy);

    fproxy.add_tcp("0.0.0.0:6194");

    server.add_service(fproxy);

    println!("Forward Proxy running on port 6194");

    server.run_forever();
}
