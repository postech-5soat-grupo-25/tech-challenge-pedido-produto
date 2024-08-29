#[derive(Debug)]
pub enum DomainError {
    AlreadyExists,
    Empty,
    Unauthorized,
    NotFound,
    Invalid(String),
    NonPositive,
    Database(String),
}