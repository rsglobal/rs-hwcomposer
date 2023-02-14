/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{
    any::Any,
    collections::HashMap,
    os::fd::OwnedFd,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result};
use internal_interface::{
    idisplay::{IDisplay, Transform},
    idisplay_composition::IDisplayComposition,
    idisplay_mode::{IDisplayModes, Mode, ModeID},
    ilayer::*,
    imanager::IDisplayCallbacks,
};

use crate::synthetic_vsync::{IVsyncCallback, SyntheticVsync};

struct Layer {
    client_data: Mutex<Option<Arc<dyn Any + Send + Sync>>>,
    id: i64,
}

impl ILayer for Layer {
    fn set_layer_properties(&self, _properties: &LayerProperties) {
        log::trace!("Setting layer properties for layer: {}", self.id);
    }
    fn get_frontend_private_data(&self) -> Option<Arc<dyn Any + Sync + Send>> {
        self.client_data.lock().unwrap().clone()
    }
    fn set_frontend_private_data(&self, data: Arc<dyn Any + Sync + Send>) {
        *self.client_data.lock().unwrap() = Some(data);
    }
}

struct DisplayMutable {
    layers: HashMap<i64, Arc<Layer>>,
    modes: HashMap<ModeID, Mode>,
}

struct Display {
    id: i64,
    client_layer: Arc<Layer>,
    mutable: Mutex<DisplayMutable>,
    client_data: Mutex<Option<Arc<dyn Any + Send + Sync>>>,
    vsync: SyntheticVsync,
}

impl IDisplay for Display {
    fn activate_display(&self, _active: bool) -> Result<()> {
        log::trace!("Activating display: {}", self.id);
        Ok(())
    }

    fn get_frontend_private_data(&self) -> Option<Arc<dyn Any + Sync + Send>> {
        self.client_data.lock().unwrap().clone()
    }

    fn set_frontend_private_data(&self, data: Arc<dyn Any + Sync + Send>) {
        *self.client_data.lock().unwrap() = Some(data);
    }

    fn get_display_rotation(&self) -> Transform {
        Transform::default()
    }

    fn set_vsync_enabled(&self, enabled: bool) -> Result<()> {
        log::trace!("Setting vsync enabled: {}", enabled);
        let mutable = self.mutable.lock().unwrap();
        if enabled {
            self.vsync.set_period(Some(Duration::from_nanos(
                mutable.modes.get(&0).unwrap().vsync_period as u64,
            )));
        } else {
            self.vsync.set_period(None);
        }
        Ok(())
    }
}

impl IDisplayComposition for Display {
    fn create_layer(&self, id: LayerID) -> Result<()> {
        log::trace!("Creating layer: {}", id);
        let mut mutable = self.mutable.lock().unwrap();
        mutable.layers.insert(id, Arc::new(Layer { client_data: Mutex::new(None), id }));
        Ok(())
    }

    fn destroy_layer(&self, id: LayerID) -> Result<()> {
        log::trace!("Destroying layer: {}", id);
        let mut mutable = self.mutable.lock().unwrap();
        mutable.layers.remove(&id);
        Ok(())
    }

    fn get_client_layer(&self) -> Arc<dyn ILayer> {
        self.client_layer.clone()
    }

    fn get_layer(&self, id: LayerID) -> Option<Arc<dyn ILayer>> {
        let s = self.mutable.lock().unwrap().layers.get(&id).cloned();
        s.map(|s| s as Arc<dyn ILayer>)
    }

    fn get_writeback_layer(&self) -> Option<Arc<dyn ILayer>> {
        None
    }

    fn validate_staged_composition(&self, out_client_layers: &mut Vec<LayerID>) -> Result<()> {
        log::trace!("Validating staged composition");
        let mutable = self.mutable.lock().unwrap();
        out_client_layers.extend(mutable.layers.keys().cloned());
        Ok(())
    }

    fn present_staged_composition(
        &self,
        present_time: Option<i64>,
    ) -> Result<(Option<Arc<OwnedFd>>, Vec<internal_interface::idisplay_composition::ReleaseFence>)>
    {
        log::trace!("Presenting staged composition");
        if let Some(present_time) = present_time {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;
            if present_time > now {
                std::thread::sleep(std::time::Duration::from_nanos((present_time - now) as u64));
            }
        }
        Ok((None, vec![]))
    }
}

impl IDisplayModes for Display {
    fn get_active_mode(&self) -> Result<ModeID> {
        log::trace!("Getting active mode");
        Ok(0)
    }

    fn get_preferred_mode(&self) -> Result<ModeID> {
        log::trace!("Getting preferred mode");
        Ok(0)
    }

    fn for_each_display_mode(&self, f: &mut dyn FnMut(ModeID, &Mode) -> Result<()>) -> Result<()> {
        log::trace!("For all display modes");
        let mutable = self.mutable.lock().unwrap();
        for (id, mode) in mutable.modes.iter() {
            f(*id, mode)?;
        }
        Ok(())
    }

    fn queue_active_mode(
        &self,
        mode: ModeID,
        _desired_time: Option<i64>,
        _seamless: bool,
    ) -> Result<internal_interface::idisplay_mode::QueuedConfigTiming> {
        log::trace!("Queueing active mode");
        let mutable = self.mutable.lock().unwrap();
        Ok(internal_interface::idisplay_mode::QueuedConfigTiming {
            refresh_time_ns: None,
            new_vsync_time_ns: mutable.modes.get(&mode).context("OptNone")?.vsync_period as i64,
        })
    }
}

struct CallbacksAdapter {
    display_id: i64,
    cbk: Arc<dyn IDisplayCallbacks>,
}

impl IVsyncCallback for CallbacksAdapter {
    fn on_vsync(&self, timestamp: u64, period: Duration) {
        log::trace!(
            "Sending vsync event to client {}, {}, {}",
            self.display_id,
            timestamp,
            period.as_nanos()
        );
        self.cbk.send_vsync_event_to_client(
            self.display_id,
            timestamp as i64,
            period.as_nanos() as i32,
        );
    }
}

pub fn create_display(
    id: i64,
    callbacks: Arc<dyn IDisplayCallbacks>,
) -> Arc<dyn IDisplay + Send + Sync> {
    let display = Arc::new(Display {
        id,
        client_layer: Arc::new(Layer { client_data: Mutex::new(None), id: -1 }),
        mutable: Mutex::new(DisplayMutable { layers: HashMap::new(), modes: HashMap::new() }),
        client_data: Mutex::new(None),
        vsync: SyntheticVsync::new(Arc::new(CallbacksAdapter { display_id: id, cbk: callbacks })),
    });

    let mut mutable = display.mutable.lock().unwrap();
    mutable.modes.insert(0, Mode {
        width: 1920,
        height: 1080,
        vsync_period: 16_666_666,
        dpi_x: 96.,
        dpi_y: 96.,
        config_group: 0,
    });
    drop(mutable);

    display
}
