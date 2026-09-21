use super::*;

#[test]
fn test_features_to_string() {
    assert_eq!(
        "// bar,foo".to_string(),
        features_to_string(&Some(BTreeSet::from([
            "foo".to_string(),
            "bar".to_string()
        ])))
    );
    assert_eq!("// ".to_string(), features_to_string(&None));
}
