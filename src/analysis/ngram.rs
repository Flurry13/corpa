use rustc_hash::FxHashMap;

/// Iterator over n-grams from a token slice. Yields joined strings.
pub fn ngrams<'a>(tokens: &'a [&str], n: usize) -> impl Iterator<Item = String> + 'a {
    tokens.windows(n).map(|window| window.join(" "))
}

/// Count n-gram frequencies from a token slice.
pub fn ngram_frequencies(tokens: &[&str], n: usize) -> FxHashMap<String, usize> {
    let mut freqs = FxHashMap::default();
    for ngram in ngrams(tokens, n) {
        *freqs.entry(ngram).or_insert(0) += 1;
    }
    freqs
}
