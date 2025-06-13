use std::fmt;

/// Базовый тип для всех доменных ошибок
#[derive(Debug)]
pub enum DomainError {
    /// Ошибки валидации
    Validation(ValidationError),
    /// Ошибки бизнес-правил
    BusinessRule(BusinessRuleError),
    /// Ошибки аутентификации
    Authentication(AuthenticationError),
    /// Ошибки авторизации
    Authorization(AuthorizationError),
    /// Ошибки, связанные с JWT
    Jwt(JwtError),
    /// Ошибки, связанные с пользователем
    User(UserError),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Validation(e) => write!(f, "Validation error: {}", e),
            DomainError::BusinessRule(e) => write!(f, "Business rule error: {}", e),
            DomainError::Authentication(e) => write!(f, "Authentication error: {}", e),
            DomainError::Authorization(e) => write!(f, "Authorization error: {}", e),
            DomainError::Jwt(e) => write!(f, "JWT error: {}", e),
            DomainError::User(e) => write!(f, "User error: {}", e),
        }
    }
}

impl std::error::Error for DomainError {}

/// Ошибки валидации
#[derive(Debug)]
pub enum ValidationError {
    /// Пустое значение
    EmptyValue(String),
    /// Неверный формат
    InvalidFormat(String),
    /// Значение вне допустимого диапазона
    OutOfRange(String),
    /// Неверная длина
    InvalidLength(String),
    /// Неверный формат email
    InvalidEmail,
    /// Неверный формат пароля
    InvalidPassword,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyValue(field) => write!(f, "Field '{}' cannot be empty", field),
            ValidationError::InvalidFormat(field) => write!(f, "Field '{}' has invalid format", field),
            ValidationError::OutOfRange(field) => write!(f, "Field '{}' is out of valid range", field),
            ValidationError::InvalidLength(field) => write!(f, "Field '{}' has invalid length", field),
            ValidationError::InvalidEmail => write!(f, "Invalid email format"),
            ValidationError::InvalidPassword => write!(f, "Invalid password format"),
        }
    }
}

/// Ошибки бизнес-правил
#[derive(Debug)]
pub enum BusinessRuleError {
    /// Сущность не найдена
    EntityNotFound(String),
    /// Сущность уже существует
    EntityAlreadyExists(String),
    /// Неверное состояние
    InvalidState(String),
    /// Операция не разрешена
    OperationNotAllowed(String),
}

impl fmt::Display for BusinessRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessRuleError::EntityNotFound(entity) => write!(f, "{} not found", entity),
            BusinessRuleError::EntityAlreadyExists(entity) => write!(f, "{} already exists", entity),
            BusinessRuleError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            BusinessRuleError::OperationNotAllowed(msg) => write!(f, "Operation not allowed: {}", msg),
        }
    }
}

/// Ошибки аутентификации
#[derive(Debug)]
pub enum AuthenticationError {
    /// Неверные учетные данные
    InvalidCredentials,
    /// Учетная запись заблокирована
    AccountLocked,
    /// Учетная запись не активирована
    AccountNotActivated,
    /// Слишком много попыток входа
    TooManyAttempts,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthenticationError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthenticationError::AccountLocked => write!(f, "Account is locked"),
            AuthenticationError::AccountNotActivated => write!(f, "Account is not activated"),
            AuthenticationError::TooManyAttempts => write!(f, "Too many login attempts"),
        }
    }
}

/// Ошибки авторизации
#[derive(Debug)]
pub enum AuthorizationError {
    /// Недостаточно прав
    InsufficientPermissions,
    /// Роль не найдена
    RoleNotFound,
    /// Неверная роль
    InvalidRole,
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorizationError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AuthorizationError::RoleNotFound => write!(f, "Role not found"),
            AuthorizationError::InvalidRole => write!(f, "Invalid role"),
        }
    }
}

/// Ошибки JWT
#[derive(Debug)]
pub enum JwtError {
    /// Неверный токен
    InvalidToken,
    /// Токен истек
    TokenExpired,
    /// Неверная подпись
    InvalidSignature,
    /// Неверный формат токена
    InvalidFormat,
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::InvalidToken => write!(f, "Invalid token"),
            JwtError::TokenExpired => write!(f, "Token has expired"),
            JwtError::InvalidSignature => write!(f, "Invalid token signature"),
            JwtError::InvalidFormat => write!(f, "Invalid token format"),
        }
    }
}

/// Ошибки пользователя
#[derive(Debug)]
pub enum UserError {
    /// Пользователь не найден
    UserNotFound,
    /// Пользователь уже существует
    UserAlreadyExists,
    /// Неверный пароль
    InvalidPassword,
    /// Email уже используется
    EmailAlreadyInUse,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserNotFound => write!(f, "User not found"),
            UserError::UserAlreadyExists => write!(f, "User already exists"),
            UserError::InvalidPassword => write!(f, "Invalid password"),
            UserError::EmailAlreadyInUse => write!(f, "Email is already in use"),
        }
    }
} 