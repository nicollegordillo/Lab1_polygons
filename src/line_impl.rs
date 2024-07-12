pub trait Line {
    fn draw_line(&mut self, start: nalgebra_glm::Vec3, end: nalgebra_glm::Vec3);
    fn draw_polygon(&mut self, points: &[nalgebra_glm::Vec3]);
}

use nalgebra_glm::Vec3;

impl Line for crate::framebuffer::Framebuffer {
    fn draw_line(&mut self, start: Vec3, end: Vec3) {
        // Convert 3D coordinates to 2D screen coordinates
        let start_screen_x = start.x as usize;
        let start_screen_y = start.y as usize;
        let end_screen_x = end.x as usize;
        let end_screen_y = end.y as usize;

        // Bresenham's line algorithm adapted for 2D screen coordinates
        let dx = (end_screen_x as i32 - start_screen_x as i32).abs();
        let dy = -(end_screen_y as i32 - start_screen_y as i32).abs();
        let mut err = dx + dy;

        let sx = if start_screen_x < end_screen_x { 1 } else { -1 };
        let sy = if start_screen_y < end_screen_y { 1 } else { -1 };

        let mut x = start_screen_x as i32;
        let mut y = start_screen_y as i32;

        loop {
            self.point(x as usize, y as usize);

            if x == end_screen_x as i32 && y == end_screen_y as i32 {
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

    fn draw_polygon(&mut self, points: &[Vec3]) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            self.draw_line(points[i], points[i + 1]);
        }

        // Draw line from the last point to the first point to close the polygon
        self.draw_line(points[points.len() - 1], points[0]);
    }
}