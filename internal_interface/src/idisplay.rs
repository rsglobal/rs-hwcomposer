/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{any::Any, sync::Arc};

use anyhow::Result;

use crate::{idisplay_composition::IDisplayComposition, idisplay_mode::IDisplayModes};

pub type DisplayID = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayStatus {
    Disconnected,
    Connected,
    LinkTrainingFailed,
}

/* Rotation is defined in the clockwise direction */
/* The flip is done before rotation */
#[derive(Default)]
pub struct Transform {
    pub hflip: bool,
    pub vflip: bool,
    pub rotate90: bool,
}

pub trait IDisplay: IDisplayModes + IDisplayComposition + Sync + Send {
    fn get_frontend_private_data(&self) -> Option<Arc<dyn Any + Sync + Send>>;
    fn set_frontend_private_data(&self, data: Arc<dyn Any + Sync + Send>);

    fn get_display_rotation(&self) -> Transform;
    fn set_vsync_enabled(&self, enabled: bool) -> Result<()>;

    fn activate_display(&self, active: bool) -> Result<()>;
}

pub trait IDisplayColorManagement {}
