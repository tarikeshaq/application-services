/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/// Define a wrapper around the the provided ErrorKind type.
/// See also `define_error` which is more likely to be what you want.

#[macro_export]
macro_rules! define_error_wrapper {
    ($Kind:ty) => {
        pub type Result<T, E = Error> = std::result::Result<T, E>;
        use std::fmt;
        use std::fmt::{Debug, Display};
        struct Context<T: Display + Sync + 'static> {
            context: T,
            backtrace: Backtrace,
        }

        impl<T: Display + Send + Sync + 'static> Context<T> {
            fn new(context: T) -> Self {
                Context {
                    context,
                    backtrace: Backtrace::new(),
                }
            }

            fn backtrace(&self) -> Option<&Backtrace> {
                Some(&self.backtrace)
            }

            fn get_context(&self) -> &T {
                &self.context
            }
        }

        impl<D: Display + Send + Sync + 'static> Debug for Context<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}\n\n{}", self.backtrace, self.context)
            }
        }

        #[derive(Debug, thiserror::Error)]
        pub struct Error(Box<Context<$Kind>>);
        impl Error {
            pub fn kind(&self) -> &$Kind {
                &*self.0.get_context()
            }

            pub fn backtrace(&self) -> Option<&Backtrace> {
                self.0.backtrace()
            }
        }

        impl Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&*self.0.get_context(), f)
            }
        }

        impl From<$Kind> for Error {
            // Cold to optimize in favor of non-error cases.
            #[cold]
            fn from(ctx: $Kind) -> Error {
                Error(Box::new(Context::new(ctx)))
            }
        }
    };
}

/// Define a set of conversions from external error types into the provided
/// error kind. Use `define_error` to do this at the same time as
/// `define_error_wrapper`.
#[macro_export]
macro_rules! define_error_conversions {
    ($Kind:ident { $(($variant:ident, $type:ty)),* $(,)? }) => ($(
        impl From<$type> for $Kind {
            // Cold to optimize in favor of non-error cases.
            #[cold]
            fn from(e: $type) -> $Kind {
                $Kind::$variant(e)
            }
        }

        impl From<$type> for Error {
            // Cold to optimize in favor of non-error cases.
            #[cold]
            fn from(e: $type) -> Self {
                Error::from($Kind::$variant(e))
            }
        }
    )*);
}

/// All the error boilerplate (okay, with a couple exceptions in some cases) in
/// one place.
#[macro_export]
macro_rules! define_error {
    ($Kind:ident { $(($variant:ident, $type:ty)),* $(,)? }) => {
        $crate::define_error_wrapper!($Kind);
        $crate::define_error_conversions! {
            $Kind {
                $(($variant, $type)),*
            }
        }
    };
}
