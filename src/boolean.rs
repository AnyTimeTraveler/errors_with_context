use crate::ErrorMessage;

/// This trait allows one to turn [bools](bool) into [ErrorMessages](ErrorMessage),
/// when they have a certain value.
///
/// This allows for cool code like this:
/// ```rust
/// # use std::path::Path;
/// use errors_with_context::{BooleanErrors, ErrorMessage};
/// # fn func() -> Result<(), ErrorMessage> {
/// let path = Path::new("test.file");
/// path.exists()
///     .error_if_false("Expected file to exist!")?;
/// #   Ok(())
/// # }
/// # fn main() {
/// #     let  string = func().unwrap_err().to_string();
/// #     assert_eq!(string, "Expected file to exist!");
/// # }
/// ```
/// or with more dynamic context:
/// ```rust
/// # use std::path::Path;
/// use errors_with_context::prelude::*;
/// # fn func() -> Result<(), ErrorMessage> {
/// let path = Path::new("test.file");
/// path.exists()
///     .error_dyn_if_false(|| format!("Expected file '{}' to exist!", path.display()))?;
/// #   Ok(())
/// # }
/// # fn main() {
/// #     let  string = func().unwrap_err().to_string();
/// #     assert_eq!(string, "Expected file 'test.file' to exist!");
/// # }
/// ```
///
/// Very useful, when doing lots of checks that aren't immediately errors.
pub trait BooleanErrors {
    /// If the [bool] is true, return it `Ok(bool)`.  
    /// If the [bool] is false, return an `Err(ErrorMessage)` with the provided context string..
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use errors_with_context::prelude::*;
    /// # fn func() -> Result<(), ErrorMessage> {
    /// let path = Path::new("test.file");
    /// path.exists()
    ///     .error_if_false("Expected file to exist!")?;
    /// #   Ok(())
    /// # }
    /// # fn main() {
    /// #     let  string = func().unwrap_err().to_string();
    /// #     assert_eq!(string, "Expected file to exist!");
    /// # }
    /// ```
    fn error_if_false(self, context: impl AsRef<str>) -> Result<bool, ErrorMessage>;

    /// If the [bool] is false, return it wrapped in a `Ok(bool)`.  
    /// If the [bool] is true, return an `Err(ErrorMessage)` with the provided context string.
    ///
    /// ```rust
    /// # use std::path::Path;
    /// # use std::ops::Not;
    /// use errors_with_context::prelude::*;
    /// # fn func() -> Result<(), ErrorMessage> {
    /// let path = Path::new("test.file");
    /// path.exists()
    ///     .not()
    ///     .error_if_true("Expected file to exist!")?;
    /// #   Ok(())
    /// # }
    /// # fn main() {
    /// #     let  string = func().unwrap_err().to_string();
    /// #     assert_eq!(string, "Expected file to exist!");
    /// # }
    /// ```
    fn error_if_true(self, context: impl AsRef<str>) -> Result<bool, ErrorMessage>;

    /// If the [bool] is true, return it `Ok(bool)`.  
    /// If the [bool] is false, compute the context and return an `Err(ErrorMessage)`.
    ///
    /// ```rust
    /// # use std::path::Path;
    /// use errors_with_context::prelude::*;
    /// # fn func() -> Result<(), ErrorMessage> {
    /// let path = Path::new("test.file");
    /// path.exists()
    ///     .error_dyn_if_false(|| format!("Expected file '{}' to exist!", path.display()))?;
    /// #   Ok(())
    /// # }
    /// # fn main() {
    /// #     let  string = func().unwrap_err().to_string();
    /// #     assert_eq!(string, "Expected file 'test.file' to exist!");
    /// # }
    fn error_dyn_if_false(self, context: impl FnOnce() -> String) -> Result<bool, ErrorMessage>;

    /// If the [bool] is false, return it `Ok(bool)`.  
    /// If the [bool] is true, compute the context and return an `Err(ErrorMessage)`.
    ///
    /// ```rust
    /// # use std::path::Path;
    /// # use std::ops::Not;
    /// use errors_with_context::prelude::*;
    /// # fn func() -> Result<(), ErrorMessage> {
    /// let path = Path::new("test.file");
    /// path.exists()
    ///     .not()
    ///     .error_dyn_if_true(|| format!("Expected file '{}' to exist!", path.display()))?;
    /// #   Ok(())
    /// # }
    /// # fn main() {
    /// #     let  string = func().unwrap_err().to_string();
    /// #     assert_eq!(string, "Expected file 'test.file' to exist!");
    /// # }
    fn error_dyn_if_true(self, context: impl FnOnce() -> String) -> Result<bool, ErrorMessage>;
}

impl BooleanErrors for bool {
    fn error_if_false(self, context: impl AsRef<str>) -> Result<bool, ErrorMessage> {
        if self {
            Ok(self)
        } else {
            let message = context.as_ref().to_owned();
            Err(ErrorMessage::new(message))
        }
    }

    fn error_if_true(self, context: impl AsRef<str>) -> Result<bool, ErrorMessage> {
        if self {
            let message = context.as_ref().to_owned();
            Err(ErrorMessage::new(message))
        } else {
            Ok(self)
        }
    }

    fn error_dyn_if_false(self, context: impl FnOnce() -> String) -> Result<bool, ErrorMessage> {
        if self {
            Ok(self)
        } else {
            let message = context();
            Err(ErrorMessage::new(message))
        }
    }

    fn error_dyn_if_true(self, context: impl FnOnce() -> String) -> Result<bool, ErrorMessage> {
        if self {
            let message = context();
            Err(ErrorMessage::new(message))
        } else {
            Ok(self)
        }
    }
}
