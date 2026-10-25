package processor

import (
	"net/http"

	"agones.dev/agones/pkg/util/runtime"
	"google.golang.org/grpc/codes"
)

var grpcLogger = runtime.NewLoggerWithSource("processor")

// GRPCCodeFromHTTPStatus converts an HTTP status code to the corresponding gRPC status code
func GRPCCodeFromHTTPStatus(httpStatusCode int) codes.Code {
	switch httpStatusCode {
	case http.StatusOK:
		return codes.OK
	case 499:
		return codes.Canceled
	case http.StatusInternalServerError:
		return codes.Internal
	case http.StatusBadRequest:
		return codes.InvalidArgument
	case http.StatusGatewayTimeout:
		return codes.DeadlineExceeded
	case http.StatusNotFound:
		return codes.NotFound
	case http.StatusConflict:
		return codes.AlreadyExists
	case http.StatusForbidden:
		return codes.PermissionDenied
	case http.StatusUnauthorized:
		return codes.Unauthenticated
	case http.StatusTooManyRequests:
		return codes.ResourceExhausted
	case http.StatusNotImplemented:
		return codes.Unimplemented
	case http.StatusUnprocessableEntity:
		return codes.InvalidArgument
	case http.StatusServiceUnavailable:
		return codes.Unavailable
	default:
		grpcLogger.WithField("httpStatusCode", httpStatusCode).Warn("received unknown http status code, defaulting to codes.ResourceExhausted / 429")
		return codes.ResourceExhausted
	}
}