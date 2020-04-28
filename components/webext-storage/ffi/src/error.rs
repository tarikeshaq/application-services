/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::result;

use ffi_support::{ErrorCode, ExternError, HandleError};
use serde_json::Error as JsonError;
use webext_storage::error::{Error as WebextStorageError, ErrorKind, QuotaReason};

use crate::error_codes;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A wrapped `webext_storage` error.
    WebextStorage(ErrorCode, WebextStorageError),
    /// An error thrown when a given JSON value is invalid.
    Json(JsonError),
    /// An error from `ffi-support`, indicating an invalid FFI handle.
    Handle(HandleError),
}

impl From<WebextStorageError> for Error {
    fn from(err: WebextStorageError) -> Error {
        let code = ErrorCode::new(match err.kind() {
            ErrorKind::QuotaError(QuotaReason::TotalBytes) => {
                error_codes::QUOTA_TOTAL_BYTES_EXCEEDED
            }
            ErrorKind::QuotaError(QuotaReason::ItemBytes) => error_codes::QUOTA_ITEM_BYTES_EXCEEDED,
            ErrorKind::QuotaError(QuotaReason::MaxItems) => error_codes::QUOTA_MAX_ITEMS_EXCEEDED,
            _ => error_codes::UNEXPECTED,
        });
        Error::WebextStorage(code, err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<HandleError> for Error {
    fn from(err: HandleError) -> Error {
        Error::Handle(err)
    }
}

impl From<Error> for ExternError {
    fn from(err: Error) -> ExternError {
        match err {
            Error::WebextStorage(code, err) => ExternError::new_error(code, err.to_string()),
            Error::Json(err) => {
                ExternError::new_error(ErrorCode::new(error_codes::INVALID_JSON), err.to_string())
            }
            Error::Handle(err) => err.into(),
        }
    }
}
