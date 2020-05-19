/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use thiserror;
use backtrace::Backtrace;
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("NSS error: {0}")]
    NSSError(#[source] nss::Error),
    #[error("Internal crypto error")]
    InternalError,
    #[error("Conversion error: {0}")]
    ConversionError(#[source] std::num::TryFromIntError),
}

error_support::define_error! {
    ErrorKind {
        (ConversionError, std::num::TryFromIntError),
        (NSSError, nss::Error),
    }
}
