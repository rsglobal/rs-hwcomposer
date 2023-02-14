/*
 * SPDX-License-Identifier: Apache-2.0
 */

use android_hardware_graphics_composer3_V3::aidl::android::hardware::graphics::composer3::{
    Capability::*,
    IComposer::{EX_NO_RESOURCES, *},
    IComposerClient::*,
};
use binder::{BinderFeatures, Strong};
use log::info;
use std::sync::{Arc, RwLock};

use crate::composer_client::{ComposerClient, ComposerClientShared};
use crate::error::{HwcError, HwcResult};

#[derive(Default)]
pub(crate) struct Composer {
    weak_client: RwLock<Option<binder::Weak<dyn IComposerClient>>>,
}

impl IComposer for Composer {
    fn createClient(&self) -> binder::Result<binder::Strong<dyn IComposerClient>> {
        let result: HwcResult<binder::Strong<dyn IComposerClient>> = try {
            info!("Creating composer client");

            if let Some(client) = self.weak_client.write()?.as_ref() {
                if client.upgrade().is_ok() {
                    Err(HwcError::ServiceSpecific(EX_NO_RESOURCES, None))?;
                }
            }

            let cc = ComposerClient {
                shared: Arc::new(RwLock::new(ComposerClientShared {
                    manager: manager::create_manager(),
                    callback: None,
                })),
            };
            let client = ComposerClient::new_native_binder(cc) as Strong<dyn IComposerClient>;
            *self.weak_client.write()? =
                Some(Strong::downgrade(&client) as binder::Weak<dyn IComposerClient + 'static>);
            client
        };
        Ok(result?)
    }
    fn getCapabilities(&self) -> binder::Result<Vec<Capability>> {
        log::trace!("Getting composer capabilities");
        Ok([Capability::LAYER_LIFECYCLE_BATCH_COMMAND].to_vec())
    }
}

impl binder::Interface for Composer {}

impl Composer {
    pub(crate) fn new_native_binder(obj: Composer) -> Strong<dyn IComposer> {
        BnComposer::new_binder(obj, binder::BinderFeatures {
            set_requesting_sid: true,
            ..BinderFeatures::default()
        })
    }
}
