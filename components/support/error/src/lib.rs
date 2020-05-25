/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Define a wrapper around the the provided ErrorKind type.
/// See also `define_error` which is more likely to be what you want.
#[macro_export]
macro_rules! define_error_wrapper {
    ($Kind:ty) => {
        pub type Result<T, E = Error> = std::result::Result<T, E>;
        struct ErrorData {
            kind: $Kind,
            backtrace: std::sync::Mutex<Backtrace>,
        }

        impl ErrorData {
            #[cold]
            fn new(kind: $Kind) -> Self {
                ErrorData {
                    kind,
                    backtrace: std::sync::Mutex::new(Backtrace::new_unresolved()),
                }
            }

            #[cold]
            fn get_backtrace(&self) -> &std::sync::Mutex<Backtrace> {
                self.backtrace.lock().unwrap().resolve();
                &self.backtrace
            }
        }

        impl std::fmt::Debug for ErrorData {
            #[cold]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut bt = self.backtrace.lock().unwrap();
                bt.resolve();
                write!(f, "{:?}\n\n{}", bt, self.kind)
            }
        }

        #[derive(Debug, thiserror::Error)]
        pub struct Error(Box<ErrorData>);
        impl Error {
            #[cold]
            pub fn kind(&self) -> &$Kind {
                &self.0.kind
            }

            #[cold]
            pub fn backtrace(&self) -> &std::sync::Mutex<Backtrace> {
                self.0.get_backtrace()
            }
        }

        impl std::fmt::Display for Error {
            #[cold]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self.kind(), f)
            }
        }

        impl From<$Kind> for Error {
            // Cold to optimize in favor of non-error cases.
            #[cold]
            fn from(ctx: $Kind) -> Error {
                Error(Box::new(ErrorData::new(ctx)))
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
