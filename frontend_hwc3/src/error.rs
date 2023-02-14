/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) enum HwcError {
    ServiceSpecific(i32, Option<String>),
    Poisoned,
    OptNone,
}

impl Display for HwcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HwcError::ServiceSpecific(code, msg) => {
                write!(f, "HwcError::ServiceSpecific({}, {:?})", code, msg)
            }
            HwcError::Poisoned => write!(f, "HwcError::Poisoned"),
            HwcError::OptNone => write!(f, "HwcError::OptNone"),
        }
    }
}

impl Error for HwcError {}

pub(crate) type HwcResult<T> = std::result::Result<T, HwcError>;

impl From<HwcError> for binder::Status {
    #[track_caller]
    fn from(err: HwcError) -> Self {
        match err {
            HwcError::ServiceSpecific(code, msg) => {
                let message = msg.unwrap_or_else(|| "Unknown error".to_string());
                let c_str = std::ffi::CString::new(message).unwrap_or_else(|_| {
                    std::ffi::CString::new("Invalid UTF-8 error message").unwrap()
                });
                let c_str = Some(c_str.as_c_str());
                binder::Status::new_service_specific_error(code, c_str)
            }
            _ => {
                log::error!("Unexpected error: {:?}", err);
                let c_str = std::ffi::CString::new(err.to_string()).unwrap_or_else(|_| {
                    std::ffi::CString::new("Invalid UTF-8 error message").unwrap()
                });
                let c_str = Some(c_str.as_c_str());
                binder::Status::new_service_specific_error(
                    binder::StatusCode::UNKNOWN_ERROR as i32,
                    c_str,
                )
            }
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for HwcError {
    #[track_caller]
    fn from(_: std::sync::PoisonError<T>) -> Self {
        HwcError::Poisoned
    }
}

impl From<anyhow::Error> for HwcError {
    #[track_caller]
    fn from(err: anyhow::Error) -> Self {
        HwcError::ServiceSpecific(binder::StatusCode::UNKNOWN_ERROR as i32, Some(err.to_string()))
    }
}
