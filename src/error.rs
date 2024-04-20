use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Critical error")]
    CriticalError(String),
}
