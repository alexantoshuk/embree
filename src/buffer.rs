use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::{mem, ptr};

use crate::device::Device;
use crate::sys::*;
use crate::BufferType;

#[derive(Copy, Clone)]
struct BufferAttachment {
    geom: RTCGeometry,
    buf_type: BufferType,
    slot: u32,
}

impl BufferAttachment {
    fn none() -> BufferAttachment {
        BufferAttachment {
            geom: ptr::null_mut(),
            buf_type: BufferType::VERTEX,
            slot: std::u32::MAX,
        }
    }
    fn is_attached(&self) -> bool {
        self.geom != ptr::null_mut()
    }
}

// TODO: To handle this nicely for sharing/re-using/changing buffer views
// we basically need an API/struct for making buffer views of existing
// larger buffers.
pub struct Buffer<'a, T> {
    device: &'a Device,
    pub(crate) handle: RTCBuffer,
    // TODO: We need a list of RTCGeometry handles
    // that we're attached to to mark buffers as updated on
    // the geometries.
    bytes: usize,
    attachment: BufferAttachment,
    marker: PhantomData<T>,
}

impl<'a, T> Buffer<'a, T> {
    /// Allocate a buffer with some raw capacity in bytes
    pub fn raw(device: &'a Device, bytes: usize) -> Buffer<'a, T> {
        // Pad to a multiple of 16 bytes
        let bytes = if bytes % 16 == 0 {
            bytes
        } else {
            bytes + bytes / 16
        };
        Buffer {
            device: device,
            handle: unsafe { rtcNewBuffer(device.handle, bytes) },
            bytes: bytes,
            attachment: BufferAttachment::none(),
            marker: PhantomData,
        }
    }
    pub fn new(device: &'a Device, len: usize) -> Buffer<'a, T> {
        let mut bytes = len * mem::size_of::<T>();
        // Pad to a multiple of 16 bytes
        bytes = if bytes % 16 == 0 {
            bytes
        } else {
            bytes + bytes / 16
        };
        Buffer {
            device: device,
            handle: unsafe { rtcNewBuffer(device.handle, bytes) },
            bytes: bytes,
            attachment: BufferAttachment::none(),
            marker: PhantomData,
        }
    }

    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        let len = self.bytes / mem::size_of::<T>();
        unsafe {
            let slice = rtcGetBufferData(self.handle) as *mut T;
            std::slice::from_raw_parts_mut(slice, len)
        }
    }

    pub fn as_slice(&mut self) -> &'a [T] {
        let len = self.bytes / mem::size_of::<T>();
        unsafe {
            let slice = rtcGetBufferData(self.handle) as *const T;
            std::slice::from_raw_parts(slice, len)
        }
    }

    pub(crate) fn set_attachment(&mut self, geom: RTCGeometry, buf_type: BufferType, slot: u32) {
        self.attachment.geom = geom;
        self.attachment.buf_type = buf_type;
        self.attachment.slot = slot;
    }
}

impl<'a, T> Drop for Buffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            rtcReleaseBuffer(self.handle);
        }
    }
}

unsafe impl<'a, T> Sync for Buffer<'a, T> {}
