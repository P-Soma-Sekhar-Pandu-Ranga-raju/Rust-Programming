fn main() {
    let x = 5;
    println!("Outer x: {}", x);

    {
        let x = 10; 
        println!("Inner x (shadowed): {}", x);
    }

    println!("Outer x after inner scope: {}", x);
}
