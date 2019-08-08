/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(asm)]
#![feature(async_await,)]
#![feature(proc_macro_hygiene)]
#![feature(concat_idents)]

#![doc(html_logo_url = "https://edp.fortanix.com/img/docs/edp-logo.svg",
       html_favicon_url = "https://edp.fortanix.com/favicon.ico",
       html_root_url = "https://edp.fortanix.com/docs/api/")]
extern crate tokio;
extern crate openssl;
extern crate sgx_isa;
extern crate sgxs;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fnv;
extern crate fortanix_sgx_abi;
#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate paste;

mod command;
mod library;
mod loader;
mod tcs;
pub mod usercalls;

pub use crate::command::Command;
pub use crate::library::Library;
pub use crate::loader::{EnclaveBuilder, EnclavePanic};
