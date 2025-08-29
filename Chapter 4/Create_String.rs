fn create_string() -> String {
    let mut s = String::from("Hello");
    s.push_str(" World");
    s
}

fn main() {
    let greeting = create_string();
    println!("{}", greeting);
}
