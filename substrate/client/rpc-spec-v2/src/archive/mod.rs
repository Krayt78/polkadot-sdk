// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate archive API.
//!
//! # Note
//!
//! Methods are prefixed by `archive`.

#[cfg(test)]
mod tests;

mod archive_storage;
mod types;

pub mod api;
pub mod archive;
pub mod error;

pub use api::ArchiveApiServer;
pub use archive::Archive;
pub use types::{MethodResult, MethodResultErr, MethodResultOk};
