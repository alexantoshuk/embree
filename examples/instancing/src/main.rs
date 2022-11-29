use embree::{
    Device, Geometry, Instance, IntersectContext, QuadMesh, Ray, RayHit, Scene, TriangleMesh,
};
use std::{f32, u32};
use support::Camera;
use ultraviolet::*;

/// Make a triangulated sphere, from the Embree tutorial:
/// https://github.com/embree/embree/blob/master/tutorials/instanced_geometry/instanced_geometry_device.cpp
fn make_triangulated_sphere<'a>(device: &'a Device, pos: Vec3, radius: f32) -> Geometry<'a> {
    let num_phi = 5;
    let num_theta = 2 * num_phi;
    let mut mesh = TriangleMesh::unanimated(
        device,
        2 * num_theta * (num_phi - 1),
        num_theta * (num_phi + 1),
    );
    {
        let mut verts = mesh.vertex_buffer.as_mut_slice();
        let mut tris = mesh.index_buffer.as_mut_slice();

        let inv_num_phi = 1.0 / (num_phi as f32);
        let inv_num_theta = 1.0 / (num_theta as f32);
        for phi in 0..num_phi + 1 {
            for theta in 0..num_theta {
                let phif = phi as f32 * f32::consts::PI * inv_num_phi;
                let thetaf = theta as f32 * f32::consts::PI * 2.0 * inv_num_theta;

                let v = &mut verts[phi * num_theta + theta];
                v.x = pos.x + radius * f32::sin(phif) * f32::sin(thetaf);
                v.y = pos.y + radius * f32::cos(phif);
                v.z = pos.z + radius * f32::sin(phif) * f32::cos(thetaf);
            }
        }

        let mut tri = 0;
        for phi in 1..num_phi + 1 {
            for theta in 1..num_theta + 1 {
                let p00 = (phi - 1) * num_theta + theta - 1;
                let p01 = (phi - 1) * num_theta + theta % num_theta;
                let p10 = phi * num_theta + theta - 1;
                let p11 = phi * num_theta + theta % num_theta;

                if phi > 1 {
                    tris[tri] = [p10 as u32, p01 as u32, p00 as u32];
                    tri += 1;
                }
                if phi < num_phi {
                    tris[tri] = [p11 as u32, p01 as u32, p10 as u32];
                    tri += 1;
                }
            }
        }
    }
    let mut mesh = Geometry::Triangle(mesh);
    mesh.commit();
    mesh
}
fn make_ground_plane<'a>(device: &'a Device) -> Geometry<'a> {
    let mut mesh = QuadMesh::unanimated(device, 1, 4);
    {
        let mut verts = mesh.vertex_buffer.as_mut_slice();
        let mut quads = mesh.index_buffer.as_mut_slice();
        verts[0] = Vec3::new(-10.0, -2.0, -10.0);
        verts[1] = Vec3::new(-10.0, -2.0, 10.0);
        verts[2] = Vec3::new(10.0, -2.0, 10.0);
        verts[3] = Vec3::new(10.0, -2.0, -10.0);

        quads[0] = [0, 1, 2, 3];
    }
    let mut mesh = Geometry::Quad(mesh);
    mesh.commit();
    mesh
}
// Animate like the Embree example, returns the (transforms, normal_transforms)
fn animate_instances(time: f32, num_instances: usize) -> (Vec<Mat4>, Vec<Mat4>) {
    let t0 = 0.7 * time;
    let t1 = 1.5 * time;

    let rot = Mat4::new(
        Vec4::new(f32::cos(t1), 0.0, f32::sin(t1), 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(-f32::sin(t1), 0.0, f32::cos(t1), 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );

    let mut transforms = Vec::with_capacity(num_instances);
    let mut normal_transforms = Vec::with_capacity(num_instances);
    for i in 0..num_instances {
        let t = t0 + i as f32 * 2.0 * f32::consts::PI / 4.0;
        let trans = Mat4::from_translation(2.2 * Vec3::new(f32::cos(t), 0.0, f32::sin(t)));
        transforms.push(trans * rot);
        normal_transforms.push(transforms[i].inversed().transposed());
    }
    (transforms, normal_transforms)
}

fn main() {
    let mut display = support::Display::new(512, 512, "instancing", None);
    let device = Device::new();

    // Make the scene we'll instance with 4 triangulated spheres.
    let spheres = vec![
        make_triangulated_sphere(&device, Vec3::new(0.0, 0.0, 1.0), 0.5),
        make_triangulated_sphere(&device, Vec3::new(1.0, 0.0, 0.0), 0.5),
        make_triangulated_sphere(&device, Vec3::new(0.0, 0.0, -1.0), 0.5),
        make_triangulated_sphere(&device, Vec3::new(-1.0, 0.0, 0.0), 0.5),
    ];
    let mut instanced_scene = Scene::new(&device);
    for s in spheres.into_iter() {
        instanced_scene.attach_geometry(s);
    }
    let committed_instance = instanced_scene.commit();

    // Make the instances first so their ids will be 0-3 that we can then use
    // directly to index into the instance_colors
    let instances = vec![
        Instance::unanimated(&device, &committed_instance),
        Instance::unanimated(&device, &committed_instance),
        Instance::unanimated(&device, &committed_instance),
        Instance::unanimated(&device, &committed_instance),
    ];
    let num_instances = instances.len();

    let mut scene = Scene::new(&device);
    for i in instances.into_iter() {
        scene.attach_geometry(Geometry::Instance(i));
    }

    let instance_colors = vec![
        vec![
            Vec3::new(0.25, 0.0, 0.0),
            Vec3::new(0.5, 0.0, 0.0),
            Vec3::new(0.75, 0.0, 0.0),
            Vec3::new(1.00, 0.0, 0.0),
        ],
        vec![
            Vec3::new(0.0, 0.25, 0.0),
            Vec3::new(0.0, 0.50, 0.0),
            Vec3::new(0.0, 0.75, 0.0),
            Vec3::new(0.0, 1.00, 0.0),
        ],
        vec![
            Vec3::new(0.0, 0.0, 0.25),
            Vec3::new(0.0, 0.0, 0.50),
            Vec3::new(0.0, 0.0, 0.75),
            Vec3::new(0.0, 0.0, 1.00),
        ],
        vec![
            Vec3::new(0.25, 0.25, 0.0),
            Vec3::new(0.50, 0.50, 0.0),
            Vec3::new(0.75, 0.75, 0.0),
            Vec3::new(1.00, 1.00, 0.0),
        ],
    ];

    let ground = make_ground_plane(&device);
    let ground_id = scene.attach_geometry(ground);

    let light_dir = Vec3::new(1.0, 1.0, -1.0).normalized();
    let mut intersection_ctx = IntersectContext::coherent();

    display.run(|image, camera_pose, time| {
        for p in image.iter_mut() {
            *p = 0;
        }

        // Update scene transformations
        let (transforms, normal_transforms) = animate_instances(time, num_instances);
        let mut tfm_iter = transforms.iter();
        for g in scene.iter_mut() {
            if let Geometry::Instance(ref mut inst) = g.1 {
                inst.set_transform(tfm_iter.next().expect("out of bounds tfm"));
            }
            // A bit annoying here that we can't call the mut on the geometry
            // part because we borred the inner instance piece as mutable
            g.1.commit();
        }

        let rtscene = scene.commit();

        let img_dims = image.dimensions();
        let camera = Camera::look_dir(
            camera_pose.pos,
            camera_pose.dir,
            camera_pose.up,
            55.0,
            img_dims,
        );
        // Render the scene
        for j in 0..img_dims.1 {
            for i in 0..img_dims.0 {
                let dir = camera.ray_dir((i as f32 + 0.5, j as f32 + 0.5));
                let mut ray_hit = RayHit::new(Ray::new(camera.pos, dir));
                rtscene.intersect(&mut intersection_ctx, &mut ray_hit);

                if ray_hit.hit.hit() {
                    // Transform the normals of the instances into world space with the normal_transforms
                    let hit = &ray_hit.hit;
                    let geom_id = hit.geomID;
                    let inst_id = hit.instID[0];
                    let mut normal = Vec3::new(hit.Ng_x, hit.Ng_y, hit.Ng_z).normalized();
                    if inst_id != u32::MAX {
                        let v = normal_transforms[inst_id as usize]
                            * Vec4::new(normal.x, normal.y, normal.z, 0.0);
                        normal = Vec3::new(v.x, v.y, v.z).normalized()
                    }
                    let mut illum = 0.3;
                    let shadow_pos = camera.pos + dir * ray_hit.ray.tfar;
                    let mut shadow_ray = Ray::segment(shadow_pos, light_dir, 0.001, f32::INFINITY);
                    rtscene.occluded(&mut intersection_ctx, &mut shadow_ray);

                    if shadow_ray.tfar >= 0.0 {
                        illum = (illum + f32::max(light_dir.dot(normal), 0.0)).clamp(0.0, 1.0);
                    }

                    let p = image.get_pixel_mut(i, j);
                    if inst_id == u32::MAX && geom_id == ground_id {
                        p[0] = (255.0 * illum) as u8;
                        p[1] = p[0];
                        p[2] = p[0];
                    } else {
                        // Shade the instances using their color
                        let color = &instance_colors[inst_id as usize][geom_id as usize];
                        p[0] = (255.0 * illum * color.x) as u8;
                        p[1] = (255.0 * illum * color.y) as u8;
                        p[2] = (255.0 * illum * color.z) as u8;
                    }
                }
            }
        }
    });
}
