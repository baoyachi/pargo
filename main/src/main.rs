#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate shadow_rs;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod api;
mod config;
mod err;
mod index;
mod util;

use crate::api::server::start_server;
use crate::util::log::init_log;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_conf()?;
    start_server().await?;
    Ok(())
}

fn init_conf() -> anyhow::Result<()> {
    init_log()
}
