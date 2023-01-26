use ultraviolet::Vec3;

use crate::buffer::Buffer;
use crate::device::Device;
use crate::geometry::Geometry;
use crate::sys::*;
use crate::{BufferType, Format, GeometryType, SubdivisionMode};

pub struct SubdivMesh<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vec3>,
    pub index_buffer: Buffer<'a, u32>,
    pub face_buffer: Buffer<'a, u32>,
}

impl<'a> SubdivMesh<'a> {
    pub fn unanimated(
        device: &'a Device,
        num_faces: usize,
        num_edges: usize,
        num_verts: usize,
        subdiv_mode: SubdivisionMode,
        subdiv_level: f32,
    ) -> SubdivMesh<'a> {
        let h = unsafe { rtcNewGeometry(device.handle, GeometryType::SUBDIVISION) };
        let mut vertex_buffer = Buffer::new(device, num_verts);
        let mut index_buffer = Buffer::new(device, num_edges);
        let mut face_buffer = Buffer::new(device, num_faces);
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
                BufferType::FACE,
                0,
                Format::UINT,
                face_buffer.handle,
                0,
                4,
                num_faces,
            );
            face_buffer.set_attachment(h, BufferType::FACE, 0);

            rtcSetGeometryBuffer(
                h,
                BufferType::INDEX,
                0,
                Format::UINT,
                index_buffer.handle,
                0,
                4,
                num_edges,
            );
            index_buffer.set_attachment(h, BufferType::INDEX, 0);
        }
        unsafe {
            rtcSetGeometrySubdivisionMode(h, 0, subdiv_mode);
            rtcSetGeometryTessellationRate(h, subdiv_level);
        }
        SubdivMesh {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            face_buffer: face_buffer,
        }
    }
}

unsafe impl<'a> Sync for SubdivMesh<'a> {}
