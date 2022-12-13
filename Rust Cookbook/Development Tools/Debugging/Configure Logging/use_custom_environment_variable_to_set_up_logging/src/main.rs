use env_logger::Builder;

fn main() {
    Builder::new().parse_env("MY_APP_LOG").init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}
