use std::env;

lazy_static! {
    static ref EXT_SCHEME: String = env::var("EXT_SCHEME")
    .expect("EXT_SCHEME not specified in .env");

    static ref EXT_PORT: String = env::var("EXT_PORT").ok()
    .expect("EXT_PORT not specified in .env");

    static ref EXT_HOSTNAME: String = env::var("EXT_HOSTNAME")
    .expect("EXT_HOSTNAME not specified in .env");

    static ref BIND_ADDR: String = env::var("BIND_ADDR")
    .expect("HOSTNAME not specified in .env");

    static ref BIND_PORT: String = env::var("BIND_PORT").ok()
    .expect("BIND_PORT not specified in .env");
}

const CARGO_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn external_base_url() -> String {
    format!("{}://{}:{}", EXT_SCHEME.clone(), EXT_HOSTNAME.clone(), port(true))
}

pub fn local_socket_address() -> String {
    format!("{}:{}", BIND_ADDR.clone(), port(false))
}

fn port(external: bool) -> i32 {
    if external {
        Some(&EXT_PORT)
            .and_then(|port| port.parse::<i32>().ok())
            .expect("could not parse EXT_PORT specified in .env")
    } else {
        Some(&BIND_PORT)
            .and_then(|port| port.parse::<i32>().ok())
            .expect("could not parse BIND_PORT specified in .env")
    }
}

pub fn version() -> String {
    return match CARGO_VERSION {
        Some(ref p) => String::from(*p),
        None => String::from("unknown"),
    };
}
