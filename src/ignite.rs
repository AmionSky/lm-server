use crate::pages::*;
use crate::SharedConfig;
use actix_files::Files;
use actix_web::{App, HttpServer};
use get_if_addrs::{get_if_addrs, IfAddr};
use std::io::ErrorKind;
use std::net::IpAddr;

#[actix_rt::main]
pub async fn start(app_cfg: SharedConfig) -> std::io::Result<()> {
    let ip = format!("{}:{}", get_local_ip()?, app_cfg.read().unwrap().port);

    HttpServer::new(move || {
        App::new()
            .data(app_cfg.clone())
            .service(index::index)
            .service(group::group)
            .service(cover::cover)
            .service(video::video)
            .service(sub::sub)
            .service(
                Files::new("/webapp", "webapp")
                    .index_file("index.html")
                    .redirect_to_slash_directory(),
            )
    })
    .bind(ip)?
    .run()
    .await
}

fn get_local_ip() -> Result<IpAddr, std::io::Error> {
    for iface in get_if_addrs()? {
        if let IfAddr::V4(addr) = &iface.addr {
            if addr.ip.is_private() {
                return Ok(IpAddr::from(addr.ip));
            }
        }
    }

    Err(std::io::Error::new(
        ErrorKind::Other,
        "Failed to get a local IPv4 address",
    ))
}
