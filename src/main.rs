 // main.rs

 mod framebuffer;
 mod line_impl;
 mod bmp;
 
 use framebuffer::Framebuffer;
 use line_impl::Line;
 use bmp::WriteBmp;
 use nalgebra_glm::vec3;
 
 fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    // Define un polígono con múltiples vértices
    let vertices = vec![
        vec3(100.0, 150.0, 0.0),
        vec3(300.0, 50.0, 0.0),
        vec3(500.0, 150.0, 0.0),
        vec3(450.0, 400.0, 0.0),
        vec3(150.0, 400.0, 0.0),
    ];

    // Colores para el borde y el relleno
    let border_color = 0x0000FF; // Azul
    let fill_color = 0x00FF00;   // Verde

    // Dibuja y rellena el polígono
    framebuffer.draw_polygon(&vertices, border_color, fill_color);

    // Guarda la imagen como BMP
    let output_file = "out.bmp";
    match framebuffer.render_buffer(output_file) {
        Ok(_) => println!("Image saved as {}", output_file),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}