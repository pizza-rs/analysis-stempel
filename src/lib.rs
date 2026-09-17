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
#[cfg(feature = "std")]
#[doc(hidden)]
/// Point the analysis dictionary directory at this crate's `data/` copy so
/// tests can construct the filter under any feature selection (the external
/// file must be laid out as `<dict_dir>/stempel/stemmer_20000.tbl`).
pub fn init_test_dict_dir() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!(
            "pizza-stempel-test-dict-{}",
            std::process::id()
        ));
        let ns = dir.join("stempel");
        if std::fs::create_dir_all(&ns).is_ok() {
            let _ = std::fs::copy(
                concat!(env!("CARGO_MANIFEST_DIR"), "/data/stemmer_20000.tbl"),
                ns.join("stemmer_20000.tbl"),
            );
        }
        pizza_engine::analysis::dict::set_dict_dir(&dir);
    });
}

mod stemmer;
mod stop;

pub use stemmer::StempelStemFilter;
pub use stop::PolishStopFilter;
pub use stop::POLISH_STOP_WORDS;
pub mod register;
pub use register::register_all;
