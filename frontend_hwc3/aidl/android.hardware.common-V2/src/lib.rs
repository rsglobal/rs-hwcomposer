#![allow(warnings)]
#![feature(custom_inner_attributes)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
pub use binder;
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod common {
                pub mod Ashmem {
          #![forbid(unsafe_code)]
          #![rustfmt::skip]
          #[derive(Debug)]
          pub struct r#Ashmem {
            pub r#fd: Option<binder::ParcelFileDescriptor>,
            pub r#size: i64,
          }
          impl Default for r#Ashmem {
            fn default() -> Self {
              Self {
                r#fd: Default::default(),
                r#size: 0,
              }
            }
          }
          impl binder::Parcelable for r#Ashmem {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                let __field_ref = self.r#fd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#size)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#fd = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#size = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#Ashmem);
          binder::impl_deserialize_for_parcelable!(r#Ashmem);
          impl binder::binder_impl::ParcelableMetadata for r#Ashmem {
            fn get_descriptor() -> &'static str { "android.hardware.common.Ashmem" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#Ashmem as _7_android_8_hardware_6_common_6_Ashmem;
          }
        }
                pub mod MappableFile {
          #![forbid(unsafe_code)]
          #![rustfmt::skip]
          #[derive(Debug)]
          pub struct r#MappableFile {
            pub r#length: i64,
            pub r#prot: i32,
            pub r#fd: Option<binder::ParcelFileDescriptor>,
            pub r#offset: i64,
          }
          impl Default for r#MappableFile {
            fn default() -> Self {
              Self {
                r#length: 0,
                r#prot: 0,
                r#fd: Default::default(),
                r#offset: 0,
              }
            }
          }
          impl binder::Parcelable for r#MappableFile {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#length)?;
                subparcel.write(&self.r#prot)?;
                let __field_ref = self.r#fd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#offset)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#length = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#prot = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#fd = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#offset = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#MappableFile);
          binder::impl_deserialize_for_parcelable!(r#MappableFile);
          impl binder::binder_impl::ParcelableMetadata for r#MappableFile {
            fn get_descriptor() -> &'static str { "android.hardware.common.MappableFile" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#MappableFile as _7_android_8_hardware_6_common_12_MappableFile;
          }
        }
                pub mod NativeHandle {
          #![forbid(unsafe_code)]
          #![rustfmt::skip]
          #[derive(Debug)]
          pub struct r#NativeHandle {
            pub r#fds: Vec<binder::ParcelFileDescriptor>,
            pub r#ints: Vec<i32>,
          }
          impl Default for r#NativeHandle {
            fn default() -> Self {
              Self {
                r#fds: Default::default(),
                r#ints: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#NativeHandle {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#fds)?;
                subparcel.write(&self.r#ints)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#fds = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#ints = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#NativeHandle);
          binder::impl_deserialize_for_parcelable!(r#NativeHandle);
          impl binder::binder_impl::ParcelableMetadata for r#NativeHandle {
            fn get_descriptor() -> &'static str { "android.hardware.common.NativeHandle" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#NativeHandle as _7_android_8_hardware_6_common_12_NativeHandle;
          }
        }
            }
        }
    }
}
pub mod mangled {
    pub use super::aidl::android::hardware::common::Ashmem::mangled::*;
    pub use super::aidl::android::hardware::common::MappableFile::mangled::*;
    pub use super::aidl::android::hardware::common::NativeHandle::mangled::*;
}
