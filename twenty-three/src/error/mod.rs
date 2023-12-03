use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No number was found on this line")]
    NoNumberFoundOnLine,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
