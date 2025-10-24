pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    BadRequest,
    IncorrectCredentials,
    MissingToken,
    InvalidToken
}