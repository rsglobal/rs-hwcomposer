/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{
    collections::HashMap,
    os::fd::{AsRawFd, RawFd},
    sync::{Arc, Mutex},
};

use android_hardware_common::aidl::android::hardware::common::NativeHandle::*;
use android_hardware_graphics_common::aidl::android::hardware::graphics::common::{
    BlendMode::BlendMode as HwcBlendMode, Dataspace::*, DisplayDecorationSupport::*,
    DisplayHotplugEvent::*, Hdr::*, HdrConversionStrategy::*, PixelFormat::*,
};
use android_hardware_graphics_common::aidl::android::hardware::graphics::common::{
    HdrConversionCapability::*, Transform::Transform,
};
use android_hardware_graphics_composer3_V3::aidl::android::hardware::graphics::composer3::{
    ChangedCompositionLayer::*,
    ChangedCompositionTypes::*,
    ClockMonotonicTimestamp::*,
    ColorMode::*,
    CommandError::*,
    CommandResultPayload::*,
    Composition::*,
    ContentType::*,
    DisplayAttribute::*,
    DisplayCapability::*,
    DisplayCommand::*,
    DisplayConfiguration::*,
    DisplayConnectionType::*,
    DisplayContentSample::*,
    DisplayContentSamplingAttributes::*,
    DisplayIdentification::*,
    FormatColorComponent::*,
    HdrCapabilities::*,
    IComposerCallback::*,
    IComposerClient::*,
    LayerCommand::*,
    LayerLifecycleBatchCommandType::*,
    OverlayProperties::*,
    PerFrameMetadataKey::*,
    PowerMode::*,
    PresentFence::*,
    PresentOrValidate::{PresentOrValidate, Result::Result as PresentOrValidateResult},
    ReadbackBufferAttributes::*,
    ReleaseFences,
    RenderIntent::*,
    VirtualDisplay::*,
    VsyncPeriodChangeConstraints::*,
    VsyncPeriodChangeTimeline::*,
};
use anyhow::Result;
use binder::{BinderFeatures, ParcelFileDescriptor, Strong};
use internal_interface::{
    ibuffer::IBuffer,
    idisplay::{DisplayID, DisplayStatus, IDisplay, Transform as ITransform},
    idisplay_mode::Mode,
    ilayer::{
        BlendMode, Buffer, BufferType, CompositionType, DstRectInfo, FRect, ILayer, IRect,
        LayerProperties, Slot, SrcRectInfo,
    },
    imanager::{IDisplayCallbacks, IManager, IManagerCallbacks},
};
use std::sync::RwLock;

use crate::error::{HwcError, HwcResult};

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let pos = name.rfind("::").ok_or(HwcError::OptNone)?;
        let name = &name[..pos];
        let pos = name.rfind("::").ok_or(HwcError::OptNone)? + 2;
        &name[pos..]
    }};
}

macro_rules! unsupported {
    () => {{
        use function;
        use android_hardware_graphics_composer3_V3::aidl::android::hardware::graphics::composer3::IComposerClient::*;
        log::trace!("Unsupported function: {}", function!());
        Err(binder::Status::new_service_specific_error(EX_UNSUPPORTED, None))
    }};
}

macro_rules! disp {
    ($shared:expr, $id:expr) => {{
        let di = $shared.read()?.manager.get_display($id);
        di.ok_or(HwcError::ServiceSpecific(EX_BAD_DISPLAY, None))
    }};
}

struct Hwc3Disp {
    pub last_layer_id: Mutex<i64>,
}

fn get_hwc3_disp(disp: &Arc<dyn IDisplay>) -> HwcResult<Arc<Hwc3Disp>> {
    let mut data = disp.get_frontend_private_data();
    if data.is_none() {
        let d = Arc::new(Hwc3Disp { last_layer_id: Mutex::new(1) });
        disp.set_frontend_private_data(d.clone());
        data = Some(d);
    }
    let result = data
        .ok_or(HwcError::ServiceSpecific(EX_NO_RESOURCES, None))?
        .downcast::<Hwc3Disp>()
        .map_err(|_| HwcError::ServiceSpecific(EX_NO_RESOURCES, None))?;

    Ok(result)
}

#[derive(Default)]
struct Hwc3Layer {
    slots: Mutex<HashMap<i32, Arc<dyn IBuffer + Send + Sync>>>,
}

fn get_hwc3_layer(layer: &Arc<dyn ILayer>) -> HwcResult<Arc<Hwc3Layer>> {
    let mut data = layer.get_frontend_private_data();
    if data.is_none() {
        let l = Arc::new(Hwc3Layer::default());
        layer.set_frontend_private_data(l.clone());
        data = Some(l);
    }
    let result = data
        .ok_or(HwcError::ServiceSpecific(EX_NO_RESOURCES, None))?
        .downcast::<Hwc3Layer>()
        .map_err(|_| HwcError::ServiceSpecific(EX_NO_RESOURCES, None))?;

    Ok(result)
}

impl Hwc3Layer {
    fn handle_next_buffer(
        &self,
        raw_handle: Option<&NativeHandle>,
        fence: Option<&ParcelFileDescriptor>,
        slot: i32,
    ) -> HwcResult<LayerProperties> {
        let mut slots = self.slots.lock()?;

        if raw_handle.is_none() && slots.contains_key(&slot) {
            let mut lp = LayerProperties::default();
            let fence = fence.map(|f| Arc::new(f.as_ref().try_clone().unwrap()));
            lp.active_slot = Some(Slot { slot_id: slot, fence });
            return Ok(lp);
        }

        if raw_handle.is_none() {
            log::error!("Buffer handle is nullopt but slot was not cached.");
            return Err(HwcError::ServiceSpecific(EX_BAD_PARAMETER, None));
        }

        let raw_handle = raw_handle.unwrap();

        let mut fds = Vec::<RawFd>::new();
        for fd in &raw_handle.fds {
            fds.push(fd.as_raw_fd());
        }

        let buffer = buffer_gralloc::import_gralloc_buffer(&fds, &raw_handle.ints)?;

        slots.insert(slot, buffer.clone());

        let fence = fence.map(|f| Arc::new(f.as_ref().try_clone().unwrap()));
        Ok(LayerProperties {
            slot_buffer: Some(Buffer { slot_id: slot, bi: buffer }),
            active_slot: Some(Slot { slot_id: slot, fence }),
            ..Default::default()
        })
    }
}

pub(crate) type ComposerClientSharedT = Arc<RwLock<ComposerClientShared>>;

pub(crate) struct ComposerClientShared {
    pub(crate) manager: Arc<dyn IManager>,
    pub(crate) callback: Option<binder::Strong<dyn IComposerCallback>>,
}

pub(crate) struct ComposerClient {
    pub(crate) shared: ComposerClientSharedT,
}

struct CallbacksAdapter {
    shared: ComposerClientSharedT,
}

impl IManagerCallbacks for CallbacksAdapter {
    fn send_hotplug_event_to_client(&self, display: DisplayID, status: DisplayStatus) {
        if let Some(callback) = &self.shared.read().unwrap().callback {
            let event = match status {
                DisplayStatus::Connected => DisplayHotplugEvent::CONNECTED,
                DisplayStatus::Disconnected => DisplayHotplugEvent::DISCONNECTED,
                DisplayStatus::LinkTrainingFailed => DisplayHotplugEvent::ERROR_INCOMPATIBLE_CABLE,
            };
            callback.onHotplugEvent(display, event).unwrap_or_else(|err| {
                log::error!("Failed to send hotplug event to client: {:?}", err);
            });
        }
    }
}

impl IDisplayCallbacks for CallbacksAdapter {
    fn send_refresh_event_to_client(&self, display: DisplayID) {
        if let Some(callback) = &self.shared.read().unwrap().callback {
            callback.onRefresh(display).unwrap_or_else(|err| {
                log::error!("Failed to send refresh event to client: {:?}", err);
            });
        }
    }
    fn send_vsync_event_to_client(&self, display: DisplayID, timestamp: i64, period: i32) {
        if let Some(callback) = &self.shared.read().unwrap().callback {
            callback.onVsync(display, timestamp, period).unwrap_or_else(|err| {
                log::error!("Failed to send vsync event to client: {:?}", err);
            });
        }
    }
    fn send_vsync_period_timing_changed_event_to_client(&self, display: DisplayID, timestamp: i64) {
        if let Some(callback) = &self.shared.read().unwrap().callback {
            let ct = VsyncPeriodChangeTimeline {
                newVsyncAppliedTimeNanos: timestamp,
                refreshRequired: false,
                refreshTimeNanos: 0,
            };
            callback.onVsyncPeriodTimingChanged(display, &ct).unwrap_or_else(|err| {
                log::error!(
                    "Failed to send vsync period timing changed event to client: {:?}",
                    err
                );
            });
        }
    }
}

impl ComposerClient {
    pub(crate) fn handle_layer_command(
        &self,
        layer: &LayerCommand,
        disp: &Arc<dyn IDisplay>,
    ) -> HwcResult<()> {
        match layer.layerLifecycleBatchCommandType {
            LayerLifecycleBatchCommandType::CREATE => {
                disp.create_layer(layer.layer)
                    .map_err(|_| HwcError::ServiceSpecific(EX_BAD_LAYER, None))?;
            }
            LayerLifecycleBatchCommandType::DESTROY => {
                disp.destroy_layer(layer.layer)
                    .map_err(|_| HwcError::ServiceSpecific(EX_BAD_LAYER, None))?;
                return Ok(());
            }
            _ => {}
        }

        let clayer =
            disp.get_layer(layer.layer).ok_or(HwcError::ServiceSpecific(EX_BAD_LAYER, None))?;
        let hwc3_layer = get_hwc3_layer(&clayer)?;

        let mut lp = LayerProperties::default();

        if let Some(buffer) = &layer.buffer {
            lp = hwc3_layer.handle_next_buffer(
                buffer.handle.as_ref(),
                buffer.fence.as_ref(),
                buffer.slot,
            )?;
        }

        if let Some(blend) = &layer.blendMode {
            log::trace!("Updating layer blend mode to {:?}", blend.blendMode);
            let bm = match blend.blendMode {
                HwcBlendMode::NONE => BlendMode::None,
                HwcBlendMode::PREMULTIPLIED => BlendMode::PreMult,
                HwcBlendMode::COVERAGE => BlendMode::Coverage,
                _ => BlendMode::Undefined,
            };
            lp.blend_mode = Some(bm);
        }

        if let Some(composition) = &layer.composition {
            log::trace!("Updating layer composition to {:?}", composition.composition);
            if composition.composition == Composition::DISPLAY_DECORATION {
                Err(HwcError::ServiceSpecific(EX_UNSUPPORTED, None))?;
            }

            let ct = match composition.composition {
                Composition::DEVICE => {
                    CompositionType::TryImplementationSpecific(BufferType::Regular)
                }
                Composition::CURSOR => {
                    CompositionType::TryImplementationSpecific(BufferType::Cursor)
                }
                _ => CompositionType::ClientOnly,
            };

            lp.composition_type = Some(ct);
        }

        if let Some(df) = &layer.displayFrame {
            log::trace!("Updating layer display frame to {:?}", df);
            let df = DstRectInfo::IRect(IRect {
                left: df.left,
                top: df.top,
                right: df.right,
                bottom: df.bottom,
            });
            lp.display_frame = Some(df);
        }

        if let Some(sc) = &layer.sourceCrop {
            log::trace!("Updating layer source crop to {:?}", sc);
            let sc = SrcRectInfo::FRect(FRect {
                left: sc.left,
                top: sc.top,
                right: sc.right,
                bottom: sc.bottom,
            });
            lp.source_crop = Some(sc);
        }

        if let Some(pa) = &layer.planeAlpha {
            log::trace!("Updating layer plane alpha to {:?}", pa);
            lp.alpha = Some(pa.alpha);
        }

        if let Some(z) = &layer.z {
            log::trace!("Updating layer z to {:?}", z);
            lp.z_order = Some(z.z as u32);
        }

        if let Some(trans) = &layer.transform {
            log::trace!("Updating color transform to {:?}", trans.transform);
            let tr = ITransform {
                hflip: (trans.transform.0 & Transform::FLIP_H.0) != 0,
                vflip: (trans.transform.0 & Transform::FLIP_V.0) != 0,
                rotate90: (trans.transform.0 & Transform::ROT_90.0) != 0,
            };
            lp.transform = Some(tr);
        }

        if let Some(brightness) = &layer.brightness {
            log::trace!("Updating layer brightness to {:?}", brightness.brightness);
            if brightness.brightness < 0.0
                || brightness.brightness > 1.0
                || brightness.brightness.is_nan()
            {
                Err(HwcError::ServiceSpecific(EX_BAD_PARAMETER, None))?;
            }
        }

        clayer.set_layer_properties(&lp);

        Ok(())
    }
}

impl IComposerClient for ComposerClient {
    fn clearBootDisplayConfig(&self, _display: i64) -> binder::Result<()> {
        unsupported!()
    }
    fn createLayer(&self, display: i64, buffer_slot_count: i32) -> binder::Result<i64> {
        let result: HwcResult<i64> = try {
            log::trace!("Creating layer with {} slots", buffer_slot_count);
            let disp = disp!(self.shared, display)?;
            let hwc3disp = get_hwc3_disp(&disp)?;
            let mut last_layer_id = hwc3disp.last_layer_id.lock()?;
            let layer_id = *last_layer_id;

            disp.create_layer(layer_id)?;
            *last_layer_id += 1;
            layer_id
        };
        Ok(result?)
    }
    fn destroyLayer(&self, display: i64, layer: i64) -> binder::Result<()> {
        let result: HwcResult<()> = try {
            log::trace!("Destroying layer {}", layer);
            let disp = disp!(self.shared, display)?;
            disp.destroy_layer(layer)?;
        };
        Ok(result?)
    }
    fn destroyVirtualDisplay(&self, display: i64) -> binder::Result<()> {
        let result: HwcResult<()> = try {
            log::trace!("Destroying virtual display {}", display);
            self.shared.read()?.manager.destroy_virtual_display(display)?
        };
        Ok(result?)
    }
    fn createVirtualDisplay(
        &self,
        width: i32,
        height: i32,
        format_hint: PixelFormat,
        _output_buffer_slot_count: i32,
    ) -> binder::Result<VirtualDisplay> {
        let result: HwcResult<VirtualDisplay> = try {
            log::trace!("Creating virtual display {}x{}", width, height);
            let display_id =
                self.shared.read()?.manager.create_virtual_display(width as u32, height as u32)?;
            VirtualDisplay { display: display_id, format: format_hint }
        };
        Ok(result?)
    }
    fn executeCommands(
        &self,
        commands: &[DisplayCommand],
    ) -> binder::Result<Vec<CommandResultPayload>> {
        log::trace!("Executing commands: size: {}", commands.len());
        let mut result_payload = Vec::<CommandResultPayload>::new();
        for (cmd_index, command) in commands.iter().enumerate() {
            let result: HwcResult<()> = try {
                let disp = disp!(self.shared, command.display)?;

                if let Some(ct) = &command.clientTarget {
                    log::trace!("Updating client target at slot[{}]", ct.buffer.slot);
                    let layer = disp.get_client_layer();

                    let hwc3_layer = get_hwc3_layer(&layer)?;
                    let lp = hwc3_layer.handle_next_buffer(
                        ct.buffer.handle.as_ref(),
                        ct.buffer.fence.as_ref(),
                        ct.buffer.slot,
                    )?;
                    layer.set_layer_properties(&lp);
                    // TODO: damage && colorspace?
                }

                for layer in &command.layers {
                    log::trace!("Updating layer {}", layer.layer);
                    let result = self.handle_layer_command(layer, &disp);
                    if let Err(err) = result {
                        result_payload.push(CommandResultPayload::Error(CommandError {
                            commandIndex: cmd_index as i32,
                            errorCode: match err {
                                HwcError::ServiceSpecific(code, _msg) => code,
                                _ => EX_NO_RESOURCES,
                            },
                        }));
                    }
                }

                let mut present_time: Option<i64> = None;
                if let Some(timestamp) = &command.expectedPresentTime {
                    log::trace!("Setting expected present time for {}", command.display);
                    present_time = Some(timestamp.timestampNanos);
                }

                let mut out_client_layers = Vec::new();
                if command.validateDisplay || command.presentOrValidateDisplay {
                    disp.validate_staged_composition(&mut out_client_layers)?;
                    let mut changed_types =
                        ChangedCompositionTypes { display: command.display, layers: Vec::new() };
                    for layer in &out_client_layers {
                        changed_types.layers.push(ChangedCompositionLayer {
                            layer: *layer,
                            composition: Composition::CLIENT,
                        });
                    }
                    result_payload
                        .push(CommandResultPayload::ChangedCompositionTypes(changed_types));
                }

                if command.presentOrValidateDisplay {
                    log::trace!("Presenting or validating display {}", command.display);
                    let result = if out_client_layers.is_empty() {
                        PresentOrValidateResult::Presented
                    } else {
                        PresentOrValidateResult::Validated
                    };
                    let result = PresentOrValidate { display: command.display, result };
                    result_payload.push(CommandResultPayload::PresentOrValidateResult(result));
                }
                if command.acceptDisplayChanges {
                    log::trace!("Accepting display changes for {}", command.display);
                    /* Not needed */
                }
                if command.presentDisplay || out_client_layers.is_empty() {
                    log::trace!("Presenting display {}", command.display);
                    let (present_fence, release_fences) =
                        disp.present_staged_composition(present_time)?;
                    // Todo add fences to output stream
                    let mut pfd: Option<ParcelFileDescriptor> = None;
                    if let Some(pf) = present_fence.as_ref() {
                        pfd = Some(ParcelFileDescriptor::new(pf.try_clone().unwrap()));
                    }
                    result_payload.push(CommandResultPayload::PresentFence(PresentFence {
                        display: command.display,
                        fence: pfd,
                    }));

                    let mut layers = Vec::new();
                    for (layer_id, file) in &release_fences {
                        pfd = Some(ParcelFileDescriptor::new(file.try_clone().unwrap()));
                        layers.push(ReleaseFences::Layer::Layer { layer: *layer_id, fence: pfd });
                    }

                    result_payload.push(CommandResultPayload::ReleaseFences(
                        ReleaseFences::ReleaseFences { display: command.display, layers },
                    ));
                }
                if command.brightness.as_ref().is_some() {
                    Err(HwcError::ServiceSpecific(EX_UNSUPPORTED, None))?;
                }
            };
            if let Err(err) = result {
                log::error!("Error processing command: {:?}", err);
                let err = match err {
                    HwcError::ServiceSpecific(code, _msg) => {
                        CommandError { commandIndex: cmd_index as i32, errorCode: code }
                    }
                    _ => {
                        CommandError { commandIndex: cmd_index as i32, errorCode: EX_NO_RESOURCES }
                    }
                };
                result_payload.push(CommandResultPayload::Error(err));
            }
        }
        Ok(result_payload)
    }
    fn getActiveConfig(&self, display: i64) -> binder::Result<i32> {
        log::trace!("Getting active config for {}", display);
        let result: HwcResult<i32> = try {
            let disp = disp!(self.shared, display)?;
            disp.get_active_mode()?
        };
        Ok(result?)
    }
    fn getColorModes(&self, display: i64) -> binder::Result<Vec<ColorMode>> {
        log::trace!("Getting color modes for {}", display);
        let result: HwcResult<Vec<ColorMode>> = try {
            let _disp = disp!(self.shared, display)?;
            [ColorMode::NATIVE].to_vec()
        };
        Ok(result?)
    }
    fn getDataspaceSaturationMatrix(&self, dataspace: Dataspace) -> binder::Result<Vec<f32>> {
        if dataspace == Dataspace::UNKNOWN {
            return Err(binder::Status::new_service_specific_error(EX_BAD_PARAMETER, None));
        }
        if dataspace == Dataspace::SRGB_LINEAR {
            #[rustfmt::skip]
            const IDENTITY: [f32; 16] = [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ];
            return Ok(IDENTITY.to_vec());
        }
        Err(binder::Status::new_service_specific_error(EX_UNSUPPORTED, None))
    }
    fn getDisplayAttribute(
        &self,
        display: i64,
        config: i32,
        attribute: DisplayAttribute,
    ) -> binder::Result<i32> {
        let result: HwcResult<i32> = try {
            log::trace!("Getting display attribute for {}", display);
            let disp = disp!(self.shared, display)?;
            let mut output = None;

            disp.for_each_display_mode(&mut |mode_id, mode| {
                if mode_id == config {
                    output = Some(match attribute {
                        DisplayAttribute::VSYNC_PERIOD => mode.vsync_period as i32,
                        DisplayAttribute::WIDTH => mode.width as i32,
                        DisplayAttribute::HEIGHT => mode.height as i32,
                        DisplayAttribute::DPI_X => mode.dpi_x as i32,
                        DisplayAttribute::DPI_Y => mode.dpi_y as i32,
                        DisplayAttribute::CONFIG_GROUP => mode.config_group as i32,
                        _ => Err(HwcError::ServiceSpecific(EX_BAD_PARAMETER, None))?,
                    });
                }
                Ok(())
            })?;

            output.ok_or(HwcError::ServiceSpecific(EX_BAD_CONFIG, None))?
        };
        Ok(result?)
    }
    fn getDisplayCapabilities(&self, display: i64) -> binder::Result<Vec<DisplayCapability>> {
        log::trace!("Getting display capabilities for {}", display);
        let result: HwcResult<Vec<DisplayCapability>> = try {
            let _disp = disp!(self.shared, display)?;
            [DisplayCapability::MULTI_THREADED_PRESENT].to_vec()
        };
        Ok(result?)
    }
    fn getDisplayConfigs(&self, display: i64) -> binder::Result<Vec<i32>> {
        log::trace!("Getting display configs for {}", display);
        let result: HwcResult<Vec<i32>> = try {
            let disp = disp!(self.shared, display)?;
            let mut configs = Vec::new();
            disp.for_each_display_mode(&mut |mode_id, _| {
                configs.push(mode_id);
                Ok(())
            })?;
            configs
        };
        Ok(result?)
    }
    fn getDisplayConnectionType(&self, display: i64) -> binder::Result<DisplayConnectionType> {
        log::trace!("Getting display connection type for {}", display);
        let result: HwcResult<DisplayConnectionType> = try {
            let _disp = disp!(self.shared, display)?;
            DisplayConnectionType::INTERNAL
        };
        Ok(result?)
    }
    fn getDisplayDecorationSupport(
        &self,
        display: i64,
    ) -> binder::Result<Option<DisplayDecorationSupport>> {
        log::trace!("Getting display decoration support for {}", display);
        let result: HwcResult<Option<DisplayDecorationSupport>> = try {
            let _disp = disp!(self.shared, display)?;
            None
        };
        Ok(result?)
    }
    fn getDisplayIdentificationData(&self, display: i64) -> binder::Result<DisplayIdentification> {
        log::trace!("Getting display identification data for {}", display);
        let result: HwcResult<DisplayIdentification> = try {
            let _disp = disp!(self.shared, display)?;
            DisplayIdentification::default()
        };
        Ok(result?)
    }
    fn getDisplayName(&self, display: i64) -> binder::Result<String> {
        log::trace!("Getting display name for {}", display);
        let result: HwcResult<String> = try {
            let _disp = disp!(self.shared, display)?;
            "".to_string()
        };
        Ok(result?)
    }
    fn getDisplayPhysicalOrientation(&self, display: i64) -> binder::Result<Transform> {
        log::trace!("Getting display physical orientation for {}", display);
        let result: HwcResult<Transform> = try {
            let disp = disp!(self.shared, display)?;
            let transform = disp.get_display_rotation();
            let mut out_transform = Transform::NONE;
            if transform.hflip {
                out_transform.0 |= Transform::FLIP_H.0;
            }
            if transform.vflip {
                out_transform.0 |= Transform::FLIP_V.0;
            }
            if transform.rotate90 {
                out_transform.0 |= Transform::ROT_90.0;
            }
            out_transform
        };
        Ok(result?)
    }
    fn getDisplayVsyncPeriod(&self, display: i64) -> binder::Result<i32> {
        log::trace!("Getting display vsync period for {}", display);
        let result: HwcResult<i32> = try {
            let disp = disp!(self.shared, display)?;
            let mode = disp.get_active_mode()?;
            let mut vsync_period = None;
            disp.for_each_display_mode(&mut |mode_id, m| {
                if mode_id == mode {
                    vsync_period = Some(m.vsync_period);
                }
                Ok(())
            })?;
            vsync_period.ok_or(HwcError::OptNone)? as i32
        };

        Ok(result?)
    }
    fn getDisplayedContentSample(
        &self,
        _display: i64,
        _max_frames: i64,
        _timestamp: i64,
    ) -> binder::Result<DisplayContentSample> {
        unsupported!()
    }
    fn getDisplayedContentSamplingAttributes(
        &self,
        _display: i64,
    ) -> binder::Result<DisplayContentSamplingAttributes> {
        unsupported!()
    }
    fn getHdrCapabilities(&self, display: i64) -> binder::Result<HdrCapabilities> {
        log::trace!("Getting HDR capabilities for {}", display);
        let result: HwcResult<HdrCapabilities> = try {
            let _disp = disp!(self.shared, display)?;
            HdrCapabilities::default()
        };
        Ok(result?)
    }
    fn getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
        log::trace!("Getting max virtual display count");
        let result: HwcResult<i32> =
            try { self.shared.read()?.manager.get_max_virtual_display_count() as i32 };
        Ok(result?)
    }
    fn getPerFrameMetadataKeys(&self, _display: i64) -> binder::Result<Vec<PerFrameMetadataKey>> {
        unsupported!()
    }
    fn getPreferredBootDisplayConfig(&self, display: i64) -> binder::Result<i32> {
        log::trace!("Getting preferred boot display config for {}", display);
        let result: HwcResult<i32> = try {
            let disp = disp!(self.shared, display)?;
            disp.get_preferred_mode()?
        };
        Ok(result?)
    }
    fn getReadbackBufferAttributes(
        &self,
        _display: i64,
    ) -> binder::Result<ReadbackBufferAttributes> {
        unsupported!()
    }
    fn getReadbackBufferFence(
        &self,
        _display: i64,
    ) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
        unsupported!()
    }
    fn getRenderIntents(&self, display: i64, mode: ColorMode) -> binder::Result<Vec<RenderIntent>> {
        log::trace!("Getting render intents for {}", display);
        let result: HwcResult<Vec<RenderIntent>> = try {
            let _disp = disp!(self.shared, display)?;
            if mode != ColorMode::NATIVE {
                Err(HwcError::ServiceSpecific(EX_BAD_PARAMETER, None))?;
            }
            [RenderIntent::COLORIMETRIC].to_vec()
        };
        Ok(result?)
    }
    fn getSupportedContentTypes(&self, display: i64) -> binder::Result<Vec<ContentType>> {
        let result: HwcResult<Vec<ContentType>> = try {
            let _disp = disp!(self.shared, display)?;
            [].to_vec()
        };
        Ok(result?)
    }
    fn setActiveConfig(&self, display: i64, config: i32) -> binder::Result<()> {
        log::trace!("Setting active config for {}", display);
        let result: HwcResult<()> = try {
            let disp = disp!(self.shared, display)?;
            disp.queue_active_mode(config, None, false)?;
        };
        Ok(result?)
    }
    fn setActiveConfigWithConstraints(
        &self,
        display: i64,
        config: i32,
        constraints: &VsyncPeriodChangeConstraints,
    ) -> binder::Result<VsyncPeriodChangeTimeline> {
        let result: HwcResult<VsyncPeriodChangeTimeline> = try {
            let disp = disp!(self.shared, display)?;
            let result = disp.queue_active_mode(
                config,
                Some(constraints.desiredTimeNanos),
                constraints.seamlessRequired,
            )?;
            VsyncPeriodChangeTimeline {
                newVsyncAppliedTimeNanos: result.new_vsync_time_ns,
                refreshTimeNanos: result.refresh_time_ns.unwrap_or(0),
                refreshRequired: result.refresh_time_ns.is_some(),
            }
        };
        Ok(result?)
    }
    fn setAutoLowLatencyMode(&self, _display: i64, _on: bool) -> binder::Result<()> {
        let result: HwcResult<()> = try {
            let _disp = disp!(self.shared, _display)?;
            Err(HwcError::ServiceSpecific(EX_UNSUPPORTED, None))?
        };
        Ok(result?)
    }
    fn setBootDisplayConfig(&self, _display: i64, _config: i32) -> binder::Result<()> {
        unsupported!()
    }
    fn setClientTargetSlotCount(
        &self,
        display: i64,
        client_target_slot_count: i32,
    ) -> binder::Result<()> {
        log::trace!(
            "Setting client target slot count for {}: {}",
            display,
            client_target_slot_count
        );
        let result: HwcResult<()> = try {
            let _ = disp!(self.shared, display)?;
        };
        Ok(result?)
    }
    fn setColorMode(
        &self,
        display: i64,
        mode: ColorMode,
        intent: RenderIntent,
    ) -> binder::Result<()> {
        log::trace!("Setting color mode for {}: {:?} {:?}", display, mode, intent);
        if mode != ColorMode::NATIVE {
            return Err(binder::Status::new_service_specific_error(EX_BAD_PARAMETER, None));
        }
        if intent != RenderIntent::COLORIMETRIC {
            return Err(binder::Status::new_service_specific_error(EX_BAD_PARAMETER, None));
        }
        let result: HwcResult<()> = try {
            let _disp = disp!(self.shared, display)?;
        };
        Ok(result?)
    }
    fn setContentType(&self, display: i64, r#type: ContentType) -> binder::Result<()> {
        let result: HwcResult<()> = try {
            let _disp = disp!(self.shared, display)?;
            if r#type != ContentType::NONE {
                Err(HwcError::ServiceSpecific(EX_UNSUPPORTED, None))?;
            };
        };
        Ok(result?)
    }
    fn setDisplayedContentSamplingEnabled(
        &self,
        _display: i64,
        _enable: bool,
        _component_mask: FormatColorComponent,
        _max_frames: i64,
    ) -> binder::Result<()> {
        unsupported!()
    }
    fn setIdleTimerEnabled(&self, _display: i64, _timeout_ms: i32) -> binder::Result<()> {
        unsupported!()
    }
    fn setPowerMode(&self, display: i64, mode: PowerMode) -> binder::Result<()> {
        log::trace!("Setting power mode for {}", display);
        if mode.0 < 0 {
            return Err(binder::Status::new_service_specific_error(EX_BAD_PARAMETER, None));
        }
        let result: HwcResult<()> = try {
            let active = match mode {
                PowerMode::OFF => false,
                PowerMode::ON => true,
                _ => {
                    return Err(binder::Status::new_service_specific_error(EX_UNSUPPORTED, None));
                }
            };
            disp!(self.shared, display)?.activate_display(active)?;
        };
        Ok(result?)
    }

    fn setReadbackBuffer(
        &self,
        _display: i64,
        _buffer: &NativeHandle,
        _release_fence: Option<&binder::ParcelFileDescriptor>,
    ) -> binder::Result<()> {
        unsupported!()
    }

    fn setVsyncEnabled(&self, display: i64, enabled: bool) -> binder::Result<()> {
        log::trace!("Setting vsync enabled for {}", display);
        let result: HwcResult<()> = try {
            let disp = disp!(self.shared, display)?;
            disp.set_vsync_enabled(enabled)?;
        };
        Ok(result?)
    }
    fn registerCallback(&self, cbk: &binder::Strong<dyn IComposerCallback>) -> binder::Result<()> {
        log::trace!("Registering callback");
        let result: HwcResult<()> = try {
            let mut shared = self.shared.write()?;
            shared.callback = Some(cbk.clone());
            let manager = shared.manager.clone();
            drop(shared);

            manager.register_callbacks(
                Arc::new(CallbacksAdapter { shared: self.shared.clone() }),
                Arc::new(CallbacksAdapter { shared: self.shared.clone() }),
            );
        };
        Ok(result?)
    }
    fn getDisplayConfigurations(
        &self,
        display: i64,
        _max_frame_interval_ns: i32,
    ) -> binder::Result<Vec<DisplayConfiguration>> {
        let result: HwcResult<Vec<DisplayConfiguration>> = try {
            let disp = disp!(self.shared, display)?;
            let mut configs = Vec::new();
            disp.for_each_display_mode(&mut |mode: i32, mode_parameters: &Mode| -> Result<()> {
                let config = DisplayConfiguration {
                    configId: mode,
                    width: mode_parameters.width as i32,
                    height: mode_parameters.height as i32,
                    vsyncPeriod: mode_parameters.vsync_period as i32,
                    dpi: Some(Dpi::Dpi { x: mode_parameters.dpi_x, y: mode_parameters.dpi_y }),
                    configGroup: mode_parameters.config_group as i32,
                    vrrConfig: None,
                };
                configs.push(config);
                Ok(())
            })?;
            configs
        };
        Ok(result?)
    }
    fn getHdrConversionCapabilities(&self) -> binder::Result<Vec<HdrConversionCapability>> {
        unsupported!()
    }

    fn r#getOverlaySupport(&self) -> binder::Result<OverlayProperties> {
        unsupported!()
    }

    fn r#setHdrConversionStrategy(
        &self,
        _conversion_strategy: &HdrConversionStrategy,
    ) -> binder::Result<Hdr> {
        unsupported!()
    }

    fn r#setRefreshRateChangedCallbackDebugEnabled(
        &self,
        _display: i64,
        _enabled: bool,
    ) -> binder::Result<()> {
        unsupported!()
    }

    fn r#notifyExpectedPresent(
        &self,
        _display: i64,
        _expected_present_time: &ClockMonotonicTimestamp,
        _frame_interval_ns: i32,
    ) -> binder::Result<()> {
        unsupported!()
    }
}

impl binder::Interface for ComposerClient {}

impl ComposerClient {
    pub(crate) fn new_native_binder(obj: ComposerClient) -> Strong<dyn IComposerClient> {
        BnComposerClient::new_binder(obj, binder::BinderFeatures {
            set_requesting_sid: true,
            ..BinderFeatures::default()
        })
    }
}
