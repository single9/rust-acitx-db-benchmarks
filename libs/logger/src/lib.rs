use env_logger::Env;
use std::env;

pub extern crate log;

pub fn init() {
    let env = Env::new().filter("LOG_LEVEL");

    env_logger::init_from_env(env);

    log::debug!(
        "logger initialized. Level: {}",
        env::var("LOG_LEVEL").unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        init();
    }
}
