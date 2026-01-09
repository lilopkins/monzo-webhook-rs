#[cfg(feature = "decode_everything")]
use crate::HasExtraData as _;

#[test]
fn test_parsable_transactions() {
    let mut tests = 0;
    for (idx, line) in include_str!("tests.txt").lines().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        let _res: super::Webhook = serde_json::from_str(line)
            .expect(&format!("Failed to decode line {}: {line}", idx + 1));
        #[cfg(feature = "decode_everything")]
        assert!(!_res.has_extra_data(), "Line {} has extra data", idx + 1);
        tests += 1;
    }
    assert!(tests >= 1);
}
