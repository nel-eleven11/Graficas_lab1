mod color;
mod framebuffer;

use color::Color;


fn main() {
    let color1 = Color::new(100, 150, 200);
    let color2 = Color::new(50, 100, 150);
    let color_hex = Color::from_hex(0x6496c8);
    let color_sum = color1 + color2;
    let color_mul = color1 * 1.5;

    println!("Color 1: {}", color1);
    println!("Color 2: {}", color2);
    println!("Color from Hex: {}", color_hex);
    println!("Sum: {}", color_sum);
    println!("Mul: {}", color_mul);

}