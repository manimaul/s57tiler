use actix_web::error;

pub trait ErrMapper<T, E> {
    fn map_internal_server_error(self, log_msg: &str) -> Result<T, error::Error>;
    fn map_bad_request(self, log_msg: &str) -> Result<T, error::Error>;
    fn map_not_found(self, log_msg: &str) -> Result<T, error::Error>;
}

impl<T, E> ErrMapper<T, E> for Result<T, E>
    where
        E: std::fmt::Display,
{
    fn map_internal_server_error(self, log_msg: &str) -> Result<T, error::Error> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorInternalServerError("something bad happened")
        })
    }

    fn map_not_found(self, log_msg: &str) -> Result<T, error::Error> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorNotFound("no such resource")
        })
    }

    fn map_bad_request(self, log_msg: &str) -> Result<T, error::Error> {
        self.map_err(|err| {
            error!("{} - error {}", log_msg, err);
            error::ErrorBadRequest("something bad happened")
        })
    }
}
