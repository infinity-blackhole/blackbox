use tonic::{Request, Response, Status};

use crate::error::GameError;

/// Shared request context extracted by interceptors.
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: Option<i64>,
    pub session_id: Option<String>,
    pub platform: String,
    pub version: String,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            platform: "unknown".to_string(),
            version: String::new(),
        }
    }
}

/// Extract platform and version from request metadata.
pub fn extract_platform(req: &Request<()>) -> RequestContext {
    let platform = req
        .metadata()
        .get("x-platform")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let version = req
        .metadata()
        .get("x-version")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    RequestContext {
        platform,
        version,
        ..Default::default()
    }
}

/// Attach server timestamp to response metadata.
pub fn attach_server_time(resp: &mut Response<()>) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    resp.metadata_mut()
        .insert("x-server-time", now.to_string().parse().unwrap_or_else(|_| {
            tonic::metadata::MetadataValue::from_static("0")
        }));
}

/// Convert a GameError to a tonic Status.
pub fn game_error_to_status(err: &GameError) -> Status {
    match err {
        GameError::NotFound(_) => Status::not_found(err.to_string()),
        GameError::InvalidRequest(_) => Status::invalid_argument(err.to_string()),
        GameError::Auth(auth_err) => match auth_err {
            blackbox_auth::AuthError::InvalidToken
            | blackbox_auth::AuthError::TokenExpired
            | blackbox_auth::AuthError::Unauthorized => {
                Status::unauthenticated(err.to_string())
            }
            _ => Status::permission_denied(err.to_string()),
        },
        GameError::Domain(_) => Status::failed_precondition(err.to_string()),
        _ => Status::internal(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_error_to_status_not_found() {
        let err = GameError::not_found("user");
        let status = game_error_to_status(&err);
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[test]
    fn test_game_error_to_status_invalid() {
        let err = GameError::invalid("bad input");
        let status = game_error_to_status(&err);
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn test_game_error_to_status_auth() {
        let err = GameError::Auth(blackbox_auth::AuthError::InvalidToken);
        let status = game_error_to_status(&err);
        assert_eq!(status.code(), tonic::Code::Unauthenticated);
    }
}
