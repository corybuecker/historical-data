mod rendering;

use log::info;
use rendering::layouts;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

async fn route() {
    let socket: std::net::SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let routes = warp::any().map(|| format!("Hello {}, whose agent is {}", "test", "agent"));

    warp::serve(routes).run(socket).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!(
        "{:?}",
        layouts::index(&layouts::Contents::new(String::from(""), String::from(""),))
    );
    // route().await;
}
