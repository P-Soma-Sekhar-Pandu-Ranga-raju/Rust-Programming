fn main() {
    let mut count = 0;
    let mut num = 1;

    while num <= 100 {
        num *= 2;
        count += 1;
    }

    println!("Loop ran {} times until num exceeded 100.", count);
}
