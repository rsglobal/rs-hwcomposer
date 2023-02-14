use std::{os::fd::RawFd, sync::Arc};

use nix::libc::uintptr_t;

use anyhow::Result;
use internal_interface::ibuffer::{BufferLayout, IBuffer, PlaneInfo};

unsafe extern "C" {
    unsafe fn gralloc_import_buffer(
        num_fds: i32,
        num_ints: i32,
        fds: *const i32,
        ints: *const i32,
    ) -> uintptr_t;

    unsafe fn gralloc_free_buffer(buffer_handle: uintptr_t);

    unsafe fn gralloc_get_plane_count(buffer_handle: uintptr_t) -> usize;

    unsafe fn gralloc_get_plane_param(
        buffer_handle: uintptr_t,
        plane_index: usize,
        offset_in_bytes: *mut i64,
        stride_in_bytes: *mut i64,
    ) -> bool;

    unsafe fn gralloc_get_format_modifier(
        buffer_handle: uintptr_t,
        out_format: *mut u32,
        out_format_modifier: *mut u64,
    ) -> bool;

    unsafe fn gralloc_get_width_height(
        buffer_handle: uintptr_t,
        out_width: *mut u64,
        out_height: *mut u64,
    ) -> bool;

    unsafe fn gralloc_get_fds(
        buffer_handle: uintptr_t,
        fds_count: *mut i32,
        fds: *mut *mut i32,
    ) -> bool;
}

struct GrallocBuffer {
    layout: BufferLayout,
    buffer_handle: uintptr_t,
    vec_fds: Vec<RawFd>,
}

impl Drop for GrallocBuffer {
    fn drop(&mut self) {
        unsafe {
            gralloc_free_buffer(self.buffer_handle);
        }
    }
}

impl IBuffer for GrallocBuffer {
    fn get_fds(&self) -> Vec<RawFd> {
        self.vec_fds.clone()
    }

    fn get_layout(&self) -> BufferLayout {
        self.layout.clone()
    }
}

pub fn import_gralloc_buffer(fds: &[i32], ints: &[i32]) -> Result<Arc<dyn IBuffer + Send + Sync>> {
    let handle = unsafe {
        gralloc_import_buffer(fds.len() as i32, ints.len() as i32, fds.as_ptr(), ints.as_ptr())
    };
    if handle == 0 {
        return Err(anyhow::anyhow!("Failed to import gralloc buffer"));
    }

    let mut layout = BufferLayout::default();
    let mut width = 0u64;
    let mut height = 0u64;
    let mut format = 0u32;
    let mut format_modifier = 0u64;
    let mut disjoint = false;
    let plane_count = unsafe { gralloc_get_plane_count(handle) };

    for i in 0..plane_count {
        let mut offset = 0i64;
        let mut stride = 0i64;
        if !unsafe { gralloc_get_plane_param(handle, i, &mut offset, &mut stride) } {
            return Err(anyhow::anyhow!("Failed to get plane param"));
        }
        layout.planes.push(PlaneInfo { offset: offset as u32, stride: stride as u32 });
        if i > 0 && offset == 0 {
            disjoint = true;
        }
    }

    if !unsafe { gralloc_get_format_modifier(handle, &mut format, &mut format_modifier) } {
        return Err(anyhow::anyhow!("Failed to get format modifier"));
    }

    if !unsafe { gralloc_get_width_height(handle, &mut width, &mut height) } {
        return Err(anyhow::anyhow!("Failed to get size"));
    }

    layout.width = width as u32;
    layout.height = height as u32;
    layout.drm_format = format;
    layout.drm_modifier = format_modifier;

    let mut imported_fds_count = 0i32;
    let mut imported_fds_ptr: *mut i32 = std::ptr::null_mut();

    if !unsafe { gralloc_get_fds(handle, &mut imported_fds_count, &mut imported_fds_ptr) } {
        return Err(anyhow::anyhow!("Failed to get fds"));
    }

    if disjoint && imported_fds_count < plane_count as i32 {
        return Err(anyhow::anyhow!("Invalid imported fds count"));
    }

    let mut imported_fds = vec![unsafe { *imported_fds_ptr.offset(0) }];
    if disjoint {
        for i in 1..plane_count {
            imported_fds.push(unsafe { *imported_fds_ptr.add(i) });
        }
    }

    log::trace!("Imported buffer layout: {:?}", layout);

    Ok(Arc::new(GrallocBuffer {
        layout,
        buffer_handle: handle as uintptr_t,
        vec_fds: imported_fds,
    }))
}
