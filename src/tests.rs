#[test]
fn test_parsable_transactions() {
    let mut tests = 0;
    for (idx, line) in include_str!("tests.txt").lines().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        let err = format!("Failed to decode line {idx}: {line}");
        let _res: super::Webhook = serde_json::from_str(line).expect(&err);
        tests += 1;
    }
    assert!(tests >= 1);
}
