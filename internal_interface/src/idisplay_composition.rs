/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::ilayer::{ILayer, LayerID};
use anyhow::Result;
use std::{os::fd::OwnedFd, sync::Arc};

pub type ReleaseFence = (LayerID, Arc<OwnedFd>);

pub trait IDisplayComposition {
    fn create_layer(&self, id: LayerID) -> Result<()>;
    fn destroy_layer(&self, id: LayerID) -> Result<()>;
    fn get_layer(&self, id: LayerID) -> Option<Arc<dyn ILayer>>;
    fn get_client_layer(&self) -> Arc<dyn ILayer>;
    fn get_writeback_layer(&self) -> Option<Arc<dyn ILayer>>;

    fn validate_staged_composition(&self, out_client_layers: &mut Vec<LayerID>) -> Result<()>;
    fn present_staged_composition(
        &self,
        present_time: Option<i64>,
    ) -> Result<(Option<Arc<OwnedFd>>, Vec<ReleaseFence>)>;
}
