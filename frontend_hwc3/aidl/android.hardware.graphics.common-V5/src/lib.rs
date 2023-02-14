#![allow(warnings)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod graphics {
                pub mod common {
                    pub mod AlphaInterpretation {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/AlphaInterpretation.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/AlphaInterpretation.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#AlphaInterpretation : [i32; 2] {
                r#COVERAGE = 0,
                r#MASK = 1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#AlphaInterpretation as _7_android_8_hardware_8_graphics_6_common_19_AlphaInterpretation;
            }
          }
                    pub mod BlendMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/BlendMode.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/BlendMode.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#BlendMode : [i32; 4] {
                r#INVALID = 0,
                r#NONE = 1,
                r#PREMULTIPLIED = 2,
                r#COVERAGE = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#BlendMode as _7_android_8_hardware_8_graphics_6_common_9_BlendMode;
            }
          }
                    pub mod BufferUsage {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/BufferUsage.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/BufferUsage.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#BufferUsage : [i64; 27] {
                r#CPU_READ_MASK = 15,
                r#CPU_READ_NEVER = 0,
                r#CPU_READ_RARELY = 2,
                r#CPU_READ_OFTEN = 3,
                r#CPU_WRITE_MASK = 240,
                r#CPU_WRITE_NEVER = 0,
                r#CPU_WRITE_RARELY = 32,
                r#CPU_WRITE_OFTEN = 48,
                r#GPU_TEXTURE = 256,
                r#GPU_RENDER_TARGET = 512,
                r#COMPOSER_OVERLAY = 2048,
                r#COMPOSER_CLIENT_TARGET = 4096,
                r#PROTECTED = 16384,
                r#COMPOSER_CURSOR = 32768,
                r#VIDEO_ENCODER = 65536,
                r#CAMERA_OUTPUT = 131072,
                r#CAMERA_INPUT = 262144,
                r#RENDERSCRIPT = 1048576,
                r#VIDEO_DECODER = 4194304,
                r#SENSOR_DIRECT_DATA = 8388608,
                r#GPU_DATA_BUFFER = 16777216,
                r#GPU_CUBE_MAP = 33554432,
                r#GPU_MIPMAP_COMPLETE = 67108864,
                r#HW_IMAGE_ENCODER = 134217728,
                r#FRONT_BUFFER = 4294967296,
                r#VENDOR_MASK = 4026531840,
                r#VENDOR_MASK_HI = -281474976710656,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#BufferUsage as _7_android_8_hardware_8_graphics_6_common_11_BufferUsage;
            }
          }
                    pub mod ChromaSiting {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/ChromaSiting.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/ChromaSiting.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#ChromaSiting : [i64; 6] {
                r#NONE = 0,
                r#UNKNOWN = 1,
                r#SITED_INTERSTITIAL = 2,
                r#COSITED_HORIZONTAL = 3,
                r#COSITED_VERTICAL = 4,
                r#COSITED_BOTH = 5,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ChromaSiting as _7_android_8_hardware_8_graphics_6_common_12_ChromaSiting;
            }
          }
                    pub mod ColorTransform {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/ColorTransform.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/ColorTransform.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#ColorTransform : [i32; 7] {
                r#IDENTITY = 0,
                r#ARBITRARY_MATRIX = 1,
                r#VALUE_INVERSE = 2,
                r#GRAYSCALE = 3,
                r#CORRECT_PROTANOPIA = 4,
                r#CORRECT_DEUTERANOPIA = 5,
                r#CORRECT_TRITANOPIA = 6,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ColorTransform as _7_android_8_hardware_8_graphics_6_common_14_ColorTransform;
            }
          }
                    pub mod Compression {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Compression.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Compression.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#Compression : [i64; 2] {
                r#NONE = 0,
                r#DISPLAY_STREAM_COMPRESSION = 1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Compression as _7_android_8_hardware_8_graphics_6_common_11_Compression;
            }
          }
                    pub mod Cta861_3 {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Cta861_3.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Cta861_3.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Cta861_3 {
              pub r#maxContentLightLevel: f32,
              pub r#maxFrameAverageLightLevel: f32,
            }
            impl Default for r#Cta861_3 {
              fn default() -> Self {
                Self {
                  r#maxContentLightLevel: 0.000000f32,
                  r#maxFrameAverageLightLevel: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#Cta861_3 {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#maxContentLightLevel)?;
                  subparcel.write(&self.r#maxFrameAverageLightLevel)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#maxContentLightLevel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxFrameAverageLightLevel = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Cta861_3);
            binder::impl_deserialize_for_parcelable!(r#Cta861_3);
            impl binder::binder_impl::ParcelableMetadata for r#Cta861_3 {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Cta861_3" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Cta861_3 as _7_android_8_hardware_8_graphics_6_common_8_Cta861_3;
            }
          }
                    pub mod Dataspace {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Dataspace.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Dataspace.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Dataspace : [i32; 63] {
                r#UNKNOWN = 0,
                r#ARBITRARY = 1,
                r#STANDARD_SHIFT = 16,
                r#STANDARD_MASK = 4128768,
                r#STANDARD_UNSPECIFIED = 0,
                r#STANDARD_BT709 = 65536,
                r#STANDARD_BT601_625 = 131072,
                r#STANDARD_BT601_625_UNADJUSTED = 196608,
                r#STANDARD_BT601_525 = 262144,
                r#STANDARD_BT601_525_UNADJUSTED = 327680,
                r#STANDARD_BT2020 = 393216,
                r#STANDARD_BT2020_CONSTANT_LUMINANCE = 458752,
                r#STANDARD_BT470M = 524288,
                r#STANDARD_FILM = 589824,
                r#STANDARD_DCI_P3 = 655360,
                r#STANDARD_ADOBE_RGB = 720896,
                r#TRANSFER_SHIFT = 22,
                r#TRANSFER_MASK = 130023424,
                r#TRANSFER_UNSPECIFIED = 0,
                r#TRANSFER_LINEAR = 4194304,
                r#TRANSFER_SRGB = 8388608,
                r#TRANSFER_SMPTE_170M = 12582912,
                r#TRANSFER_GAMMA2_2 = 16777216,
                r#TRANSFER_GAMMA2_6 = 20971520,
                r#TRANSFER_GAMMA2_8 = 25165824,
                r#TRANSFER_ST2084 = 29360128,
                r#TRANSFER_HLG = 33554432,
                r#RANGE_SHIFT = 27,
                r#RANGE_MASK = 939524096,
                r#RANGE_UNSPECIFIED = 0,
                r#RANGE_FULL = 134217728,
                r#RANGE_LIMITED = 268435456,
                r#RANGE_EXTENDED = 402653184,
                r#SRGB_LINEAR = 138477568,
                r#SCRGB_LINEAR = 406913024,
                r#SRGB = 142671872,
                r#SCRGB = 411107328,
                r#JFIF = 146931712,
                r#BT601_625 = 281149440,
                r#BT601_525 = 281280512,
                r#BT709 = 281083904,
                r#DCI_P3_LINEAR = 139067392,
                r#DCI_P3 = 155844608,
                r#DISPLAY_P3_LINEAR = 139067392,
                r#DISPLAY_P3 = 143261696,
                r#ADOBE_RGB = 151715840,
                r#ADOBE_RGB_LINEAR = 139132928,
                r#BT2020_LINEAR = 138805248,
                r#BT2020 = 147193856,
                r#BT2020_PQ = 163971072,
                r#BT2020_LINEAR_EXTENDED = 407240704,
                r#DEPTH = 4096,
                r#SENSOR = 4097,
                r#BT2020_ITU = 281411584,
                r#BT2020_ITU_PQ = 298188800,
                r#BT2020_ITU_HLG = 302383104,
                r#BT2020_HLG = 168165376,
                r#DISPLAY_BT2020 = 142999552,
                r#DYNAMIC_DEPTH = 4098,
                r#JPEG_APP_SEGMENTS = 4099,
                r#HEIF = 4100,
                r#JPEG_R = 4101,
                r#BT709_FULL_RANGE = 146866176,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Dataspace as _7_android_8_hardware_8_graphics_6_common_9_Dataspace;
            }
          }
                    pub mod DisplayDecorationSupport {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/DisplayDecorationSupport.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/DisplayDecorationSupport.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#DisplayDecorationSupport {
              pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
              pub r#alphaInterpretation: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_AlphaInterpretation,
            }
            impl Default for r#DisplayDecorationSupport {
              fn default() -> Self {
                Self {
                  r#format: Default::default(),
                  r#alphaInterpretation: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#DisplayDecorationSupport {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#format)?;
                  subparcel.write(&self.r#alphaInterpretation)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#format = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#alphaInterpretation = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#DisplayDecorationSupport);
            binder::impl_deserialize_for_parcelable!(r#DisplayDecorationSupport);
            impl binder::binder_impl::ParcelableMetadata for r#DisplayDecorationSupport {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.DisplayDecorationSupport" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayDecorationSupport as _7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport;
            }
          }
                    pub mod DisplayHotplugEvent {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/DisplayHotplugEvent.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/DisplayHotplugEvent.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#DisplayHotplugEvent : [i32; 5] {
                r#CONNECTED = 0,
                r#DISCONNECTED = 1,
                r#ERROR_UNKNOWN = -1,
                r#ERROR_INCOMPATIBLE_CABLE = -2,
                r#ERROR_TOO_MANY_DISPLAYS = -3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#DisplayHotplugEvent as _7_android_8_hardware_8_graphics_6_common_19_DisplayHotplugEvent;
            }
          }
                    pub mod ExtendableType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/ExtendableType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/ExtendableType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#ExtendableType {
              pub r#name: String,
              pub r#value: i64,
            }
            impl Default for r#ExtendableType {
              fn default() -> Self {
                Self {
                  r#name: Default::default(),
                  r#value: 0,
                }
              }
            }
            impl binder::Parcelable for r#ExtendableType {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#name)?;
                  subparcel.write(&self.r#value)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#value = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#ExtendableType);
            binder::impl_deserialize_for_parcelable!(r#ExtendableType);
            impl binder::binder_impl::ParcelableMetadata for r#ExtendableType {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.ExtendableType" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#ExtendableType as _7_android_8_hardware_8_graphics_6_common_14_ExtendableType;
            }
          }
                    pub mod FRect {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/FRect.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/FRect.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#FRect {
              pub r#left: f32,
              pub r#top: f32,
              pub r#right: f32,
              pub r#bottom: f32,
            }
            impl Default for r#FRect {
              fn default() -> Self {
                Self {
                  r#left: 0.000000f32,
                  r#top: 0.000000f32,
                  r#right: 0.000000f32,
                  r#bottom: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#FRect {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#left)?;
                  subparcel.write(&self.r#top)?;
                  subparcel.write(&self.r#right)?;
                  subparcel.write(&self.r#bottom)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#left = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#top = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#right = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bottom = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#FRect);
            binder::impl_deserialize_for_parcelable!(r#FRect);
            impl binder::binder_impl::ParcelableMetadata for r#FRect {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.FRect" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#FRect as _7_android_8_hardware_8_graphics_6_common_5_FRect;
            }
          }
                    pub mod HardwareBuffer {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/HardwareBuffer.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/HardwareBuffer.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            #[deprecated = ": Use instead android.hardware.HardwareBuffer in frameworks/base"]
            pub struct r#HardwareBuffer {
              pub r#description: crate::mangled::_7_android_8_hardware_8_graphics_6_common_25_HardwareBufferDescription,
              pub r#handle: crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle,
            }
            impl Default for r#HardwareBuffer {
              fn default() -> Self {
                Self {
                  r#description: Default::default(),
                  r#handle: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#HardwareBuffer {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#description)?;
                  subparcel.write(&self.r#handle)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#description = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#handle = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HardwareBuffer);
            binder::impl_deserialize_for_parcelable!(r#HardwareBuffer);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareBuffer {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HardwareBuffer" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareBuffer as _7_android_8_hardware_8_graphics_6_common_14_HardwareBuffer;
            }
          }
                    pub mod HardwareBufferDescription {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/HardwareBufferDescription.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/HardwareBufferDescription.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#HardwareBufferDescription {
              pub r#width: i32,
              pub r#height: i32,
              pub r#layers: i32,
              pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
              pub r#usage: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_BufferUsage,
              pub r#stride: i32,
            }
            impl Default for r#HardwareBufferDescription {
              fn default() -> Self {
                Self {
                  r#width: 0,
                  r#height: 0,
                  r#layers: 0,
                  r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat::UNSPECIFIED,
                  r#usage: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_BufferUsage::CPU_READ_NEVER,
                  r#stride: 0,
                }
              }
            }
            impl binder::Parcelable for r#HardwareBufferDescription {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#width)?;
                  subparcel.write(&self.r#height)?;
                  subparcel.write(&self.r#layers)?;
                  subparcel.write(&self.r#format)?;
                  subparcel.write(&self.r#usage)?;
                  subparcel.write(&self.r#stride)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#width = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#height = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#layers = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#format = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#usage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#stride = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HardwareBufferDescription);
            binder::impl_deserialize_for_parcelable!(r#HardwareBufferDescription);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareBufferDescription {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HardwareBufferDescription" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareBufferDescription as _7_android_8_hardware_8_graphics_6_common_25_HardwareBufferDescription;
            }
          }
                    pub mod Hdr {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Hdr.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Hdr.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Hdr : [i32; 6] {
                r#INVALID = 0,
                r#DOLBY_VISION = 1,
                r#HDR10 = 2,
                r#HLG = 3,
                r#HDR10_PLUS = 4,
                r#DOLBY_VISION_4K30 = 5,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Hdr as _7_android_8_hardware_8_graphics_6_common_3_Hdr;
            }
          }
                    pub mod HdrConversionCapability {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/HdrConversionCapability.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/HdrConversionCapability.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#HdrConversionCapability {
              pub r#sourceType: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr,
              pub r#outputType: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr,
              pub r#addsLatency: bool,
            }
            impl Default for r#HdrConversionCapability {
              fn default() -> Self {
                Self {
                  r#sourceType: Default::default(),
                  r#outputType: Default::default(),
                  r#addsLatency: false,
                }
              }
            }
            impl binder::Parcelable for r#HdrConversionCapability {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sourceType)?;
                  subparcel.write(&self.r#outputType)?;
                  subparcel.write(&self.r#addsLatency)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sourceType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#outputType = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#addsLatency = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#HdrConversionCapability);
            binder::impl_deserialize_for_parcelable!(r#HdrConversionCapability);
            impl binder::binder_impl::ParcelableMetadata for r#HdrConversionCapability {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HdrConversionCapability" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HdrConversionCapability as _7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability;
            }
          }
                    pub mod HdrConversionStrategy {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/HdrConversionStrategy.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/HdrConversionStrategy.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub enum r#HdrConversionStrategy {
              Passthrough(bool),
              AutoAllowedHdrTypes(Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>),
              ForceHdrConversion(crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr),
            }
            impl Default for r#HdrConversionStrategy {
              fn default() -> Self {
                Self::Passthrough(true)
              }
            }
            impl binder::Parcelable for r#HdrConversionStrategy {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Passthrough(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::AutoAllowedHdrTypes(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::ForceHdrConversion(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: bool = parcel.read()?;
                    *self = Self::Passthrough(value);
                    Ok(())
                  }
                  1 => {
                    let value: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> = parcel.read()?;
                    *self = Self::AutoAllowedHdrTypes(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr = parcel.read()?;
                    *self = Self::ForceHdrConversion(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#HdrConversionStrategy);
            binder::impl_deserialize_for_parcelable!(r#HdrConversionStrategy);
            impl binder::binder_impl::ParcelableMetadata for r#HdrConversionStrategy {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HdrConversionStrategy" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(4))]
                r#Tag : [i32; 3] {
                  r#passthrough = 0,
                  r#autoAllowedHdrTypes = 1,
                  r#forceHdrConversion = 2,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#HdrConversionStrategy as _7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy_3_Tag;
            }
          }
                    pub mod Interlaced {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Interlaced.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Interlaced.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#Interlaced : [i64; 3] {
                r#NONE = 0,
                r#TOP_BOTTOM = 1,
                r#RIGHT_LEFT = 2,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Interlaced as _7_android_8_hardware_8_graphics_6_common_10_Interlaced;
            }
          }
                    pub mod PixelFormat {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/PixelFormat.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/PixelFormat.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#PixelFormat : [i32; 33] {
                r#UNSPECIFIED = 0,
                r#RGBA_8888 = 1,
                r#RGBX_8888 = 2,
                r#RGB_888 = 3,
                r#RGB_565 = 4,
                r#BGRA_8888 = 5,
                r#YCBCR_422_SP = 16,
                r#YCRCB_420_SP = 17,
                r#YCBCR_422_I = 20,
                r#RGBA_FP16 = 22,
                r#RAW16 = 32,
                r#BLOB = 33,
                r#IMPLEMENTATION_DEFINED = 34,
                r#YCBCR_420_888 = 35,
                r#RAW_OPAQUE = 36,
                r#RAW10 = 37,
                r#RAW12 = 38,
                r#RGBA_1010102 = 43,
                r#Y8 = 538982489,
                r#Y16 = 540422489,
                r#YV12 = 842094169,
                r#DEPTH_16 = 48,
                r#DEPTH_24 = 49,
                r#DEPTH_24_STENCIL_8 = 50,
                r#DEPTH_32F = 51,
                r#DEPTH_32F_STENCIL_8 = 52,
                r#STENCIL_8 = 53,
                r#YCBCR_P010 = 54,
                r#HSV_888 = 55,
                r#R_8 = 56,
                r#R_16_UINT = 57,
                r#RG_1616_UINT = 58,
                r#RGBA_10101010 = 59,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PixelFormat as _7_android_8_hardware_8_graphics_6_common_11_PixelFormat;
            }
          }
                    pub mod PlaneLayout {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/PlaneLayout.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/PlaneLayout.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PlaneLayout {
              pub r#components: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_20_PlaneLayoutComponent>,
              pub r#offsetInBytes: i64,
              pub r#sampleIncrementInBits: i64,
              pub r#strideInBytes: i64,
              pub r#widthInSamples: i64,
              pub r#heightInSamples: i64,
              pub r#totalSizeInBytes: i64,
              pub r#horizontalSubsampling: i64,
              pub r#verticalSubsampling: i64,
            }
            impl Default for r#PlaneLayout {
              fn default() -> Self {
                Self {
                  r#components: Default::default(),
                  r#offsetInBytes: 0,
                  r#sampleIncrementInBits: 0,
                  r#strideInBytes: 0,
                  r#widthInSamples: 0,
                  r#heightInSamples: 0,
                  r#totalSizeInBytes: 0,
                  r#horizontalSubsampling: 0,
                  r#verticalSubsampling: 0,
                }
              }
            }
            impl binder::Parcelable for r#PlaneLayout {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#components)?;
                  subparcel.write(&self.r#offsetInBytes)?;
                  subparcel.write(&self.r#sampleIncrementInBits)?;
                  subparcel.write(&self.r#strideInBytes)?;
                  subparcel.write(&self.r#widthInSamples)?;
                  subparcel.write(&self.r#heightInSamples)?;
                  subparcel.write(&self.r#totalSizeInBytes)?;
                  subparcel.write(&self.r#horizontalSubsampling)?;
                  subparcel.write(&self.r#verticalSubsampling)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#components = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#offsetInBytes = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sampleIncrementInBits = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#strideInBytes = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#widthInSamples = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#heightInSamples = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#totalSizeInBytes = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#horizontalSubsampling = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#verticalSubsampling = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PlaneLayout);
            binder::impl_deserialize_for_parcelable!(r#PlaneLayout);
            impl binder::binder_impl::ParcelableMetadata for r#PlaneLayout {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.PlaneLayout" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PlaneLayout as _7_android_8_hardware_8_graphics_6_common_11_PlaneLayout;
            }
          }
                    pub mod PlaneLayoutComponent {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/PlaneLayoutComponent.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/PlaneLayoutComponent.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#PlaneLayoutComponent {
              pub r#type: crate::mangled::_7_android_8_hardware_8_graphics_6_common_14_ExtendableType,
              pub r#offsetInBits: i64,
              pub r#sizeInBits: i64,
            }
            impl Default for r#PlaneLayoutComponent {
              fn default() -> Self {
                Self {
                  r#type: Default::default(),
                  r#offsetInBits: 0,
                  r#sizeInBits: 0,
                }
              }
            }
            impl binder::Parcelable for r#PlaneLayoutComponent {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#type)?;
                  subparcel.write(&self.r#offsetInBits)?;
                  subparcel.write(&self.r#sizeInBits)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#type = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#offsetInBits = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sizeInBits = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PlaneLayoutComponent);
            binder::impl_deserialize_for_parcelable!(r#PlaneLayoutComponent);
            impl binder::binder_impl::ParcelableMetadata for r#PlaneLayoutComponent {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.PlaneLayoutComponent" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PlaneLayoutComponent as _7_android_8_hardware_8_graphics_6_common_20_PlaneLayoutComponent;
            }
          }
                    pub mod PlaneLayoutComponentType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/PlaneLayoutComponentType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/PlaneLayoutComponentType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#PlaneLayoutComponentType : [i64; 8] {
                r#Y = 1,
                r#CB = 2,
                r#CR = 4,
                r#R = 1024,
                r#G = 2048,
                r#B = 4096,
                r#RAW = 1048576,
                r#A = 1073741824,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PlaneLayoutComponentType as _7_android_8_hardware_8_graphics_6_common_24_PlaneLayoutComponentType;
            }
          }
                    pub mod Point {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Point.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Point.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Point {
              pub r#x: i32,
              pub r#y: i32,
            }
            impl Default for r#Point {
              fn default() -> Self {
                Self {
                  r#x: 0,
                  r#y: 0,
                }
              }
            }
            impl binder::Parcelable for r#Point {
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
            binder::impl_serialize_for_parcelable!(r#Point);
            binder::impl_deserialize_for_parcelable!(r#Point);
            impl binder::binder_impl::ParcelableMetadata for r#Point {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Point" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Point as _7_android_8_hardware_8_graphics_6_common_5_Point;
            }
          }
                    pub mod Rect {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Rect.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Rect.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Rect {
              pub r#left: i32,
              pub r#top: i32,
              pub r#right: i32,
              pub r#bottom: i32,
            }
            impl Default for r#Rect {
              fn default() -> Self {
                Self {
                  r#left: 0,
                  r#top: 0,
                  r#right: 0,
                  r#bottom: 0,
                }
              }
            }
            impl binder::Parcelable for r#Rect {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#left)?;
                  subparcel.write(&self.r#top)?;
                  subparcel.write(&self.r#right)?;
                  subparcel.write(&self.r#bottom)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#left = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#top = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#right = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#bottom = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Rect);
            binder::impl_deserialize_for_parcelable!(r#Rect);
            impl binder::binder_impl::ParcelableMetadata for r#Rect {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Rect" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Rect as _7_android_8_hardware_8_graphics_6_common_4_Rect;
            }
          }
                    pub mod Smpte2086 {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Smpte2086.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Smpte2086.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Smpte2086 {
              pub r#primaryRed: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
              pub r#primaryGreen: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
              pub r#primaryBlue: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
              pub r#whitePoint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
              pub r#maxLuminance: f32,
              pub r#minLuminance: f32,
            }
            impl Default for r#Smpte2086 {
              fn default() -> Self {
                Self {
                  r#primaryRed: Default::default(),
                  r#primaryGreen: Default::default(),
                  r#primaryBlue: Default::default(),
                  r#whitePoint: Default::default(),
                  r#maxLuminance: 0.000000f32,
                  r#minLuminance: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#Smpte2086 {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#primaryRed)?;
                  subparcel.write(&self.r#primaryGreen)?;
                  subparcel.write(&self.r#primaryBlue)?;
                  subparcel.write(&self.r#whitePoint)?;
                  subparcel.write(&self.r#maxLuminance)?;
                  subparcel.write(&self.r#minLuminance)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#primaryRed = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#primaryGreen = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#primaryBlue = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#whitePoint = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#maxLuminance = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#minLuminance = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Smpte2086);
            binder::impl_deserialize_for_parcelable!(r#Smpte2086);
            impl binder::binder_impl::ParcelableMetadata for r#Smpte2086 {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Smpte2086" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Smpte2086 as _7_android_8_hardware_8_graphics_6_common_9_Smpte2086;
            }
          }
                    pub mod StandardMetadataType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/StandardMetadataType.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/StandardMetadataType.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(8))]
              r#StandardMetadataType : [i64; 24] {
                r#INVALID = 0,
                r#BUFFER_ID = 1,
                r#NAME = 2,
                r#WIDTH = 3,
                r#HEIGHT = 4,
                r#LAYER_COUNT = 5,
                r#PIXEL_FORMAT_REQUESTED = 6,
                r#PIXEL_FORMAT_FOURCC = 7,
                r#PIXEL_FORMAT_MODIFIER = 8,
                r#USAGE = 9,
                r#ALLOCATION_SIZE = 10,
                r#PROTECTED_CONTENT = 11,
                r#COMPRESSION = 12,
                r#INTERLACED = 13,
                r#CHROMA_SITING = 14,
                r#PLANE_LAYOUTS = 15,
                r#CROP = 16,
                r#DATASPACE = 17,
                r#BLEND_MODE = 18,
                r#SMPTE2086 = 19,
                r#CTA861_3 = 20,
                r#SMPTE2094_40 = 21,
                r#SMPTE2094_10 = 22,
                r#STRIDE = 23,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#StandardMetadataType as _7_android_8_hardware_8_graphics_6_common_20_StandardMetadataType;
            }
          }
                    pub mod Transform {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/Transform.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/Transform.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Transform : [i32; 6] {
                r#NONE = 0,
                r#FLIP_H = 1,
                r#FLIP_V = 2,
                r#ROT_90 = 4,
                r#ROT_180 = 3,
                r#ROT_270 = 7,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Transform as _7_android_8_hardware_8_graphics_6_common_9_Transform;
            }
          }
                    pub mod XyColor {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 2ffe8da1136972e9b6bed7903f0d5aca289005a9 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/common/aidl/android.hardware.common_interface/2/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen/android/hardware/graphics/common/XyColor.rs.d -o out/soong/.intermediates/hardware/interfaces/graphics/common/aidl/android.hardware.graphics.common-V5-rust-source/gen -Nhardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5 hardware/interfaces/graphics/common/aidl/aidl_api/android.hardware.graphics.common/5/android/hardware/graphics/common/XyColor.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#XyColor {
              pub r#x: f32,
              pub r#y: f32,
            }
            impl Default for r#XyColor {
              fn default() -> Self {
                Self {
                  r#x: 0.000000f32,
                  r#y: 0.000000f32,
                }
              }
            }
            impl binder::Parcelable for r#XyColor {
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
            binder::impl_serialize_for_parcelable!(r#XyColor);
            binder::impl_deserialize_for_parcelable!(r#XyColor);
            impl binder::binder_impl::ParcelableMetadata for r#XyColor {
              fn get_descriptor() -> &'static str { "android.hardware.graphics.common.XyColor" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#XyColor as _7_android_8_hardware_8_graphics_6_common_7_XyColor;
            }
          }
                }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::graphics::common::AlphaInterpretation::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::BlendMode::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::BufferUsage::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::ChromaSiting::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::ColorTransform::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Compression::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Cta861_3::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Dataspace::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::DisplayDecorationSupport::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::DisplayHotplugEvent::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::ExtendableType::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::FRect::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::HardwareBuffer::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::HardwareBufferDescription::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Hdr::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::HdrConversionCapability::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::HdrConversionStrategy::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Interlaced::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::PixelFormat::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::PlaneLayout::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::PlaneLayoutComponent::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::PlaneLayoutComponentType::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Point::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Rect::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Smpte2086::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::StandardMetadataType::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::Transform::mangled::*;
    pub use super::aidl::android::hardware::graphics::common::XyColor::mangled::*;
    pub(crate) use android_hardware_common::mangled::*;
}
