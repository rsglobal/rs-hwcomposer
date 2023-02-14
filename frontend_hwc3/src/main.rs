/*
 * SPDX-License-Identifier: Apache-2.0
 */

#![feature(try_blocks)]

mod composer;
mod composer_client;
mod error;

use android_hardware_graphics_composer3_V3::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_IComposer;
use composer::Composer;
use log::{error, info};

fn main() {
    // Initialize android logging.
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("drmhwc-rs")
            .with_max_level(log::LevelFilter::Debug)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} ({}:{})",
                    record.args(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                )
            }),
    );

    // Redirect panic messages to logcat.
    std::panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info);
    }));

    info!("drm-hwcomposer is starting.");

    let composer = Composer::default();
    let service = Composer::new_native_binder(composer);
    let service_name = Composer::get_descriptor().to_string() + "/default".to_string().as_str();
    binder::add_service(&service_name, service.as_binder())
        .unwrap_or_else(|_| panic!("Failed to add service {service_name} to binder"));

    info!("Successfully registered rs-hwcomposer service.");

    info!("Joining thread pool now.");
    binder::ProcessState::join_thread_pool();

    panic!("Should not reach here.");
}
