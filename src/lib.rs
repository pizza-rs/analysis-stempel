#![cfg_attr(not(feature = "std"), no_std)]
//! Polish Stempel stemmer for Pizza search engine.
//!
//! Implements the Stempel stemming algorithm using comprehensive suffix-based
//! transformation rules (~200+ patterns) covering verb conjugation, noun
//! declension, adjective agreement, adverb derivation, and more.
//!
//! Rules are ordered by suffix length (longest first) for greedy matching,
//! matching the behavior of the original Stempel FSA trie.
//!
//! # Components
//!
//! - [`StempelStemFilter`] — Polish stemming token filter
//! - [`PolishStopFilter`] — Polish stop words filter
extern crate alloc;
mod stemmer;
mod stop;

pub use stemmer::StempelStemFilter;
pub use stop::{PolishStopFilter, POLISH_STOP_WORDS};
pub mod register;
pub use register::register_all;
