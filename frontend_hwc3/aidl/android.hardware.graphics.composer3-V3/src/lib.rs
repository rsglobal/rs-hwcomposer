#![allow(warnings)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod graphics {
                pub mod composer3 {
                    pub mod Buffer {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/Buffer.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/Buffer.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Buffer {
              pub r#slot: i32,
              pub r#handle: Option<crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle>,
              pub r#fence: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#Buffer {
              fn default() -> Self {
                Self {
                  r#slot: 0,
                  r#handle: Default::default(),
                  r#fence: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Buffer {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#slot)?;
                  subparcel.write(&self.r#handle)?;
                  subparcel.write(&self.r#fence)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#slot = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#handle = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#fence = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Buffer);
            binder::impl_deserialize_for_parcelable!(r#Buffer);
            impl binder::binder_impl::ParcelableMetadata for r#Buffer {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.Buffer" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Buffer as _7_android_8_hardware_8_graphics_9_composer3_6_Buffer;
            }
          }
                    pub mod Capability {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/Capability.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/Capability.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Capability : [i32; 9] {
                r#INVALID = 0,
                r#SIDEBAND_STREAM = 1,
                r#SKIP_CLIENT_COLOR_TRANSFORM = 2,
                r#PRESENT_FENCE_IS_NOT_RELIABLE = 3,
                #[deprecated = "- enabled by default."]
                r#SKIP_VALIDATE = 4,
                r#BOOT_DISPLAY_CONFIG = 5,
                r#HDR_OUTPUT_CONVERSION_CONFIG = 6,
                r#REFRESH_RATE_CHANGED_CALLBACK_DEBUG = 7,
                r#LAYER_LIFECYCLE_BATCH_COMMAND = 8,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Capability as _7_android_8_hardware_8_graphics_9_composer3_10_Capability;
            }
          }
                    pub mod ChangedCompositionLayer {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ChangedCompositionLayer.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ChangedCompositionLayer {
              pub r#layer: i64,
              pub r#composition: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_Composition,
            }
            impl Default for r#ChangedCompositionLayer {
              fn default() -> Self {
                Self {
                  r#layer: 0,
                  r#composition: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ChangedCompositionLayer {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#layer)?;
                  subparcel.write(&self.r#composition)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#layer = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#composition = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ChangedCompositionLayer);
            binder::impl_deserialize_for_parcelable!(r#ChangedCompositionLayer);
            impl binder::binder_impl::ParcelableMetadata for r#ChangedCompositionLayer {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ChangedCompositionLayer" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ChangedCompositionLayer as _7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionLayer;
            }
          }
                    pub mod ChangedCompositionTypes {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ChangedCompositionTypes.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ChangedCompositionTypes {
              pub r#display: i64,
              pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionLayer>,
            }
            impl Default for r#ChangedCompositionTypes {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#layers: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ChangedCompositionTypes {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#layers)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layers = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ChangedCompositionTypes);
            binder::impl_deserialize_for_parcelable!(r#ChangedCompositionTypes);
            impl binder::binder_impl::ParcelableMetadata for r#ChangedCompositionTypes {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ChangedCompositionTypes" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ChangedCompositionTypes as _7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes;
            }
          }
                    pub mod ClientTarget {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ClientTarget.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ClientTarget.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ClientTarget {
              pub r#buffer: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer,
              pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
              pub r#damage: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>,
              pub r#hdrSdrRatio: f32,
            }
            impl Default for r#ClientTarget {
              fn default() -> Self {
                Self {
                  r#buffer: Default::default(),
                  r#dataspace: Default::default(),
                  r#damage: Default::default(),
                  r#hdrSdrRatio: 1.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#ClientTarget {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#buffer)?;
                  subparcel.write(&self.r#dataspace)?;
                  subparcel.write(&self.r#damage)?;
                  subparcel.write(&self.r#hdrSdrRatio)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#buffer = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#damage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#hdrSdrRatio = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ClientTarget);
            binder::impl_deserialize_for_parcelable!(r#ClientTarget);
            impl binder::binder_impl::ParcelableMetadata for r#ClientTarget {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTarget" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ClientTarget as _7_android_8_hardware_8_graphics_9_composer3_12_ClientTarget;
            }
          }
                    pub mod ClientTargetProperty {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ClientTargetProperty.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ClientTargetProperty.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ClientTargetProperty {
              pub r#pixelFormat: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
              pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
            }
            impl Default for r#ClientTargetProperty {
              fn default() -> Self {
                Self {
                  r#pixelFormat: Default::default(),
                  r#dataspace: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ClientTargetProperty {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#pixelFormat)?;
                  subparcel.write(&self.r#dataspace)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pixelFormat = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ClientTargetProperty);
            binder::impl_deserialize_for_parcelable!(r#ClientTargetProperty);
            impl binder::binder_impl::ParcelableMetadata for r#ClientTargetProperty {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTargetProperty" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ClientTargetProperty as _7_android_8_hardware_8_graphics_9_composer3_20_ClientTargetProperty;
            }
          }
                    pub mod ClientTargetPropertyWithBrightness {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ClientTargetPropertyWithBrightness {
              pub r#display: i64,
              pub r#clientTargetProperty: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_ClientTargetProperty,
              pub r#brightness: f32,
              pub r#dimmingStage: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_DimmingStage,
            }
            impl Default for r#ClientTargetPropertyWithBrightness {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#clientTargetProperty: Default::default(),
                  r#brightness: 0.000000f32,
                  r#dimmingStage: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ClientTargetPropertyWithBrightness {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#clientTargetProperty)?;
                  subparcel.write(&self.r#brightness)?;
                  subparcel.write(&self.r#dimmingStage)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#clientTargetProperty = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#brightness = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dimmingStage = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ClientTargetPropertyWithBrightness);
            binder::impl_deserialize_for_parcelable!(r#ClientTargetPropertyWithBrightness);
            impl binder::binder_impl::ParcelableMetadata for r#ClientTargetPropertyWithBrightness {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTargetPropertyWithBrightness" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ClientTargetPropertyWithBrightness as _7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness;
            }
          }
                    pub mod ClockMonotonicTimestamp {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ClockMonotonicTimestamp.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ClockMonotonicTimestamp {
              pub r#timestampNanos: i64,
            }
            impl Default for r#ClockMonotonicTimestamp {
              fn default() -> Self {
                Self {
                  r#timestampNanos: 0,
                }
              }
            }
            impl binder::Parcelable for r#ClockMonotonicTimestamp {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#timestampNanos)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#timestampNanos = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ClockMonotonicTimestamp);
            binder::impl_deserialize_for_parcelable!(r#ClockMonotonicTimestamp);
            impl binder::binder_impl::ParcelableMetadata for r#ClockMonotonicTimestamp {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClockMonotonicTimestamp" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ClockMonotonicTimestamp as _7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp;
            }
          }
                    pub mod Color {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/Color.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/Color.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Color {
              pub r#r: f32,
              pub r#g: f32,
              pub r#b: f32,
              pub r#a: f32,
            }
            impl Default for r#Color {
              fn default() -> Self {
                Self {
                  r#r: 0.000000f32,
                  r#g: 0.000000f32,
                  r#b: 0.000000f32,
                  r#a: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#Color {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#r)?;
                  subparcel.write(&self.r#g)?;
                  subparcel.write(&self.r#b)?;
                  subparcel.write(&self.r#a)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#r = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#g = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#b = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#a = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Color);
            binder::impl_deserialize_for_parcelable!(r#Color);
            impl binder::binder_impl::ParcelableMetadata for r#Color {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.Color" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Color as _7_android_8_hardware_8_graphics_9_composer3_5_Color;
            }
          }
                    pub mod ColorMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ColorMode.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ColorMode.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#ColorMode : [i32; 14] {
                r#NATIVE = 0,
                r#STANDARD_BT601_625 = 1,
                r#STANDARD_BT601_625_UNADJUSTED = 2,
                r#STANDARD_BT601_525 = 3,
                r#STANDARD_BT601_525_UNADJUSTED = 4,
                r#STANDARD_BT709 = 5,
                r#DCI_P3 = 6,
                r#SRGB = 7,
                r#ADOBE_RGB = 8,
                r#DISPLAY_P3 = 9,
                r#BT2020 = 10,
                r#BT2100_PQ = 11,
                r#BT2100_HLG = 12,
                r#DISPLAY_BT2020 = 13,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ColorMode as _7_android_8_hardware_8_graphics_9_composer3_9_ColorMode;
            }
          }
                    pub mod CommandError {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/CommandError.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/CommandError.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#CommandError {
              pub r#commandIndex: i32,
              pub r#errorCode: i32,
            }
            impl Default for r#CommandError {
              fn default() -> Self {
                Self {
                  r#commandIndex: 0,
                  r#errorCode: 0,
                }
              }
            }
            impl binder::Parcelable for r#CommandError {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#commandIndex)?;
                  subparcel.write(&self.r#errorCode)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#commandIndex = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#errorCode = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CommandError);
            binder::impl_deserialize_for_parcelable!(r#CommandError);
            impl binder::binder_impl::ParcelableMetadata for r#CommandError {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.CommandError" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#CommandError as _7_android_8_hardware_8_graphics_9_composer3_12_CommandError;
            }
          }
                    pub mod CommandResultPayload {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/CommandResultPayload.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/CommandResultPayload.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub enum r#CommandResultPayload {
              Error(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_CommandError),
              ChangedCompositionTypes(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes),
              DisplayRequest(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest),
              PresentFence(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_PresentFence),
              ReleaseFences(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences),
              PresentOrValidateResult(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate),
              ClientTargetProperty(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness),
            }
            impl Default for r#CommandResultPayload {
              fn default() -> Self {
                Self::Error(Default::default())
              }
            }
            impl binder::Parcelable for r#CommandResultPayload {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Error(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::ChangedCompositionTypes(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::DisplayRequest(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::PresentFence(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::ReleaseFences(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                  Self::PresentOrValidateResult(v) => {
                    parcel.write(&5i32)?;
                    parcel.write(v)
                  }
                  Self::ClientTargetProperty(v) => {
                    parcel.write(&6i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_CommandError = parcel.read()?;
                    *self = Self::Error(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes = parcel.read()?;
                    *self = Self::ChangedCompositionTypes(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest = parcel.read()?;
                    *self = Self::DisplayRequest(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_PresentFence = parcel.read()?;
                    *self = Self::PresentFence(value);
                    Ok(())
                  }
                  4 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences = parcel.read()?;
                    *self = Self::ReleaseFences(value);
                    Ok(())
                  }
                  5 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate = parcel.read()?;
                    *self = Self::PresentOrValidateResult(value);
                    Ok(())
                  }
                  6 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness = parcel.read()?;
                    *self = Self::ClientTargetProperty(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#CommandResultPayload);
            binder::impl_deserialize_for_parcelable!(r#CommandResultPayload);
            impl binder::binder_impl::ParcelableMetadata for r#CommandResultPayload {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.CommandResultPayload" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(4))]
                r#Tag : [i32; 7] {
                  r#error = 0,
                  r#changedCompositionTypes = 1,
                  r#displayRequest = 2,
                  r#presentFence = 3,
                  r#releaseFences = 4,
                  r#presentOrValidateResult = 5,
                  r#clientTargetProperty = 6,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#CommandResultPayload as _7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload_3_Tag;
            }
          }
                    pub mod Composition {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/Composition.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/Composition.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Composition : [i32; 8] {
                r#INVALID = 0,
                r#CLIENT = 1,
                r#DEVICE = 2,
                r#SOLID_COLOR = 3,
                r#CURSOR = 4,
                r#SIDEBAND = 5,
                r#DISPLAY_DECORATION = 6,
                r#REFRESH_RATE_INDICATOR = 7,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Composition as _7_android_8_hardware_8_graphics_9_composer3_11_Composition;
            }
          }
                    pub mod ContentType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ContentType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ContentType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#ContentType : [i32; 5] {
                r#NONE = 0,
                r#GRAPHICS = 1,
                r#PHOTO = 2,
                r#CINEMA = 3,
                r#GAME = 4,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ContentType as _7_android_8_hardware_8_graphics_9_composer3_11_ContentType;
            }
          }
                    pub mod DimmingStage {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DimmingStage.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DimmingStage.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#DimmingStage : [i32; 3] {
                r#NONE = 0,
                r#LINEAR = 1,
                r#GAMMA_OETF = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DimmingStage as _7_android_8_hardware_8_graphics_9_composer3_12_DimmingStage;
            }
          }
                    pub mod DisplayAttribute {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayAttribute.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayAttribute.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#DisplayAttribute : [i32; 7] {
                r#INVALID = 0,
                r#WIDTH = 1,
                r#HEIGHT = 2,
                r#VSYNC_PERIOD = 3,
                r#DPI_X = 4,
                r#DPI_Y = 5,
                r#CONFIG_GROUP = 7,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayAttribute as _7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute;
            }
          }
                    pub mod DisplayBrightness {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayBrightness.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayBrightness.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayBrightness {
              pub r#brightness: f32,
              pub r#brightnessNits: f32,
            }
            impl Default for r#DisplayBrightness {
              fn default() -> Self {
                Self {
                  r#brightness: 0.000000f32,
                  r#brightnessNits: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#DisplayBrightness {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#brightness)?;
                  subparcel.write(&self.r#brightnessNits)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#brightness = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#brightnessNits = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayBrightness);
            binder::impl_deserialize_for_parcelable!(r#DisplayBrightness);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayBrightness {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayBrightness" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayBrightness as _7_android_8_hardware_8_graphics_9_composer3_17_DisplayBrightness;
            }
          }
                    pub mod DisplayCapability {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayCapability.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayCapability.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#DisplayCapability : [i32; 9] {
                r#INVALID = 0,
                r#SKIP_CLIENT_COLOR_TRANSFORM = 1,
                r#DOZE = 2,
                r#BRIGHTNESS = 3,
                r#PROTECTED_CONTENTS = 4,
                r#AUTO_LOW_LATENCY_MODE = 5,
                r#SUSPEND = 6,
                r#DISPLAY_IDLE_TIMER = 7,
                r#MULTI_THREADED_PRESENT = 8,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayCapability as _7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability;
            }
          }
                    pub mod DisplayCommand {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayCommand.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayCommand.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayCommand {
              pub r#display: i64,
              pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_LayerCommand>,
              pub r#colorTransformMatrix: Option<Vec<f32>>,
              pub r#brightness: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayBrightness>,
              pub r#clientTarget: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_ClientTarget>,
              pub r#virtualDisplayOutputBuffer: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer>,
              pub r#expectedPresentTime: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp>,
              pub r#validateDisplay: bool,
              pub r#acceptDisplayChanges: bool,
              pub r#presentDisplay: bool,
              pub r#presentOrValidateDisplay: bool,
              pub r#frameIntervalNs: i32,
            }
            impl Default for r#DisplayCommand {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#layers: Default::default(),
                  r#colorTransformMatrix: Default::default(),
                  r#brightness: Default::default(),
                  r#clientTarget: Default::default(),
                  r#virtualDisplayOutputBuffer: Default::default(),
                  r#expectedPresentTime: Default::default(),
                  r#validateDisplay: false,
                  r#acceptDisplayChanges: false,
                  r#presentDisplay: false,
                  r#presentOrValidateDisplay: false,
                  r#frameIntervalNs: 0,
                }
              }
            }
            impl binder::Parcelable for r#DisplayCommand {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#layers)?;
                  subparcel.write(&self.r#colorTransformMatrix)?;
                  subparcel.write(&self.r#brightness)?;
                  subparcel.write(&self.r#clientTarget)?;
                  subparcel.write(&self.r#virtualDisplayOutputBuffer)?;
                  subparcel.write(&self.r#expectedPresentTime)?;
                  subparcel.write(&self.r#validateDisplay)?;
                  subparcel.write(&self.r#acceptDisplayChanges)?;
                  subparcel.write(&self.r#presentDisplay)?;
                  subparcel.write(&self.r#presentOrValidateDisplay)?;
                  subparcel.write(&self.r#frameIntervalNs)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layers = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#colorTransformMatrix = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#brightness = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#clientTarget = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#virtualDisplayOutputBuffer = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#expectedPresentTime = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#validateDisplay = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#acceptDisplayChanges = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#presentDisplay = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#presentOrValidateDisplay = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#frameIntervalNs = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayCommand);
            binder::impl_deserialize_for_parcelable!(r#DisplayCommand);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayCommand {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayCommand" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayCommand as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand;
            }
          }
                    pub mod DisplayConfiguration {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayConfiguration.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayConfiguration.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayConfiguration {
              pub r#configId: i32,
              pub r#width: i32,
              pub r#height: i32,
              pub r#dpi: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration_3_Dpi>,
              pub r#configGroup: i32,
              pub r#vsyncPeriod: i32,
              pub r#vrrConfig: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig>,
            }
            impl Default for r#DisplayConfiguration {
              fn default() -> Self {
                Self {
                  r#configId: 0,
                  r#width: 0,
                  r#height: 0,
                  r#dpi: Default::default(),
                  r#configGroup: 0,
                  r#vsyncPeriod: 0,
                  r#vrrConfig: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayConfiguration {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#configId)?;
                  subparcel.write(&self.r#width)?;
                  subparcel.write(&self.r#height)?;
                  subparcel.write(&self.r#dpi)?;
                  subparcel.write(&self.r#configGroup)?;
                  subparcel.write(&self.r#vsyncPeriod)?;
                  subparcel.write(&self.r#vrrConfig)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#configId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#width = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#height = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dpi = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#configGroup = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#vsyncPeriod = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#vrrConfig = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayConfiguration);
            binder::impl_deserialize_for_parcelable!(r#DisplayConfiguration);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayConfiguration {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayConfiguration" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Dpi {
              #[derive(Debug)]
              pub struct r#Dpi {
                pub r#x: f32,
                pub r#y: f32,
              }
              impl Default for r#Dpi {
                fn default() -> Self {
                  Self {
                    r#x: 0.000000f32,
                    r#y: 0.000000f32,
                  }
                }
              }
              impl binder::Parcelable for r#Dpi {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#x)?;
                    subparcel.write(&self.r#y)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#x = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#y = subparcel.read()?;
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#Dpi);
              binder::impl_deserialize_for_parcelable!(r#Dpi);
              impl binder::binder_impl::ParcelableMetadata for r#Dpi {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayConfiguration.Dpi" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayConfiguration as _7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration;
             pub use super::r#Dpi::r#Dpi as _7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration_3_Dpi;
            }
          }
                    pub mod DisplayConnectionType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayConnectionType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayConnectionType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#DisplayConnectionType : [i32; 2] {
                r#INTERNAL = 0,
                r#EXTERNAL = 1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayConnectionType as _7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType;
            }
          }
                    pub mod DisplayContentSample {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayContentSample.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayContentSample.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayContentSample {
              pub r#frameCount: i64,
              pub r#sampleComponent0: Vec<i64>,
              pub r#sampleComponent1: Vec<i64>,
              pub r#sampleComponent2: Vec<i64>,
              pub r#sampleComponent3: Vec<i64>,
            }
            impl Default for r#DisplayContentSample {
              fn default() -> Self {
                Self {
                  r#frameCount: 0,
                  r#sampleComponent0: Default::default(),
                  r#sampleComponent1: Default::default(),
                  r#sampleComponent2: Default::default(),
                  r#sampleComponent3: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayContentSample {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#frameCount)?;
                  subparcel.write(&self.r#sampleComponent0)?;
                  subparcel.write(&self.r#sampleComponent1)?;
                  subparcel.write(&self.r#sampleComponent2)?;
                  subparcel.write(&self.r#sampleComponent3)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#frameCount = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sampleComponent0 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sampleComponent1 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sampleComponent2 = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sampleComponent3 = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayContentSample);
            binder::impl_deserialize_for_parcelable!(r#DisplayContentSample);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayContentSample {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayContentSample" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayContentSample as _7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample;
            }
          }
                    pub mod DisplayContentSamplingAttributes {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayContentSamplingAttributes {
              pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
              pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
              pub r#componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent,
            }
            impl Default for r#DisplayContentSamplingAttributes {
              fn default() -> Self {
                Self {
                  r#format: Default::default(),
                  r#dataspace: Default::default(),
                  r#componentMask: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayContentSamplingAttributes {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#format)?;
                  subparcel.write(&self.r#dataspace)?;
                  subparcel.write(&self.r#componentMask)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#format = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#componentMask = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayContentSamplingAttributes);
            binder::impl_deserialize_for_parcelable!(r#DisplayContentSamplingAttributes);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayContentSamplingAttributes {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayContentSamplingAttributes" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayContentSamplingAttributes as _7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes;
            }
          }
                    pub mod DisplayIdentification {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayIdentification.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayIdentification.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayIdentification {
              pub r#port: i8,
              pub r#data: Vec<u8>,
            }
            impl Default for r#DisplayIdentification {
              fn default() -> Self {
                Self {
                  r#port: 0,
                  r#data: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayIdentification {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#port)?;
                  subparcel.write(&self.r#data)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#port = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#data = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayIdentification);
            binder::impl_deserialize_for_parcelable!(r#DisplayIdentification);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayIdentification {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayIdentification" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayIdentification as _7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification;
            }
          }
                    pub mod DisplayRequest {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/DisplayRequest.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/DisplayRequest.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayRequest {
              pub r#display: i64,
              pub r#mask: i32,
              pub r#layerRequests: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest_12_LayerRequest>,
            }
            pub const r#FLIP_CLIENT_TARGET: i32 = 1;
            pub const r#WRITE_CLIENT_TARGET_TO_OUTPUT: i32 = 2;
            impl Default for r#DisplayRequest {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#mask: 0,
                  r#layerRequests: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayRequest {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#mask)?;
                  subparcel.write(&self.r#layerRequests)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mask = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layerRequests = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayRequest);
            binder::impl_deserialize_for_parcelable!(r#DisplayRequest);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayRequest {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayRequest" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#LayerRequest {
              #[derive(Debug)]
              pub struct r#LayerRequest {
                pub r#layer: i64,
                pub r#mask: i32,
              }
              pub const r#CLEAR_CLIENT_TARGET: i32 = 1;
              impl Default for r#LayerRequest {
                fn default() -> Self {
                  Self {
                    r#layer: 0,
                    r#mask: 0,
                  }
                }
              }
              impl binder::Parcelable for r#LayerRequest {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#layer)?;
                    subparcel.write(&self.r#mask)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#layer = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#mask = subparcel.read()?;
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#LayerRequest);
              binder::impl_deserialize_for_parcelable!(r#LayerRequest);
              impl binder::binder_impl::ParcelableMetadata for r#LayerRequest {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayRequest.LayerRequest" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayRequest as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest;
             pub use super::r#LayerRequest::r#LayerRequest as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest_12_LayerRequest;
            }
          }
                    pub mod FormatColorComponent {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/FormatColorComponent.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/FormatColorComponent.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(1))]
              r#FormatColorComponent : [i8; 4] {
                r#FORMAT_COMPONENT_0 = 1,
                r#FORMAT_COMPONENT_1 = 2,
                r#FORMAT_COMPONENT_2 = 4,
                r#FORMAT_COMPONENT_3 = 8,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#FormatColorComponent as _7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent;
            }
          }
                    pub mod HdrCapabilities {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/HdrCapabilities.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/HdrCapabilities.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#HdrCapabilities {
              pub r#types: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>,
              pub r#maxLuminance: f32,
              pub r#maxAverageLuminance: f32,
              pub r#minLuminance: f32,
            }
            impl Default for r#HdrCapabilities {
              fn default() -> Self {
                Self {
                  r#types: Default::default(),
                  r#maxLuminance: 0.000000f32,
                  r#maxAverageLuminance: 0.000000f32,
                  r#minLuminance: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#HdrCapabilities {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#types)?;
                  subparcel.write(&self.r#maxLuminance)?;
                  subparcel.write(&self.r#maxAverageLuminance)?;
                  subparcel.write(&self.r#minLuminance)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#types = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxLuminance = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxAverageLuminance = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#minLuminance = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HdrCapabilities);
            binder::impl_deserialize_for_parcelable!(r#HdrCapabilities);
            impl binder::binder_impl::ParcelableMetadata for r#HdrCapabilities {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.HdrCapabilities" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HdrCapabilities as _7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities;
            }
          }
                    pub mod IComposer {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/IComposer.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/IComposer.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IComposer["android.hardware.graphics.composer3.IComposer"] {
                native: BnComposer(on_transact),
                proxy: BpComposer {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IComposerAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IComposer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
              fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>;
              fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IComposerDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IComposerDefaultRef) -> IComposerDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IComposerAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IComposerAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
              fn r#createClient<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>>;
              fn r#getCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IComposerAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
              async fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>;
              async fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>;
            }
            impl BnComposer {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposer>
              where
                T: IComposerAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
                }
                impl<T, R> IComposer for Wrapper<T, R>
                where
                  T: IComposerAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
                    self._rt.block_on(self._inner.r#createClient())
                  }
                  fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
                    self._rt.block_on(self._inner.r#getCapabilities())
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IComposerAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IComposerAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnComposer>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IComposerAsync<P> for Wrapper {
                  fn r#createClient<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#createClient())
                  }
                  fn r#getCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getCapabilities())
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IComposerAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IComposerDefault: Send + Sync {
              fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#createClient: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#getCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IComposerDefaultRef = Option<std::sync::Arc<dyn IComposerDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IComposerDefaultRef> = std::sync::Mutex::new(None);
            pub const r#EX_NO_RESOURCES: i32 = 6;
            pub const VERSION: i32 = 3;
            pub const HASH: &str = "d24fcd9648b8b2e7287f9238eee9180244612c10";
            impl BpComposer {
              fn build_parcel_createClient(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_createClient(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposer>::getDefaultImpl() {
                    return _aidl_default_impl.r#createClient();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getCapabilities(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getCapabilities(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposer>::getDefaultImpl() {
                    return _aidl_default_impl.r#getCapabilities();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IComposer for BpComposer {
              fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
                let _aidl_data = self.build_parcel_createClient()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#createClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_createClient(_aidl_reply)
              }
              fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
                let _aidl_data = self.build_parcel_getCapabilities()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getCapabilities(_aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IComposerAsync<P> for BpComposer {
              fn r#createClient<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>> {
                let _aidl_data = match self.build_parcel_createClient() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#createClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_createClient(_aidl_reply)
                  }
                )
              }
              fn r#getCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>> {
                let _aidl_data = match self.build_parcel_getCapabilities() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getCapabilities(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IComposer for binder::binder_impl::Binder<BnComposer> {
              fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> { self.0.r#createClient() }
              fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> { self.0.r#getCapabilities() }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IComposer, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#createClient => {
                  let _aidl_return = _aidl_service.r#createClient();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getCapabilities => {
                  let _aidl_return = _aidl_service.r#getCapabilities();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IComposer as _7_android_8_hardware_8_graphics_9_composer3_9_IComposer;
            }
          }
                    pub mod IComposerCallback {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/IComposerCallback.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/IComposerCallback.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IComposerCallback["android.hardware.graphics.composer3.IComposerCallback"] {
                native: BnComposerCallback(on_transact),
                proxy: BpComposerCallback {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IComposerCallbackAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IComposerCallback: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
              #[deprecated = ": Use instead onHotplugEvent"]
              fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()>;
              fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()>;
              fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()>;
              fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()>;
              fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()>;
              fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()>;
              fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()>;
              fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IComposerCallbackDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IComposerCallbackDefaultRef) -> IComposerCallbackDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IComposerCallbackAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IComposerCallbackAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
              #[deprecated = ": Use instead onHotplugEvent"]
              fn r#onHotplug<'a>(&'a self, _arg_display: i64, _arg_connected: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onRefresh<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onSeamlessPossible<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onVsync<'a>(&'a self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onVsyncPeriodTimingChanged<'a>(&'a self, _arg_display: i64, _arg_updatedTimeline: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onVsyncIdle<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onRefreshRateChangedDebug<'a>(&'a self, _arg_data: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#onHotplugEvent<'a>(&'a self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IComposerCallbackAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
              #[deprecated = ": Use instead onHotplugEvent"]
              async fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()>;
              async fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()>;
              async fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()>;
              async fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()>;
              async fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()>;
              async fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()>;
              async fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()>;
              async fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()>;
            }
            impl BnComposerCallback {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposerCallback>
              where
                T: IComposerCallbackAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
                }
                impl<T, R> IComposerCallback for Wrapper<T, R>
                where
                  T: IComposerCallbackAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onHotplug(_arg_display, _arg_connected))
                  }
                  fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onRefresh(_arg_display))
                  }
                  fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onSeamlessPossible(_arg_display))
                  }
                  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos))
                  }
                  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline))
                  }
                  fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onVsyncIdle(_arg_display))
                  }
                  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onRefreshRateChangedDebug(_arg_data))
                  }
                  fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#onHotplugEvent(_arg_display, _arg_event))
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IComposerCallbackAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IComposerCallbackAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnComposerCallback>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IComposerCallbackAsync<P> for Wrapper {
                  fn r#onHotplug<'a>(&'a self, _arg_display: i64, _arg_connected: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onHotplug(_arg_display, _arg_connected))
                  }
                  fn r#onRefresh<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onRefresh(_arg_display))
                  }
                  fn r#onSeamlessPossible<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onSeamlessPossible(_arg_display))
                  }
                  fn r#onVsync<'a>(&'a self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos))
                  }
                  fn r#onVsyncPeriodTimingChanged<'a>(&'a self, _arg_display: i64, _arg_updatedTimeline: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline))
                  }
                  fn r#onVsyncIdle<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onVsyncIdle(_arg_display))
                  }
                  fn r#onRefreshRateChangedDebug<'a>(&'a self, _arg_data: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onRefreshRateChangedDebug(_arg_data))
                  }
                  fn r#onHotplugEvent<'a>(&'a self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#onHotplugEvent(_arg_display, _arg_event))
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IComposerCallbackAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IComposerCallbackDefault: Send + Sync {
              fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#onHotplug: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#onRefresh: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#onSeamlessPossible: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#onVsync: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#onVsyncPeriodTimingChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#onVsyncIdle: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#onRefreshRateChangedDebug: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#onHotplugEvent: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IComposerCallbackDefaultRef = Option<std::sync::Arc<dyn IComposerCallbackDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IComposerCallbackDefaultRef> = std::sync::Mutex::new(None);
            pub const VERSION: i32 = 3;
            pub const HASH: &str = "d24fcd9648b8b2e7287f9238eee9180244612c10";
            impl BpComposerCallback {
              fn build_parcel_onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_connected)?;
                Ok(aidl_data)
              }
              fn read_response_onHotplug(&self, _arg_display: i64, _arg_connected: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onHotplug(_arg_display, _arg_connected);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_onRefresh(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_onRefresh(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onRefresh(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_onSeamlessPossible(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onSeamlessPossible(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_timestamp)?;
                aidl_data.write(&_arg_vsyncPeriodNanos)?;
                Ok(aidl_data)
              }
              fn read_response_onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(_arg_updatedTimeline)?;
                Ok(aidl_data)
              }
              fn read_response_onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onVsyncIdle(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_onVsyncIdle(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onVsyncIdle(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_data)?;
                Ok(aidl_data)
              }
              fn read_response_onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onRefreshRateChangedDebug(_arg_data);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_event)?;
                Ok(aidl_data)
              }
              fn read_response_onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
                    return _aidl_default_impl.r#onHotplugEvent(_arg_display, _arg_event);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IComposerCallback for BpComposerCallback {
              fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onHotplug(_arg_display, _arg_connected)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onHotplug, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onHotplug(_arg_display, _arg_connected, _aidl_reply)
              }
              fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onRefresh(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onRefresh(_arg_display, _aidl_reply)
              }
              fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onSeamlessPossible(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onSeamlessPossible, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onSeamlessPossible(_arg_display, _aidl_reply)
              }
              fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onVsync, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos, _aidl_reply)
              }
              fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncPeriodTimingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline, _aidl_reply)
              }
              fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onVsyncIdle(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncIdle, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onVsyncIdle(_arg_display, _aidl_reply)
              }
              fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onRefreshRateChangedDebug(_arg_data)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onRefreshRateChangedDebug, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onRefreshRateChangedDebug(_arg_data, _aidl_reply)
              }
              fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_onHotplugEvent(_arg_display, _arg_event)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#onHotplugEvent, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_onHotplugEvent(_arg_display, _arg_event, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IComposerCallbackAsync<P> for BpComposerCallback {
              fn r#onHotplug<'a>(&'a self, _arg_display: i64, _arg_connected: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onHotplug(_arg_display, _arg_connected) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onHotplug, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onHotplug(_arg_display, _arg_connected, _aidl_reply)
                  }
                )
              }
              fn r#onRefresh<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onRefresh(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onRefresh(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#onSeamlessPossible<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onSeamlessPossible(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onSeamlessPossible, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onSeamlessPossible(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#onVsync<'a>(&'a self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onVsync, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos, _aidl_reply)
                  }
                )
              }
              fn r#onVsyncPeriodTimingChanged<'a>(&'a self, _arg_display: i64, _arg_updatedTimeline: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onVsyncPeriodTimingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline, _aidl_reply)
                  }
                )
              }
              fn r#onVsyncIdle<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onVsyncIdle(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onVsyncIdle, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onVsyncIdle(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#onRefreshRateChangedDebug<'a>(&'a self, _arg_data: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onRefreshRateChangedDebug(_arg_data) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onRefreshRateChangedDebug, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onRefreshRateChangedDebug(_arg_data, _aidl_reply)
                  }
                )
              }
              fn r#onHotplugEvent<'a>(&'a self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_onHotplugEvent(_arg_display, _arg_event) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#onHotplugEvent, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_onHotplugEvent(_arg_display, _arg_event, _aidl_reply)
                  }
                )
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IComposerCallback for binder::binder_impl::Binder<BnComposerCallback> {
              fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> { self.0.r#onHotplug(_arg_display, _arg_connected) }
              fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onRefresh(_arg_display) }
              fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onSeamlessPossible(_arg_display) }
              fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> { self.0.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos) }
              fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> { self.0.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline) }
              fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onVsyncIdle(_arg_display) }
              fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> { self.0.r#onRefreshRateChangedDebug(_arg_data) }
              fn r#onHotplugEvent(&self, _arg_display: i64, _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent) -> binder::Result<()> { self.0.r#onHotplugEvent(_arg_display, _arg_event) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IComposerCallback, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#onHotplug => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_connected: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onHotplug(_arg_display, _arg_connected);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#onRefresh => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onRefresh(_arg_display);
                  Ok(())
                }
                transactions::r#onSeamlessPossible => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onSeamlessPossible(_arg_display);
                  Ok(())
                }
                transactions::r#onVsync => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_timestamp: i64 = _aidl_data.read()?;
                  let _arg_vsyncPeriodNanos: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos);
                  Ok(())
                }
                transactions::r#onVsyncPeriodTimingChanged => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_updatedTimeline: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onVsyncPeriodTimingChanged(_arg_display, &_arg_updatedTimeline);
                  Ok(())
                }
                transactions::r#onVsyncIdle => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onVsyncIdle(_arg_display);
                  Ok(())
                }
                transactions::r#onRefreshRateChangedDebug => {
                  let _arg_data: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onRefreshRateChangedDebug(&_arg_data);
                  Ok(())
                }
                transactions::r#onHotplugEvent => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_event: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#onHotplugEvent(_arg_display, _arg_event);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IComposerCallback as _7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback;
            }
          }
                    pub mod IComposerClient {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/IComposerClient.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/IComposerClient.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IComposerClient["android.hardware.graphics.composer3.IComposerClient"] {
                native: BnComposerClient(on_transact),
                proxy: BpComposerClient {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IComposerClientAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IComposerClient: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
              fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64>;
              fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>;
              fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()>;
              fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()>;
              fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>;
              fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32>;
              fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>;
              fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>>;
              #[deprecated = "use getDisplayConfigurations instead. Returns a display attribute value for a particular display configuration. For legacy support getDisplayAttribute should return valid values for any requested DisplayAttribute, and for all of the configs obtained either through getDisplayConfigs or getDisplayConfigurations."]
              fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32>;
              fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>;
              #[deprecated = "use getDisplayConfigurations instead. For legacy support getDisplayConfigs should return at least one valid config. All the configs returned from the getDisplayConfigs should also be returned from getDisplayConfigurations."]
              fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>>;
              fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>;
              fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>;
              fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String>;
              fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32>;
              fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>;
              fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>;
              fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>;
              fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>;
              fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32>;
              fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>;
              fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>;
              fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>>;
              fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>;
              fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>;
              fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>;
              fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()>;
              fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
              fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>;
              fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
              fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()>;
              fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32>;
              fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()>;
              fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()>;
              fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()>;
              fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()>;
              fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()>;
              fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()>;
              fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()>;
              fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
              fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()>;
              fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>;
              fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>;
              fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>;
              fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
              fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>>;
              fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IComposerClientDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IComposerClientDefaultRef) -> IComposerClientDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IComposerClientAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IComposerClientAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
              fn r#createLayer<'a>(&'a self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<i64>>;
              fn r#createVirtualDisplay<'a>(&'a self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>>;
              fn r#destroyLayer<'a>(&'a self, _arg_display: i64, _arg_layer: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#destroyVirtualDisplay<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#executeCommands<'a>(&'a self, _arg_commands: &'a [crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>>;
              fn r#getActiveConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
              fn r#getColorModes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>>;
              fn r#getDataspaceSaturationMatrix<'a>(&'a self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::BoxFuture<'a, binder::Result<Vec<f32>>>;
              #[deprecated = "use getDisplayConfigurations instead. Returns a display attribute value for a particular display configuration. For legacy support getDisplayAttribute should return valid values for any requested DisplayAttribute, and for all of the configs obtained either through getDisplayConfigs or getDisplayConfigurations."]
              fn r#getDisplayAttribute<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::BoxFuture<'a, binder::Result<i32>>;
              fn r#getDisplayCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>>;
              #[deprecated = "use getDisplayConfigurations instead. For legacy support getDisplayConfigs should return at least one valid config. All the configs returned from the getDisplayConfigs should also be returned from getDisplayConfigurations."]
              fn r#getDisplayConfigs<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>>;
              fn r#getDisplayConnectionType<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>>;
              fn r#getDisplayIdentificationData<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>>;
              fn r#getDisplayName<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<String>>;
              fn r#getDisplayVsyncPeriod<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
              fn r#getDisplayedContentSample<'a>(&'a self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>>;
              fn r#getDisplayedContentSamplingAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>>;
              fn r#getDisplayPhysicalOrientation<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>>;
              fn r#getHdrCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>>;
              fn r#getMaxVirtualDisplayCount<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
              fn r#getPerFrameMetadataKeys<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>>;
              fn r#getReadbackBufferAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>>;
              fn r#getReadbackBufferFence<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<binder::ParcelFileDescriptor>>>;
              fn r#getRenderIntents<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>>;
              fn r#getSupportedContentTypes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>>;
              fn r#getDisplayDecorationSupport<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>>;
              fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setActiveConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setActiveConfigWithConstraints<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>>;
              fn r#setBootDisplayConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#clearBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getPreferredBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
              fn r#setAutoLowLatencyMode<'a>(&'a self, _arg_display: i64, _arg_on: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setClientTargetSlotCount<'a>(&'a self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setColorMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setContentType<'a>(&'a self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setDisplayedContentSamplingEnabled<'a>(&'a self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setPowerMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setReadbackBuffer<'a>(&'a self, _arg_display: i64, _arg_buffer: &'a crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setVsyncEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setIdleTimerEnabled<'a>(&'a self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getOverlaySupport<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>>;
              fn r#getHdrConversionCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>>;
              fn r#setHdrConversionStrategy<'a>(&'a self, _arg_conversionStrategy: &'a crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>>;
              fn r#setRefreshRateChangedCallbackDebugEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getDisplayConfigurations<'a>(&'a self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>>>;
              fn r#notifyExpectedPresent<'a>(&'a self, _arg_display: i64, _arg_expectedPresentTime: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IComposerClientAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
              async fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64>;
              async fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>;
              async fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()>;
              async fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()>;
              async fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>;
              async fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32>;
              async fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>;
              async fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>>;
              #[deprecated = "use getDisplayConfigurations instead. Returns a display attribute value for a particular display configuration. For legacy support getDisplayAttribute should return valid values for any requested DisplayAttribute, and for all of the configs obtained either through getDisplayConfigs or getDisplayConfigurations."]
              async fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32>;
              async fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>;
              #[deprecated = "use getDisplayConfigurations instead. For legacy support getDisplayConfigs should return at least one valid config. All the configs returned from the getDisplayConfigs should also be returned from getDisplayConfigurations."]
              async fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>>;
              async fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>;
              async fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>;
              async fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String>;
              async fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32>;
              async fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>;
              async fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>;
              async fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>;
              async fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>;
              async fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32>;
              async fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>;
              async fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>;
              async fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>>;
              async fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>;
              async fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>;
              async fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>;
              async fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()>;
              async fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
              async fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>;
              async fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
              async fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()>;
              async fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32>;
              async fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()>;
              async fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()>;
              async fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()>;
              async fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()>;
              async fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()>;
              async fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()>;
              async fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()>;
              async fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
              async fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()>;
              async fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>;
              async fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>;
              async fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>;
              async fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
              async fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>>;
              async fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()>;
            }
            impl BnComposerClient {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposerClient>
              where
                T: IComposerClientAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
                }
                impl<T, R> IComposerClient for Wrapper<T, R>
                where
                  T: IComposerClientAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
                    self._rt.block_on(self._inner.r#createLayer(_arg_display, _arg_bufferSlotCount))
                  }
                  fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
                    self._rt.block_on(self._inner.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount))
                  }
                  fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#destroyLayer(_arg_display, _arg_layer))
                  }
                  fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#destroyVirtualDisplay(_arg_display))
                  }
                  fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
                    self._rt.block_on(self._inner.r#executeCommands(_arg_commands))
                  }
                  fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                    self._rt.block_on(self._inner.r#getActiveConfig(_arg_display))
                  }
                  fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
                    self._rt.block_on(self._inner.r#getColorModes(_arg_display))
                  }
                  fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
                    self._rt.block_on(self._inner.r#getDataspaceSaturationMatrix(_arg_dataspace))
                  }
                  fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
                    self._rt.block_on(self._inner.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute))
                  }
                  fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
                    self._rt.block_on(self._inner.r#getDisplayCapabilities(_arg_display))
                  }
                  fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
                    self._rt.block_on(self._inner.r#getDisplayConfigs(_arg_display))
                  }
                  fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
                    self._rt.block_on(self._inner.r#getDisplayConnectionType(_arg_display))
                  }
                  fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
                    self._rt.block_on(self._inner.r#getDisplayIdentificationData(_arg_display))
                  }
                  fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
                    self._rt.block_on(self._inner.r#getDisplayName(_arg_display))
                  }
                  fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
                    self._rt.block_on(self._inner.r#getDisplayVsyncPeriod(_arg_display))
                  }
                  fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
                    self._rt.block_on(self._inner.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp))
                  }
                  fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
                    self._rt.block_on(self._inner.r#getDisplayedContentSamplingAttributes(_arg_display))
                  }
                  fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
                    self._rt.block_on(self._inner.r#getDisplayPhysicalOrientation(_arg_display))
                  }
                  fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
                    self._rt.block_on(self._inner.r#getHdrCapabilities(_arg_display))
                  }
                  fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
                    self._rt.block_on(self._inner.r#getMaxVirtualDisplayCount())
                  }
                  fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
                    self._rt.block_on(self._inner.r#getPerFrameMetadataKeys(_arg_display))
                  }
                  fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
                    self._rt.block_on(self._inner.r#getReadbackBufferAttributes(_arg_display))
                  }
                  fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
                    self._rt.block_on(self._inner.r#getReadbackBufferFence(_arg_display))
                  }
                  fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
                    self._rt.block_on(self._inner.r#getRenderIntents(_arg_display, _arg_mode))
                  }
                  fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
                    self._rt.block_on(self._inner.r#getSupportedContentTypes(_arg_display))
                  }
                  fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
                    self._rt.block_on(self._inner.r#getDisplayDecorationSupport(_arg_display))
                  }
                  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#registerCallback(_arg_callback))
                  }
                  fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setActiveConfig(_arg_display, _arg_config))
                  }
                  fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
                    self._rt.block_on(self._inner.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints))
                  }
                  fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setBootDisplayConfig(_arg_display, _arg_config))
                  }
                  fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#clearBootDisplayConfig(_arg_display))
                  }
                  fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                    self._rt.block_on(self._inner.r#getPreferredBootDisplayConfig(_arg_display))
                  }
                  fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAutoLowLatencyMode(_arg_display, _arg_on))
                  }
                  fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount))
                  }
                  fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setColorMode(_arg_display, _arg_mode, _arg_intent))
                  }
                  fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setContentType(_arg_display, _arg_type))
                  }
                  fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames))
                  }
                  fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setPowerMode(_arg_display, _arg_mode))
                  }
                  fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence))
                  }
                  fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setVsyncEnabled(_arg_display, _arg_enabled))
                  }
                  fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs))
                  }
                  fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
                    self._rt.block_on(self._inner.r#getOverlaySupport())
                  }
                  fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
                    self._rt.block_on(self._inner.r#getHdrConversionCapabilities())
                  }
                  fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
                    self._rt.block_on(self._inner.r#setHdrConversionStrategy(_arg_conversionStrategy))
                  }
                  fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled))
                  }
                  fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>> {
                    self._rt.block_on(self._inner.r#getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs))
                  }
                  fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs))
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IComposerClientAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IComposerClientAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnComposerClient>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IComposerClientAsync<P> for Wrapper {
                  fn r#createLayer<'a>(&'a self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<i64>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#createLayer(_arg_display, _arg_bufferSlotCount))
                  }
                  fn r#createVirtualDisplay<'a>(&'a self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount))
                  }
                  fn r#destroyLayer<'a>(&'a self, _arg_display: i64, _arg_layer: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#destroyLayer(_arg_display, _arg_layer))
                  }
                  fn r#destroyVirtualDisplay<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#destroyVirtualDisplay(_arg_display))
                  }
                  fn r#executeCommands<'a>(&'a self, _arg_commands: &'a [crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#executeCommands(_arg_commands))
                  }
                  fn r#getActiveConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getActiveConfig(_arg_display))
                  }
                  fn r#getColorModes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getColorModes(_arg_display))
                  }
                  fn r#getDataspaceSaturationMatrix<'a>(&'a self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::BoxFuture<'a, binder::Result<Vec<f32>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDataspaceSaturationMatrix(_arg_dataspace))
                  }
                  fn r#getDisplayAttribute<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::BoxFuture<'a, binder::Result<i32>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute))
                  }
                  fn r#getDisplayCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayCapabilities(_arg_display))
                  }
                  fn r#getDisplayConfigs<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayConfigs(_arg_display))
                  }
                  fn r#getDisplayConnectionType<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayConnectionType(_arg_display))
                  }
                  fn r#getDisplayIdentificationData<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayIdentificationData(_arg_display))
                  }
                  fn r#getDisplayName<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<String>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayName(_arg_display))
                  }
                  fn r#getDisplayVsyncPeriod<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayVsyncPeriod(_arg_display))
                  }
                  fn r#getDisplayedContentSample<'a>(&'a self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp))
                  }
                  fn r#getDisplayedContentSamplingAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayedContentSamplingAttributes(_arg_display))
                  }
                  fn r#getDisplayPhysicalOrientation<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayPhysicalOrientation(_arg_display))
                  }
                  fn r#getHdrCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getHdrCapabilities(_arg_display))
                  }
                  fn r#getMaxVirtualDisplayCount<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getMaxVirtualDisplayCount())
                  }
                  fn r#getPerFrameMetadataKeys<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getPerFrameMetadataKeys(_arg_display))
                  }
                  fn r#getReadbackBufferAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getReadbackBufferAttributes(_arg_display))
                  }
                  fn r#getReadbackBufferFence<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<binder::ParcelFileDescriptor>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getReadbackBufferFence(_arg_display))
                  }
                  fn r#getRenderIntents<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getRenderIntents(_arg_display, _arg_mode))
                  }
                  fn r#getSupportedContentTypes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getSupportedContentTypes(_arg_display))
                  }
                  fn r#getDisplayDecorationSupport<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayDecorationSupport(_arg_display))
                  }
                  fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#registerCallback(_arg_callback))
                  }
                  fn r#setActiveConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setActiveConfig(_arg_display, _arg_config))
                  }
                  fn r#setActiveConfigWithConstraints<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints))
                  }
                  fn r#setBootDisplayConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setBootDisplayConfig(_arg_display, _arg_config))
                  }
                  fn r#clearBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#clearBootDisplayConfig(_arg_display))
                  }
                  fn r#getPreferredBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getPreferredBootDisplayConfig(_arg_display))
                  }
                  fn r#setAutoLowLatencyMode<'a>(&'a self, _arg_display: i64, _arg_on: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setAutoLowLatencyMode(_arg_display, _arg_on))
                  }
                  fn r#setClientTargetSlotCount<'a>(&'a self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount))
                  }
                  fn r#setColorMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setColorMode(_arg_display, _arg_mode, _arg_intent))
                  }
                  fn r#setContentType<'a>(&'a self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setContentType(_arg_display, _arg_type))
                  }
                  fn r#setDisplayedContentSamplingEnabled<'a>(&'a self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames))
                  }
                  fn r#setPowerMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setPowerMode(_arg_display, _arg_mode))
                  }
                  fn r#setReadbackBuffer<'a>(&'a self, _arg_display: i64, _arg_buffer: &'a crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence))
                  }
                  fn r#setVsyncEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setVsyncEnabled(_arg_display, _arg_enabled))
                  }
                  fn r#setIdleTimerEnabled<'a>(&'a self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs))
                  }
                  fn r#getOverlaySupport<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getOverlaySupport())
                  }
                  fn r#getHdrConversionCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getHdrConversionCapabilities())
                  }
                  fn r#setHdrConversionStrategy<'a>(&'a self, _arg_conversionStrategy: &'a crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setHdrConversionStrategy(_arg_conversionStrategy))
                  }
                  fn r#setRefreshRateChangedCallbackDebugEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled))
                  }
                  fn r#getDisplayConfigurations<'a>(&'a self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs))
                  }
                  fn r#notifyExpectedPresent<'a>(&'a self, _arg_display: i64, _arg_expectedPresentTime: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs))
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IComposerClientAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IComposerClientDefault: Send + Sync {
              fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#createLayer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#createVirtualDisplay: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#destroyLayer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#destroyVirtualDisplay: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#executeCommands: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#getActiveConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#getColorModes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#getDataspaceSaturationMatrix: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#getDisplayAttribute: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#getDisplayCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#getDisplayConfigs: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#getDisplayConnectionType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#getDisplayIdentificationData: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getDisplayName: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getDisplayVsyncPeriod: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getDisplayedContentSample: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#getDisplayedContentSamplingAttributes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#getDisplayPhysicalOrientation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#getHdrCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
              pub const r#getMaxVirtualDisplayCount: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
              pub const r#getPerFrameMetadataKeys: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
              pub const r#getReadbackBufferAttributes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
              pub const r#getReadbackBufferFence: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
              pub const r#getRenderIntents: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
              pub const r#getSupportedContentTypes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
              pub const r#getDisplayDecorationSupport: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
              pub const r#registerCallback: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
              pub const r#setActiveConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
              pub const r#setActiveConfigWithConstraints: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
              pub const r#setBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
              pub const r#clearBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
              pub const r#getPreferredBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
              pub const r#setAutoLowLatencyMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
              pub const r#setClientTargetSlotCount: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
              pub const r#setColorMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
              pub const r#setContentType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
              pub const r#setDisplayedContentSamplingEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 36;
              pub const r#setPowerMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 37;
              pub const r#setReadbackBuffer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 38;
              pub const r#setVsyncEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 39;
              pub const r#setIdleTimerEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 40;
              pub const r#getOverlaySupport: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 41;
              pub const r#getHdrConversionCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 42;
              pub const r#setHdrConversionStrategy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 43;
              pub const r#setRefreshRateChangedCallbackDebugEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 44;
              pub const r#getDisplayConfigurations: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 45;
              pub const r#notifyExpectedPresent: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 46;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IComposerClientDefaultRef = Option<std::sync::Arc<dyn IComposerClientDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IComposerClientDefaultRef> = std::sync::Mutex::new(None);
            pub const r#EX_BAD_CONFIG: i32 = 1;
            pub const r#EX_BAD_DISPLAY: i32 = 2;
            pub const r#EX_BAD_LAYER: i32 = 3;
            pub const r#EX_BAD_PARAMETER: i32 = 4;
            pub const r#EX_RESERVED: i32 = 5;
            pub const r#EX_NO_RESOURCES: i32 = 6;
            pub const r#EX_NOT_VALIDATED: i32 = 7;
            pub const r#EX_UNSUPPORTED: i32 = 8;
            pub const r#EX_SEAMLESS_NOT_ALLOWED: i32 = 9;
            pub const r#EX_SEAMLESS_NOT_POSSIBLE: i32 = 10;
            pub const r#INVALID_CONFIGURATION: i32 = 2147483647;
            pub const VERSION: i32 = 3;
            pub const HASH: &str = "d24fcd9648b8b2e7287f9238eee9180244612c10";
            impl BpComposerClient {
              fn build_parcel_createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_bufferSlotCount)?;
                Ok(aidl_data)
              }
              fn read_response_createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i64> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#createLayer(_arg_display, _arg_bufferSlotCount);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i64 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_width)?;
                aidl_data.write(&_arg_height)?;
                aidl_data.write(&_arg_formatHint)?;
                aidl_data.write(&_arg_outputBufferSlotCount)?;
                Ok(aidl_data)
              }
              fn read_response_createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_layer)?;
                Ok(aidl_data)
              }
              fn read_response_destroyLayer(&self, _arg_display: i64, _arg_layer: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#destroyLayer(_arg_display, _arg_layer);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_destroyVirtualDisplay(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#destroyVirtualDisplay(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_commands)?;
                Ok(aidl_data)
              }
              fn read_response_executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#executeCommands(_arg_commands);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getActiveConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getActiveConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getActiveConfig(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getColorModes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getColorModes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getColorModes(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_dataspace)?;
                Ok(aidl_data)
              }
              fn read_response_getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<f32>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDataspaceSaturationMatrix(_arg_dataspace);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<f32> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_config)?;
                aidl_data.write(&_arg_attribute)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayCapabilities(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayCapabilities(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayConfigs(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<i32>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayConfigs(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<i32> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayConnectionType(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayConnectionType(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayIdentificationData(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayIdentificationData(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayName(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayName(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayName(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayVsyncPeriod(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayVsyncPeriod(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_maxFrames)?;
                aidl_data.write(&_arg_timestamp)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayedContentSamplingAttributes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayedContentSamplingAttributes(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayPhysicalOrientation(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayPhysicalOrientation(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getHdrCapabilities(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHdrCapabilities(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getMaxVirtualDisplayCount(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getMaxVirtualDisplayCount(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getMaxVirtualDisplayCount();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getPerFrameMetadataKeys(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPerFrameMetadataKeys(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getReadbackBufferAttributes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getReadbackBufferAttributes(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getReadbackBufferFence(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getReadbackBufferFence(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Option<binder::ParcelFileDescriptor> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_mode)?;
                Ok(aidl_data)
              }
              fn read_response_getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getRenderIntents(_arg_display, _arg_mode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getSupportedContentTypes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getSupportedContentTypes(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayDecorationSupport(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayDecorationSupport(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_callback)?;
                Ok(aidl_data)
              }
              fn read_response_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#registerCallback(_arg_callback);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_config)?;
                Ok(aidl_data)
              }
              fn read_response_setActiveConfig(&self, _arg_display: i64, _arg_config: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setActiveConfig(_arg_display, _arg_config);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_config)?;
                aidl_data.write(_arg_vsyncPeriodChangeConstraints)?;
                Ok(aidl_data)
              }
              fn read_response_setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_config)?;
                Ok(aidl_data)
              }
              fn read_response_setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setBootDisplayConfig(_arg_display, _arg_config);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_clearBootDisplayConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#clearBootDisplayConfig(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                Ok(aidl_data)
              }
              fn read_response_getPreferredBootDisplayConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getPreferredBootDisplayConfig(_arg_display);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_on)?;
                Ok(aidl_data)
              }
              fn read_response_setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAutoLowLatencyMode(_arg_display, _arg_on);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_clientTargetSlotCount)?;
                Ok(aidl_data)
              }
              fn read_response_setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_mode)?;
                aidl_data.write(&_arg_intent)?;
                Ok(aidl_data)
              }
              fn read_response_setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setColorMode(_arg_display, _arg_mode, _arg_intent);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_type)?;
                Ok(aidl_data)
              }
              fn read_response_setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setContentType(_arg_display, _arg_type);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_enable)?;
                aidl_data.write(&_arg_componentMask)?;
                aidl_data.write(&_arg_maxFrames)?;
                Ok(aidl_data)
              }
              fn read_response_setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_mode)?;
                Ok(aidl_data)
              }
              fn read_response_setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setPowerMode(_arg_display, _arg_mode);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(_arg_buffer)?;
                aidl_data.write(&_arg_releaseFence)?;
                Ok(aidl_data)
              }
              fn read_response_setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_enabled)?;
                Ok(aidl_data)
              }
              fn read_response_setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setVsyncEnabled(_arg_display, _arg_enabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_timeoutMs)?;
                Ok(aidl_data)
              }
              fn read_response_setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_getOverlaySupport(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getOverlaySupport(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getOverlaySupport();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getHdrConversionCapabilities(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getHdrConversionCapabilities(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHdrConversionCapabilities();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_conversionStrategy)?;
                Ok(aidl_data)
              }
              fn read_response_setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setHdrConversionStrategy(_arg_conversionStrategy);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_enabled)?;
                Ok(aidl_data)
              }
              fn read_response_setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(&_arg_maxFrameIntervalNs)?;
                Ok(aidl_data)
              }
              fn read_response_getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_display)?;
                aidl_data.write(_arg_expectedPresentTime)?;
                aidl_data.write(&_arg_frameIntervalNs)?;
                Ok(aidl_data)
              }
              fn read_response_notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
                    return _aidl_default_impl.r#notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                Ok(())
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IComposerClient for BpComposerClient {
              fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
                let _aidl_data = self.build_parcel_createLayer(_arg_display, _arg_bufferSlotCount)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#createLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_createLayer(_arg_display, _arg_bufferSlotCount, _aidl_reply)
              }
              fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
                let _aidl_data = self.build_parcel_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#createVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount, _aidl_reply)
              }
              fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_destroyLayer(_arg_display, _arg_layer)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#destroyLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_destroyLayer(_arg_display, _arg_layer, _aidl_reply)
              }
              fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_destroyVirtualDisplay(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#destroyVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_destroyVirtualDisplay(_arg_display, _aidl_reply)
              }
              fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
                let _aidl_data = self.build_parcel_executeCommands(_arg_commands)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#executeCommands, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_executeCommands(_arg_commands, _aidl_reply)
              }
              fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                let _aidl_data = self.build_parcel_getActiveConfig(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getActiveConfig(_arg_display, _aidl_reply)
              }
              fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
                let _aidl_data = self.build_parcel_getColorModes(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getColorModes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getColorModes(_arg_display, _aidl_reply)
              }
              fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
                let _aidl_data = self.build_parcel_getDataspaceSaturationMatrix(_arg_dataspace)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDataspaceSaturationMatrix, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDataspaceSaturationMatrix(_arg_dataspace, _aidl_reply)
              }
              fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
                let _aidl_data = self.build_parcel_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayAttribute, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute, _aidl_reply)
              }
              fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
                let _aidl_data = self.build_parcel_getDisplayCapabilities(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayCapabilities(_arg_display, _aidl_reply)
              }
              fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
                let _aidl_data = self.build_parcel_getDisplayConfigs(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayConfigs, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayConfigs(_arg_display, _aidl_reply)
              }
              fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
                let _aidl_data = self.build_parcel_getDisplayConnectionType(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayConnectionType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayConnectionType(_arg_display, _aidl_reply)
              }
              fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
                let _aidl_data = self.build_parcel_getDisplayIdentificationData(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayIdentificationData, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayIdentificationData(_arg_display, _aidl_reply)
              }
              fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
                let _aidl_data = self.build_parcel_getDisplayName(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayName(_arg_display, _aidl_reply)
              }
              fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
                let _aidl_data = self.build_parcel_getDisplayVsyncPeriod(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayVsyncPeriod, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayVsyncPeriod(_arg_display, _aidl_reply)
              }
              fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
                let _aidl_data = self.build_parcel_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayedContentSample, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp, _aidl_reply)
              }
              fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
                let _aidl_data = self.build_parcel_getDisplayedContentSamplingAttributes(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayedContentSamplingAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayedContentSamplingAttributes(_arg_display, _aidl_reply)
              }
              fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
                let _aidl_data = self.build_parcel_getDisplayPhysicalOrientation(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayPhysicalOrientation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayPhysicalOrientation(_arg_display, _aidl_reply)
              }
              fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
                let _aidl_data = self.build_parcel_getHdrCapabilities(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHdrCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHdrCapabilities(_arg_display, _aidl_reply)
              }
              fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
                let _aidl_data = self.build_parcel_getMaxVirtualDisplayCount()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getMaxVirtualDisplayCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getMaxVirtualDisplayCount(_aidl_reply)
              }
              fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
                let _aidl_data = self.build_parcel_getPerFrameMetadataKeys(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPerFrameMetadataKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPerFrameMetadataKeys(_arg_display, _aidl_reply)
              }
              fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
                let _aidl_data = self.build_parcel_getReadbackBufferAttributes(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getReadbackBufferAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getReadbackBufferAttributes(_arg_display, _aidl_reply)
              }
              fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
                let _aidl_data = self.build_parcel_getReadbackBufferFence(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getReadbackBufferFence, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getReadbackBufferFence(_arg_display, _aidl_reply)
              }
              fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
                let _aidl_data = self.build_parcel_getRenderIntents(_arg_display, _arg_mode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRenderIntents, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getRenderIntents(_arg_display, _arg_mode, _aidl_reply)
              }
              fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
                let _aidl_data = self.build_parcel_getSupportedContentTypes(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getSupportedContentTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getSupportedContentTypes(_arg_display, _aidl_reply)
              }
              fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
                let _aidl_data = self.build_parcel_getDisplayDecorationSupport(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayDecorationSupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayDecorationSupport(_arg_display, _aidl_reply)
              }
              fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_registerCallback(_arg_callback)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_registerCallback(_arg_callback, _aidl_reply)
              }
              fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setActiveConfig(_arg_display, _arg_config)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setActiveConfig(_arg_display, _arg_config, _aidl_reply)
              }
              fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
                let _aidl_data = self.build_parcel_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setActiveConfigWithConstraints, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints, _aidl_reply)
              }
              fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setBootDisplayConfig(_arg_display, _arg_config)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setBootDisplayConfig(_arg_display, _arg_config, _aidl_reply)
              }
              fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_clearBootDisplayConfig(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#clearBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_clearBootDisplayConfig(_arg_display, _aidl_reply)
              }
              fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
                let _aidl_data = self.build_parcel_getPreferredBootDisplayConfig(_arg_display)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getPreferredBootDisplayConfig(_arg_display, _aidl_reply)
              }
              fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAutoLowLatencyMode(_arg_display, _arg_on)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAutoLowLatencyMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setAutoLowLatencyMode(_arg_display, _arg_on, _aidl_reply)
              }
              fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setClientTargetSlotCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount, _aidl_reply)
              }
              fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setColorMode(_arg_display, _arg_mode, _arg_intent)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setColorMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setColorMode(_arg_display, _arg_mode, _arg_intent, _aidl_reply)
              }
              fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setContentType(_arg_display, _arg_type)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setContentType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setContentType(_arg_display, _arg_type, _aidl_reply)
              }
              fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setDisplayedContentSamplingEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames, _aidl_reply)
              }
              fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setPowerMode(_arg_display, _arg_mode)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setPowerMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setPowerMode(_arg_display, _arg_mode, _aidl_reply)
              }
              fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setReadbackBuffer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence, _aidl_reply)
              }
              fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setVsyncEnabled(_arg_display, _arg_enabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setVsyncEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setVsyncEnabled(_arg_display, _arg_enabled, _aidl_reply)
              }
              fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setIdleTimerEnabled(_arg_display, _arg_timeoutMs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setIdleTimerEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setIdleTimerEnabled(_arg_display, _arg_timeoutMs, _aidl_reply)
              }
              fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
                let _aidl_data = self.build_parcel_getOverlaySupport()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getOverlaySupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getOverlaySupport(_aidl_reply)
              }
              fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
                let _aidl_data = self.build_parcel_getHdrConversionCapabilities()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHdrConversionCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getHdrConversionCapabilities(_aidl_reply)
              }
              fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
                let _aidl_data = self.build_parcel_setHdrConversionStrategy(_arg_conversionStrategy)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setHdrConversionStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setHdrConversionStrategy(_arg_conversionStrategy, _aidl_reply)
              }
              fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setRefreshRateChangedCallbackDebugEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled, _aidl_reply)
              }
              fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>> {
                let _aidl_data = self.build_parcel_getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayConfigurations, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs, _aidl_reply)
              }
              fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#notifyExpectedPresent, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IComposerClientAsync<P> for BpComposerClient {
              fn r#createLayer<'a>(&'a self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<i64>> {
                let _aidl_data = match self.build_parcel_createLayer(_arg_display, _arg_bufferSlotCount) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#createLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_createLayer(_arg_display, _arg_bufferSlotCount, _aidl_reply)
                  }
                )
              }
              fn r#createVirtualDisplay<'a>(&'a self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>> {
                let _aidl_data = match self.build_parcel_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#createVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount, _aidl_reply)
                  }
                )
              }
              fn r#destroyLayer<'a>(&'a self, _arg_display: i64, _arg_layer: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_destroyLayer(_arg_display, _arg_layer) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#destroyLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_destroyLayer(_arg_display, _arg_layer, _aidl_reply)
                  }
                )
              }
              fn r#destroyVirtualDisplay<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_destroyVirtualDisplay(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#destroyVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_destroyVirtualDisplay(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#executeCommands<'a>(&'a self, _arg_commands: &'a [crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>> {
                let _aidl_data = match self.build_parcel_executeCommands(_arg_commands) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#executeCommands, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_executeCommands(_arg_commands, _aidl_reply)
                  }
                )
              }
              fn r#getActiveConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_data = match self.build_parcel_getActiveConfig(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getActiveConfig(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getColorModes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>> {
                let _aidl_data = match self.build_parcel_getColorModes(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getColorModes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getColorModes(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDataspaceSaturationMatrix<'a>(&'a self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::BoxFuture<'a, binder::Result<Vec<f32>>> {
                let _aidl_data = match self.build_parcel_getDataspaceSaturationMatrix(_arg_dataspace) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDataspaceSaturationMatrix, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDataspaceSaturationMatrix(_arg_dataspace, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayAttribute<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_data = match self.build_parcel_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayAttribute, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>> {
                let _aidl_data = match self.build_parcel_getDisplayCapabilities(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayCapabilities(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayConfigs<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>> {
                let _aidl_data = match self.build_parcel_getDisplayConfigs(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayConfigs, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayConfigs(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayConnectionType<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>> {
                let _aidl_data = match self.build_parcel_getDisplayConnectionType(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayConnectionType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayConnectionType(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayIdentificationData<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>> {
                let _aidl_data = match self.build_parcel_getDisplayIdentificationData(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayIdentificationData, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayIdentificationData(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayName<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<String>> {
                let _aidl_data = match self.build_parcel_getDisplayName(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayName(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayVsyncPeriod<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_data = match self.build_parcel_getDisplayVsyncPeriod(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayVsyncPeriod, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayVsyncPeriod(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayedContentSample<'a>(&'a self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>> {
                let _aidl_data = match self.build_parcel_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayedContentSample, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayedContentSamplingAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>> {
                let _aidl_data = match self.build_parcel_getDisplayedContentSamplingAttributes(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayedContentSamplingAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayedContentSamplingAttributes(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayPhysicalOrientation<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>> {
                let _aidl_data = match self.build_parcel_getDisplayPhysicalOrientation(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayPhysicalOrientation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayPhysicalOrientation(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getHdrCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>> {
                let _aidl_data = match self.build_parcel_getHdrCapabilities(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getHdrCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getHdrCapabilities(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getMaxVirtualDisplayCount<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_data = match self.build_parcel_getMaxVirtualDisplayCount() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getMaxVirtualDisplayCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getMaxVirtualDisplayCount(_aidl_reply)
                  }
                )
              }
              fn r#getPerFrameMetadataKeys<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>> {
                let _aidl_data = match self.build_parcel_getPerFrameMetadataKeys(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getPerFrameMetadataKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getPerFrameMetadataKeys(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getReadbackBufferAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>> {
                let _aidl_data = match self.build_parcel_getReadbackBufferAttributes(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getReadbackBufferAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getReadbackBufferAttributes(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getReadbackBufferFence<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<binder::ParcelFileDescriptor>>> {
                let _aidl_data = match self.build_parcel_getReadbackBufferFence(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getReadbackBufferFence, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getReadbackBufferFence(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getRenderIntents<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>> {
                let _aidl_data = match self.build_parcel_getRenderIntents(_arg_display, _arg_mode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getRenderIntents, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getRenderIntents(_arg_display, _arg_mode, _aidl_reply)
                  }
                )
              }
              fn r#getSupportedContentTypes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>> {
                let _aidl_data = match self.build_parcel_getSupportedContentTypes(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getSupportedContentTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getSupportedContentTypes(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayDecorationSupport<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>> {
                let _aidl_data = match self.build_parcel_getDisplayDecorationSupport(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayDecorationSupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayDecorationSupport(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_registerCallback(_arg_callback) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_registerCallback(_arg_callback, _aidl_reply)
                  }
                )
              }
              fn r#setActiveConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setActiveConfig(_arg_display, _arg_config) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setActiveConfig(_arg_display, _arg_config, _aidl_reply)
                  }
                )
              }
              fn r#setActiveConfigWithConstraints<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>> {
                let _aidl_data = match self.build_parcel_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setActiveConfigWithConstraints, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints, _aidl_reply)
                  }
                )
              }
              fn r#setBootDisplayConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setBootDisplayConfig(_arg_display, _arg_config) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setBootDisplayConfig(_arg_display, _arg_config, _aidl_reply)
                  }
                )
              }
              fn r#clearBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_clearBootDisplayConfig(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#clearBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_clearBootDisplayConfig(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#getPreferredBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_data = match self.build_parcel_getPreferredBootDisplayConfig(_arg_display) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getPreferredBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getPreferredBootDisplayConfig(_arg_display, _aidl_reply)
                  }
                )
              }
              fn r#setAutoLowLatencyMode<'a>(&'a self, _arg_display: i64, _arg_on: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAutoLowLatencyMode(_arg_display, _arg_on) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setAutoLowLatencyMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setAutoLowLatencyMode(_arg_display, _arg_on, _aidl_reply)
                  }
                )
              }
              fn r#setClientTargetSlotCount<'a>(&'a self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setClientTargetSlotCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount, _aidl_reply)
                  }
                )
              }
              fn r#setColorMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setColorMode(_arg_display, _arg_mode, _arg_intent) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setColorMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setColorMode(_arg_display, _arg_mode, _arg_intent, _aidl_reply)
                  }
                )
              }
              fn r#setContentType<'a>(&'a self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setContentType(_arg_display, _arg_type) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setContentType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setContentType(_arg_display, _arg_type, _aidl_reply)
                  }
                )
              }
              fn r#setDisplayedContentSamplingEnabled<'a>(&'a self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setDisplayedContentSamplingEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames, _aidl_reply)
                  }
                )
              }
              fn r#setPowerMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setPowerMode(_arg_display, _arg_mode) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setPowerMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setPowerMode(_arg_display, _arg_mode, _aidl_reply)
                  }
                )
              }
              fn r#setReadbackBuffer<'a>(&'a self, _arg_display: i64, _arg_buffer: &'a crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setReadbackBuffer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence, _aidl_reply)
                  }
                )
              }
              fn r#setVsyncEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setVsyncEnabled(_arg_display, _arg_enabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setVsyncEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setVsyncEnabled(_arg_display, _arg_enabled, _aidl_reply)
                  }
                )
              }
              fn r#setIdleTimerEnabled<'a>(&'a self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setIdleTimerEnabled(_arg_display, _arg_timeoutMs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setIdleTimerEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setIdleTimerEnabled(_arg_display, _arg_timeoutMs, _aidl_reply)
                  }
                )
              }
              fn r#getOverlaySupport<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>> {
                let _aidl_data = match self.build_parcel_getOverlaySupport() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getOverlaySupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getOverlaySupport(_aidl_reply)
                  }
                )
              }
              fn r#getHdrConversionCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>> {
                let _aidl_data = match self.build_parcel_getHdrConversionCapabilities() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getHdrConversionCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getHdrConversionCapabilities(_aidl_reply)
                  }
                )
              }
              fn r#setHdrConversionStrategy<'a>(&'a self, _arg_conversionStrategy: &'a crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>> {
                let _aidl_data = match self.build_parcel_setHdrConversionStrategy(_arg_conversionStrategy) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setHdrConversionStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setHdrConversionStrategy(_arg_conversionStrategy, _aidl_reply)
                  }
                )
              }
              fn r#setRefreshRateChangedCallbackDebugEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setRefreshRateChangedCallbackDebugEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled, _aidl_reply)
                  }
                )
              }
              fn r#getDisplayConfigurations<'a>(&'a self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>>> {
                let _aidl_data = match self.build_parcel_getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getDisplayConfigurations, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs, _aidl_reply)
                  }
                )
              }
              fn r#notifyExpectedPresent<'a>(&'a self, _arg_display: i64, _arg_expectedPresentTime: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#notifyExpectedPresent, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs, _aidl_reply)
                  }
                )
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IComposerClient for binder::binder_impl::Binder<BnComposerClient> {
              fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> { self.0.r#createLayer(_arg_display, _arg_bufferSlotCount) }
              fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> { self.0.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount) }
              fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> { self.0.r#destroyLayer(_arg_display, _arg_layer) }
              fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#destroyVirtualDisplay(_arg_display) }
              fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> { self.0.r#executeCommands(_arg_commands) }
              fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getActiveConfig(_arg_display) }
              fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> { self.0.r#getColorModes(_arg_display) }
              fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> { self.0.r#getDataspaceSaturationMatrix(_arg_dataspace) }
              fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> { self.0.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute) }
              fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> { self.0.r#getDisplayCapabilities(_arg_display) }
              fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> { self.0.r#getDisplayConfigs(_arg_display) }
              fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> { self.0.r#getDisplayConnectionType(_arg_display) }
              fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> { self.0.r#getDisplayIdentificationData(_arg_display) }
              fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> { self.0.r#getDisplayName(_arg_display) }
              fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getDisplayVsyncPeriod(_arg_display) }
              fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> { self.0.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp) }
              fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> { self.0.r#getDisplayedContentSamplingAttributes(_arg_display) }
              fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> { self.0.r#getDisplayPhysicalOrientation(_arg_display) }
              fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> { self.0.r#getHdrCapabilities(_arg_display) }
              fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> { self.0.r#getMaxVirtualDisplayCount() }
              fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> { self.0.r#getPerFrameMetadataKeys(_arg_display) }
              fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> { self.0.r#getReadbackBufferAttributes(_arg_display) }
              fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> { self.0.r#getReadbackBufferFence(_arg_display) }
              fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> { self.0.r#getRenderIntents(_arg_display, _arg_mode) }
              fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> { self.0.r#getSupportedContentTypes(_arg_display) }
              fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> { self.0.r#getDisplayDecorationSupport(_arg_display) }
              fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> { self.0.r#registerCallback(_arg_callback) }
              fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> { self.0.r#setActiveConfig(_arg_display, _arg_config) }
              fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> { self.0.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints) }
              fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> { self.0.r#setBootDisplayConfig(_arg_display, _arg_config) }
              fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#clearBootDisplayConfig(_arg_display) }
              fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getPreferredBootDisplayConfig(_arg_display) }
              fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> { self.0.r#setAutoLowLatencyMode(_arg_display, _arg_on) }
              fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> { self.0.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount) }
              fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> { self.0.r#setColorMode(_arg_display, _arg_mode, _arg_intent) }
              fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> { self.0.r#setContentType(_arg_display, _arg_type) }
              fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> { self.0.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames) }
              fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> { self.0.r#setPowerMode(_arg_display, _arg_mode) }
              fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> { self.0.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence) }
              fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> { self.0.r#setVsyncEnabled(_arg_display, _arg_enabled) }
              fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> { self.0.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs) }
              fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> { self.0.r#getOverlaySupport() }
              fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> { self.0.r#getHdrConversionCapabilities() }
              fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> { self.0.r#setHdrConversionStrategy(_arg_conversionStrategy) }
              fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> { self.0.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled) }
              fn r#getDisplayConfigurations(&self, _arg_display: i64, _arg_maxFrameIntervalNs: i32) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayConfiguration>> { self.0.r#getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs) }
              fn r#notifyExpectedPresent(&self, _arg_display: i64, _arg_expectedPresentTime: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp, _arg_frameIntervalNs: i32) -> binder::Result<()> { self.0.r#notifyExpectedPresent(_arg_display, _arg_expectedPresentTime, _arg_frameIntervalNs) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IComposerClient, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#createLayer => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_bufferSlotCount: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#createLayer(_arg_display, _arg_bufferSlotCount);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#createVirtualDisplay => {
                  let _arg_width: i32 = _aidl_data.read()?;
                  let _arg_height: i32 = _aidl_data.read()?;
                  let _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat = _aidl_data.read()?;
                  let _arg_outputBufferSlotCount: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#destroyLayer => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_layer: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#destroyLayer(_arg_display, _arg_layer);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#destroyVirtualDisplay => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#destroyVirtualDisplay(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#executeCommands => {
                  let _arg_commands: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#executeCommands(&_arg_commands);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getActiveConfig => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getActiveConfig(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getColorModes => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getColorModes(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDataspaceSaturationMatrix => {
                  let _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDataspaceSaturationMatrix(_arg_dataspace);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayAttribute => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_config: i32 = _aidl_data.read()?;
                  let _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayCapabilities => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayCapabilities(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayConfigs => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayConfigs(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayConnectionType => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayConnectionType(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayIdentificationData => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayIdentificationData(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayName => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayName(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayVsyncPeriod => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayVsyncPeriod(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayedContentSample => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_maxFrames: i64 = _aidl_data.read()?;
                  let _arg_timestamp: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayedContentSamplingAttributes => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayedContentSamplingAttributes(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayPhysicalOrientation => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayPhysicalOrientation(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getHdrCapabilities => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getHdrCapabilities(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getMaxVirtualDisplayCount => {
                  let _aidl_return = _aidl_service.r#getMaxVirtualDisplayCount();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getPerFrameMetadataKeys => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPerFrameMetadataKeys(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getReadbackBufferAttributes => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getReadbackBufferAttributes(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getReadbackBufferFence => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getReadbackBufferFence(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getRenderIntents => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getRenderIntents(_arg_display, _arg_mode);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getSupportedContentTypes => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getSupportedContentTypes(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayDecorationSupport => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayDecorationSupport(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#registerCallback => {
                  let _arg_callback: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#registerCallback(&_arg_callback);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setActiveConfig => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_config: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setActiveConfig(_arg_display, _arg_config);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setActiveConfigWithConstraints => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_config: i32 = _aidl_data.read()?;
                  let _arg_vsyncPeriodChangeConstraints: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setActiveConfigWithConstraints(_arg_display, _arg_config, &_arg_vsyncPeriodChangeConstraints);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setBootDisplayConfig => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_config: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setBootDisplayConfig(_arg_display, _arg_config);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#clearBootDisplayConfig => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#clearBootDisplayConfig(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getPreferredBootDisplayConfig => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getPreferredBootDisplayConfig(_arg_display);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setAutoLowLatencyMode => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_on: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAutoLowLatencyMode(_arg_display, _arg_on);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setClientTargetSlotCount => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_clientTargetSlotCount: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setColorMode => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode = _aidl_data.read()?;
                  let _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setColorMode(_arg_display, _arg_mode, _arg_intent);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setContentType => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setContentType(_arg_display, _arg_type);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setDisplayedContentSamplingEnabled => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_enable: bool = _aidl_data.read()?;
                  let _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent = _aidl_data.read()?;
                  let _arg_maxFrames: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setPowerMode => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setPowerMode(_arg_display, _arg_mode);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setReadbackBuffer => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_buffer: crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle = _aidl_data.read()?;
                  let _arg_releaseFence: Option<binder::ParcelFileDescriptor> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setReadbackBuffer(_arg_display, &_arg_buffer, _arg_releaseFence.as_ref());
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setVsyncEnabled => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_enabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setVsyncEnabled(_arg_display, _arg_enabled);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setIdleTimerEnabled => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_timeoutMs: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getOverlaySupport => {
                  let _aidl_return = _aidl_service.r#getOverlaySupport();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getHdrConversionCapabilities => {
                  let _aidl_return = _aidl_service.r#getHdrConversionCapabilities();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setHdrConversionStrategy => {
                  let _arg_conversionStrategy: crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setHdrConversionStrategy(&_arg_conversionStrategy);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setRefreshRateChangedCallbackDebugEnabled => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_enabled: bool = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getDisplayConfigurations => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_maxFrameIntervalNs: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getDisplayConfigurations(_arg_display, _arg_maxFrameIntervalNs);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#notifyExpectedPresent => {
                  let _arg_display: i64 = _aidl_data.read()?;
                  let _arg_expectedPresentTime: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp = _aidl_data.read()?;
                  let _arg_frameIntervalNs: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#notifyExpectedPresent(_arg_display, &_arg_expectedPresentTime, _arg_frameIntervalNs);
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IComposerClient as _7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient;
            }
          }
                    pub mod LayerBrightness {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/LayerBrightness.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/LayerBrightness.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#LayerBrightness {
              pub r#brightness: f32,
            }
            impl Default for r#LayerBrightness {
              fn default() -> Self {
                Self {
                  r#brightness: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#LayerBrightness {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#brightness)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#brightness = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LayerBrightness);
            binder::impl_deserialize_for_parcelable!(r#LayerBrightness);
            impl binder::binder_impl::ParcelableMetadata for r#LayerBrightness {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.LayerBrightness" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LayerBrightness as _7_android_8_hardware_8_graphics_9_composer3_15_LayerBrightness;
            }
          }
                    pub mod LayerCommand {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/LayerCommand.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/LayerCommand.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#LayerCommand {
              pub r#layer: i64,
              pub r#cursorPosition: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_5_Point>,
              pub r#buffer: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer>,
              pub r#damage: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
              pub r#blendMode: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableBlendMode>,
              pub r#color: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_5_Color>,
              pub r#composition: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_ParcelableComposition>,
              pub r#dataspace: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableDataspace>,
              pub r#displayFrame: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>,
              pub r#planeAlpha: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_PlaneAlpha>,
              pub r#sidebandStream: Option<crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle>,
              pub r#sourceCrop: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_5_FRect>,
              pub r#transform: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableTransform>,
              pub r#visibleRegion: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
              pub r#z: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_ZOrder>,
              pub r#colorTransform: Option<Vec<f32>>,
              pub r#brightness: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_LayerBrightness>,
              pub r#perFrameMetadata: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_PerFrameMetadata>>>,
              pub r#perFrameMetadataBlob: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_PerFrameMetadataBlob>>>,
              pub r#blockingRegion: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
              pub r#bufferSlotsToClear: Option<Vec<i32>>,
              pub r#layerLifecycleBatchCommandType: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_30_LayerLifecycleBatchCommandType,
              pub r#newBufferSlotCount: i32,
            }
            impl Default for r#LayerCommand {
              fn default() -> Self {
                Self {
                  r#layer: 0,
                  r#cursorPosition: Default::default(),
                  r#buffer: Default::default(),
                  r#damage: Default::default(),
                  r#blendMode: Default::default(),
                  r#color: Default::default(),
                  r#composition: Default::default(),
                  r#dataspace: Default::default(),
                  r#displayFrame: Default::default(),
                  r#planeAlpha: Default::default(),
                  r#sidebandStream: Default::default(),
                  r#sourceCrop: Default::default(),
                  r#transform: Default::default(),
                  r#visibleRegion: Default::default(),
                  r#z: Default::default(),
                  r#colorTransform: Default::default(),
                  r#brightness: Default::default(),
                  r#perFrameMetadata: Default::default(),
                  r#perFrameMetadataBlob: Default::default(),
                  r#blockingRegion: Default::default(),
                  r#bufferSlotsToClear: Default::default(),
                  r#layerLifecycleBatchCommandType: Default::default(),
                  r#newBufferSlotCount: 0,
                }
              }
            }
            impl binder::Parcelable for r#LayerCommand {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#layer)?;
                  subparcel.write(&self.r#cursorPosition)?;
                  subparcel.write(&self.r#buffer)?;
                  subparcel.write(&self.r#damage)?;
                  subparcel.write(&self.r#blendMode)?;
                  subparcel.write(&self.r#color)?;
                  subparcel.write(&self.r#composition)?;
                  subparcel.write(&self.r#dataspace)?;
                  subparcel.write(&self.r#displayFrame)?;
                  subparcel.write(&self.r#planeAlpha)?;
                  subparcel.write(&self.r#sidebandStream)?;
                  subparcel.write(&self.r#sourceCrop)?;
                  subparcel.write(&self.r#transform)?;
                  subparcel.write(&self.r#visibleRegion)?;
                  subparcel.write(&self.r#z)?;
                  subparcel.write(&self.r#colorTransform)?;
                  subparcel.write(&self.r#brightness)?;
                  subparcel.write(&self.r#perFrameMetadata)?;
                  subparcel.write(&self.r#perFrameMetadataBlob)?;
                  subparcel.write(&self.r#blockingRegion)?;
                  subparcel.write(&self.r#bufferSlotsToClear)?;
                  subparcel.write(&self.r#layerLifecycleBatchCommandType)?;
                  subparcel.write(&self.r#newBufferSlotCount)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#layer = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#cursorPosition = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#buffer = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#damage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#blendMode = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#color = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#composition = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#displayFrame = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#planeAlpha = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sidebandStream = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sourceCrop = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#transform = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#visibleRegion = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#z = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#colorTransform = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#brightness = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#perFrameMetadata = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#perFrameMetadataBlob = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#blockingRegion = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bufferSlotsToClear = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layerLifecycleBatchCommandType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#newBufferSlotCount = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#LayerCommand);
            binder::impl_deserialize_for_parcelable!(r#LayerCommand);
            impl binder::binder_impl::ParcelableMetadata for r#LayerCommand {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.LayerCommand" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#LayerCommand as _7_android_8_hardware_8_graphics_9_composer3_12_LayerCommand;
            }
          }
                    pub mod LayerLifecycleBatchCommandType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/LayerLifecycleBatchCommandType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/LayerLifecycleBatchCommandType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#LayerLifecycleBatchCommandType : [i32; 3] {
                r#MODIFY = 0,
                r#CREATE = 1,
                r#DESTROY = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#LayerLifecycleBatchCommandType as _7_android_8_hardware_8_graphics_9_composer3_30_LayerLifecycleBatchCommandType;
            }
          }
                    pub mod OverlayProperties {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/OverlayProperties.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/OverlayProperties.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#OverlayProperties {
              pub r#combinations: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations>,
              pub r#supportMixedColorSpaces: bool,
            }
            impl Default for r#OverlayProperties {
              fn default() -> Self {
                Self {
                  r#combinations: Default::default(),
                  r#supportMixedColorSpaces: false,
                }
              }
            }
            impl binder::Parcelable for r#OverlayProperties {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#combinations)?;
                  subparcel.write(&self.r#supportMixedColorSpaces)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#combinations = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#supportMixedColorSpaces = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#OverlayProperties);
            binder::impl_deserialize_for_parcelable!(r#OverlayProperties);
            impl binder::binder_impl::ParcelableMetadata for r#OverlayProperties {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#SupportedBufferCombinations {
              #[derive(Debug)]
              pub struct r#SupportedBufferCombinations {
                pub r#pixelFormats: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat>,
                pub r#standards: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
                pub r#transfers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
                pub r#ranges: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
              }
              impl Default for r#SupportedBufferCombinations {
                fn default() -> Self {
                  Self {
                    r#pixelFormats: Default::default(),
                    r#standards: Default::default(),
                    r#transfers: Default::default(),
                    r#ranges: Default::default(),
                  }
                }
              }
              impl binder::Parcelable for r#SupportedBufferCombinations {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#pixelFormats)?;
                    subparcel.write(&self.r#standards)?;
                    subparcel.write(&self.r#transfers)?;
                    subparcel.write(&self.r#ranges)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#pixelFormats = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#standards = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#transfers = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#ranges = subparcel.read()?;
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#SupportedBufferCombinations);
              binder::impl_deserialize_for_parcelable!(r#SupportedBufferCombinations);
              impl binder::binder_impl::ParcelableMetadata for r#SupportedBufferCombinations {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties.SupportedBufferCombinations" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#OverlayProperties as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties;
             pub use super::r#SupportedBufferCombinations::r#SupportedBufferCombinations as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations;
            }
          }
                    pub mod ParcelableBlendMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ParcelableBlendMode.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ParcelableBlendMode.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ParcelableBlendMode {
              pub r#blendMode: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_BlendMode,
            }
            impl Default for r#ParcelableBlendMode {
              fn default() -> Self {
                Self {
                  r#blendMode: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ParcelableBlendMode {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#blendMode)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#blendMode = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ParcelableBlendMode);
            binder::impl_deserialize_for_parcelable!(r#ParcelableBlendMode);
            impl binder::binder_impl::ParcelableMetadata for r#ParcelableBlendMode {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableBlendMode" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ParcelableBlendMode as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableBlendMode;
            }
          }
                    pub mod ParcelableComposition {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ParcelableComposition.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ParcelableComposition.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ParcelableComposition {
              pub r#composition: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_Composition,
            }
            impl Default for r#ParcelableComposition {
              fn default() -> Self {
                Self {
                  r#composition: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ParcelableComposition {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#composition)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#composition = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ParcelableComposition);
            binder::impl_deserialize_for_parcelable!(r#ParcelableComposition);
            impl binder::binder_impl::ParcelableMetadata for r#ParcelableComposition {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableComposition" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ParcelableComposition as _7_android_8_hardware_8_graphics_9_composer3_21_ParcelableComposition;
            }
          }
                    pub mod ParcelableDataspace {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ParcelableDataspace.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ParcelableDataspace.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ParcelableDataspace {
              pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
            }
            impl Default for r#ParcelableDataspace {
              fn default() -> Self {
                Self {
                  r#dataspace: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ParcelableDataspace {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#dataspace)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ParcelableDataspace);
            binder::impl_deserialize_for_parcelable!(r#ParcelableDataspace);
            impl binder::binder_impl::ParcelableMetadata for r#ParcelableDataspace {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableDataspace" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ParcelableDataspace as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableDataspace;
            }
          }
                    pub mod ParcelableTransform {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ParcelableTransform.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ParcelableTransform.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ParcelableTransform {
              pub r#transform: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform,
            }
            impl Default for r#ParcelableTransform {
              fn default() -> Self {
                Self {
                  r#transform: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ParcelableTransform {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#transform)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#transform = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ParcelableTransform);
            binder::impl_deserialize_for_parcelable!(r#ParcelableTransform);
            impl binder::binder_impl::ParcelableMetadata for r#ParcelableTransform {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableTransform" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ParcelableTransform as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableTransform;
            }
          }
                    pub mod PerFrameMetadata {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PerFrameMetadata.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PerFrameMetadata.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PerFrameMetadata {
              pub r#key: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey,
              pub r#value: f32,
            }
            impl Default for r#PerFrameMetadata {
              fn default() -> Self {
                Self {
                  r#key: Default::default(),
                  r#value: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#PerFrameMetadata {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#key)?;
                  subparcel.write(&self.r#value)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#key = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#value = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PerFrameMetadata);
            binder::impl_deserialize_for_parcelable!(r#PerFrameMetadata);
            impl binder::binder_impl::ParcelableMetadata for r#PerFrameMetadata {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PerFrameMetadata" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PerFrameMetadata as _7_android_8_hardware_8_graphics_9_composer3_16_PerFrameMetadata;
            }
          }
                    pub mod PerFrameMetadataBlob {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PerFrameMetadataBlob.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PerFrameMetadataBlob {
              pub r#key: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey,
              pub r#blob: Vec<u8>,
            }
            impl Default for r#PerFrameMetadataBlob {
              fn default() -> Self {
                Self {
                  r#key: Default::default(),
                  r#blob: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PerFrameMetadataBlob {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#key)?;
                  subparcel.write(&self.r#blob)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#key = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#blob = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PerFrameMetadataBlob);
            binder::impl_deserialize_for_parcelable!(r#PerFrameMetadataBlob);
            impl binder::binder_impl::ParcelableMetadata for r#PerFrameMetadataBlob {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PerFrameMetadataBlob" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PerFrameMetadataBlob as _7_android_8_hardware_8_graphics_9_composer3_20_PerFrameMetadataBlob;
            }
          }
                    pub mod PerFrameMetadataKey {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PerFrameMetadataKey.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#PerFrameMetadataKey : [i32; 13] {
                r#DISPLAY_RED_PRIMARY_X = 0,
                r#DISPLAY_RED_PRIMARY_Y = 1,
                r#DISPLAY_GREEN_PRIMARY_X = 2,
                r#DISPLAY_GREEN_PRIMARY_Y = 3,
                r#DISPLAY_BLUE_PRIMARY_X = 4,
                r#DISPLAY_BLUE_PRIMARY_Y = 5,
                r#WHITE_POINT_X = 6,
                r#WHITE_POINT_Y = 7,
                r#MAX_LUMINANCE = 8,
                r#MIN_LUMINANCE = 9,
                r#MAX_CONTENT_LIGHT_LEVEL = 10,
                r#MAX_FRAME_AVERAGE_LIGHT_LEVEL = 11,
                r#HDR10_PLUS_SEI = 12,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PerFrameMetadataKey as _7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey;
            }
          }
                    pub mod PlaneAlpha {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PlaneAlpha.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PlaneAlpha.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PlaneAlpha {
              pub r#alpha: f32,
            }
            impl Default for r#PlaneAlpha {
              fn default() -> Self {
                Self {
                  r#alpha: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#PlaneAlpha {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#alpha)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#alpha = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PlaneAlpha);
            binder::impl_deserialize_for_parcelable!(r#PlaneAlpha);
            impl binder::binder_impl::ParcelableMetadata for r#PlaneAlpha {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PlaneAlpha" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PlaneAlpha as _7_android_8_hardware_8_graphics_9_composer3_10_PlaneAlpha;
            }
          }
                    pub mod PowerMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PowerMode.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PowerMode.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#PowerMode : [i32; 5] {
                r#OFF = 0,
                r#DOZE = 1,
                r#DOZE_SUSPEND = 3,
                r#ON = 2,
                r#ON_SUSPEND = 4,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PowerMode as _7_android_8_hardware_8_graphics_9_composer3_9_PowerMode;
            }
          }
                    pub mod PresentFence {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PresentFence.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PresentFence.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PresentFence {
              pub r#display: i64,
              pub r#fence: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#PresentFence {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#fence: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PresentFence {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  let __field_ref = self.r#fence.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#fence = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PresentFence);
            binder::impl_deserialize_for_parcelable!(r#PresentFence);
            impl binder::binder_impl::ParcelableMetadata for r#PresentFence {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PresentFence" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PresentFence as _7_android_8_hardware_8_graphics_9_composer3_12_PresentFence;
            }
          }
                    pub mod PresentOrValidate {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/PresentOrValidate.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/PresentOrValidate.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PresentOrValidate {
              pub r#display: i64,
              pub r#result: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate_6_Result,
            }
            impl Default for r#PresentOrValidate {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#result: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PresentOrValidate {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#result)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#result = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PresentOrValidate);
            binder::impl_deserialize_for_parcelable!(r#PresentOrValidate);
            impl binder::binder_impl::ParcelableMetadata for r#PresentOrValidate {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PresentOrValidate" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Result {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(1))]
                r#Result : [i8; 2] {
                  r#Validated = 0,
                  r#Presented = 1,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PresentOrValidate as _7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate;
             pub use super::r#Result::r#Result as _7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate_6_Result;
            }
          }
                    pub mod ReadbackBufferAttributes {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ReadbackBufferAttributes.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ReadbackBufferAttributes {
              pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
              pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
            }
            impl Default for r#ReadbackBufferAttributes {
              fn default() -> Self {
                Self {
                  r#format: Default::default(),
                  r#dataspace: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ReadbackBufferAttributes {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#format)?;
                  subparcel.write(&self.r#dataspace)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#format = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dataspace = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ReadbackBufferAttributes);
            binder::impl_deserialize_for_parcelable!(r#ReadbackBufferAttributes);
            impl binder::binder_impl::ParcelableMetadata for r#ReadbackBufferAttributes {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReadbackBufferAttributes" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ReadbackBufferAttributes as _7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes;
            }
          }
                    pub mod RefreshRateChangedDebugData {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/RefreshRateChangedDebugData.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#RefreshRateChangedDebugData {
              pub r#display: i64,
              pub r#vsyncPeriodNanos: i32,
              pub r#refreshPeriodNanos: i32,
            }
            impl Default for r#RefreshRateChangedDebugData {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#vsyncPeriodNanos: 0,
                  r#refreshPeriodNanos: 0,
                }
              }
            }
            impl binder::Parcelable for r#RefreshRateChangedDebugData {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#vsyncPeriodNanos)?;
                  subparcel.write(&self.r#refreshPeriodNanos)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#vsyncPeriodNanos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#refreshPeriodNanos = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#RefreshRateChangedDebugData);
            binder::impl_deserialize_for_parcelable!(r#RefreshRateChangedDebugData);
            impl binder::binder_impl::ParcelableMetadata for r#RefreshRateChangedDebugData {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.RefreshRateChangedDebugData" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#RefreshRateChangedDebugData as _7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData;
            }
          }
                    pub mod ReleaseFences {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ReleaseFences.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ReleaseFences.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ReleaseFences {
              pub r#display: i64,
              pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences_5_Layer>,
            }
            impl Default for r#ReleaseFences {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#layers: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#ReleaseFences {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#layers)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layers = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ReleaseFences);
            binder::impl_deserialize_for_parcelable!(r#ReleaseFences);
            impl binder::binder_impl::ParcelableMetadata for r#ReleaseFences {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReleaseFences" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Layer {
              #[derive(Debug)]
              pub struct r#Layer {
                pub r#layer: i64,
                pub r#fence: Option<binder::ParcelFileDescriptor>,
              }
              impl Default for r#Layer {
                fn default() -> Self {
                  Self {
                    r#layer: 0,
                    r#fence: Default::default(),
                  }
                }
              }
              impl binder::Parcelable for r#Layer {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#layer)?;
                    let __field_ref = self.r#fence.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                    subparcel.write(__field_ref)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#layer = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#fence = Some(subparcel.read()?);
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#Layer);
              binder::impl_deserialize_for_parcelable!(r#Layer);
              impl binder::binder_impl::ParcelableMetadata for r#Layer {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReleaseFences.Layer" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ReleaseFences as _7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences;
             pub use super::r#Layer::r#Layer as _7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences_5_Layer;
            }
          }
                    pub mod RenderIntent {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/RenderIntent.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/RenderIntent.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#RenderIntent : [i32; 4] {
                r#COLORIMETRIC = 0,
                r#ENHANCE = 1,
                r#TONE_MAP_COLORIMETRIC = 2,
                r#TONE_MAP_ENHANCE = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#RenderIntent as _7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent;
            }
          }
                    pub mod VirtualDisplay {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/VirtualDisplay.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/VirtualDisplay.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#VirtualDisplay {
              pub r#display: i64,
              pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
            }
            impl Default for r#VirtualDisplay {
              fn default() -> Self {
                Self {
                  r#display: 0,
                  r#format: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#VirtualDisplay {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#display)?;
                  subparcel.write(&self.r#format)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#display = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#format = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#VirtualDisplay);
            binder::impl_deserialize_for_parcelable!(r#VirtualDisplay);
            impl binder::binder_impl::ParcelableMetadata for r#VirtualDisplay {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VirtualDisplay" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#VirtualDisplay as _7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay;
            }
          }
                    pub mod VrrConfig {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/VrrConfig.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/VrrConfig.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#VrrConfig {
              pub r#minFrameIntervalNs: i32,
              pub r#frameIntervalPowerHints: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig_22_FrameIntervalPowerHint>>>,
              pub r#notifyExpectedPresentConfig: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig_27_NotifyExpectedPresentConfig>,
            }
            impl Default for r#VrrConfig {
              fn default() -> Self {
                Self {
                  r#minFrameIntervalNs: 0,
                  r#frameIntervalPowerHints: Default::default(),
                  r#notifyExpectedPresentConfig: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#VrrConfig {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#minFrameIntervalNs)?;
                  subparcel.write(&self.r#frameIntervalPowerHints)?;
                  subparcel.write(&self.r#notifyExpectedPresentConfig)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#minFrameIntervalNs = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#frameIntervalPowerHints = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#notifyExpectedPresentConfig = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#VrrConfig);
            binder::impl_deserialize_for_parcelable!(r#VrrConfig);
            impl binder::binder_impl::ParcelableMetadata for r#VrrConfig {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VrrConfig" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#FrameIntervalPowerHint {
              #[derive(Debug)]
              pub struct r#FrameIntervalPowerHint {
                pub r#frameIntervalNs: i32,
                pub r#averageRefreshPeriodNs: i32,
              }
              impl Default for r#FrameIntervalPowerHint {
                fn default() -> Self {
                  Self {
                    r#frameIntervalNs: 0,
                    r#averageRefreshPeriodNs: 0,
                  }
                }
              }
              impl binder::Parcelable for r#FrameIntervalPowerHint {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#frameIntervalNs)?;
                    subparcel.write(&self.r#averageRefreshPeriodNs)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#frameIntervalNs = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#averageRefreshPeriodNs = subparcel.read()?;
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#FrameIntervalPowerHint);
              binder::impl_deserialize_for_parcelable!(r#FrameIntervalPowerHint);
              impl binder::binder_impl::ParcelableMetadata for r#FrameIntervalPowerHint {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VrrConfig.FrameIntervalPowerHint" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub mod r#NotifyExpectedPresentConfig {
              #[derive(Debug)]
              pub struct r#NotifyExpectedPresentConfig {
                pub r#headsUpNs: i32,
                pub r#timeoutNs: i32,
              }
              impl Default for r#NotifyExpectedPresentConfig {
                fn default() -> Self {
                  Self {
                    r#headsUpNs: 0,
                    r#timeoutNs: 0,
                  }
                }
              }
              impl binder::Parcelable for r#NotifyExpectedPresentConfig {
                fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_write(|subparcel| {
                    subparcel.write(&self.r#headsUpNs)?;
                    subparcel.write(&self.r#timeoutNs)?;
                    Ok(())
                  })
                }
                fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                  parcel.sized_read(|subparcel| {
                    if subparcel.has_more_data() {
                      self.r#headsUpNs = subparcel.read()?;
                    }
                    if subparcel.has_more_data() {
                      self.r#timeoutNs = subparcel.read()?;
                    }
                    Ok(())
                  })
                }
              }
              binder::impl_serialize_for_parcelable!(r#NotifyExpectedPresentConfig);
              binder::impl_deserialize_for_parcelable!(r#NotifyExpectedPresentConfig);
              impl binder::binder_impl::ParcelableMetadata for r#NotifyExpectedPresentConfig {
                fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VrrConfig.NotifyExpectedPresentConfig" }
                fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#VrrConfig as _7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig;
             pub use super::r#FrameIntervalPowerHint::r#FrameIntervalPowerHint as _7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig_22_FrameIntervalPowerHint;
             pub use super::r#NotifyExpectedPresentConfig::r#NotifyExpectedPresentConfig as _7_android_8_hardware_8_graphics_9_composer3_9_VrrConfig_27_NotifyExpectedPresentConfig;
            }
          }
                    pub mod VsyncPeriodChangeConstraints {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#VsyncPeriodChangeConstraints {
              pub r#desiredTimeNanos: i64,
              pub r#seamlessRequired: bool,
            }
            impl Default for r#VsyncPeriodChangeConstraints {
              fn default() -> Self {
                Self {
                  r#desiredTimeNanos: 0,
                  r#seamlessRequired: false,
                }
              }
            }
            impl binder::Parcelable for r#VsyncPeriodChangeConstraints {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#desiredTimeNanos)?;
                  subparcel.write(&self.r#seamlessRequired)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#desiredTimeNanos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#seamlessRequired = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#VsyncPeriodChangeConstraints);
            binder::impl_deserialize_for_parcelable!(r#VsyncPeriodChangeConstraints);
            impl binder::binder_impl::ParcelableMetadata for r#VsyncPeriodChangeConstraints {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VsyncPeriodChangeConstraints" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#VsyncPeriodChangeConstraints as _7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints;
            }
          }
                    pub mod VsyncPeriodChangeTimeline {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#VsyncPeriodChangeTimeline {
              pub r#newVsyncAppliedTimeNanos: i64,
              pub r#refreshRequired: bool,
              pub r#refreshTimeNanos: i64,
            }
            impl Default for r#VsyncPeriodChangeTimeline {
              fn default() -> Self {
                Self {
                  r#newVsyncAppliedTimeNanos: 0,
                  r#refreshRequired: false,
                  r#refreshTimeNanos: 0,
                }
              }
            }
            impl binder::Parcelable for r#VsyncPeriodChangeTimeline {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#newVsyncAppliedTimeNanos)?;
                  subparcel.write(&self.r#refreshRequired)?;
                  subparcel.write(&self.r#refreshTimeNanos)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#newVsyncAppliedTimeNanos = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#refreshRequired = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#refreshTimeNanos = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#VsyncPeriodChangeTimeline);
            binder::impl_deserialize_for_parcelable!(r#VsyncPeriodChangeTimeline);
            impl binder::binder_impl::ParcelableMetadata for r#VsyncPeriodChangeTimeline {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VsyncPeriodChangeTimeline" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#VsyncPeriodChangeTimeline as _7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline;
            }
          }
                    pub mod ZOrder {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 3 --hash d24fcd9648b8b2e7287f9238eee9180244612c10 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common_interface/5/preprocessed.aidl -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen/android/hardware/graphics/composer3/ZOrder.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/composer/aidl/android.hardware.graphics.composer3-V3-rust-source/gen -Nhardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3 hardware/interfaces/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/3/android/hardware/graphics/composer3/ZOrder.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ZOrder {
              pub r#z: i32,
            }
            impl Default for r#ZOrder {
              fn default() -> Self {
                Self {
                  r#z: 0,
                }
              }
            }
            impl binder::Parcelable for r#ZOrder {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#z)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#z = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ZOrder);
            binder::impl_deserialize_for_parcelable!(r#ZOrder);
            impl binder::binder_impl::ParcelableMetadata for r#ZOrder {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ZOrder" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ZOrder as _7_android_8_hardware_8_graphics_9_composer3_6_ZOrder;
            }
          }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::graphics::composer3::Buffer::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::Capability::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ChangedCompositionLayer::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ChangedCompositionTypes::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ClientTarget::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ClientTargetProperty::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ClientTargetPropertyWithBrightness::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ClockMonotonicTimestamp::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::Color::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ColorMode::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::CommandError::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::CommandResultPayload::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::Composition::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ContentType::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DimmingStage::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayAttribute::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayBrightness::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayCapability::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayCommand::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayConfiguration::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayConnectionType::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayContentSample::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayContentSamplingAttributes::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayIdentification::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::DisplayRequest::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::FormatColorComponent::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::HdrCapabilities::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::IComposer::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::IComposerCallback::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::IComposerClient::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::LayerBrightness::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::LayerCommand::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::LayerLifecycleBatchCommandType::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::OverlayProperties::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ParcelableBlendMode::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ParcelableComposition::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ParcelableDataspace::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ParcelableTransform::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PerFrameMetadata::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PerFrameMetadataBlob::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PerFrameMetadataKey::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PlaneAlpha::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PowerMode::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PresentFence::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::PresentOrValidate::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ReadbackBufferAttributes::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::RefreshRateChangedDebugData::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ReleaseFences::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::RenderIntent::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::VirtualDisplay::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::VrrConfig::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::VsyncPeriodChangeConstraints::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::VsyncPeriodChangeTimeline::mangled::*;
  pub use super::aidl::android::hardware::graphics::composer3::ZOrder::mangled::*;
  pub(crate) use android_hardware_graphics_common::mangled::*;
  pub(crate) use android_hardware_common::mangled::*;
}
