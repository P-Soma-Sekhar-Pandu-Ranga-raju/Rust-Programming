fn push_string(mut s: String, add_str: &str) -> String {
    s.push_str(add_str);
    s
}

fn main() {
    let mut base = String::from("Hello");
    base = push_string(base, ", World!");
    println!("{}", base);
}
