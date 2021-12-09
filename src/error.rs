/// Generic thread-safe error type
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
/// Generic `Result` type that uses [`BoxError`]
pub type Result<T> = std::result::Result<T, BoxError>;
