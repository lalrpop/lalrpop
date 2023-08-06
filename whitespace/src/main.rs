fn main() {
    use std::io::*;

    let mut source = String::new();
    match std::env::args().nth(1) {
        Some(filename) => {
            use std::fs::File;

            File::open(&filename)
                .unwrap_or_else(|_| panic!("Can't open {}", &filename))
                .read_to_string(&mut source)
                .unwrap_or_else(|_| panic!("Can't read contents of {}", &filename));
        }

        None => {
            stdin()
                .read_to_string(&mut source)
                .expect("Can't read stdin");
        }
    }

    if source.is_empty() {
        println!("Empty file");
        return;
    }

    whitespace::compile(&source).expect("OH NO").interpret();
}
