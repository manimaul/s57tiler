use std::env;

const CARGO_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref HOSTNAME: String = env::var("HOSTNAME")
    .expect("HOSTNAME not specified in .env");
}

lazy_static! {
    static ref PORT: i32 = env::var("PORT").ok()
    .and_then(|port| port.parse::<i32>().ok())
    .expect("PORT not specified in .env");
}

pub fn socket_address() -> String {
    format!("{}:{}", host_name(), port())
}

pub fn port() -> i32 {
    PORT.clone()
}

pub fn host_name() -> String {
    HOSTNAME.clone()
}

pub fn version() -> String {
    return match CARGO_VERSION {
        Some(ref p) => String::from(*p),
        None => String::from("unknown"),
    };
}
