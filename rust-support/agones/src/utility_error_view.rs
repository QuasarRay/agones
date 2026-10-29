//! Data-interface bridge for source errors carrying APIStatus methods.
//! Each error owns its implementation; there is no global type/provider map.
use std::{any::Any, error::Error, fmt};
use crate::{errors::BoxError, runtime::SourceLogType, utility_admission::Status};

pub trait SourceApiStatus { fn source_api_status(&self) -> Status; }
trait DynamicApiStatus: Error + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn status(&self) -> Status;
    fn original_source(&self) -> Option<&(dyn Error + 'static)>;
    fn source_type(&self) -> &'static str;
}
impl<T> DynamicApiStatus for T
where T: Error + SourceApiStatus + SourceLogType + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any { self }
    fn status(&self) -> Status { self.source_api_status() }
    fn original_source(&self) -> Option<&(dyn Error + 'static)> { self.source() }
    fn source_type(&self) -> &'static str { T::SOURCE_LOG_TYPE }
}
#[derive(Debug)]
pub struct ApiStatusEnvelope { inner: Box<dyn DynamicApiStatus> }
impl ApiStatusEnvelope {
    pub fn status(&self) -> Status { self.inner.status() }
    pub fn original_any(&self) -> &dyn Any { self.inner.as_any() }
    pub fn source_type(&self) -> &'static str { self.inner.source_type() }
}
impl fmt::Display for ApiStatusEnvelope { fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self.inner.as_ref(), out) } }
impl Error for ApiStatusEnvelope { fn source(&self) -> Option<&(dyn Error + 'static)> { self.inner.original_source() } }
pub fn with_api_status<T>(error: T) -> BoxError
where T: Error + SourceApiStatus + SourceLogType + Send + Sync + 'static,
{ Box::new(ApiStatusEnvelope { inner: Box::new(error) }) }

/// Source type metadata for the concrete interface bridges and canonical
/// errors already owned here. Unknown native types remain unclassified.
pub fn source_error_type(error: &(dyn Error + 'static)) -> Option<&'static str> {
    if let Some(error) = error.downcast_ref::<crate::utility_error_alias::SourceSharedError>() { return Some(error.source_type_name()); }
    if let Some(error) = error.downcast_ref::<ApiStatusEnvelope>() { return Some(error.source_type()); }
    if let Some(source_type) = crate::utility_panic::runtime_error_type(error) { return Some(source_type); }
    macro_rules! source_types {
        ($($ty:ty),+ $(,)?) => { $(if error.is::<$ty>() { return Some(<$ty as SourceLogType>::SOURCE_LOG_TYPE); })+ };
    }
    source_types!(crate::utility_error_format::MessageError, crate::utility_error_format::WrappedError,
        crate::utility_error_format::FormattedError, crate::utility_panic::SourceAbortHandler);
    None
}
