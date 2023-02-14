/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::sync::Arc;

use anyhow::Result;

use crate::idisplay::{DisplayID, DisplayStatus, IDisplay};

pub trait IManagerCallbacks: Send + Sync {
    fn send_hotplug_event_to_client(&self, display: DisplayID, status: DisplayStatus);
}

pub trait IDisplayCallbacks: Send + Sync {
    fn send_vsync_event_to_client(&self, display: DisplayID, timestamp: i64, period: i32);
    fn send_vsync_period_timing_changed_event_to_client(&self, display: DisplayID, timestamp: i64);
    fn send_refresh_event_to_client(&self, display: DisplayID);
}

pub trait IManager: Send + Sync {
    fn register_callbacks(
        &self,
        callback: Arc<dyn IManagerCallbacks>,
        display_callback: Arc<dyn IDisplayCallbacks>,
    );
    fn unregister_callbacks(&self);

    fn get_display(&self, display_id: DisplayID) -> Option<Arc<dyn IDisplay>>;
    fn create_virtual_display(&self, width: u32, height: u32) -> Result<DisplayID>;
    fn destroy_virtual_display(&self, display_id: DisplayID) -> Result<()>;
    fn get_max_virtual_display_count(&self) -> u32;
}
