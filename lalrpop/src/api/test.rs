use std::env::{current_dir, set_current_dir, set_var, temp_dir};
use std::fs;
use std::path;
use std::sync::{Mutex, MutexGuard};

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
    OutDirSlashOther,
    CustomOut,
    DoesntExist,
}

// This holds state during the test to be cleaned up on drop
struct TestState {
    orig_dir: path::PathBuf,
    lock: MutexGuard<'static, i32>,
}

impl TestState {
    fn new(orig_dir: path::PathBuf, lock: MutexGuard<'static, i32>) -> Self {
        TestState { orig_dir, lock }
    }
}

impl Drop for TestState {
    fn drop(&mut self) {
        remove_local_generated_files();
        set_current_dir(&self.orig_dir).unwrap();
        let out_dir = temp_dir().join(TEST_DIR);
        fs::remove_dir_all(out_dir).unwrap();
        // the lock is automatically released when it goes out of scope
    }
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
fn setup() -> TestState {
    let out_dir = temp_dir().join(TEST_DIR);

    // lock() can return an error if another thread panicked while holding the mutex.  In our case,
    // that represents a test failure.  If another test failed, state was already cleaned up on
    // drop.  So we check that and clear the mutex poison to resume processing
    let lock = API_TEST_MUTEX.lock().unwrap_or_else(|e| {
        if fs::exists(&out_dir).unwrap() {
            // Uh oh, we didn't clean up after all
            panic!("This test was started in an unclean state because another test failed but didn't manage to clean up test state");
        }
        API_TEST_MUTEX.clear_poison();
        e.into_inner()
    });
    let orig_dir = current_dir().unwrap();
    set_current_dir(path::Path::new("./src/api/test_files")).unwrap();
    if fs::exists(&out_dir).unwrap() {
        // unclean data from previous failed test run.  Clean up
        fs::remove_dir_all(&out_dir).unwrap();
    }
    // If we have unclean state from a previous run, clean it up
    remove_local_generated_files();

    fs::create_dir(&out_dir).unwrap();

    // Safety note:
    // set_var is marked as unsafe starting in the 2025 rust edition.  This is because C calls to
    // set and read environmental variables are not thread safe.  Specifically, reading an
    // environmental variable while it is being written to can result in unspecified data being
    // read.  In rust alone, these calls are protected via a mutex in the standard library, but if
    // we call into C code (e.g. via libc in a dependency), we do not get those protections.
    //
    // In practice for us, we do have libc in some of our dependencies, and we can't necessarily
    // know or predict where they might read environmental variables.  These tests are under a
    // mutex for the tests in this file only, but may run concurrently with other tests.
    //
    // It is recommended to run the lalrpop test suite using cargo-nextest.  See CONTRIBUTING.md
    // for details and instructions on using cargo-nextest.  You can run cargo test at your own
    // risk, knowing that this thread safety issue may cause undefined behavior.  The risk is
    // mitigated by the following factors:
    //
    // 1. This is only ran in test code.
    // 2. Our testing so far has not encountered this issue in practice.
    // 3. As stated above, it is only calls in C code that are a concern - any calls in rust
    //    dependency would not introduce thread safety issues.
    //
    // That said, cargo-nextest actually makes this safe.  In cargo-nextest, each test is ran in a
    // separate process, and therefore will have its own copy of the environment.  As a result,
    // this safety issue cannot occur under cargo-nextest, as this code will not run in a
    // multi-threaded context.
    unsafe {
        set_var("OUT_DIR", out_dir);
    }
    TestState::new(orig_dir, lock)
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
    let custom_dir = temp_dir().join(CUSTOM_TEST_DIR);
    if fs::exists(&custom_dir).unwrap() {
        fs::remove_dir_all(&custom_dir).unwrap();
    }
}

// This is maybe a little nonintuitive at first.  We verify that the file exists where we expect
// it, and nowhere else.  So fs::exists().unwrap() for a given location must be equal to our
// expectation that it's in that location.
fn verify_file(filename: &str, expected_location: GenFileLoc) {
    println!("Checking the location of {filename}");
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
    assert_eq!(
        fs::exists(
            temp_dir()
                .join(TEST_DIR)
                .join(path::Path::new("other"))
                .join(filename)
        )
        .unwrap(),
        expected_location == GenFileLoc::OutDirSlashOther
    );
    // For GenFileLoc::DoesntExist, we should have returned false for all others.  There is nothing
    // to positive test
}

#[test]
fn test_process_root() {
    let _state = setup();

    process_root().unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::OutDirSlashOther);
    verify_file("outer.rs", GenFileLoc::OutDir);
}

#[test]
fn test_process_src() {
    let _state = setup();

    process_src().unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::DoesntExist);
    verify_file("outer.rs", GenFileLoc::DoesntExist);
}

#[test]
fn test_process_file() {
    let _state = setup();

    // This test is noting that with cargo_dir_conventions, "src/src.lalrpop"
    // will work, but prepending "../test_files" does not as it is an unexpected
    // file prefix.
    Configuration::new().use_cargo_dir_conventions().process_file("../test_files/src/src.lalrpop").unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::DoesntExist);
    verify_file("outer.rs", GenFileLoc::DoesntExist);
}

#[test]
fn test_explicit_in_out() {
    let _state = setup();

    let custom_dir = temp_dir().join(CUSTOM_TEST_DIR);
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
}

#[test]
fn test_cargo_dir_conventions() {
    let _state = setup();

    Configuration::new()
        .use_cargo_dir_conventions()
        .process()
        .unwrap();

    verify_file("src.rs", GenFileLoc::OutDir);
    verify_file("other.rs", GenFileLoc::DoesntExist);
    verify_file("outer.rs", GenFileLoc::DoesntExist);
}
