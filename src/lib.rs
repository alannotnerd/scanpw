//! Read a password from standard input
//!
//! # Overview
//!
//! `scanpw` provides a macro and a function (for more granular error handling) to
//! facilitate reading passwords from standard input in a secure manner. It expands
//! to an expression that returns a [`String`], so it can be assigned to
//! a variable or used directly. The macro may take arguments like those to
//! [`print`], which can be used to generate a prompt.
//!
//! # Examples
//!
//! ## Simple prompt
//!
//! ```no_run
//! # #[macro_use] extern crate scanpw;
//! let password = scanpw!("Password: ");
//! ```
//!
//! This results in a prompt that looks like this (where `_` represents where the
//! user will start typing):
//!
//! ```text
//! Password: _
//! ```
//!
//! ## No prompt
//!
//! ```no_run
//! # #[macro_use] extern crate scanpw;
//! let password = scanpw!();
//! ```
//!
//! ## Formatted prompt
//!
//! ```no_run
//! # #[macro_use] extern crate scanpw;
//! # let username = "";
//! let password = scanpw!("Password for {}: ", username);
//! ```
//!
//! ## Custom echo behavior
//!
//! If the first argument to [`scanpw`] is an expression of type `Option<char>`
//! instead of a string literal, it is used to either set a custom replacement
//! character (like `Some('X')`) or disable echoing entirely (like `None`). For
//! example:
//!
//! ```no_run
//! # #[macro_use] extern crate scanpw;
//! // Don't print a '*' for each character the user types
//! let echo_settings: Option<char> = None;
//!
//! let password = scanpw!(echo_settings, "Password: ");
//! ```
//!
//! The default behavior is to echo `*`s for each character entered.

mod try_scanpw;

pub use try_scanpw::try_scanpw;

/// Reads a password from standard input
///
/// Invocations of [`scanpw`] expand to an expression retuning a [`String`] that
/// contains a line of input from `stdin`. It can be invoked with arguments
/// identical to those of [`print`], and if so, those arguments will be used
/// to generate a prompt on the standard output. Input will begin on the same
/// line that the prompt ends, if any. If no arguments are provided, input will
/// start where the cursor is, which is likely to be on its own empty line.
///
/// # Panics
///
/// This macro will panic if there are IO errors on the standard input or
/// output.
#[macro_export] macro_rules! scanpw {
    // No prompt, echo '*'s
    () => {{
        $crate::scanpw!(Some('*'))
    }};

    // Prompt, echo '*'s
    ( $fmt:literal ) => {{
        print!($fmt);
        use ::std::io::Write;
        ::std::io::stdout().flush().unwrap();

        $crate::try_scanpw(Some('*')).unwrap()
    }};

    // Formatted prompt, echo '*'s
    ( $fmt:literal, $($args:tt)* ) => {{
        print!("{}", format_args!($fmt, $($args)*));
        use ::std::io::Write;
        ::std::io::stdout().flush().unwrap();

        $crate::try_scanpw(Some('*')).unwrap()
    }};

    // Manually set echo mode with Option<char>, no prompt
    ( $echo:expr ) => {{
        $crate::try_scanpw($echo).unwrap()
    }};

    // Manually set echo mode, with prompt
    ( $echo:expr, $fmt:literal ) => {{
        print!($fmt);
        use ::std::io::Write;
        ::std::io::stdout().flush().unwrap();

        $crate::try_scanpw($echo).unwrap()
    }};

    // Manually set echo mode, with formatted prompt
    ( $echo:expr, $fmt:literal, $($args:tt)* ) => {{
        print!("{}", format_args!($fmt, $($args)*));
        use ::std::io::Write;
        ::std::io::stdout().flush().unwrap();

        $crate::try_scanpw($echo).unwrap()
    }};
}
