fn tokenize_string(s: &str) {
    for token in s.split_whitespace() {
        println!("{}", token);
    }
}

fn main() {
    let s = "This is a test string";
    tokenize_string(s);
}
