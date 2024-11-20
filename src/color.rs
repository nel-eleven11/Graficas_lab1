// colors.rs

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color {
            r: r.clamp(0, 255) as u8,
            g: g.clamp(0, 255) as u8,
            b: b.clamp(0, 255) as u8
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::new(r as i32, g as i32, b as i32)
    }

    pub fn to_hex(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color { r, g, b }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color {
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clamps_values() {
        let color = Color::new(300, 150, -20);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_multiply_by_negative() {
        let color = Color::new(100, 150, 200);
        let result = color * -1.0;
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
    }

    #[test]
    fn test_add_colors() {
        let color1 = Color::new(200, 100, 50);
        let color2 = Color::new(100, 100, 100);
        let result = color1 + color2;
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 200);
        assert_eq!(result.b, 150);
    }

    #[test]
    fn test_multiply_by_large_number() {
        let color = Color::new(100, 100, 100);
        let result = color * 3.0;
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
    }
}
