fn string_manipulation_operations(s: String) {
    println!("Capacity: {}", s.capacity());
    println!("Contains 'sub_str': {}", s.contains("sub_str"));
    let replaced = s.replace("replace_from", "replace_to");
    println!("Replaced String: {}", replaced);
    let trimmed = s.trim().to_string();
    println!("Trimmed String: {}", trimmed);
}

fn main() {
    let s = String::from("Hello, replace_from world!");
    string_manipulation_operations(s);
}
