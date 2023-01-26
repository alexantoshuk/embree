use embree::{
    Device, Geometry, IntersectContext, RayHitN, RayN, Scene, SubdivMesh, SubdivisionMode,
};
use ultraviolet::*;

fn main() {
    let device = Device::new();

    // Make a quad
    let mut quad = SubdivMesh::unanimated(&device, 1, 4, 4, SubdivisionMode::SMOOTH_BOUNDARY, 4.0);
    {
        let mut verts = quad.vertex_buffer.as_mut_slice();
        let mut face = quad.face_buffer.as_mut_slice();
        let mut idx = quad.index_buffer.as_mut_slice();
        verts[0] = Vec3::new(-1.0, 0.0, 0.0);
        verts[1] = Vec3::new(-1.0, 1.0, 0.0);
        verts[2] = Vec3::new(1.0, 1.0, 0.0);
        verts[3] = Vec3::new(1.0, 0.0, 0.0);

        face[0] = 4;
        idx[0] = 0;
        idx[1] = 1;
        idx[2] = 2;
        idx[3] = 3;
    }
    let mut subdiv_geom = Geometry::Subdiv(quad);
    subdiv_geom.commit();

    let mut scene = Scene::new(&device);
    scene.attach_geometry(subdiv_geom);
    let rtscene = scene.commit();

    let mut intersection_ctx = IntersectContext::coherent();

    let display = support::Display::new(512, 512, "quad", None);
    display.run(|image, _, _| {
        let img_dims = image.dimensions();
        // Render the scene
        for j in 0..img_dims.1 {
            let y = -(j as f32 + 0.5) / img_dims.1 as f32 + 0.5;

            // Try out streams of scanlines across x
            let mut rays = RayN::new(img_dims.0 as usize);
            for (i, mut ray) in rays.iter_mut().enumerate() {
                let x = (i as f32 + 0.5) / img_dims.0 as f32 - 0.5;
                let dir_len = f32::sqrt(x * x + y * y + 1.0);
                ray.set_origin(Vec3::new(0.0, 0.5, 3.0));
                ray.set_dir(Vec3::new(x / dir_len, y / dir_len, -1.0 / dir_len));
            }

            let mut ray_hit = RayHitN::new(rays);

            rtscene.intersect_stream_soa(&mut intersection_ctx, &mut ray_hit);
            for (i, hit) in ray_hit.hit.iter().enumerate().filter(|(_i, h)| h.hit()) {
                let p = image.get_pixel_mut(i as u32, j);
                let uv = hit.uv();
                p[0] = (uv.0 * 255.0) as u8;
                p[1] = (uv.1 * 255.0) as u8;
                p[2] = 0;
            }
        }
    });
}
