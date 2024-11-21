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
    draw_polygon3(&mut framebuffer);
    draw_polygon4_with_hole(&mut framebuffer);

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
    let fill_color = 0x00FFFF;   //amarillo
    let border_color = 0xFFFFFF; //blanco

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
    let fill_color = 0xFF0000;  // azul
    let border_color = 0xFFFFFF; // Blanco

    // Dibujar el polígono
    framebuffer.draw_polygon(&vertices, border_color, fill_color);
}

fn draw_polygon3(framebuffer: &mut Framebuffer) {
    // Definir los vértices del Polígono 3
    let vertices = vec![
        vec3(377.0, 249.0, 0.0),
        vec3(411.0, 197.0, 0.0),
        vec3(436.0, 249.0, 0.0),
    ];

    // Colores
    let fill_color = 0x0000FF;   // Rojo (Hex: RGB)
    let border_color = 0xFFFFFF; // Blanco (Hex: RGB)

    // Dibujar el polígono
    framebuffer.draw_polygon(&vertices, border_color, fill_color);
}

fn draw_polygon4_with_hole(framebuffer: &mut Framebuffer) {
    // Definir los vértices del Polígono 4
    let vertices_polygon4 = vec![
        vec3(413.0, 177.0, 0.0),
        vec3(448.0, 159.0, 0.0),
        vec3(502.0, 88.0, 0.0),
        vec3(553.0, 53.0, 0.0),
        vec3(535.0, 36.0, 0.0),
        vec3(676.0, 37.0, 0.0),
        vec3(660.0, 52.0, 0.0),
        vec3(750.0, 145.0, 0.0),
        vec3(761.0, 179.0, 0.0),
        vec3(672.0, 192.0, 0.0),
        vec3(659.0, 214.0, 0.0),
        vec3(615.0, 214.0, 0.0),
        vec3(632.0, 230.0, 0.0),
        vec3(580.0, 230.0, 0.0),
        vec3(597.0, 215.0, 0.0),
        vec3(552.0, 214.0, 0.0),
        vec3(517.0, 144.0, 0.0),
        vec3(466.0, 180.0, 0.0),
    ];

    // Definir los vértices del Polígono 5 (agujero)
    let vertices_polygon5 = vec![
        vec3(682.0, 175.0, 0.0),
        vec3(708.0, 120.0, 0.0),
        vec3(735.0, 148.0, 0.0),
        vec3(739.0, 170.0, 0.0),
    ];

    // Colores
    let fill_color = 0x00FF00;   // Verde (Hex: RGB)
    let border_color = 0xFFFFFF; // Blanco (Hex: RGB)
    let background_color = 0x000000; // Negro para "borrar" el área del agujero

    // Dibujar el Polígono 4 (relleno verde con borde blanco)
    framebuffer.draw_polygon(&vertices_polygon4, border_color, fill_color);

    // "Borrar" el área del Polígono 5 (relleno negro)
    framebuffer.draw_polygon(&vertices_polygon5, background_color, background_color);

    // Dibujar el contorno del Polígono 5 (borde blanco)
    framebuffer.set_current_color(border_color);
    for i in 0..vertices_polygon5.len() {
        let start = vertices_polygon5[i];
        let end = vertices_polygon5[(i + 1) % vertices_polygon5.len()];
        framebuffer.line(start, end);
    }
}