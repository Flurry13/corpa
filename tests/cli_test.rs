use std::process::Command;

fn txtstat(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_txtstat"))
        .args(args)
        .output()
        .expect("failed to run txtstat");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_stats_json() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt", "--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    let tokens_row = records
        .iter()
        .find(|r| r.get("metric").and_then(|m| m.as_str()) == Some("Tokens (words)"))
        .expect("missing Tokens row");
    let token_val: usize = tokens_row["value"]
        .as_str()
        .unwrap()
        .replace(',', "")
        .parse()
        .unwrap();
    assert!(token_val > 0);
}

#[test]
fn test_stats_table_output() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt"]);
    assert!(out.contains("Tokens"));
    assert!(out.contains("Types"));
    assert!(out.contains("Type-Token Ratio"));
}

#[test]
fn test_stats_csv() {
    let out = txtstat(&["stats", "tests/fixtures/small.txt", "--format", "csv"]);
    assert!(out.contains("Metric,Value"));
}

#[test]
fn test_ngrams_bigrams_json() {
    let out = txtstat(&[
        "ngrams",
        "-n",
        "2",
        "--top",
        "3",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() <= 3);
}

#[test]
fn test_ngrams_case_insensitive() {
    let out = txtstat(&[
        "ngrams",
        "--case-insensitive",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(!parsed.as_array().unwrap().is_empty());
}

#[test]
fn test_tokens_command() {
    let out = txtstat(&[
        "tokens",
        "tests/fixtures/small.txt",
        "--format",
        "json",
    ]);
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    let records = parsed.as_array().unwrap();
    assert!(records.len() >= 3);
}
