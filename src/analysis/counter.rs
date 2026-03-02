use crate::analysis::tokenizer;
use rustc_hash::FxHashMap;

/// Count word frequencies from text. Case-sensitive.
pub fn word_frequencies(text: &str) -> FxHashMap<String, usize> {
    let words = tokenizer::words(text);
    let mut freqs = FxHashMap::default();
    for word in words {
        *freqs.entry(word.to_string()).or_insert(0) += 1;
    }
    freqs
}

/// Count word frequencies case-insensitively.
pub fn word_frequencies_case_insensitive(text: &str) -> FxHashMap<String, usize> {
    let words = tokenizer::words(text);
    let mut freqs = FxHashMap::default();
    for word in words {
        *freqs.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    freqs
}

/// Return top N entries sorted by frequency (descending), then alphabetically.
pub fn top_n(freqs: &FxHashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut entries: Vec<(&str, usize)> = freqs.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    entries.truncate(n);
    entries
}

/// Number of unique word types.
pub fn type_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.len()
}

/// Total token (word) count.
pub fn token_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.values().sum()
}

/// Count of words appearing exactly once (hapax legomena).
pub fn hapax_count(freqs: &FxHashMap<String, usize>) -> usize {
    freqs.values().filter(|&&v| v == 1).count()
}

/// Type-token ratio.
pub fn type_token_ratio(freqs: &FxHashMap<String, usize>) -> f64 {
    let types = type_count(freqs);
    let tokens = token_count(freqs);
    if tokens == 0 {
        return 0.0;
    }
    types as f64 / tokens as f64
}
