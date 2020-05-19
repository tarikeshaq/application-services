/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use backtrace::Backtrace;
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Error synchronizing: {0}")]
    SyncAdapterError(#[source] sync15::Error),

    #[error("Error parsing JSON data: {0}")]
    JsonError(#[source] serde_json::Error),

    #[error("Error parsing URL: {0}")]
    UrlParseError(#[source] url::ParseError),

    #[error("Protobuf decode error: {0}")]
    ProtobufDecodeError(#[source] prost::DecodeError),
}

error_support::define_error! {
    ErrorKind {
        (SyncAdapterError, sync15::Error),
        (JsonError, serde_json::Error),
        (UrlParseError, url::ParseError),
        (ProtobufDecodeError, prost::DecodeError),
    }
}
