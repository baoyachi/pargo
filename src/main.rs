#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate shadow_rs;

#[macro_use]
extern crate serde_derive;

mod api;
mod err;
mod util;

use crate::util::log::init_log;
use crate::api::server::start_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_conf()?;
    start_server().await;
    Ok(())
}

fn init_conf() -> anyhow::Result<()> {
    init_log()
}



