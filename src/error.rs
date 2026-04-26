use axum::{Json, http::StatusCode, response::IntoResponse};
use jsonwebtoken::errors::ErrorKind as JwtErr;
use mongodb::error::{Error as MongoDBErr, ErrorKind as MdbErrKind};

use exn::Exn;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Error {
    #[serde(rename = "type")]
    pub error_type: Source,
    message: String,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Authentication(AuthenticationError),
    Database(DatabaseError),
    Internal,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationError {
    InvalidFormat,
    InvalidIssuer,
    InvalidAlgorithm,
    InvalidSignature,
    InvalidKey,
    InvalidClaim,
    Missing,
    NoMatchingKey,
    InvalidTimeRange,
    InsufficientPermissions,
    NotInNetwork,
    Unknown,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseError {
    Serialization,
    Deserialization,
    Insertion,
    Conflict,
    NotFound,
    Other,
}

impl Error {
    #[must_use]
    pub const fn authentication(reason: AuthenticationError, message: String) -> Self {
        Self {
            error_type: Source::Authentication(reason),
            message,
        }
    }

    #[must_use]
    pub const fn database(reason: DatabaseError, message: String) -> Self {
        Self {
            error_type: Source::Database(reason),
            message,
        }
    }

    #[must_use]
    pub const fn upstream(message: String) -> Self {
        Self {
            error_type: Source::Internal,
            message,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl std::error::Error for Error {}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.into_kind() {
            JwtErr::InvalidToken
            | JwtErr::InvalidKeyFormat
            | JwtErr::Base64(_)
            | JwtErr::Json(_)
            | JwtErr::Utf8(_) => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidFormat),
                message: "JWT could not be decoded or deserialized".into(),
            },

            JwtErr::InvalidSignature => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidSignature),
                message: "JWT signature does not match".into(),
            },

            JwtErr::InvalidEcdsaKey | JwtErr::InvalidEddsaKey | JwtErr::InvalidRsaKey(_) => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidSignature),
                message: "Cryptographic key is not correctly formatted".into(),
            },

            JwtErr::InvalidAlgorithmName | JwtErr::InvalidAlgorithm | JwtErr::MissingAlgorithm => {
                Self {
                    error_type: Source::Authentication(AuthenticationError::InvalidAlgorithm),
                    message: "Algorythm does not match key id".into(),
                }
            }

            JwtErr::MissingRequiredClaim(err) | JwtErr::InvalidClaimFormat(err) => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidClaim),
                message: format!("Missing or invalid claim(s): {err}"),
            },

            JwtErr::ExpiredSignature | JwtErr::ImmatureSignature => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidTimeRange),
                message: "JWT is expired or immature. Re-authentication is required".into(),
            },

            JwtErr::InvalidIssuer => Self {
                error_type: Source::Authentication(AuthenticationError::InvalidIssuer),
                message: "JWT is not issued by a trusted source".into(),
            },

            JwtErr::InvalidAudience => Self {
                error_type: Source::Authentication(AuthenticationError::InsufficientPermissions),
                message: "This JWT does not grant permissions for this operation".into(),
            },

            _ => Self {
                error_type: Source::Authentication(AuthenticationError::Unknown),
                message: "Something unexpected happened during authentication".into(),
            },
        }
    }
}

impl From<MongoDBErr> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        match *value.kind {
            MdbErrKind::BsonSerialization(err) => Self {
                error_type: Source::Database(DatabaseError::Serialization),
                message: format!("Serialization failed: {err:?}"),
            },

            MdbErrKind::BsonDeserialization(err) => Self {
                error_type: Source::Database(DatabaseError::Deserialization),
                message: format!("Deserialization failed: {err:?}"),
            },

            MdbErrKind::InsertMany(err) => Self {
                error_type: Source::Database(DatabaseError::Insertion),
                message: format!("Failed to insert: {err:?}"),
            },

            MdbErrKind::Write(err) => Self {
                error_type: Source::Database(DatabaseError::Conflict),
                message: format!("Failed to write the value: {err:?}"),
            },

            err => Self {
                error_type: Source::Internal,
                message: format!("{err:?}"),
            },
        }
    }
}

#[derive(Debug)]
pub struct ErrorResponse(pub Exn<Error>);

impl ErrorResponse {
    #[must_use]
    pub fn insufficient_permissions() -> Self {
        Self(exn::Exn::new(Error::authentication(
            AuthenticationError::InsufficientPermissions,
            "This user is not authorized to do this operation".into(),
        )))
    }
}

impl std::convert::From<exn::Exn<Error>> for ErrorResponse {
    fn from(value: Exn<Error>) -> Self {
        Self(value)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        // The root cause is the one that's propagated to the caller
        let mut lowest_level = self.0.frame();

        while let Some(lower_level) = lowest_level.children().first() {
            lowest_level = lower_level;
        }

        log::warn!("{lowest_level:?}");

        #[allow(clippy::option_if_let_else)]
        let error: &Error = match lowest_level.error().downcast_ref() {
            Some(error) => error,
            None => &Error::upstream("Failed to downcast error. This should never happen".into()),
        };

        let http_code = match error.error_type {
            // Authentication errors
            Source::Authentication(AuthenticationError::Unknown) => {
                log::warn!("{error:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Source::Authentication(_) => StatusCode::UNAUTHORIZED,

            // Database errors
            Source::Database(DatabaseError::Serialization | DatabaseError::Deserialization) => {
                StatusCode::BAD_REQUEST
            }

            Source::Database(DatabaseError::NotFound) => StatusCode::NOT_FOUND,

            Source::Database(DatabaseError::Conflict) => StatusCode::CONFLICT,

            // Anything else
            _ => {
                log::warn!("{error:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (http_code, Json(error)).into_response()
    }
}
