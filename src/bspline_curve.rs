use cgmath::{Vector2, Vector3, Vector4};

use buffer::Buffer;
use device::Device;
use geometry::Geometry;
use sys::*;
use {BufferType, Format, GeometryType, CurveType};

pub struct BsplineCurve<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vector4<f32>>,
    pub index_buffer: Buffer<'a, u32>,
    pub normal_buffer: Option< Buffer<'a, Vector3<f32>> >,
}

impl<'a> BsplineCurve<'a> {
    pub fn flat( device: &'a Device, num_segments: usize, num_verts: usize, use_normals: bool) -> BsplineCurve<'a> {
        BsplineCurve::unanimated(device, num_segments, num_verts, CurveType::Flat, use_normals)
    }
    pub fn round( device: &'a Device, num_segments: usize, num_verts: usize, use_normals: bool) -> BsplineCurve<'a> {
        BsplineCurve::unanimated(device, num_segments, num_verts, CurveType::Round, use_normals)
    }
    pub fn normal_oriented( device: &'a Device, num_segments: usize, num_verts: usize) -> BsplineCurve<'a> {
        BsplineCurve::unanimated( device, num_segments, num_verts, CurveType::NormalOriented, true)
    }

    fn unanimated(device: &'a Device, num_segments: usize, num_verts: usize, curve_type: CurveType, use_normals: bool) -> BsplineCurve<'a> {
        let h: RTCGeometry;
        match curve_type {
        CurveType::NormalOriented => h = unsafe { rtcNewGeometry(device.handle, GeometryType::NORMAL_ORIENTED_BSPLINE_CURVE) },
        CurveType::Round => h = unsafe { rtcNewGeometry(device.handle, GeometryType::ROUND_BSPLINE_CURVE) },
        _ => h = unsafe { rtcNewGeometry(device.handle, GeometryType::FLAT_BSPLINE_CURVE) },
        };
        let mut vertex_buffer = Buffer::new(device, num_verts);
        let mut index_buffer = Buffer::new(device, num_segments);
        let mut normal_buffer = None;
        
        unsafe {
            rtcSetGeometryBuffer(
                h,
                BufferType::VERTEX,
                0,
                Format::FLOAT4,
                vertex_buffer.handle,
                0,
                16,
                num_verts,
            );
            vertex_buffer.set_attachment(h, BufferType::VERTEX, 0);

            rtcSetGeometryBuffer(
                h,
                BufferType::INDEX,
                0,
                Format::UINT,
                index_buffer.handle,
                0,
                4,
                num_segments,
            );
            index_buffer.set_attachment(h, BufferType::INDEX, 0);

            if use_normals {
                let mut temp_normal_buffer = Buffer::new(device, num_verts);
                rtcSetGeometryBuffer(
                    h,
                    BufferType::NORMAL,
                    0,
                    Format::FLOAT3,
                    temp_normal_buffer.handle,
                    0,
                    12,
                    num_verts,
                );
                temp_normal_buffer.set_attachment(h, BufferType::NORMAL, 0);
                normal_buffer = Some(temp_normal_buffer);
            }

        }
        BsplineCurve {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            normal_buffer: normal_buffer,
        }
    }
}

unsafe impl<'a> Sync for BsplineCurve<'a> {}

