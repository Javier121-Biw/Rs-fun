use std::io;
fn main(){
    println!("x =? m");
    let mut x = String::new();
    io::stdin()
    .read_line(&mut x)
    .expect("Failed to read x");
    println!("y =? m");
    let mut y = String::new();
    io::stdin()
    .read_line(&mut y)
    .expect("Failed to read y");
    
    let x:f64 = x.trim()
    .parse()
    .expect("Failed to parse x");
    let y:f64 = y.trim()
    .parse()
    .expect("Failed to parse y");
     let area = calculate_area(x, y);
     println!("Area = {} m²", area);
}
fn calculate_area(x:f64, y:f64) -> f64 {
    x * y
}