fn calculate_area_perimeter(x: f32, y: f32) -> (f32, f32) {
    let area = x * y;
    let perimeter = 2.0 * (x + y);
    (area, perimeter)
}

fn main() {
    let (area, perimeter) = calculate_area_perimeter(5.0, 10.0);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
