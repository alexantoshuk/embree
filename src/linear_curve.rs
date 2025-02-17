use ultraviolet::{Vec2, Vec3, Vec4};

use crate::buffer::Buffer;
use crate::device::Device;
use crate::geometry::Geometry;
use crate::sys::*;
use crate::{BufferType, CurveType, Format, GeometryType};

pub struct LinearCurve<'a> {
    device: &'a Device,
    pub(crate) handle: RTCGeometry,
    pub vertex_buffer: Buffer<'a, Vec4>,
    pub index_buffer: Buffer<'a, u32>,
    pub flag_buffer: Buffer<'a, u32>,
    pub normal_buffer: Option<Buffer<'a, Vec3>>,
}

impl<'a> LinearCurve<'a> {
    pub fn flat(
        device: &'a Device,
        num_segments: usize,
        num_verts: usize,
        use_normals: bool,
    ) -> LinearCurve<'a> {
        LinearCurve::unanimated(
            device,
            num_segments,
            num_verts,
            CurveType::Flat,
            use_normals,
        )
    }
    pub fn round(
        device: &'a Device,
        num_segments: usize,
        num_verts: usize,
        use_normals: bool,
    ) -> LinearCurve<'a> {
        LinearCurve::unanimated(
            device,
            num_segments,
            num_verts,
            CurveType::Round,
            use_normals,
        )
    }
    pub fn cone(
        device: &'a Device,
        num_segments: usize,
        num_verts: usize,
        use_normals: bool,
    ) -> LinearCurve<'a> {
        LinearCurve::unanimated(
            device,
            num_segments,
            num_verts,
            CurveType::Cone,
            use_normals,
        )
    }
    fn unanimated(
        device: &'a Device,
        num_segments: usize,
        num_verts: usize,
        curve_type: CurveType,
        use_normals: bool,
    ) -> LinearCurve<'a> {
        let h: RTCGeometry;
        match curve_type {
            CurveType::Cone => {
                h = unsafe { rtcNewGeometry(device.handle, GeometryType::CONE_LINEAR_CURVE) }
            }
            CurveType::Round => {
                h = unsafe { rtcNewGeometry(device.handle, GeometryType::ROUND_LINEAR_CURVE) }
            }
            _ => h = unsafe { rtcNewGeometry(device.handle, GeometryType::FLAT_LINEAR_CURVE) },
        };
        let mut vertex_buffer = Buffer::new(device, num_verts);
        let mut index_buffer = Buffer::new(device, num_segments);
        let mut flag_buffer = Buffer::new(device, num_segments);
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

            rtcSetGeometryBuffer(
                h,
                BufferType::FLAGS,
                0,
                Format::UCHAR,
                flag_buffer.handle,
                0,
                1,
                num_verts,
            );
            flag_buffer.set_attachment(h, BufferType::FLAGS, 0);

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
            };
        }

        LinearCurve {
            device: device,
            handle: h,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            flag_buffer: flag_buffer,
            normal_buffer: normal_buffer,
        }
    }
}

unsafe impl<'a> Sync for LinearCurve<'a> {}
