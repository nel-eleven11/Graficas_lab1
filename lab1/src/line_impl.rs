use crate::framebuffer::{Framebuffer, Color};
use crate::vertex::Vertex;

pub trait Line {
    fn line(&mut self, start: Vertex, end: Vertex);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vertex, end: Vertex) {
        let Vertex { x: x1, y: y1 } = start;
        let Vertex { x: x2, y: y2 } = end;

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.point(x, y); // Dibuja el punto en el framebuffer

            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::{Framebuffer, Color};
    use crate::vertex::Vertex;

    #[test]
    fn test_draw_line() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(Color::new(255, 255, 255)); // Color blanco

        let start = Vertex::new(0, 0);
        let end = Vertex::new(9, 9);
        fb.line(start, end);

        for i in 0..10 {
            assert_eq!(fb.get_color(i, i).unwrap(), Color::new(255, 255, 255));
        }
    }

    #[test]
    fn test_draw_horizontal_line() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(Color::new(255, 255, 255)); // Color blanco

        let start = Vertex::new(0, 5);
        let end = Vertex::new(9, 5);
        fb.line(start, end);

        for i in 0..10 {
            assert_eq!(fb.get_color(i, 5).unwrap(), Color::new(255, 255, 255));
        }
    }

    #[test]
    fn test_draw_vertical_line() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(Color::new(255, 255, 255)); // Color blanco

        let start = Vertex::new(5, 0);
        let end = Vertex::new(5, 9);
        fb.line(start, end);

        for i in 0..10 {
            assert_eq!(fb.get_color(5, i).unwrap(), Color::new(255, 255, 255));
        }
    }
}
