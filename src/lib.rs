#![cfg_attr(feature = "simd", feature(portable_simd))]

mod error;
mod standalone;
mod utilities;
mod whisper_ctx;
mod whisper_params;

pub use error::WhisperError;
pub use standalone::*;
pub use utilities::*;
pub use whisper_ctx::WhisperContext;
pub use whisper_params::{DecodeStrategy, FullParams};

pub type WhisperToken = std::ffi::c_int;