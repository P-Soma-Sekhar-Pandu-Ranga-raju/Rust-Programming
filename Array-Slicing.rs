fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice_a = &arr[1..3]; 
    let slice_b = &arr[..3];  
    let slice_c = &arr[7..]; 
    let slice_d = &arr[..];   

    println!("Slice a: {:?}", slice_a);
    println!("Slice b: {:?}", slice_b);
    println!("Slice c: {:?}", slice_c);
    println!("Slice d: {:?}", slice_d);
}
