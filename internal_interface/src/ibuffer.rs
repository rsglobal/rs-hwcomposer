/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::os::fd::RawFd;

#[derive(Debug, Default, Clone)]
pub struct PlaneInfo {
    pub offset: u32,
    pub stride: u32,
}

#[derive(Debug, Default, Clone)]
pub struct BufferLayout {
    pub width: u32,
    pub height: u32,
    pub drm_format: u32, // DRM_FORMAT_* from drm_fourcc.h
    pub drm_modifier: u64,
    pub planes: Vec<PlaneInfo>,
}

pub trait IBuffer: Send + Sync {
    /* get_fds result.size == 1 for single plane buffer or non-disjoint multi-plane buffer
     * get_fds result.size == planes.size for disjoint multi-plane buffer
     */
    fn get_fds(&self) -> Vec<RawFd>;
    fn get_layout(&self) -> BufferLayout;
}
