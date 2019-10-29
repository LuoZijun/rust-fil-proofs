#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::type_repetition_in_bounds)]

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
pub mod test_helper;

pub mod example_helper;

pub mod circuit;
pub mod compound_proof;
pub mod crypto;
pub mod drgraph;
pub mod error;
pub mod fr32;
pub mod hasher;
pub mod merkle;
pub mod merklepor;
pub mod parameter_cache;
pub mod partitions;
pub mod pieces;
pub mod porep;
pub mod proof;
pub mod rational_post;
pub mod sector;
pub mod settings;
pub mod stacked;
pub mod util;
