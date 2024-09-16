fn main() {
    #[cfg(not(all(feature = "test-set", feature = "test-not-set")))]
    lalrpop::process_src().unwrap();

    #[cfg(all(feature = "test-set", feature = "test-not-set"))]
    lalrpop::process_src().unwrap_err();
}
