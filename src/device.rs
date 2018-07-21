use uvc_sys::*;

use std::ffi::CStr;

pub struct Device {
    pub(crate) dev: *mut uvc_device,
}

impl Device {
    pub fn open(&self) -> Result<DeviceHandle, ::UvcError> {
        unsafe {
            let mut devh = ::std::mem::uninitialized();
            let err = uvc_open(self.dev, &mut devh);
            if err != uvc_error::UVC_SUCCESS {
                return Err(err);
            }
            Ok(DeviceHandle { devh })
        }
    }
    pub fn description(&self) -> Result<DeviceDescription, ::UvcError> {
        unsafe {
            let mut desc = ::std::mem::uninitialized();
            let err = uvc_get_device_descriptor(self.dev, &mut desc);
            if err != uvc_error::UVC_SUCCESS {
                return Err(err);
            }

            let id_vendor = (*desc).idVendor;
            let id_product = (*desc).idProduct;
            let bcd_uvc = (*desc).bcdUVC;

            let serial_number_c_str = (*desc).serialNumber;
            let serial_number = if serial_number_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(serial_number_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let manufacturer_c_str = (*desc).manufacturer;
            let manufacturer = if manufacturer_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(manufacturer_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let product_c_str = (*desc).product;
            let product = if product_c_str.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(product_c_str)
                        .to_owned()
                        .into_string()
                        .unwrap(),
                )
            };
            let descp = Ok(DeviceDescription {
                id_vendor,
                id_product,
                bcd_uvc,
                serial_number,
                manufacturer,
                product,
            });

            uvc_free_device_descriptor(desc);

            descp
        }
    }
}

pub struct DeviceHandle {
    pub devh: *mut uvc_device_handle,
}

impl DeviceHandle {
    pub fn get_stream_ctrl_with_size_and_fps(
        &self,
        width: u32,
        height: u32,
        fps: u32,
    ) -> Result<::StreamCtrl, ::UvcError> {
        unsafe {
            let mut ctrl = ::std::mem::uninitialized();
            let err = uvc_get_stream_ctrl_format_size(
                self.devh,
                &mut ctrl,
                uvc_frame_format::UVC_FRAME_FORMAT_YUYV,
                width as i32,
                height as i32,
                fps as i32,
            );
            if err != uvc_error::UVC_SUCCESS {
                Err(err)
            } else {
                Ok(::StreamCtrl { ctrl })
            }
        }
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        unsafe {
            uvc_close(self.devh);
        }
    }
}

#[derive(Debug)]
pub struct DeviceDescription {
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_uvc: u16,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}
