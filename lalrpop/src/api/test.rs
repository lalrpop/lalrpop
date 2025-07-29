use std::env::{current_dir, set_current_dir, set_var, temp_dir};
use std::fs;
use std::path;
use std::sync::{LockResult, Mutex, MutexGuard};

use super::*;

// tests may be run in parallel and these tests affect global state, so lock
static API_TEST_MUTEX: Mutex<i32> = Mutex::new(0);

const TEST_DIR: &str = "lalrpop-test";
const CUSTOM_TEST_DIR: &str = "lalrpop-test2";

#[derive(Debug, PartialEq)]
enum GenFileLoc {
    Src,
    Other,
    Root,
    OutDir,
    CustomOut,
    DoesntExist,
}

// Set up for API tests.  The directory structure in test_files
// is:
//
// outer.lalrpop
// other
//   - other.lalrpop
// src
//   - src.lalrpop
//
// So we want to set CWD to directly above that, and OUT_DIR to a temp directory
fn setup() -> (path::PathBuf, LockResult<MutexGuard<'static, i32>>) {
    let lock = API_TEST_MUTEX.lock();
    let orig_dir = current_dir().unwrap();
    set_current_dir(path::Path::new("./src/api/test_files")).unwrap();
    let out_dir = temp_dir().join(TEST_DIR);
    if fs::exists(&out_dir).unwrap() {
        // unclean data from previous failed test run.  Clean up
        fs::remove_dir_all(&out_dir).unwrap();
    }
    // If we have unclean state from a previous run, clean it up
    remove_local_generated_files();

    fs::create_dir(&out_dir).unwrap();
    set_var("OUT_DIR", out_dir);
    (orig_dir, lock)
}

fn teardown(orig_dir: PathBuf) {
    remove_local_generated_files();
    set_current_dir(orig_dir).unwrap();
    let out_dir = temp_dir().join(TEST_DIR);
    fs::remove_dir_all(out_dir).unwrap();
    // The lock is automatically released when it goes out of scope
}

// Assumes CWD is test_files
fn remove_local_generated_files() {
    for f in ["src.rs", "other.rs", "outer.rs"] {
        for loc in [".", "src", "other"] {
            let file_path = path::Path::new(loc).join(f);
            if fs::exists(&file_path).unwrap() {
                fs::remove_file(file_path).unwrap();
            }
        }
    }
}

// This is maybe a little nonintuitive at first.  We verify that the file exists where we expect
// it, and nowhere else.  So fs::exists().unwrap() for a given location must be equal to our
// expectation that it's in that location.
fn verify_file(filename: &str, expected_location: GenFileLoc) {
    println!("Checking the location of {}", filename);
    assert_eq!(
        fs::exists(path::Path::new("src").join(filename)).unwrap(),
        expected_location == GenFileLoc::Src
    );
    assert_eq!(
        fs::exists(path::Path::new("other").join(filename)).unwrap(),
        expected_location == GenFileLoc::Other
    );
    assert_eq!(
        fs::exists(filename).unwrap(),
        expected_location == GenFileLoc::Root
    );
    if fs::exists(temp_dir().join(CUSTOM_TEST_DIR)).unwrap() {
        // Some tests create a custom output directory here.  We only check for contents if it
        // exists
        assert_eq!(
            fs::exists(temp_dir().join(CUSTOM_TEST_DIR).join(filename)).unwrap(),
            expected_location == GenFileLoc::CustomOut
        )
    }
    assert_eq!(
        fs::exists(temp_dir().join(TEST_DIR).join(filename)).unwrap(),
        expected_location == GenFileLoc::OutDir
    );
    // For GenFileLoc::DoesntExist, we should have returned false for all others.  There is nothing
    // to positive test
}

#[test]
fn test_process_root() {
    let (orig_dir, _lock) = setup();

    process_root().unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::OutDir);
    verify_file("outer.rs", GenFileLoc::OutDir);

    teardown(orig_dir);
}

#[test]
fn test_process_src() {
    let (orig_dir, _lock) = setup();

    process_src().unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::DoesntExist);
    verify_file("outer.rs", GenFileLoc::DoesntExist);

    teardown(orig_dir);
}

#[test]
fn test_explicit_in_out() {
    let (orig_dir, _lock) = setup();

    let custom_dir = temp_dir().join(CUSTOM_TEST_DIR);
    if fs::exists(&custom_dir).unwrap() {
        fs::remove_dir_all(&custom_dir).unwrap();
    }
    fs::create_dir(&custom_dir).unwrap();

    Configuration::new()
        .set_in_dir("other")
        .set_out_dir(custom_dir.to_str().unwrap())
        .process()
        .unwrap();

    verify_file("src.rs", GenFileLoc::DoesntExist);
    verify_file("other.rs", GenFileLoc::CustomOut);
    verify_file("outer.rs", GenFileLoc::DoesntExist);

    fs::remove_dir_all(&custom_dir).unwrap();
    teardown(orig_dir);
}

#[test]
fn test_cargo_dir_conventions() {
    let (orig_dir, _lock) = setup();

    Configuration::new()
        .use_cargo_dir_conventions()
        .process()
        .unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::DoesntExist);
    verify_file("outer.rs", GenFileLoc::DoesntExist);

    teardown(orig_dir);
}
