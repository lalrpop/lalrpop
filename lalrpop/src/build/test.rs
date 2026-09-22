use super::*;

use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_features_to_string() {
    assert_eq!(
        "// features: bar,foo".to_string(),
        features_to_string(&Some(BTreeSet::from([
            "foo".to_string(),
            "bar".to_string()
        ])))
    );
    assert_eq!("// features: ".to_string(), features_to_string(&None));
}

#[test]
fn test_needs_rebuild() {
    let mut lalrpop_file = NamedTempFile::new().unwrap();

    writeln!(lalrpop_file, "Some grammar file").unwrap();

    let mut rs_file = NamedTempFile::new().unwrap();

    writeln!(rs_file, "{}", LALRPOP_VERSION_HEADER).unwrap();
    writeln!(rs_file, "deadbeef").unwrap();
    writeln!(rs_file, "// features: ").unwrap();

    assert!(needs_rebuild(lalrpop_file.path(), rs_file.path(), &None).unwrap());

    let mut rs_file2 = NamedTempFile::new().unwrap();

    writeln!(rs_file2, "{}", LALRPOP_VERSION_HEADER).unwrap();
    writeln!(rs_file2, "{}", hash_file(lalrpop_file.path()).unwrap()).unwrap();
    writeln!(rs_file2, "// features: ").unwrap();

    let feats = Some(BTreeSet::from([
        "ghi".to_string(),
        "def".to_string(),
        "abc".to_string(),
    ]));
    let feats2 = Some(BTreeSet::from([
        "ghi".to_string(),
        "def".to_string(),
        "abc".to_string(),
        "xyz".to_string(),
    ]));

    assert!(!needs_rebuild(lalrpop_file.path(), rs_file2.path(), &None).unwrap());
    assert!(needs_rebuild(lalrpop_file.path(), rs_file2.path(), &feats).unwrap());

    let mut rs_file3 = NamedTempFile::new().unwrap();

    writeln!(rs_file3, "{}", LALRPOP_VERSION_HEADER).unwrap();
    writeln!(rs_file3, "{}", hash_file(lalrpop_file.path()).unwrap()).unwrap();
    writeln!(rs_file3, "// features: abc,def,ghi").unwrap();

    assert!(!needs_rebuild(lalrpop_file.path(), rs_file3.path(), &feats).unwrap());
    assert!(needs_rebuild(lalrpop_file.path(), rs_file3.path(), &None).unwrap());
    assert!(needs_rebuild(lalrpop_file.path(), rs_file3.path(), &feats2).unwrap());
}
