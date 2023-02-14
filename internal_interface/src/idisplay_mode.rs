/*
 * SPDX-License-Identifier: Apache-2.0
 */

use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub vsync_period: u32,
    pub dpi_x: f32,
    pub dpi_y: f32,
    pub config_group: u32,
}

pub type ModeID = i32;

pub struct QueuedConfigTiming {
    // In order for the new config to be applied, the client must send a new frame
    // at this time.
    pub refresh_time_ns: Option<i64>,

    // The time when the display will start to refresh at the new vsync period.
    pub new_vsync_time_ns: i64,
}

pub trait IDisplayModes {
    fn for_each_display_mode(&self, f: &mut dyn FnMut(ModeID, &Mode) -> Result<()>) -> Result<()>;
    fn get_active_mode(&self) -> Result<ModeID>;
    fn get_preferred_mode(&self) -> Result<ModeID>;

    fn queue_active_mode(
        &self,
        mode: ModeID,
        desired_time: Option<i64>,
        seamless: bool,
    ) -> Result<QueuedConfigTiming>;
}
