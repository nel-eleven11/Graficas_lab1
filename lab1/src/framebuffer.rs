
use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    // Constructor para la clase framebuffer que no recibe el color de fondo
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let background_color = Color::new(0, 0, 0); // Color de fondo negro por defecto
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color: Color::new(255, 255, 255), // Por defecto, el color actual es blanco
        }
    }

    // Función clear para llenar todo el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    // Función point para colocar un punto en una coordenada x, y con el color actual
    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[(y as usize) * self.width + (x as usize)] = self.current_color;
        }
    }

    // Función para retornar el color de un punto en una coordenada x, y
    pub fn get_color(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some(self.buffer[(y as usize) * self.width + (x as usize)])
        } else {
            None
        }
    }

    // Método para settear el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    // Método para settear el color actual
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Prueba la creación de un nuevo framebuffer y verifica si los valores iniciales están correctamente configurados.
    #[test]
    fn test_new_framebuffer() {
        let fb = Framebuffer::new(10, 10);
        assert_eq!(fb.width, 10);
        assert_eq!(fb.height, 10);
        assert_eq!(fb.background_color, Color::new(0, 0, 0));  // Asumiendo que el color de fondo es negro
    }

    // Prueba la función clear para asegurarse de que el framebuffer se llena completamente con el color de fondo.
    #[test]
    fn test_clear() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(Color::new(100, 100, 100));  // Cambia el color de fondo a gris
        fb.clear();
        for i in 0..fb.width * fb.height {
            assert_eq!(fb.buffer[i], Color::new(100, 100, 100));
        }
    }

    // Verifica que point establece correctamente un color en las coordenadas especificadas.
    #[test]
    fn test_point() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(Color::new(255, 0, 0));  // Color rojo
        fb.point(5, 5);
        assert_eq!(fb.get_color(5, 5).unwrap(), Color::new(255, 0, 0));
        assert_eq!(fb.get_color(0, 0).unwrap(), Color::new(0, 0, 0));  // Asegura que otros puntos no han cambiado
    }

    // Prueba la función point con coordenadas fuera de los límites y verifica que se ignore.
    #[test]
    fn test_point_out_of_bounds() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(Color::new(255, 255, 255));
        fb.point(10, 10);
        fb.point(-1, -1);
        assert_eq!(fb.get_color(10, 10), None);
        assert_eq!(fb.get_color(-1, -1), None);
    }

    // Verifica la función get_color para asegurarse de que retorne el color correcto o None si está fuera de rango.
    #[test]
    fn test_get_color() {
        let fb = Framebuffer::new(10, 10);
        // Verifica que los puntos no establecidos devuelvan el color de fondo
        assert_eq!(fb.get_color(0, 0).unwrap(), Color::new(0, 0, 0));  // Color de fondo negro
        // Verifica que la consulta fuera de rango devuelva None
        assert_eq!(fb.get_color(10, 10), None);
    }

    // Testea la actualización de los colores de fondo y actual.
    #[test]
    fn test_set_colors() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(Color::new(123, 123, 123));  // Color de fondo gris
        fb.set_current_color(Color::new(200, 150, 100));  // Un color marrón

        assert_eq!(fb.background_color, Color::new(123, 123, 123));
        assert_eq!(fb.current_color, Color::new(200, 150, 100));
    }
}
