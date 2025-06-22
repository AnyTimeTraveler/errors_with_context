use std::convert::Infallible;
use crate::error_message::ErrorMessage;
use crate::WithContext;

impl<T> WithContext<T, Infallible> for Option<T> {
    fn with_err_context(self, context: impl ToString) -> Result<T, ErrorMessage> {
        match self {
            Some(value) => Ok(value),
            None => {
                ErrorMessage::err(context)
            }
        }
    }
    fn with_dyn_err_context(self, context: impl FnOnce() -> String) -> Result<T, ErrorMessage> {
        match self {
            Some(value) => Ok(value),
            None => {
                ErrorMessage::err(context())
            }
        }
    }
}
