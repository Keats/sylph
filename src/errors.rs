
pub type SylphResult<T> = Result<T, SylphError>;

#[derive(Debug)]
pub enum SylphError {
    NotFound,
    NotAllowed,
    InternalServerError
}
