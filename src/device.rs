use streaming;
use uvc_sys::*;

use std::ffi::CStr;

use error::{Error, Result};

use std::marker::PhantomData;
use std::ptr::NonNull;

unsafe impl<'a> Send for DeviceList<'a> {}
unsafe impl<'a> Sync for DeviceList<'a> {}
#[derive(Debug)]
/// List of camera devices, iterate to get the device(s)
pub struct DeviceList<'a> {
    list: NonNull<*mut uvc_device>,
    _ph: PhantomData<&'a &'a uvc_device>,

    reached_end: bool,
    index: usize,
}

impl<'a> Drop for DeviceList<'a> {
    fn drop(&mut self) {
        unsafe { uvc_free_device_list(self.list.as_ptr(), false as u8) }
    }
}

impl<'a> DeviceList<'a> {
    pub(crate) fn new(list: NonNull<*mut uvc_device>) -> Self {
        Self {
            list,
            _ph: PhantomData,
            reached_end: false,
            index: 0,
        }
    }
}

impl<'a> Iterator for DeviceList<'a> {
    type Item = Device<'a>;

    fn next(&mut self) -> Option<Device<'a>> {
        if self.reached_end {
            return None;
        }

        let item = unsafe { self.list.as_ptr().offset(self.index as isize) };
        if item.is_null() {
            self.reached_end = true;
            return None;
        }

        self.index += 1;
        Some(unsafe { Device::from_raw(*item) })
    }
}

unsafe impl<'a> Send for Device<'a> {}
unsafe impl<'a> Sync for Device<'a> {}
#[derive(Debug)]
/// Device that can be opened
pub struct Device<'a> {
    dev: NonNull<uvc_device>,
    _dev: PhantomData<&'a uvc_device>,
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe { uvc_unref_device(self.dev.as_ptr()) };
    }
}

impl<'a> Device<'a> {
    pub(crate) unsafe fn from_raw(dev: *mut uvc_device) -> Self {
        Device {
            dev: NonNull::new(dev).unwrap(),
            _dev: PhantomData,
        }
    }
    /// Create handle to a device
    pub fn open(&'a self) -> Result<DeviceHandle<'a>> {
        unsafe {
            let mut devh = ::std::mem::uninitialized();
            let err = uvc_open(self.dev.as_ptr(), &mut devh).into();
            match err {
                Error::Success => Ok(DeviceHandle {
                    devh: NonNull::new(devh).unwrap(),
                    _devh: PhantomData,
                }),
                err => Err(err),
            }
        }
    }
    /// Get the description of a device
    pub fn description(&self) -> Result<DeviceDescription> {
        unsafe {
            let mut desc = ::std::mem::uninitialized();
            let err = uvc_get_device_descriptor(self.dev.as_ptr(), &mut desc).into();
            if err != Error::Success {
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

unsafe impl<'a> Send for DeviceHandle<'a> {}
unsafe impl<'a> Sync for DeviceHandle<'a> {}
#[derive(Debug)]
/// Open handle to a device
pub struct DeviceHandle<'a> {
    pub(crate) devh: NonNull<uvc_device_handle>,
    _devh: PhantomData<&'a uvc_device_handle>,
}

impl<'a> DeviceHandle<'a> {
    /// Creates a stream handle
    pub fn get_stream_ctrl_with_size_and_fps(
        &self,
        width: u32,
        height: u32,
        fps: u32,
    ) -> Result<streaming::StreamCtrl<'a>> {
        unsafe {
            let mut ctrl = ::std::mem::uninitialized();
            let err = uvc_get_stream_ctrl_format_size(
                self.devh.as_ptr(),
                &mut ctrl,
                uvc_frame_format_UVC_FRAME_FORMAT_YUYV,
                width as i32,
                height as i32,
                fps as i32,
            ).into();
            if err != Error::Success {
                Err(err)
            } else {
                Ok(::StreamCtrl {
                    ctrl,
                    _ctrl: PhantomData,
                })
            }
        }
    }
}

impl<'a> Drop for DeviceHandle<'a> {
    fn drop(&mut self) {
        unsafe {
            uvc_close(self.devh.as_ptr());
        }
    }
}

#[derive(Debug)]
/// Describes the device
pub struct DeviceDescription {
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_uvc: u16,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}
