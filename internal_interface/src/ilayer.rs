/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{any::Any, os::fd::OwnedFd, sync::Arc};

use crate::{ibuffer::IBuffer, idisplay::Transform};

pub type LayerID = i64;

pub struct FRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub enum SrcRectInfo {
    FRect(FRect),
    WholeBuffer,
}

pub enum DstRectInfo {
    IRect(IRect),
    WholeDisplay,
}

pub const K_ALPHA_OPAQUE: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorSpace {
    Undefined,
    ItuRec601,
    ItuRec709,
    ItuRec2020,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SampleRange {
    Undefined,
    FullRange,
    LimitedRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlendMode {
    Undefined,
    None,
    PreMult,
    Coverage,
}

pub struct Buffer {
    pub slot_id: i32,
    pub bi: Arc<dyn IBuffer>,
}

pub struct Slot {
    pub slot_id: i32,
    pub fence: Option<Arc<OwnedFd>>,
}

pub enum BufferType {
    Regular,
    Cursor,
}

pub enum CompositionType {
    ClientOnly,
    TryImplementationSpecific(BufferType),
}

#[derive(Default)]
pub struct LayerProperties {
    pub slot_buffer: Option<Buffer>,
    pub active_slot: Option<Slot>,
    pub blend_mode: Option<BlendMode>,
    pub color_space: Option<ColorSpace>,
    pub sample_range: Option<SampleRange>,
    pub composition_type: Option<CompositionType>,
    pub display_frame: Option<DstRectInfo>,
    pub alpha: Option<f32>,
    pub source_crop: Option<SrcRectInfo>,
    pub transform: Option<Transform>,
    pub z_order: Option<u32>,
}

pub trait ILayer {
    fn set_layer_properties(&self, properties: &LayerProperties);

    fn get_frontend_private_data(&self) -> Option<Arc<dyn Any + Sync + Send>>;
    fn set_frontend_private_data(&self, data: Arc<dyn Any + Sync + Send>);
}
