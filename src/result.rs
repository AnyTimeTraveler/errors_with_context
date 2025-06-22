use crate::error_message::ErrorMessage;
use crate::WithContext;
use std::error::Error;

#[cfg(feature = "send")]
impl<T, E: Error + Send + 'static> WithContext<T, E> for Result<T, E> {
    fn with_err_context(self, context: impl ToString) -> Result<T, ErrorMessage> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorMessage::with_context(context, error)),
        }
    }

    fn with_dyn_err_context(self, context: impl FnOnce() -> String) -> Result<T, ErrorMessage> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorMessage::with_context(context(), error)),
        }
    }
}

#[cfg(not(feature = "send"))]
impl<T, E: Error + 'static> WithContext<T, E> for Result<T, E> {
    fn with_err_context(self, context: impl ToString) -> Result<T, ErrorMessage> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorMessage::with_context(context, error)),
        }
    }

    fn with_dyn_err_context(self, context: impl FnOnce() -> String) -> Result<T, ErrorMessage> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorMessage::with_context(context(), error)),
        }
    }
}