use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// To get an [ErrorMessage] without an underlying [Error](std::error::Error).
/// ```rust
/// use errors_with_context::ErrorMessage;
/// ErrorMessage::new("Error description".to_owned());
/// // prints "Error description" without listing a cause
/// ```
///
/// Most of the time, you need a [Result<T, ErrorMessage>] instead.
/// [ErrorMessage::err] does exactly that, so you can immediately throw it with `?`:
/// ```rust
/// # use errors_with_context::ErrorMessage;
/// fn erroring_function() -> Result<String, ErrorMessage> {
///     ErrorMessage::err("Error description".to_owned())?
/// // [...]
/// }
/// ```
///
/// If you want to manually wrap an [Error](std::error::Error), there is the function [ErrorMessage::with_context].
/// ```rust
/// # use std::io;
/// # use errors_with_context::ErrorMessage;
/// ErrorMessage::with_context("Error description".to_owned(), io::Error::last_os_error());
/// ```
pub struct ErrorMessage {
    pub(crate) message: String,
    pub(crate) cause: Option<Box<dyn Error>>,
}

impl ErrorMessage {
    /// To get an [ErrorMessage] without an underlying [Error](std::error::Error) as a cause.
    /// 
    /// Example:
    /// ```rust
    /// use errors_with_context::ErrorMessage;
    /// ErrorMessage::new("Error description".to_owned());
    /// // prints "Error description" without listing a cause
    /// ```
    pub fn new(message: String) -> ErrorMessage {
        ErrorMessage { message, cause: None }
    }
    /// This function creates a [Result<T, ErrorMessage>], so you can immediately throw it with `?`.
    /// 
    /// Example:
    /// ```rust
    /// # use errors_with_context::ErrorMessage;
    /// fn erroring_function() -> Result<String, ErrorMessage> {
    ///     ErrorMessage::err("Error description".to_owned())?
    /// // [...]
    /// }
    /// ```
    pub fn err<T>(message: String) -> Result<T, ErrorMessage> {
        Err(ErrorMessage { message, cause: None })
    }
    /// This function allows one to manually wrap an [Error](std::error::Error).
    /// 
    /// Example:
    /// ```rust
    /// # use std::io;
    /// # use errors_with_context::ErrorMessage;
    /// ErrorMessage::with_context("Error description".to_owned(), io::Error::last_os_error());
    /// ```
    pub fn with_context<E: Error + 'static>(message: String, cause: E) -> ErrorMessage {
        ErrorMessage { message, cause: Some(Box::new(cause)) }
    }
}

impl Error for ErrorMessage {}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;
        if let Some(cause) = &self.cause {
            fmt_cause(cause, f)?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "pretty_debug_errors"))]
impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("ErrorMessage");
        s.field("message", &self.message);

        if let Some(cause) = &self.cause {
            if let Some(cause) = cause.downcast_ref::<ErrorMessage>() {
                s.field("cause", cause);
            } else {
                s.field("cause", &Some(cause));
            }
        } else {
            s.field("cause", &None::<ErrorMessage>);
        }

        s.finish()
    }
}

#[cfg(feature = "pretty_debug_errors")]
impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

#[allow(clippy::borrowed_box)]
fn fmt_cause(error: &Box<dyn Error>, f: &mut Formatter) -> fmt::Result {
    f.write_str("\n  caused by: ")?;
    if let Some(cause) = error.downcast_ref::<ErrorMessage>() {
        f.write_str(&cause.message)?;
        if let Some(cause) = &cause.cause {
            fmt_cause(cause, f)?;
        }
    } else {
        Debug::fmt(error, f)?;
    }
    Ok(())
}
