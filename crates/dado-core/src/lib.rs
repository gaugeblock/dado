//! # `dado-core`
//!
//! Deterministic AST Pattern & Typed Hole Engine.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod hole;
pub mod mixin;
pub mod model;
pub mod polyglot;
pub mod registry;
pub mod splice;

pub use error::{DadoError, Result};
pub use hole::{InlineHoleSpan, parse_inline_holes, validate_pattern_holes};
pub use mixin::{apply_mixin, apply_mixins};
pub use model::{
    DadoMixin, DadoPattern, PatternVerificationReceipt, SpliceResult, TypedHole, language_serde,
};
pub use polyglot::{
    CanonicalType, generate_rust_struct, generate_ts_interface, generate_zod_schema,
    parse_canonical_type,
};
pub use registry::PatternRegistry;
pub use registry::verifier::verify_pack;
pub use splice::{SpliceOptions, ast_check, splice_pattern};
