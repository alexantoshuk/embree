use cgmath::Vector4;

use buffer::Buffer;
use device::Device;
use geometry::Geometry;
use sys::*;
use {BufferType, Format, GeometryType};

pub struct QuadMesh<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vector4<f32>>,
    pub index_buffer: Buffer<'a, Vector4<u32>>,
}
impl<'a> QuadMesh<'a> {
    pub fn unanimated(device: &'a Device, num_quads: usize, num_verts: usize) -> QuadMesh<'a> {
        let h = unsafe { rtcNewGeometry(device.handle, GeometryType::QUAD) };
        let vertex_buffer = Buffer::new(device, num_verts);
        let index_buffer = Buffer::new(device, num_quads);
        unsafe {
            rtcSetGeometryBuffer(
                h,
                BufferType::VERTEX,
                0,
                Format::FLOAT3,
                vertex_buffer.handle,
                0,
                16,
                num_verts,
            );

            rtcSetGeometryBuffer(
                h,
                BufferType::INDEX,
                0,
                Format::UINT4,
                index_buffer.handle,
                0,
                16,
                num_quads,
            );
        }
        QuadMesh {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }
}

