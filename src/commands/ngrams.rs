use crate::analysis::{ngram, tokenizer};
use crate::output::ResultTable;
use anyhow::Result;

pub fn run(
    text: &str,
    source_name: &str,
    n: usize,
    top: usize,
    min_freq: Option<usize>,
    case_insensitive: bool,
) -> Result<ResultTable> {
    let words = tokenizer::words(text);

    let freqs = if case_insensitive {
        let lowered: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
        let refs: Vec<&str> = lowered.iter().map(|s| s.as_str()).collect();
        ngram::ngram_frequencies(&refs, n)
    } else {
        ngram::ngram_frequencies(&words, n)
    };

    let total: usize = freqs.values().sum();

    let mut entries: Vec<(&str, usize)> = freqs.iter().map(|(k, &v)| (k.as_str(), v)).collect();

    if let Some(min) = min_freq {
        entries.retain(|&(_, freq)| freq >= min);
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    entries.truncate(top);

    let n_label = match n {
        1 => "Unigram",
        2 => "Bigram",
        3 => "Trigram",
        _ => "N-gram",
    };

    let mut table = ResultTable::new(source_name, vec![n_label, "Freq", "Rel %"]);
    for (ngram_str, freq) in entries {
        let pct = if total > 0 {
            freq as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        table.add_row(vec![
            format!("\"{}\"", ngram_str),
            format_num(freq),
            format!("{:.2}%", pct),
        ]);
    }

    Ok(table)
}

fn format_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
