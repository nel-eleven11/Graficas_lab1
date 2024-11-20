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
    
            self.line_clipped(x1, y1, x2, y2);
        }
    }

    pub fn compute_outcode(x: isize, y: isize, x_min: isize, y_min: isize, x_max: isize, y_max: isize) -> u8 {
        let mut code = 0;
        if x < x_min {
            code |= 1; // Izquierda
        } else if x > x_max {
            code |= 2; // Derecha
        }
        if y < y_min {
            code |= 4; // Inferior
        } else if y > y_max {
            code |= 8; // Superior
        }
        code
    }

    pub fn line_clipped(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let (x_min, y_min, x_max, y_max) = (0, 0, self.width as isize - 1, self.height as isize - 1);
    
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let mut x2 = x2 as isize;
        let mut y2 = y2 as isize;
    
        let mut outcode1 = Self::compute_outcode(x1, y1, x_min, y_min, x_max, y_max);
        let mut outcode2 = Self::compute_outcode(x2, y2, x_min, y_min, x_max, y_max);
    
        while outcode1 != 0 || outcode2 != 0 {
            if (outcode1 & outcode2) != 0 {
                return; // Ambos puntos están completamente fuera de la pantalla en la misma región
            }
    
            let outcode = if outcode1 != 0 { outcode1 } else { outcode2 };
            let (x, y);
    
            if (outcode & 1) != 0 {
                // Izquierda
                x = x_min;
                y = y1 + (y2 - y1) * (x_min - x1) / (x2 - x1);
            } else if (outcode & 2) != 0 {
                // Derecha
                x = x_max;
                y = y1 + (y2 - y1) * (x_max - x1) / (x2 - x1);
            } else if (outcode & 4) != 0 {
                // Inferior
                y = y_min;
                x = x1 + (x2 - x1) * (y_min - y1) / (y2 - y1);
            } else {
                // Superior
                y = y_max;
                x = x1 + (x2 - x1) * (y_max - y1) / (y2 - y1);
            }
    
            if outcode == outcode1 {
                x1 = x;
                y1 = y;
                outcode1 = Self::compute_outcode(x1, y1, x_min, y_min, x_max, y_max);
            } else {
                x2 = x;
                y2 = y;
                outcode2 = Self::compute_outcode(x2, y2, x_min, y_min, x_max, y_max);
            }
        }
    
        self.line(x1 as usize, y1 as usize, x2 as usize, y2 as usize);
    }
    
    
    
}