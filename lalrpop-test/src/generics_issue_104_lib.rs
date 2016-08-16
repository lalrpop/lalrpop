pub trait Generator {
    fn schema(s: &str) -> String;
}

impl Generator for () {
    fn schema(s: &str) -> String {
        s.to_string()
    }
}
