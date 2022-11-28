use std::f32;

use ultraviolet::*;

#[derive(PartialEq)]
pub struct Camera {
    pub pos: Vec3,
    dir_top_left: Vec3,
    screen_du: Vec3,
    screen_dv: Vec3,
    img: (u32, u32),
}

impl Camera {
    pub fn look_dir(pos: Vec3, dir: Vec3, up: Vec3, fov: f32, img: (u32, u32)) -> Camera {
        let dz = dir.normalized();
        let dx = -dz.cross(up).normalized();
        let dy = dx.cross(dz).normalized();
        let dim_y = 2.0 * f32::tan((fov / 2.0) * f32::consts::PI / 180.0);
        let aspect_ratio = img.0 as f32 / img.1 as f32;
        let dim_x = dim_y * aspect_ratio;
        let screen_du = dx * dim_x;
        let screen_dv = dy * dim_y;
        let dir_top_left = dz - 0.5 * screen_du - 0.5 * screen_dv;
        Camera {
            pos: pos,
            dir_top_left: dir_top_left,
            screen_du: screen_du,
            screen_dv: screen_dv,
            img: img,
        }
    }
    pub fn look_at(pos: Vec3, at: Vec3, up: Vec3, fov: f32, img: (u32, u32)) -> Camera {
        let dir = at - pos;
        Camera::look_dir(pos, dir, up, fov, img)
    }
    /// Compute the ray direction going through the pixel passed
    pub fn ray_dir(&self, px: (f32, f32)) -> Vec3 {
        (self.dir_top_left
            + px.0 / (self.img.0 as f32) * self.screen_du
            + px.1 / (self.img.1 as f32) * self.screen_dv)
            .normalized()
    }
}
