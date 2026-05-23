//! Comprehensive tests for pizza-analysis-stempel (Polish stemmer + stop filter).

use pizza_analysis_stempel::{PolishStopFilter, StempelStemFilter, POLISH_STOP_WORDS};
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}



// ═══════════════════════════════════════════════════════════════════════════════
// StempelStemFilter — construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stemmer_construction() {
    let _filter = StempelStemFilter::new();
}

#[test]
fn stemmer_default_trait() {
    let _filter = StempelStemFilter::default();
}

#[test]
fn stemmer_clone() {
    let f1 = StempelStemFilter::new();
    let _f2 = f1.clone();
}

#[test]
fn stemmer_debug() {
    let f = StempelStemFilter::new();
    let dbg = format!("{:?}", f);
    assert!(dbg.contains("StempelStemFilter"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// StempelStemFilter — stem() direct API
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_polish_noun_plural() {
    let f = StempelStemFilter::new();
    // "domy" (houses) should reduce towards "dom"
    if let Some(stemmed) = f.stem("domy") {
        assert!(!stemmed.is_empty());
        assert_ne!(stemmed, "domy");
    }
}

#[test]
fn stem_polish_verb_conjugation() {
    let f = StempelStemFilter::new();
    // "pisałem" (I was writing) should stem
    if let Some(stemmed) = f.stem("pisałem") {
        assert!(!stemmed.is_empty());
    }
}

#[test]
fn stem_short_word_returns_none() {
    let f = StempelStemFilter::new();
    // Words shorter than 3 chars should return None
    assert!(f.stem("do").is_none());
    assert!(f.stem("w").is_none());
}

#[test]
fn stem_empty_string() {
    let f = StempelStemFilter::new();
    assert!(f.stem("").is_none());
}

#[test]
fn stem_already_base_form() {
    let f = StempelStemFilter::new();
    // Some base forms may return None (no transformation)
    let result = f.stem("dom");
    // Either None or a valid string
    if let Some(ref s) = result {
        assert!(!s.is_empty());
    }
}

#[test]
fn stem_adjective_forms() {
    let f = StempelStemFilter::new();
    // "pięknego" (beautiful, genitive) should stem
    if let Some(stemmed) = f.stem("pięknego") {
        assert!(!stemmed.is_empty());
    }
}

#[test]
fn stem_non_polish_text() {
    let f = StempelStemFilter::new();
    // English word — may or may not stem, but should not panic
    let _result = f.stem("running");
}

// ═══════════════════════════════════════════════════════════════════════════════
// StempelStemFilter — TokenFilter trait
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn filter_stems_polish_word() {
    let f = StempelStemFilter::new();
    let mut token = make_token("komputerów");
    let (deleted, _extra) = f.filter(&mut token);
    // Stempel filter returns false (never deletes)
    assert!(!deleted);
}

#[test]
fn filter_preserves_offsets() {
    let f = StempelStemFilter::new();
    let mut token = Token::new("domy", 5, 9, 2);
    let _ = f.filter(&mut token);
    assert_eq!(token.start_offset, 5);
    assert_eq!(token.end_offset, 9);
    assert_eq!(token.position, 2);
}

#[test]
fn filter_empty_token() {
    let f = StempelStemFilter::new();
    let mut token = make_token("");
    let (deleted, _extra) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn filter_single_char() {
    let f = StempelStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _extra) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PolishStopFilter — construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_filter_construction() {
    let _f = PolishStopFilter::new();
}

#[test]
fn stop_filter_default() {
    let _f = PolishStopFilter::default();
}

#[test]
fn stop_filter_custom_words() {
    let words = vec!["foo".to_string(), "bar".to_string()];
    let _f = PolishStopFilter::with_words(words);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PolishStopFilter — filtering
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_filter_removes_polish_stop_words() {
    let f = PolishStopFilter::new();
    for &word in &["i", "nie", "się", "na", "że", "to", "do"] {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "expected '{}' to be deleted as stop word", word);
    }
}

#[test]
fn stop_filter_keeps_content_words() {
    let f = PolishStopFilter::new();
    for &word in &["komputer", "programowanie", "dom", "szkoła"] {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "expected '{}' to NOT be deleted", word);
    }
}

#[test]
fn stop_filter_empty_token() {
    let f = PolishStopFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stop_filter_custom_words_work() {
    let words = vec!["custom".to_string(), "stop".to_string()];
    let f = PolishStopFilter::with_words(words);
    let mut token = make_token("custom");
    let (deleted, _) = f.filter(&mut token);
    assert!(deleted);

    let mut token2 = make_token("other");
    let (deleted2, _) = f.filter(&mut token2);
    assert!(!deleted2);
}

// ═══════════════════════════════════════════════════════════════════════════════
// POLISH_STOP_WORDS constant
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_words_list_not_empty() {
    assert!(!POLISH_STOP_WORDS.is_empty());
    assert!(POLISH_STOP_WORDS.len() > 50);
}

#[test]
fn stop_words_contain_common_words() {
    assert!(POLISH_STOP_WORDS.contains(&"i"));
    assert!(POLISH_STOP_WORDS.contains(&"nie"));
    assert!(POLISH_STOP_WORDS.contains(&"się"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_does_not_panic() {
    let mut factory = AnalysisFactory::new();
    pizza_analysis_stempel::register_all(&mut factory);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pipeline integration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pipeline_stop_then_stem() {
    let stop = PolishStopFilter::new();
    let stem = StempelStemFilter::new();

    let words = ["dom", "jest", "piękny", "i", "duży"];
    let mut surviving = Vec::new();

    for &w in &words {
        let mut token = make_token(w);
        let (deleted, _) = stop.filter(&mut token);
        if !deleted {
            let _ = stem.filter(&mut token);
            surviving.push(token.term.to_string());
        }
    }

    // "i" and "jest" should be removed by stop filter
    assert!(!surviving.iter().any(|s| s == "i"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unicode handling
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_unicode_polish_chars() {
    let f = StempelStemFilter::new();
    // Polish-specific characters: ą ć ę ł ń ó ś ź ż
    let _result = f.stem("źródło");
    let _result2 = f.stem("łódź");
}

#[test]
fn stop_filter_unicode_stop_words() {
    let f = PolishStopFilter::new();
    let mut token = make_token("będzie");
    let (deleted, _) = f.filter(&mut token);
    assert!(deleted);
}
