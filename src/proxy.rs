use async_trait::async_trait;
use log::info;
use pingora_core::{server::Server, upstreams::peer::HttpPeer, Result};
use pingora_http::RequestHeader;
use pingora_load_balancing::{selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();
        info!("Selected upstream: {:?}", upstream);
        Ok(Box::new(HttpPeer::new(
            upstream,
            true,
            "www.google.com".to_string(),
        )))
    }

    // async fn upstream_request_filter(
    //     &self,
    //     _session: &mut Session,
    //     upstream_request: &mut RequestHeader,
    //     _ctx: &mut Self::CTX,
    // ) -> Result<()> {
    //     upstream_request
    //         .insert_header("Host", "one.one.one.one")
    //         .unwrap();
    //     Ok(())
    // }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "www.google.com")
            .unwrap();
        upstream_request
            .insert_header("User-Agent", "Mozilla/5.0 (compatible; PingoraProxy)")
            .unwrap();
        upstream_request
            .insert_header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .unwrap();
        upstream_request
            .insert_header("Accept-Language", "en-US,en;q=0.5")
            .unwrap();
        upstream_request
            .insert_header("Connection", "keep-alive")
            .unwrap();
        Ok(())
    }
}

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let upstreams = LoadBalancer::try_from_iter(["www.google.com:443"]).unwrap();
    let mut proxy_service =
        pingora_proxy::http_proxy_service(&server.configuration, LB(Arc::new(upstreams)));

    proxy_service.add_tcp("0.0.0.0:6188");

    server.add_service(proxy_service);
    server.run_forever();
}
