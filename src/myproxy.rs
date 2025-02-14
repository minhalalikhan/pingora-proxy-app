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
            String::from("1.1.1.1:443"),
            // Enable TLS
            true,
            // Set SNI hostname for TLS handshake
            "one.one.one.one".to_string(),
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
            .insert_header("Host", "one.one.one.one")
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

        let uri = session.req_header().uri.to_string();

        let method = session.req_header().method.to_string();

        let (host, port) = if let Some((h, p)) = host_header.rsplit_once(':') {
            if let Ok(p) = p.parse::<u16>() {
                (h.to_string(), p) // Use provided port
            } else {
                (host_header.to_string(), 80) // Default to port 80 if parsing fails
            }
        } else {
            (host_header.to_string(), 80) // Default to port 80 if none provided
        };

        if session.req_header().method == "CONNECT" {
            println!("Handling CONNECT request for {}", session.req_header().uri);
        }

        let resolved_addr = format!("{}:{}", normalize_host(&host), port);
        let is_https = method == "CONNECT" || uri.starts_with("https");

        println!(
            " add of target is {} and port {} and connection will be secure : {}",
            &host, port, is_https
        );

        let peer = HttpPeer::new(
            // target URL
            resolved_addr,
            // TLS : false
            is_https,     // Enable TLS if needed
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

    // FORWARD PROXY
    // let mut fproxy = pingora_proxy::http_proxy_service(&server.configuration, ForwardProxy);

    // fproxy.add_tcp("0.0.0.0:6194");

    // server.add_service(fproxy);

    server.run_forever();
}
