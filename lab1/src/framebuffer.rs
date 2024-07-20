use crate::color::Color;
//use crate::vertex::Vertex;

pub enum ColorInput {
    Hex(u32),
    Rgb(Color),
}

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
    pub fn set_background_color(&mut self, color: ColorInput) {
        self.background_color = match color {
            ColorInput::Hex(hex) => Color::from_hex(hex),
            ColorInput::Rgb(rgb) => rgb,
        };
    }

    // Método para settear el color actual
    pub fn set_current_color(&mut self, color: ColorInput) {
        self.current_color = match color {
            ColorInput::Hex(hex) => Color::from_hex(hex),
            ColorInput::Rgb(rgb) => rgb,
        };
    }

    // Método para renderizar el framebuffer a un archivo BMP
    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u32> = self.buffer.iter().map(|color| color.to_hex()).collect();
        crate::bmp::write_bmp_file(file_path, &buffer, self.width, self.height)
    }
}
