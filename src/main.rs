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

    // Define los vértices del polígono
    let vertices = vec![
        vec3(100.0, 100.0, 0.0),
        vec3(300.0, 50.0, 0.0),
        vec3(500.0, 200.0, 0.0),
        vec3(400.0, 400.0, 0.0),
        vec3(200.0, 300.0, 0.0),
    ];

    let vertices2= vec![
        vec3(200.0, 200.0, 0.0),
        vec3(400.0, 150.0, 0.0),
        vec3(200.0, 200.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        vec3(250.0, 380.0, 0.0),
    ];

    // Dibujar el polígono
    framebuffer.set_background_color(0x0000FF); 
    framebuffer.set_current_color(0xFF0000); 
    framebuffer.draw_polygon(&vertices);


    framebuffer.set_background_color(0xFF00FF); 
    framebuffer.set_current_color(0xFF5555); 
    framebuffer.draw_polygon(&vertices2);


    // Guardar la imagen como BMP
    let output_file = "out.bmp";
    match framebuffer.render_buffer(output_file) {
        Ok(_) => println!("Image saved as {}", output_file),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
 } 