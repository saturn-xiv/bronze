use std::ops::Deref;

use actix_web::http::StatusCode;
use clap::{App, Arg};

use super::{
    env,
    errors::{Error, Result},
};

pub fn launch() -> Result<()> {
    let matches = App::new(env::NAME)
        .version(env::VERSION)
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .about("Config file(.toml)")
                .default_value("config.toml")
                .takes_value(true),
        )
        .get_matches();
    debug!("run on debug mode");
    Ok(())
}
