const CARGO_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
pub const MAX_POST_SIZE: usize = 262_144; // max payload size is 256k

pub fn version() -> String {
    return match CARGO_VERSION {
        Some(ref p) => String::from(*p),
        None => String::from("unknown"),
    };
}