mod config;
mod server;
mod common;
mod client;
mod clipboard;


fn main() {
    match config::process() {
        Some(info) => {
            if info.is_server {
                server::run(&info.net_addr, &info.uid);
            } else {
                client::run(&info.net_addr, &info.uid);
            }
        }
        None => {}
    };
}
