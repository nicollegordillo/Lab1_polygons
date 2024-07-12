mod framebuffer;
mod bmp;
mod vertex3_util;
mod line_impl;

use framebuffer::Framebuffer;
use bmp::Bitmap;
use vertex3_util::Vec3;
use line_impl::Line;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Define vertices for each polygon
    let polygon1_vertices = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    framebuffer.set_current_color(0xFFC857);
    framebuffer.fill_polygon(&polygon1_vertices, 0xFFC857);
    
    // Draw each polygon with its respective colors
    framebuffer.set_current_color(0xffffff);
    draw_polygon(&mut framebuffer, &polygon1_vertices); 
    

    // Flip the image vertically (BMP format requirement)
    framebuffer.flip_vertical();

    // Save the framebuffer as a BMP file
    let bitmap = Bitmap::from_framebuffer(&framebuffer);
    bitmap.save_to_file("output.bmp").expect("Failed to save BMP file");
}

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3]) {
    // Check if the number of vertices is at least 3 (minimum required for a polygon)
    if vertices.len() < 3 {
        return; // Cannot draw a polygon with less than 3 vertices
    }

    // Draw lines between consecutive vertices
    for i in 0..vertices.len() - 1 {
        framebuffer.draw_line(vertices[i], vertices[i + 1]);
    }

    // Draw line to close the polygon (between last vertex and first vertex)
    framebuffer.draw_line(vertices[vertices.len() - 1], vertices[0]);

   
}