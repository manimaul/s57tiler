use actix_web::error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait ErrMapper<T, E> {
    fn map_internal_server_error(self, log_msg: &str) -> Result<T>;
    fn map_bad_request(self, log_msg: &str) -> Result<T>;
    fn map_not_found(self, log_msg: &str) -> Result<T>;
}

impl<T, E> ErrMapper<T, E> for std::result::Result<T, E>
    where
        E: std::fmt::Display,
{
    fn map_internal_server_error(self, log_msg: &str) -> Result<T> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorInternalServerError("something bad happened")
        })
    }

    fn map_bad_request(self, log_msg: &str) -> Result<T> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorBadRequest("something bad happened")
        })
    }

    fn map_not_found(self, log_msg: &str) -> Result<T> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorNotFound("no such resource")
        })
    }
}
