use crate::device::DeviceHandle;
use crate::error::{Error, Result};
use uvc_sys::*;

#[derive(Copy, Clone, Debug)]
pub enum ScanningMode {
    Interlaced,
    Progressive,
}

#[derive(Copy, Clone, Debug)]
pub enum AutoExposureMode {
    Manual,
    Auto,
    ShutterPriority,
    AperturePriority,
}

#[derive(Copy, Clone, Debug)]
pub enum AutoExposurePriority {
    Constant,
    Variable,
}

impl<'a> DeviceHandle<'a> {
    pub fn scanning_mode(&self) -> Result<ScanningMode> {
        unsafe {
            let mut mode = std::mem::uninitialized();
            let err =
                uvc_get_scanning_mode(self.devh.as_ptr(), &mut mode, uvc_req_code_UVC_GET_CUR)
                    .into();
            if err != Error::Success {
                return Err(err);
            }
            match mode {
                0 => Ok(ScanningMode::Interlaced),
                1 => Ok(ScanningMode::Progressive),
                _ => Err(Error::Other),
            }
        }
    }
    pub fn ae_mode(&self) -> Result<AutoExposureMode> {
        unsafe {
            let mut mode = std::mem::uninitialized();
            let err =
                uvc_get_ae_mode(self.devh.as_ptr(), &mut mode, uvc_req_code_UVC_GET_CUR).into();
            if err != Error::Success {
                return Err(err);
            }
            match mode {
                1 => Ok(AutoExposureMode::Manual),
                2 => Ok(AutoExposureMode::Auto),
                4 => Ok(AutoExposureMode::ShutterPriority),
                8 => Ok(AutoExposureMode::AperturePriority),
                _ => Err(Error::Other),
            }
        }
    }
    pub fn ae_priority(&self) -> Result<AutoExposurePriority> {
        unsafe {
            let mut priority = std::mem::uninitialized();
            let err =
                uvc_get_ae_priority(self.devh.as_ptr(), &mut priority, uvc_req_code_UVC_GET_CUR)
                    .into();
            if err != Error::Success {
                return Err(err);
            }
            match priority {
                0 => Ok(AutoExposurePriority::Constant),
                1 => Ok(AutoExposurePriority::Variable),
                _ => Err(Error::Other),
            }
        }
    }
    pub fn exposure_abs(&self) -> Result<u32> {
        unsafe {
            let mut time = std::mem::uninitialized();
            let err = uvc_get_exposure_abs(self.devh.as_ptr(), &mut time, uvc_req_code_UVC_GET_CUR)
                .into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(time)
            }
        }
    }
    pub fn exposure_rel(&self) -> Result<i8> {
        unsafe {
            let mut step = std::mem::uninitialized();
            let err = uvc_get_exposure_rel(self.devh.as_ptr(), &mut step, uvc_req_code_UVC_GET_CUR)
                .into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(step)
            }
        }
    }
    pub fn focus_abs(&self) -> Result<u16> {
        unsafe {
            let mut focus = std::mem::uninitialized();
            let err =
                uvc_get_focus_abs(self.devh.as_ptr(), &mut focus, uvc_req_code_UVC_GET_CUR).into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(focus)
            }
        }
    }
    pub fn focus_rel(&self) -> Result<(i8, u8)> {
        unsafe {
            let mut focus_rel = std::mem::uninitialized();
            let mut speed = std::mem::uninitialized();
            let err = uvc_get_focus_rel(
                self.devh.as_ptr(),
                &mut focus_rel,
                &mut speed,
                uvc_req_code_UVC_GET_CUR,
            )
            .into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok((focus_rel, speed))
            }
        }
    }
}
