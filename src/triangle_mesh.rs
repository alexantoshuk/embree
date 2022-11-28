use ultraviolet::Vec3;

use crate::buffer::Buffer;
use crate::device::Device;
use crate::geometry::Geometry;
use crate::sys::*;
use crate::{BufferType, Format, GeometryType};

pub struct TriangleMesh<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vec3>,
    pub index_buffer: Buffer<'a, [u32; 3]>,
}

impl<'a> TriangleMesh<'a> {
    pub fn unanimated(device: &'a Device, num_tris: usize, num_verts: usize) -> TriangleMesh<'a> {
        let h = unsafe { rtcNewGeometry(device.handle, GeometryType::TRIANGLE) };
        let mut vertex_buffer = Buffer::new(device, num_verts);
        let mut index_buffer = Buffer::new(device, num_tris);
        unsafe {
            rtcSetGeometryBuffer(
                h,
                BufferType::VERTEX,
                0,
                Format::FLOAT3,
                vertex_buffer.handle,
                0,
                12,
                num_verts,
            );
            vertex_buffer.set_attachment(h, BufferType::VERTEX, 0);

            rtcSetGeometryBuffer(
                h,
                BufferType::INDEX,
                0,
                Format::UINT3,
                index_buffer.handle,
                0,
                12,
                num_tris,
            );
            index_buffer.set_attachment(h, BufferType::INDEX, 0);
        }
        TriangleMesh {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }
}

unsafe impl<'a> Sync for TriangleMesh<'a> {}
