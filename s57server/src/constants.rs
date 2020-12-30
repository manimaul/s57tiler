const CARGO_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn version() -> String {
    return match CARGO_VERSION {
        Some(ref p) => String::from(*p),
        None => String::from("unknown"),
    };
}
