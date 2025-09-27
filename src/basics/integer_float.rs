pub fn run() {
    println!(">>> Integer & Float Demo <<<");

    let x: i8 = -10;
    let y: u32 = 10;
    let z: f32 = 10.0;

    println!("x = {}, y = {}, z = {}", x, y, z);

    // Basic arithmetic
    println!("x + y = {}", x as i32 + y as i32);
    println!("y - x = {}", y as i32 - x as i32);
    println!("x * y = {}", x as i32 * y as i32);
    println!("y / z = {:.2}", y as f32 / z);
    println!("y % 3 = {}", y % 3);

    // Shadowing
    let x = x + 5;
    println!("x after adding 5 (shadowing) = {}", x);
}
