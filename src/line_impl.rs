// line.rs
use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
    fn draw_polygon(&mut self, vertices: &[Vec3], border_color: u32, fill_color: u32);
    fn fill_polygon(&mut self, vertices: &[Vec3]);
}

impl Line for Framebuffer {
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

    fn draw_polygon(&mut self, vertices: &[Vec3], border_color: u32, fill_color: u32) {
        if vertices.len() < 3 {
            return; // No se puede dibujar un polígono con menos de 3 vértices
        }

        // Dibujar el relleno primero
        self.set_current_color(fill_color);
        self.fill_polygon(vertices);

        // Dibujar el contorno después
        self.set_current_color(border_color);
        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = vertices[(i + 1) % vertices.len()]; // Conectar el último vértice con el primero
            self.line(start, end);
        }
    }

    /// Rellena un polígono definido por una lista de vértices
    fn fill_polygon(&mut self, vertices: &[Vec3]) {
        if vertices.len() < 3 {
            return; // No se puede rellenar un polígono con menos de 3 vértices
        }

        // Encontrar el rango de y (mínimo y máximo)
        let mut min_y = vertices[0].y;
        let mut max_y = vertices[0].y;
        for vertex in vertices.iter().skip(1) {
            if vertex.y < min_y {
                min_y = vertex.y;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
        }

        // Escaneo por líneas horizontales desde min_y hasta max_y
        for y in min_y as i32..=max_y as i32 {
            let mut intersections = Vec::new();

            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];

                let mut x1 = v1.x;
                let mut y1 = v1.y;
                let mut x2 = v2.x;
                let mut y2 = v2.y;

                // Asegurar que y1 <= y2 para evitar errores
                if y1 > y2 {
                    std::mem::swap(&mut x1, &mut x2);
                    std::mem::swap(&mut y1, &mut y2);
                }

                // Si la línea cruza el escaneo actual en y
                if (y as f32) >= y1 && (y as f32) < y2 {
                    let x_intersect = x1 + (y as f32 - y1) * (x2 - x1) / (y2 - y1);
                    intersections.push(x_intersect);
                }
            }

            // Ordenar las intersecciones de izquierda a derecha
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Rellenar entre pares de intersecciones
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x_start = intersections[i].round() as i32;
                    let x_end = intersections[i + 1].round() as i32;
                    for x in x_start..=x_end {
                        self.point(x as usize, y as usize);
                    }
                }
            }
        }
    }



}

