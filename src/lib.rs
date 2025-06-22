//! # Why did my program crash?
//! I wanted a simple answer and would love something similar to a stack trace.
//! I tried some popular crates and found that they are either too complex or too much work to use.
//! If the error is application-ending, I just want to pass it up the stack and eventually either print or serialize it.
//! And that's exactly what this library does, while remaining as small as possible.
//! 
//! - 0 required dependencies (1 optional dependency for serialization or errors)
//! - 0 macros
//! - many optional features, so you only compile what you need
//!
//! It implements a single type [ErrorMessage] and a single trait [WithContext], which is implemented for [Result] and [Option].
//! It provides two functions:
//!  - [with_err_context](WithContext::with_err_context) which takes anything that can be converted to [&str]
//!  - [with_dyn_err_context](WithContext::with_dyn_err_context) which instead takes a closure that is only run in the error case
//!
//! No macros and no learning curve.
//! Wherever you use Rust's [?] operator, you now add `.with_err_context("Short description of what you are currently doing")?`.
//!
//! The output is neatly formatted, almost like a stacktrace.
//!
//!
//! # A simple example
//! ```rust,should_panic
//! # use std::fs::File;
//! # use std::io;
//! # use std::io::ErrorKind;
//! # struct Config;
//! use errors_with_context::prelude::*;
//! // OR just use WithContext
//! use errors_with_context::WithContext;
//!
//! fn read_file(path: &str) -> Result<File, io::Error> {
//!    Err(io::Error::from(ErrorKind::UnexpectedEof))
//! }
//!
//! fn load_config() -> Result<Config, ErrorMessage> {
//!     // [...]
//!     let config_path = "config.json";
//!     read_file(config_path)
//!         .with_dyn_err_context(|| format!("Failed to read file '{}'", config_path))?;
//!     // [...]
//! # Ok(Config)
//! }
//!
//! fn init_program() -> Result<(), ErrorMessage> {
//!     load_config()
//!         .with_err_context("Failed to load configuration")?;
//!     // [...]
//! # Ok(())
//! }
//!
//! fn main() -> Result<(), ErrorMessage> {
//!     init_program()
//!         .with_err_context("Failed to start the program")?;
//!     // [...]
//! # Ok(())
//! }
//! ```
//! prints
//! ```text
//! Failed to start the program
//!   caused by: Failed to load configuration
//!   caused by: Failed to read file 'config.json'
//!   caused by: Kind(UnexpectedEof)
//! ```
//!
//! <br><br><br>
//!
//! # Using [ErrorMessages](ErrorMessage)
//!
//! You can use the functions [with_err_context](WithContext::with_err_context) and [with_dyn_err_context](WithContext::with_dyn_err_context) with [Results](Result) and [Options](Option).
//!
//! ## [.with_err_context()](WithContext::with_err_context)
//!
//! If your error messages are static strings, you can just include them like this:
//!
//! ```rust
//! # use std::io;
//! use errors_with_context::WithContext;
//! fn produce_none() -> Option<()> { None }
//! fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
//!
//! # let e =
//! produce_none()
//!     .with_err_context("Something went wrong in function 'produce_none'");
//! # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_none'");
//! ```
//! prints
//! ```text
//! Something went wrong in function 'produce_none'
//! ```
//!
//! ```rust
//! # use std::io;
//! # use errors_with_context::WithContext;
//! # fn produce_none() -> Option<()> { None }
//! # fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
//! #
//! # let e =
//! produce_err()
//!     .with_err_context("Something went wrong in function 'produce_err'");
//! # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_err'\n  caused by: Kind(UnexpectedEof)");
//! ```
//! prints
//! ```text
//! Something went wrong in function 'produce_err'
//!  caused by: Kind(UnexpectedEof)
//! ```
//!
//! ## [.with_dyn_err_context()](WithContext::with_dyn_err_context)
//!
//! ```rust
//! use errors_with_context::prelude::*;
//! # use std::io;
//! fn produce_none() -> Option<()> { None }
//! fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
//!
//! let variable = "Test";
//!
//! # let e =
//! produce_none()
//!     .with_dyn_err_context(|| format!("Something went wrong in function 'produce_none'. Extra info: {variable}"));
//! # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_none'. Extra info: Test");
//! ```
//! prints
//! ```text
//! Something went wrong in function 'produce_none'. Extra info: Test
//! ```
//!
//! ```rust
//! # use errors_with_context::prelude::*;
//! # use std::io;
//! # fn produce_none() -> Option<()> { None }
//! # fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
//! # let variable = "Test";
//! #
//! # let e =
//! produce_err()
//!     .with_dyn_err_context(|| format!("Something went wrong in function 'produce_err'. Extra info: {variable}"));
//! # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_err'. Extra info: Test\n  caused by: Kind(UnexpectedEof)");
//! ```
//! prints
//! ```text
//! Something went wrong in function 'produce_err'. Extra info: Test
//!  caused by: Kind(UnexpectedEof)
//! ```
//!
//! <br><br>
//!
//! # Creating an error message from scratch
//!
//! To get an [ErrorMessage] without an underlying [Error](std::error::Error)
//! ```rust
//! use errors_with_context::ErrorMessage;
//! ErrorMessage::new("Error description");
//! // prints "Error description" without listing a cause
//! ```
//!
//! Most of the time, you need a [Result<T, ErrorMessage>] instead.
//! [ErrorMessage::err] does exactly that, so you can immediately throw it with `?`:
//! ```rust
//! # use errors_with_context::ErrorMessage;
//! fn erroring_function() -> Result<String, ErrorMessage> {
//!     ErrorMessage::err("Error description")?
//! // [...]
//! }
//! ```
//!
//! If you want to manually wrap an [Error](std::error::Error), there is the function [ErrorMessage::with_context].
//! Example:
//! ```rust
//! # use std::io;
//! # use errors_with_context::ErrorMessage;
//! ErrorMessage::with_context("Error description", io::Error::last_os_error());
//! ```
//!
//! <br><br>
//!
//! # A real-world example
//!
//! ```rust,should_panic
//! # use errors_with_context::{ErrorMessage, WithContext};
//! # use std::ffi::OsStr;
//! # use std::io::{Read, Error};
//! # use std::process::{Command, Stdio};
//! # struct Outputs{}
//! # fn parse_json(input: &str) -> Result<String, Error> {
//! #     unimplemented!()
//! # }
//! fn main() -> Result<(), ErrorMessage> {
//! let outputs = get_outputs()
//!         .with_err_context("Failed to get outputs")?;
//! #   unimplemented!();
//!     // [...]
//! }
//!
//! fn get_outputs() -> Result<Outputs, ErrorMessage> {
//! let cmd = "swaynsg"; // <-- misspelled "swaymsg" command will cause an error
//!     let args = ["-t", "get_outputs"];
//!     let process_output = run(cmd, &args)
//!         .with_dyn_err_context(|| format!("Failed to run command '{cmd}' with args {args:?}"))?;
//!     let outputs = parse_json(&process_output)
//!         .with_err_context("Failed to parse swaymsg outputs JSON")?;
//! #   unimplemented!()
//!     // [...]
//! }
//!
//! fn run<I, S>(cmd: &str, args: I) -> Result<String, ErrorMessage>
//! where
//!     I: IntoIterator<Item=S>,
//!     S: AsRef<OsStr>,
//! {
//! let child = Command::new(cmd)
//!         .args(args)
//!         .stdin(Stdio::null())
//!         .stdout(Stdio::piped())
//!         .stderr(Stdio::piped())
//!         .spawn()
//!         .with_err_context("Failed to spawn process")?;
//!     let mut stdout = child.stdout.with_err_context("Failed to get stdout from process")?;
//!     let mut output = String::new();
//!     stdout.read_to_string(&mut output).with_err_context("Failed to read stdout into buffer")?;
//!     Ok(output)
//! }
//! ```
//! prints
//! ```text
//! Error: Failed to get outputs
//!   caused by: Failed to run command 'swaynsg' with args ["-t", "get_outputs"]
//!   caused by: Failed to spawn process
//!   caused by: Os { code: 2, kind: NotFound, message: "No such file or directory" }
//! ```
//!
//! Much nicer to understand what the program was doing :)
//!
//! If you had just used the `?` operator everywhere, the error would have just said:
//! ```text
//! Os { code: 2, kind: NotFound, message: "No such file or directory" }
//! ```
//!
//! <br><br><br>
//!
//! # Features
//!
//! | Feature             | Enabled by default | Dependencies                   | Effect                                                     |
//! |---------------------|--------------------|--------------------------------|------------------------------------------------------------|
//! | default             | true               | feature: "pretty_debug_errors" | Enable pretty debug errors                                 |
//! | pretty_debug_errors | true               |                                | Enable pretty debug errors                                 |
//! | boolean_errors      | false              |                                | Allow turning booleans into [ErrorMessages](ErrorMessage)  |
//! | serde               | false              | dependency: "serde"            | Allow serialization of [ErrorMessages](ErrorMessage)       |
//!
//! <br><br>
//!
//! ## Feature: `pretty_debug_errors`
//!
//! This feature is enabled by default and is responsible for the pretty error messages in the rest of this page.
//! If the feature is disabled, the more ideomatic rust debug error formatting is used:
//! ```text
//!  Error: ErrorMessage { message: "Failed to get outputs", cause: ErrorMessage { message: "Failed to run command 'swaynsg' with args [\"-t\", \"get_outputs\"]", cause: ErrorMessage { message: "Failed to spawn process", cause: Some(Os { code: 2, kind: NotFound, message: "No such file or directory" }) } } }
//! ```
//!
//! Formatted by hand, it looks like this:
//! ```text
//! Error: ErrorMessage {
//!     message: "Failed to get outputs",
//!     cause: ErrorMessage {
//!         message: "Failed to run command 'swaynsg' with args [\"-t\", \"get_outputs\"]",
//!         cause: ErrorMessage {
//!             message: "Failed to spawn process",
//!             cause: Some(
//!                 Os {
//!                     code: 2,
//!                     kind: NotFound,
//!                     message: "No such file or directory"
//!                 }
//!             )
//!         }
//!     }
//! }
//! ```
//!
//! Still readable, but not as nice for humans.
//!
//! <br>
//!
//! ## Feature: `boolean_errors`
//!
//! (disabled by default)
//!
//! This adds the trait [BooleanErrors] and with it 4 functions to the [bool] type:
//! - [error_if_true](BooleanErrors::error_if_true)
//! - [error_if_false](BooleanErrors::error_if_false)
//! - [error_dyn_if_true](BooleanErrors::error_dyn_if_true)
//! - [error_dyn_if_false](BooleanErrors::error_dyn_if_false)
//!
//! This allows for cool code like this:
//! ```rust
//! # use std::path::Path;
//! use errors_with_context::{BooleanErrors, ErrorMessage};
//! # fn func() -> Result<(), ErrorMessage> {
//! let path = Path::new("test.file");
//! path.exists()
//!     .error_if_false("Expected file to exist!")?;
//! #   Ok(())
//! # }
//! # fn main() {
//! #     let  string = func().unwrap_err().to_string();
//! #     assert_eq!(string, "Expected file to exist!");
//! # }
//! ```
//! or with more dynamic context:
//! ```rust
//! # use std::path::Path;
//! use errors_with_context::prelude::*;
//! # fn func() -> Result<(), ErrorMessage> {
//! let path = Path::new("test.file");
//! path.exists()
//!     .error_dyn_if_false(|| format!("Expected file '{}' to exist!", path.display()))?;
//! #   Ok(())
//! # }
//! # fn main() {
//! #     let  string = func().unwrap_err().to_string();
//! #     assert_eq!(string, "Expected file 'test.file' to exist!");
//! # }
//! ```
//!
//! Very useful, when doing lots of checks that aren't immediately errors.
//!
//! <br>
//!
//!
//! ## Feature: `send`
//!
//! (disabled by default)
//!
//! This adds the requirements for all causes to be [Send].
//! This is useful for sending errors between threads.
//!
//! Example:
//! ```rust
//! # mod tokio {
//! #     use std::thread::JoinHandle;
//! #     use errors_with_context::ErrorMessage;
//! #     pub async fn spawn(future: impl Future) -> Result<Result<bool, ErrorMessage>, ErrorMessage> {
//! #         ErrorMessage::err("mocked function")
//! #     }
//! # }
//! # async fn ping(){}
//! # use errors_with_context::{ErrorMessage, WithContext};
//! # async fn func() -> Result<(), ErrorMessage>{
//!  let host_reachable = tokio::spawn(ping()).await
//!     .with_err_context("Failed to ping host")??;
//! # Ok(())
//! # }
//! ```
//!
//! <br>
//!
//!
//! ## Feature: `serde`
//!
//! (disabled by default)
//!
//! This feature enables serialization of [ErrorMessage]s with [serde](https://crates.io/crates/serde).
//!
//! ```
//! # use std::convert::Infallible;
//! # use std::io;
//! # use errors_with_context::prelude::*;
//! #
//! let result: Result<Infallible, _> = Err(io::Error::from(io::ErrorKind::NotFound))
//! .with_err_context("Failed to read file")
//! .with_err_context("Failed to load configuration")
//! .with_err_context("Failed to start the program");
//! let message = result.unwrap_err();
//! let json = serde_json::to_string_pretty(&message).unwrap();
//! #
//! # assert_eq!(json, r#"{
//! #   "message": "Failed to start the program",
//! #   "cause": {
//! #     "message": "Failed to load configuration",
//! #     "cause": {
//! #       "message": "Failed to read file",
//! #       "cause": {
//! #         "message": "entity not found",
//! #         "cause": null
//! #       }
//! #     }
//! #   }
//! # }"#)
//! ```
//! results in
//! ```json
//! {
//!   "message": "Failed to start the program",
//!   "cause": {
//!     "message": "Failed to load configuration",
//!     "cause": {
//!       "message": "Failed to read file",
//!       "cause": {
//!         "message": "entity not found",
//!         "cause": null
//!       }
//!     }
//!   }
//! }
//! ```

#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

mod error_message;
mod option;
mod result;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod tests;

pub use crate::error_message::ErrorMessage;

/// Group all traits of this crate
pub mod prelude {
    #[cfg(feature = "boolean_errors")]
    pub use super::boolean::BooleanErrors;
    pub use super::error_message::ErrorMessage;
    pub use super::WithContext;
}

/// [WithContext] is implemented for [Result] and [Option]
/// see [with_err_context](WithContext::with_err_context) and [with_dyn_err_context](WithContext::with_dyn_err_context) for more details
pub trait WithContext<T, E> {
    /// Convert an [`Option<T>`] or [`Result<T,_>`] into [`Result<T, ErrorMessage>`]
    ///
    /// Example usage for [`Option<T>`]
    /// ```rust
    /// # use errors_with_context::WithContext;
    /// fn produce_none() -> Option<()> { None }
    /// # let e =
    /// produce_none()
    ///     .with_err_context("Something went wrong in function 'produce_none'");
    /// # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_none'");
    /// ```
    /// prints
    /// ```text
    /// Something went wrong in function 'produce_none'
    /// ```
    ///
    /// Example usage for [`Result<T,_>`]
    /// ```rust
    /// # use std::io;
    /// # use errors_with_context::WithContext;
    /// fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
    /// # let e =
    /// produce_err()
    ///     .with_err_context("Something went wrong in function 'produce_err'");
    /// # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_err'\n  caused by: Kind(UnexpectedEof)");
    /// ```
    /// prints
    /// ```text
    /// Something went wrong in function 'produce_err'
    ///  caused by: Kind(UnexpectedEof)
    /// ```
    fn with_err_context(self, reason: impl ToString) -> Result<T, ErrorMessage>;

    /// Convert an [`Option<T>`] or [`Result<T,_>`] into [`Result<T, ErrorMessage>`]
    ///
    /// Example usage for [`Option<T>`]
    /// ```rust
    /// # use std::io;
    /// use errors_with_context::prelude::*;
    /// fn produce_none() -> Option<()> { None }
    /// let variable = "Test";
    /// # let e =
    /// produce_none()
    ///     .with_dyn_err_context(|| format!("Something went wrong in function 'produce_none'. Extra info: {variable}"));
    /// # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_none'. Extra info: Test");
    /// ```
    /// prints
    /// ```text
    /// Something went wrong in function 'produce_err'. Extra info: Test
    ///  caused by: Kind(UnexpectedEof)
    /// ```
    /// and
    /// ```text
    /// Something went wrong in function 'produce_none'. Extra info: Test
    /// ```
    ///
    /// Example usage for [`Result<T,_>`]
    /// ```rust
    /// # use std::io;
    /// use errors_with_context::prelude::*;
    /// fn produce_err() -> Result<(), io::Error> { Err(io::ErrorKind::UnexpectedEof.into())}
    /// let variable = "Test";
    /// # let e =
    /// produce_err()
    ///     .with_dyn_err_context(|| format!("Something went wrong in function 'produce_err'. Extra info: {variable}"));
    /// # assert_eq!(e.unwrap_err().to_string(), "Something went wrong in function 'produce_err'. Extra info: Test\n  caused by: Kind(UnexpectedEof)");
    /// ```
    /// prints
    /// ```text
    /// Something went wrong in function 'produce_err'
    ///  caused by: Kind(UnexpectedEof)
    /// ```
    fn with_dyn_err_context(self, reason: impl FnOnce() -> String) -> Result<T, ErrorMessage>;
}

#[cfg(feature = "boolean_errors")]
mod boolean;
#[cfg(feature = "boolean_errors")]
pub use boolean::BooleanErrors;
