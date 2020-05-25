/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use backtrace::Backtrace;
use rc_crypto::hawk;
use std::string;
use thiserror;
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Unknown OAuth State")]
    UnknownOAuthState,

    #[error("The client requested keys alongside the token but they were not included")]
    TokenWithoutKeys,

    #[error("Login state needs to be Married for the current operation")]
    NotMarried,

    #[error("Multiple OAuth scopes requested")]
    MultipleScopesRequested,

    #[error("No cached token for scope {0}")]
    NoCachedToken(String),

    #[error("No cached scoped keys for scope {0}")]
    NoScopedKey(String),

    #[error("No stored refresh token")]
    NoRefreshToken,

    #[error("No stored session token")]
    NoSessionToken,

    #[error("No stored migration data")]
    NoMigrationData,

    #[error("No stored current device id")]
    NoCurrentDeviceId,

    #[error("Could not find a refresh token in the server response")]
    RefreshTokenNotPresent,

    #[error("Action requires a prior device registration")]
    DeviceUnregistered,

    #[error("Device target is unknown (Device ID: {0})")]
    UnknownTargetDevice(String),

    #[error("Unrecoverable server error {0}")]
    UnrecoverableServerError(&'static str),

    #[error("Invalid OAuth scope value {0}")]
    InvalidOAuthScopeValue(String),

    #[error("Illegal state: {0}")]
    IllegalState(&'static str),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Send Tab diagnosis error: {0}")]
    SendTabDiagnosisError(&'static str),

    #[error("Empty names")]
    EmptyOAuthScopeNames,

    #[error("Key {0} had wrong length, got {1}, expected {2}")]
    BadKeyLength(&'static str, usize, usize),

    #[error("Cannot xor arrays with different lengths: {0} and {1}")]
    XorLengthMismatch(usize, usize),

    #[error("Audience URL without a host")]
    AudienceURLWithoutHost,

    #[error("Origin mismatch")]
    OriginMismatch,

    #[error("JWT signature validation failed")]
    JWTSignatureValidationFailed,

    #[error("ECDH key generation failed")]
    KeyGenerationFailed,

    #[error("Public key computation failed")]
    PublicKeyComputationFailed,

    #[error("Remote key and local key mismatch")]
    MismatchedKeys,

    #[error("Key import failed")]
    KeyImportFailed,

    #[error("AEAD open failure")]
    AEADOpenFailure,

    #[error("Random number generation failure")]
    RngFailure,

    #[error("HMAC mismatch")]
    HmacMismatch,

    #[error("Unsupported command: {0}")]
    UnsupportedCommand(&'static str),

    #[error("Remote server error: '{code}' '{errno}' '{message}' '{message}' '{info}'")]
    RemoteError {
        code: u64,
        errno: u64,
        error: String,
        message: String,
        info: String,
    },

    // Basically reimplement error_chain's foreign_links. (Ugh, this sucks).
    #[error("Crypto/NSS error: {0}")]
    CryptoError(#[source] rc_crypto::Error),

    // BRING BACK #[source] BEFORE MERGING!!! Needs ece::Error to use thiserror instead of failure
    #[error("http-ece encryption error: {0}")]
    EceError(rc_crypto::ece::Error),

    #[error("Hex decode error: {0}")]
    HexDecodeError(#[source] hex::FromHexError),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[source] base64::DecodeError),

    #[error("JSON error: {0}")]
    JsonError(#[source] serde_json::Error),

    #[error("UTF8 decode error: {0}")]
    UTF8DecodeError(#[source] string::FromUtf8Error),

    #[error("Network error: {0}")]
    RequestError(#[source] viaduct::Error),

    #[error("Malformed URL error: {0}")]
    MalformedUrl(#[source] url::ParseError),

    #[error("Unexpected HTTP status: {0}")]
    UnexpectedStatus(#[source] viaduct::UnexpectedStatus),

    #[error("Sync15 error: {0}")]
    SyncError(#[source] sync15::Error),

    // BRING #[source] BACK BEFORE MERGING!!!! Needs hawk::Error to use thiserror instead of failure
    #[error("HAWK error: {0}")]
    HawkError(hawk::Error),

    #[error("Protobuf decode error: {0}")]
    ProtobufDecodeError(#[source] prost::DecodeError),
}

error_support::define_error! {
    ErrorKind {
        (CryptoError, rc_crypto::Error),
        (EceError, rc_crypto::ece::Error),
        (HexDecodeError, hex::FromHexError),
        (Base64Decode, base64::DecodeError),
        (JsonError, serde_json::Error),
        (UTF8DecodeError, std::string::FromUtf8Error),
        (RequestError, viaduct::Error),
        (UnexpectedStatus, viaduct::UnexpectedStatus),
        (MalformedUrl, url::ParseError),
        (SyncError, sync15::Error),
        (ProtobufDecodeError, prost::DecodeError),
    }
}

error_support::define_error_conversions! {
    ErrorKind {
        (HawkError, hawk::Error),
    }
}
