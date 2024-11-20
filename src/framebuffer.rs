// framebuffer.rs

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = -(y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1 as i32;
        let mut y = y1 as i32;

        loop {
            self.point(x as usize, y as usize);

            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

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

    pub fn draw_polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 2 {
            return; // No se puede dibujar un polígono con menos de dos puntos
        }
    
        // Iterar a través de los puntos y conectar cada punto al siguiente
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = if i == points.len() - 1 {
                points[0] // Conectar el último punto al primero
            } else {
                points[i + 1]
            };
    
            self.line(x1, y1, x2, y2);
        }
    }
    
}