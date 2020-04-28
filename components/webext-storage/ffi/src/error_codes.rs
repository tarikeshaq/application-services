/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// An unexpected error occurred which likely cannot be meaningfully handled
/// by the application.
pub const UNEXPECTED: i32 = 1;

/// The application passed an invalid JSON string for a storage key or value.
pub const INVALID_JSON: i32 = 2;

/// The total number of bytes stored in the database for this extension,
/// counting all key-value pairs serialized to JSON, exceeds the allowed limit.
pub const QUOTA_TOTAL_BYTES_EXCEEDED: i32 = 32;

/// A single key-value pair exceeds the allowed byte limit when serialized
/// to JSON.
pub const QUOTA_ITEM_BYTES_EXCEEDED: i32 = 32 + 1;

/// The total number of key-value pairs stored for this extension exceeded the
/// allowed limit.
pub const QUOTA_MAX_ITEMS_EXCEEDED: i32 = 32 + 2;
