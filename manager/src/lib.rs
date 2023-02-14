/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use internal_interface::{
    idisplay::{DisplayID, DisplayStatus, IDisplay},
    imanager::{IDisplayCallbacks, IManager, IManagerCallbacks},
};

use display_headless::display_headless;

#[derive(Default)]
struct ManagerMutable {
    displays: HashMap<DisplayID, Arc<dyn IDisplay>>,
    mgr_callbacks: Option<std::sync::Arc<dyn IManagerCallbacks>>,
    disp_callbacks: Option<std::sync::Arc<dyn IDisplayCallbacks>>,
}

#[derive(Default)]
struct Manager {
    mutable: Mutex<ManagerMutable>,
}

impl IManager for Manager {
    fn create_virtual_display(&self, _width: u32, _height: u32) -> Result<DisplayID> {
        Err(anyhow::anyhow!("Not implemented"))
    }
    fn destroy_virtual_display(&self, _display_id: DisplayID) -> Result<()> {
        Err(anyhow::anyhow!("Not implemented"))
    }
    fn get_display(&self, display_id: DisplayID) -> Option<std::sync::Arc<dyn IDisplay>> {
        let mutable = self.mutable.lock().unwrap();
        mutable.displays.get(&display_id).cloned()
    }
    fn get_max_virtual_display_count(&self) -> u32 {
        0
    }
    fn register_callbacks(
        &self,
        callback: std::sync::Arc<dyn IManagerCallbacks>,
        display_callback: std::sync::Arc<dyn IDisplayCallbacks>,
    ) {
        let display_id = 0;
        let callbacks = {
            let mut mutable = self.mutable.lock().unwrap();
            mutable.mgr_callbacks = Some(callback);
            mutable.disp_callbacks = Some(display_callback.clone());

            // Add Headless Display
            let headless_display = display_headless::create_display(display_id, display_callback);
            mutable.displays.insert(display_id, headless_display);
            mutable.mgr_callbacks.clone()
        };
        // Send Hotplug Event
        if let Some(callback) = &callbacks {
            log::trace!("Sending hotplug event to client");
            callback.send_hotplug_event_to_client(display_id, DisplayStatus::Connected);
        }
    }
    fn unregister_callbacks(&self) {
        let mut mutable = self.mutable.lock().unwrap();
        mutable.mgr_callbacks = None;
        mutable.disp_callbacks = None;
    }
}

pub fn create_manager() -> Arc<dyn IManager> {
    Arc::new(Manager::default())
}
