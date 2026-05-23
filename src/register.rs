//! Register Stempel (Polish) analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;

use pizza_engine::analysis::AnalysisFactory;

use crate::{PolishStopFilter, StempelStemFilter};

/// Register Stempel token filters and a Polish analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("stempel_stem", Box::new(StempelStemFilter::new()));
    factory.register_token_filter("polish_stop", Box::new(PolishStopFilter::new()));
}
