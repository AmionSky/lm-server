use crate::pages::*;
use crate::SharedConfig;
use get_if_addrs::{get_if_addrs, IfAddr};
use rocket::config::{Config, Environment};
use std::error::Error;
use std::io::ErrorKind;
use std::net::IpAddr;

pub fn start(app_cfg: SharedConfig) -> Result<(), Box<dyn Error>> {
    let rocket_cfg = Config::build(Environment::Production)
        .address(get_local_ip()?.to_string())
        .port(app_cfg.read().unwrap().port)
        .finalize()?;

    rocket::custom(rocket_cfg)
        .manage(app_cfg)
        .mount(
            "/",
            routes![
                index::index,
                cover::cover,
                group::group,
                video::video,
                sub::sub
            ],
        )
        .launch();

    Ok(())
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
