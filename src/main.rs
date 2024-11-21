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

    // Dibujar el Polígono 1
    draw_polygon1(&mut framebuffer);
    draw_polygon2(&mut framebuffer);

    // Guardar la imagen como BMP
    let output_file = "out.bmp";
    match framebuffer.render_buffer(output_file) {
        Ok(_) => println!("Image saved as {}", output_file),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}
 
 fn draw_polygon1(framebuffer: &mut Framebuffer) {
    // Definir los vértices del Polígono 1
    let vertices = vec![
        vec3(165.0, 380.0, 0.0),
        vec3(185.0, 360.0, 0.0),
        vec3(180.0, 330.0, 0.0),
        vec3(207.0, 345.0, 0.0),
        vec3(233.0, 330.0, 0.0),
        vec3(230.0, 360.0, 0.0),
        vec3(250.0, 380.0, 0.0),
        vec3(220.0, 385.0, 0.0),
        vec3(205.0, 410.0, 0.0),
        vec3(193.0, 383.0, 0.0),
    ];

    // Colores
    let fill_color = 0x00FFFF;   
    let border_color = 0xFFFFFF; 

    // Dibujar el polígono
    framebuffer.draw_polygon(&vertices, border_color, fill_color);
}

fn draw_polygon2(framebuffer: &mut Framebuffer) {
    // Definir los vértices del Polígono 2
    let vertices = vec![
        vec3(321.0, 335.0, 0.0),
        vec3(288.0, 286.0, 0.0),
        vec3(339.0, 251.0, 0.0),
        vec3(374.0, 302.0, 0.0),
    ];

    // Colores
    let fill_color = 0xFF0000;   
    let border_color = 0xFFFFFF; 

    // Dibujar el polígono
    framebuffer.draw_polygon(&vertices, border_color, fill_color);
}