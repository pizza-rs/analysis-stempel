//! Register Stempel (Polish) analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;

use pizza_engine::analysis::AnalysisFactory;

use crate::PolishStopFilter;
use crate::StempelStemFilter;

/// Register Stempel token filters and a Polish analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    // Lazy: the stemmer loads its trie table on first use, and in the
    // default no-embed build a missing external dictionary must fail only
    // the requests that actually use this filter — an eager registration
    // would take the whole factory (every analyzer) down with it.
    factory.register_token_filter_with("stempel_stem", || Box::new(StempelStemFilter::new()));
    factory.register_token_filter("polish_stop", Box::new(PolishStopFilter::new()));
}
