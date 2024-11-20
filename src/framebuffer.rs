// framebuffer.rs

use nalgebra_glm::Vec3;

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

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }
    

    fn line(&mut self, start: Vec3, end: Vec3) {
        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        // Determine the deltas
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        // Determine the direction of the increment
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            // Draw the current point
            self.point(x0 as usize, y0 as usize);

            // Check if the end point has been reached
            if x0 == x1 && y0 == y1 { break; }

            // Calculate error
            let e2 = err;

            // Adjust error and coordinates based on the dominant direction
            if e2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if e2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_polygon(&mut self, vertices: &[Vec3]) {
        if vertices.len() < 3 {
            // No se puede dibujar un polígono con menos de 3 vértices
            return;
        }

        // Iterar sobre los vértices y dibujar líneas entre cada par consecutivo
        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 {
                vertices[0] // Conectar el último vértice con el primero
            } else {
                vertices[i + 1]
            };

            self.line(start, end);
        }
    }

    
}