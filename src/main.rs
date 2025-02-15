mod backend_proxy;
mod forward_proxy;

use pingora_core::prelude::Server;

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    // REVERSE PROXY

    let mut proxy =
        pingora_proxy::http_proxy_service(&server.configuration, backend_proxy::ReverseProxy);

    proxy.add_tcp("0.0.0.0:6193");

    server.add_service(proxy);

    println!("Reverse Proxy (for JSONplaceholder) running on port 6193");

    // FORWARD PROXY
    let mut fproxy =
        pingora_proxy::http_proxy_service(&server.configuration, forward_proxy::ForwardProxy);

    fproxy.add_tcp("0.0.0.0:6194");

    server.add_service(fproxy);

    println!("Forward Proxy running on port 6194");

    server.run_forever();
}
