use logger::log;

fn main() {
    logger::init();

    log::debug!("This is debug!");
    log::info!("This is info!");
    log::warn!("This is warning!");
    log::error!("This is error!");
}
