use summer_grpc::tonic::Code;
use tracing::warn;

/// Converts an HTTP status code to the corresponding gRPC status code.
pub fn grpc_code_from_http_status(status: u16) -> Code {
    match status {
        200 => Code::Ok,
        499 => Code::Cancelled,
        500 => Code::Internal,
        400 | 422 => Code::InvalidArgument,
        504 => Code::DeadlineExceeded,
        404 => Code::NotFound,
        409 => Code::AlreadyExists,
        403 => Code::PermissionDenied,
        401 => Code::Unauthenticated,
        429 => Code::ResourceExhausted,
        501 => Code::Unimplemented,
        503 => Code::Unavailable,
        _ => {
            warn!(
                http_status_code = status,
                "received unknown HTTP status code; \
                 defaulting to ResourceExhausted / 429"
            );
            Code::ResourceExhausted
        }
    }
}