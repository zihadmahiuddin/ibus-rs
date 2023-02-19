#![cfg_attr(feature = "dox", feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// No-op.
macro_rules! assert_initialized_main_thread {
    () => {};
}

mod auto;
mod bus;
mod component;
mod emoji_data;
mod engine_desc;
mod ibus;
mod input_context;
mod lookup_table;
mod observed_path;
mod registry;
mod xml;

pub mod keys;
pub mod prelude;

pub use auto::*;

pub use auto::builders::*;
pub use auto::functions::*;

pub use prelude::*;
