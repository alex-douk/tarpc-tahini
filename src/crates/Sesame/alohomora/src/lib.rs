#![feature(box_into_inner)]
#![feature(specialization)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

// Re-export our derive macros
#[cfg(feature = "alohomora_derive")]
extern crate alohomora_derive;
extern crate alohomora_sandbox;

// Export these
pub mod bbox;
pub mod context;
pub mod db;

#[cfg(feature = "orm")]
pub mod orm;

pub mod policy;
pub mod rocket;
pub mod sandbox;
pub mod testing;
pub mod fold;
pub mod pcr;
pub mod pure;
pub mod unbox;
pub mod tarpc_serde;
pub mod tarpc;
pub mod tahini_pure;

// Export this directly under alohomora::
mod r#type;
pub use r#type::*;

#[cfg(feature = "alohomora_derive")]
pub use alohomora_derive::{AlohomoraType};
